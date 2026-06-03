use std::cmp::max;
use zhc_config::hpu::HpuConfig;
use zhc_ir::translation::{Order, translate};
use zhc_ir::{AnnIR, AnnOpRef, IR, OpId, OpIdRaw};
use zhc_langs::hpulang::HpuLang;
use zhc_utils::data_visulization::Histogram;
use zhc_utils::iter::{CollectInSmallVec, CollectInVec, MultiZip};
use zhc_utils::small::SmallVec;
use zhc_utils::{Dumpable, fsm};
use zhc_utils::{SafeAs, svec};

use super::utils::{Batch, Batches};

use super::SchedPolicy;

static TRACE_EXECUTION: bool = false;
static PBS_COST: OpIdRaw = 1000;
static NON_PBS_COST: OpIdRaw = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Criticallity {
    depth: OpIdRaw,
    height: OpIdRaw,
    slack: OpIdRaw,
}

type CritOpRef<'a, 'b> = AnnOpRef<'a, 'b, HpuLang, Criticallity, ()>;
type CritIR<'a> = AnnIR<'a, HpuLang, Criticallity, ()>;

fn analyze<'a>(ir: &'a IR<HpuLang>) -> CritIR<'a> {
    let a = ir.forward_dataflow_analysis(|opref| {
        let previous_depth: OpIdRaw = opref
            .get_predecessors_iter()
            .map(|p| p.get_annotation().clone().unwrap_analyzed())
            .max()
            .unwrap_or(0);
        if opref.get_instruction().is_pbs() {
            (
                previous_depth + PBS_COST,
                svec![(); opref.get_return_arity()],
            )
        } else {
            (
                previous_depth + NON_PBS_COST,
                svec![(); opref.get_return_arity()],
            )
        }
    });
    let critical_path_length = a
        .walk_ops_linear()
        .map(|op| *op.get_annotation())
        .max()
        .unwrap();
    a.backward_dataflow_analysis::<Criticallity, ()>(|opref, old_opref| {
        let depth = *old_opref.get_annotation();
        let previous_height = opref
            .get_users_iter()
            .map(|p| p.get_annotation().clone().unwrap_analyzed().height)
            .max()
            .unwrap_or(0);

        if opref.get_instruction().is_pbs() {
            let slack = critical_path_length + PBS_COST - depth - previous_height;
            (
                Criticallity {
                    depth,
                    height: previous_height + PBS_COST,
                    slack,
                },
                svec![(); opref.get_return_arity()],
            )
        } else {
            (
                Criticallity {
                    depth,
                    height: previous_height + NON_PBS_COST,
                    slack: critical_path_length + NON_PBS_COST - depth - previous_height,
                },
                svec![(); opref.get_return_arity()],
            )
        }
    })
}

impl Dumpable for Batch<CritOpRef<'_, '_>> {
    fn dump_to_string(&self) -> String {
        let mut result = format!("[{}/{}", self.ops.len(), self.cap);
        let mut slacks = self.ops.iter().map(|op| op.get_annotation().slack).cosvec();
        slacks.as_mut_slice().sort();
        for slack in slacks.into_iter() {
            result.push_str(&format!(" {}", slack));
        }
        result.push(']');
        result
    }
}

impl<'a, 'b> Batch<CritOpRef<'a, 'b>> {
    #[allow(unused)]
    pub fn slacks(&self) -> SmallVec<OpIdRaw> {
        self.ops.iter().map(|a| a.get_annotation().slack).collect()
    }

    pub fn min_slack(&self) -> OpIdRaw {
        self.ops
            .iter()
            .map(|a| a.get_annotation().slack)
            .min()
            .unwrap()
    }
}

impl Dumpable for Batches<CritOpRef<'_, '_>> {
    fn dump_to_string(&self) -> String {
        let mut result = String::new();
        for (i, batch) in self.0.iter().enumerate() {
            result.push_str(&format!("{}: {}\n", i + 1, batch.dump_to_string()));
        }
        result
    }
}

#[fsm]
#[derive(Debug)]
enum State {
    Scheduled,
    Ready,
    Waiting(usize),
}

impl Dumpable for State {
    fn dump_to_string(&self) -> String {
        format!("{:?}", self)
    }
}

fn extract_batches<'a, 'b>(
    dir: &'b CritIR<'a>,
    batch_size: usize,
    policy: SchedPolicy,
) -> (Batches<CritOpRef<'a, 'b>>, Vec<OpId>) {
    let mut batches = Batches::new();
    let mut batch: Batch<_>;

    let mut states = dir.totally_mapped_opmap(|op| {
        let count = match policy {
            SchedPolicy::AsSoonAsPossible => op.get_predecessors_iter().count(),
            SchedPolicy::AsLateAsPossible => op.get_users_iter().count(),
        };
        match count {
            0 => State::Ready,
            n => State::Waiting(n),
        }
    });

    let mut worklist: std::collections::VecDeque<OpId> = states
        .iter()
        .filter_map(|(opid, state)| match state {
            State::Ready => Some(opid),
            _ => None,
        })
        .collect();
    let mut ready_list = Vec::new();
    let mut order = Vec::new();

    loop {
        while !worklist.is_empty() {
            let op = dir.get_op(worklist.pop_front().unwrap());
            states.get_mut(&op).unwrap().transition(|old| match old {
                State::Ready => {
                    order.push(op.get_id());
                    State::Scheduled
                }
                _ => unreachable!(),
            });
            let neighbors: Vec<CritOpRef<'a, 'b>> = match policy {
                SchedPolicy::AsSoonAsPossible => op.get_users_iter().covec(),
                SchedPolicy::AsLateAsPossible => op.get_predecessors_iter().covec(),
            };
            for neighbor in neighbors {
                states
                    .get_mut(&neighbor)
                    .unwrap()
                    .transition(|old| match old {
                        State::Waiting(0) => unreachable!(),
                        State::Waiting(1) => {
                            if !neighbor.get_instruction().is_pbs() {
                                worklist.push_back(neighbor.get_id());
                            } else {
                                ready_list.push(neighbor);
                            }
                            State::Ready
                        }
                        State::Waiting(n) => State::Waiting(n - 1),
                        _ => unreachable!(),
                    });
            }
        }

        if ready_list.is_empty() {
            break;
        }

        batch = Batch::new(batch_size);
        if ready_list.len() < batch_size {
            ready_list.drain(..).for_each(|op| batch.push(op));
        } else {
            ready_list.reverse();
            ready_list.sort_by_key(|v| match policy {
                SchedPolicy::AsSoonAsPossible => v.get_annotation().height,
                SchedPolicy::AsLateAsPossible => v.get_annotation().depth,
            });
            ready_list.reverse();
            ready_list.drain(..batch_size).for_each(|op| batch.push(op));
        }

        worklist.extend(batch.iter_members().map(|op| op.get_id()));
        batches.push(batch.clone());
    }

    if policy == SchedPolicy::AsLateAsPossible {
        order.reverse();
    }

    (batches, order)
}

pub struct PbsStatistics {
    pub depth_distribution: Histogram<OpIdRaw>,
    pub height_distribution: Histogram<OpIdRaw>,
    pub slack_distribution: Histogram<OpIdRaw>,
    pub critical_path_length: OpIdRaw,
}

impl PbsStatistics {
    pub fn extract<'a>(ir: &CritIR<'a>) -> Self {
        let mut output = PbsStatistics {
            depth_distribution: Histogram::empty(),
            height_distribution: Histogram::empty(),
            slack_distribution: Histogram::empty(),
            critical_path_length: 0,
        };
        for op in ir
            .walk_ops_linear()
            .filter(|op| op.get_instruction().is_pbs())
        {
            let Criticallity {
                depth,
                height,
                slack,
                ..
            } = op.get_annotation();
            output.depth_distribution.count(depth);
            output.height_distribution.count(height);
            output.slack_distribution.count(slack);
            output.critical_path_length = max(output.critical_path_length, *depth);
        }
        output
    }
}

impl Dumpable for PbsStatistics {
    fn dump_to_string(&self) -> String {
        format!(
            "Depth:\n{}\nHeight:\n{}\nSlack:\n{}\nCritical Path Length: {}",
            self.depth_distribution,
            self.height_distribution,
            self.slack_distribution,
            self.critical_path_length
        )
    }
}

pub struct BatchingStatistics {
    pub size_distribution: Histogram<OpIdRaw>,
    pub min_slack_distribution: Histogram<OpIdRaw>,
}

impl BatchingStatistics {
    fn extract<'a, 'b>(batches: &Batches<CritOpRef<'a, 'b>>) -> Self {
        let mut output = BatchingStatistics {
            size_distribution: Histogram::empty(),
            min_slack_distribution: Histogram::empty(),
        };
        for batch in batches.batch_iter() {
            output
                .size_distribution
                .count(&(batch.len().sas::<OpIdRaw>()));
            output.min_slack_distribution.count(&batch.min_slack());
        }
        output
    }
}

impl Dumpable for BatchingStatistics {
    fn dump_to_string(&self) -> String {
        format!(
            "Sizes:\n{}\nMinSlack:\n{}",
            self.size_distribution, self.min_slack_distribution
        )
    }
}

pub fn batch<'a, 'b>(
    ir: &'a IR<HpuLang>,
    config: &'b HpuConfig,
    policy: SchedPolicy,
) -> IR<HpuLang> {
    let air = analyze(ir);
    if TRACE_EXECUTION {
        let pbs_stats = PbsStatistics::extract(&air);
        pbs_stats.dump();
    }
    let (batches, order) = extract_batches(&air, config.pbs_max_batch_size, policy);

    if TRACE_EXECUTION {
        let batching_stats = BatchingStatistics::extract(&batches);
        batching_stats.dump();
    }
    let batchmap = batches.into_batch_map();

    translate(ir, Order::Custom(order), move |opref, engine| {
        use zhc_langs::hpulang::HpuInstructionSet::*;
        match opref.get_instruction() {
            AddCt
            | SubCt
            | Mac { .. }
            | AddPt
            | SubPt
            | PtSub
            | MulPt
            | AddCst { .. }
            | SubCst { .. }
            | CstSub { .. }
            | MulCst { .. }
            | CstCt { .. }
            | ImmLd { .. }
            | DstSt { .. }
            | SrcLd { .. } => {
                let new_args = opref
                    .get_arg_valids()
                    .iter()
                    .map(|valid| engine.translate_val(*valid))
                    .cosvec();
                let new_rets = engine.add_op(opref.get_instruction(), new_args);
                (opref.get_return_valids().iter(), new_rets.into_iter())
                    .mzip()
                    .for_each(|(old, new)| engine.register_translation(*old, new));
            }
            Pbs { .. }
            | Pbs2 { .. }
            | Pbs4 { .. }
            | Pbs8 { .. }
            | PbsF { .. }
            | Pbs2F { .. }
            | Pbs4F { .. }
            | Pbs8F { .. } => {
                if engine.has_translation(opref.get_return_valids()[0]) {
                    return;
                }
                let batch = batchmap.get(&opref.get_id()).unwrap();
                let (batch_ir, inputs, outputs) = batch.gen_batch_ir();
                let block = Box::new(batch_ir);
                let new_args = inputs
                    .into_iter()
                    .map(|arg| engine.translate_val(arg.get_id()))
                    .collect();
                let new_rets = engine.add_op(Batch { block }, new_args);
                (outputs.into_iter(), new_rets.into_iter())
                    .mzip()
                    .for_each(|(old, new)| engine.register_translation(old.get_id(), new));
            }
            Batch { .. } | BatchArg { .. } | BatchRet { .. } => {
                panic!("Unexpected batch operations encountered.")
            }
            _ => unreachable!(),
        }
    })
    .output
}

#[cfg(test)]
mod test {
    use crate::{hpu::lowering::lower_iop_to_hpu, test::check_iop_hpu_equivalence};
    use zhc_builder::{
        Builder, CiphertextSpec, add, adds, bitwise_and, bitwise_or, bitwise_xor, div, if_then_else, if_then_zero, mul
    };
    use zhc_config::hpu::PhysicalConfig;
    use zhc_ir::IR;
    use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
    use zhc_utils::assert_display_is;

    use super::*;

    fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
        let ir = lower_iop_to_hpu(&ir).output;
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        batch(&ir, &config, SchedPolicy::AsSoonAsPossible)
    }

    #[test]
    fn test_batch_scheduler() {
        let ir = pipeline(&add(CiphertextSpec::new(16, 2, 2)).optimize_ir());
        assert_display_is!(
            ir.format().show_types(false),
            r#"
                %0 = src_ld<0.0_tsrc>();
                %1 = src_ld<0.1_tsrc>();
                %2 = src_ld<0.2_tsrc>();
                %3 = src_ld<0.3_tsrc>();
                %4 = src_ld<0.4_tsrc>();
                %5 = src_ld<0.5_tsrc>();
                %6 = src_ld<0.6_tsrc>();
                %7 = src_ld<0.7_tsrc>();
                %8 = src_ld<1.0_tsrc>();
                %9 = src_ld<1.1_tsrc>();
                %10 = src_ld<1.2_tsrc>();
                %11 = src_ld<1.3_tsrc>();
                %12 = src_ld<1.4_tsrc>();
                %13 = src_ld<1.5_tsrc>();
                %14 = src_ld<1.6_tsrc>();
                %15 = src_ld<1.7_tsrc>();
                %16 = add_ct(%0, %8);
                %17 = add_ct(%1, %9);
                %18 = add_ct(%2, %10);
                %19 = add_ct(%3, %11);
                %20 = add_ct(%4, %12);
                %21 = add_ct(%5, %13);
                %22 = add_ct(%6, %14);
                %23 = add_ct(%7, %15);
                %24, %25, %26, %27, %28, %29, %30, %31 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3 = batch_arg<3, CtRegister>();
                    %a4 = batch_arg<4, CtRegister>();
                    %a5 = batch_arg<5, CtRegister>();
                    %a6 = batch_arg<6, CtRegister>();
                    %a7, %a8 = pbs_2<Lut@26>(%a0);
                    %a9 = pbs<Lut@47>(%a1);
                    %a10 = pbs<Lut@48>(%a2);
                    %a11 = pbs<Lut@49>(%a3);
                    %a12 = pbs<Lut@47>(%a4);
                    %a13 = pbs<Lut@48>(%a5);
                    %a14 = pbs_f<Lut@49>(%a6);
                    batch_ret<0, CtRegister>(%a7);
                    batch_ret<1, CtRegister>(%a8);
                    batch_ret<2, CtRegister>(%a9);
                    batch_ret<3, CtRegister>(%a10);
                    batch_ret<4, CtRegister>(%a11);
                    batch_ret<5, CtRegister>(%a12);
                    batch_ret<6, CtRegister>(%a13);
                    batch_ret<7, CtRegister>(%a14);
                }(%16, %17, %18, %19, %20, %21, %22);
                %32 = add_ct(%17, %25);
                %33 = add_ct(%25, %26);
                %34 = add_ct(%29, %30);
                %35 = add_ct(%33, %27);
                %36 = add_ct(%34, %31);
                %37 = add_ct(%35, %28);
                %38, %39, %40, %41, %42 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3 = batch_arg<3, CtRegister>();
                    %a4 = batch_arg<4, CtRegister>();
                    %a5 = pbs<Lut@1>(%a0);
                    %a6 = pbs<Lut@1>(%a4);
                    %a7 = pbs<Lut@44>(%a1);
                    %a8 = pbs<Lut@45>(%a2);
                    %a9 = pbs_f<Lut@46>(%a3);
                    batch_ret<0, CtRegister>(%a9);
                    batch_ret<1, CtRegister>(%a7);
                    batch_ret<2, CtRegister>(%a8);
                    batch_ret<3, CtRegister>(%a5);
                    batch_ret<4, CtRegister>(%a6);
                }(%24, %33, %35, %37, %32);
                dst_st<0.0_tdst>(%41);
                dst_st<0.1_tdst>(%42);
                %43 = add_ct(%18, %39);
                %44 = add_ct(%19, %40);
                %45 = add_ct(%29, %38);
                %46 = add_ct(%34, %38);
                %47 = add_ct(%36, %38);
                %48 = add_ct(%20, %38);
                %49, %50, %51, %52, %53, %54 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3 = batch_arg<3, CtRegister>();
                    %a4 = batch_arg<4, CtRegister>();
                    %a5 = batch_arg<5, CtRegister>();
                    %a6 = pbs<Lut@1>(%a3);
                    %a7 = pbs<Lut@1>(%a4);
                    %a8 = pbs<Lut@44>(%a0);
                    %a9 = pbs<Lut@45>(%a1);
                    %a10 = pbs<Lut@46>(%a2);
                    %a11 = pbs_f<Lut@1>(%a5);
                    batch_ret<0, CtRegister>(%a8);
                    batch_ret<1, CtRegister>(%a9);
                    batch_ret<2, CtRegister>(%a10);
                    batch_ret<3, CtRegister>(%a6);
                    batch_ret<4, CtRegister>(%a7);
                    batch_ret<5, CtRegister>(%a11);
                }(%45, %46, %47, %43, %44, %48);
                dst_st<0.2_tdst>(%52);
                dst_st<0.3_tdst>(%53);
                %55 = add_ct(%21, %49);
                %56 = add_ct(%22, %50);
                %57 = add_ct(%23, %51);
                dst_st<0.4_tdst>(%54);
                %58, %59, %60 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3 = pbs<Lut@1>(%a0);
                    %a4 = pbs<Lut@1>(%a1);
                    %a5 = pbs_f<Lut@1>(%a2);
                    batch_ret<0, CtRegister>(%a3);
                    batch_ret<1, CtRegister>(%a4);
                    batch_ret<2, CtRegister>(%a5);
                }(%55, %56, %57);
                dst_st<0.5_tdst>(%58);
                dst_st<0.6_tdst>(%59);
                dst_st<0.7_tdst>(%60);
            "#
        )
    }

    #[test]
    fn correctness() {
        let check = |b: Builder| {
            let spec = *b.spec();
            let iop_ir = b.optimize_ir();
            let hpu_ir = pipeline(&iop_ir);
            check_iop_hpu_equivalence(&iop_ir, &hpu_ir, spec, 100);
        };
        for size in 2..=64 {
            let spec = CiphertextSpec::new(size, 2, 2);
            check(add(spec));
            check(adds(spec));
            check(bitwise_and(spec));
            check(bitwise_or(spec));
            check(bitwise_xor(spec));
            check(if_then_else(spec));
            check(if_then_zero(spec));
            check(mul(spec));
            check(div(spec));
        }
    }
}

// Notes
// =====
//
// [1]: The list scheduler can be used in both direction. There is no clear winner in terms of
// performances, hence, we highlight some important matters here, in case this needs more thinking.
// The key to understanding batching performance is to notice that inspite of the fact that they
// perform a very similar processing; they both greedily schedule batches of pbses; both scheduler
// work on different rankings.
//
// The forward approach batches by increasing Pbs Depth (starting from input), while the backward
// approach batches by increasing Pbs Height (starting from effect). Some circuits will have pretty
// symmetric Depth/Height ranking, other will not. This imbalance will greatly impacts how batching
// performs.
//
// Another point of importance is the priority scheme used to select the next element to be added.
// For now, we prioritise based on the opposite of the ranking naturally traversed by the scheduler.
// That is we prioritise deeper pbses when scheduling backward, and we prioritise higher pbses when
// scheduling forward. This gives better performances because essentially when traversing the
// operations in forward order, operations with bigger height will have more operations depending on
// it (it will be closer to the critical path). Hence, scheduling them as soon as possible will
// unlock more operations as we go through scheduling, and will prevent starving the scheduler.
//
// [2]:
// FIFO ordering is required for Backward: when batch members are enqueued together, FIFO
// ensures they are all dequeued (and added to `order`) before any of their predecessors,
// which are the batch inputs. LIFO would interleave predecessors between batch members,
// causing translate_val to be called on an untranslated value after the order is reversed.
