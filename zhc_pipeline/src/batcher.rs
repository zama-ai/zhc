use std::cmp::max;
use std::rc::Rc;

use zhc_ir::translation::lazy_translate;
use zhc_ir::{AnnIR, AnnOpRef, AnnValRef, IR, OpId};
use zhc_langs::hpulang::{HpuInstructionSet, HpuLang};
use zhc_sim::hpu::HpuConfig;
use zhc_utils::data_visulization::Histogram;
use zhc_utils::iter::{CollectInSmallVec, CollectInVec, DedupedByKey, MultiZip};
use zhc_utils::small::{SmallMap, SmallVec};
use zhc_utils::{Dumpable, FastMap, fsm};
use zhc_utils::{SafeAs, svec};

static TRACE_EXECUTION: bool = false;

fn flush_pbs(instruction: HpuInstructionSet) -> HpuInstructionSet {
    match instruction {
        HpuInstructionSet::Pbs { lut } | HpuInstructionSet::PbsF { lut } => {
            HpuInstructionSet::PbsF { lut }
        }
        HpuInstructionSet::Pbs2 { lut } | HpuInstructionSet::Pbs2F { lut } => {
            HpuInstructionSet::Pbs2F { lut }
        }
        HpuInstructionSet::Pbs4 { lut } | HpuInstructionSet::Pbs4F { lut } => {
            HpuInstructionSet::Pbs4F { lut }
        }
        HpuInstructionSet::Pbs8 { lut } | HpuInstructionSet::Pbs8F { lut } => {
            HpuInstructionSet::Pbs8F { lut }
        }
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Criticallity {
    depth: u16,
    height: u16,
    slack: u16,
}

type CritOpRef<'a, 'b> = AnnOpRef<'a, 'b, HpuLang, Criticallity, ()>;
type CritValRef<'a, 'b> = AnnValRef<'a, 'b, HpuLang, Criticallity, ()>;
type CritIR<'a> = AnnIR<'a, HpuLang, Criticallity, ()>;

fn analyze<'a>(ir: &'a IR<HpuLang>) -> CritIR<'a> {
    let a = ir.forward_dataflow_analysis(|opref| {
        let previous_depth = opref
            .get_predecessors_iter()
            .map(|p| p.get_annotation().clone().unwrap_analyzed())
            .max()
            .unwrap_or(0);
        if opref.get_instruction().is_pbs() {
            (previous_depth + 1000, svec![(); opref.get_return_arity()])
        } else {
            (previous_depth + 1, svec![(); opref.get_return_arity()])
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
            (
                Criticallity {
                    depth,
                    height: previous_height + 1000,
                    slack: critical_path_length - depth - previous_height + 1,
                },
                svec![(); opref.get_return_arity()],
            )
        } else {
            (
                Criticallity {
                    depth,
                    height: previous_height + 1,
                    slack: critical_path_length - depth - previous_height,
                },
                svec![(); opref.get_return_arity()],
            )
        }
    })
}

#[derive(Clone)]
struct Batch<'a, 'b> {
    ops: Vec<CritOpRef<'a, 'b>>,
    cap: usize,
}

impl Dumpable for Batch<'_, '_> {
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

impl<'a, 'b> Batch<'a, 'b> {
    pub fn new(batch_size: usize) -> Self {
        let output = Batch {
            ops: Vec::with_capacity(batch_size),
            cap: batch_size,
        };
        output
    }

    pub fn is_full(&self) -> bool {
        self.ops.len() == self.cap
    }

    pub fn push(&mut self, op: CritOpRef<'a, 'b>) {
        assert!(op.get_instruction().is_pbs());
        if self.is_full() {
            panic!()
        }
        self.ops.push(op);
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }

    #[allow(unused)]
    pub fn slacks(&self) -> SmallVec<u16> {
        self.ops.iter().map(|a| a.get_annotation().slack).collect()
    }

    pub fn min_slack(&self) -> u16 {
        self.ops
            .iter()
            .map(|a| a.get_annotation().slack)
            .min()
            .unwrap()
    }

    pub fn iter_members(&self) -> impl Iterator<Item = CritOpRef<'a, 'b>> {
        self.ops.iter().cloned()
    }

    pub fn gen_batch_ir(
        &self,
    ) -> (
        IR<HpuLang>,
        Vec<CritValRef<'a, 'b>>,
        Vec<CritValRef<'a, 'b>>,
    ) {
        // We collect the inputs and outputs of the batch.
        let mut inputs = self
            .ops
            .iter()
            .map(|op| op.get_args_iter())
            .flatten()
            .filter(|arg| {
                // To be a batch input, an op arg origin must not be in the batch.
                !self.ops.as_slice().contains(&arg.get_origin().opref)
            })
            .dedup_by_key(|op| op.get_id())
            .covec();
        inputs.sort_unstable_by_key(|a| a.get_id());
        let mut outputs = self
            .ops
            .iter()
            .map(|op| op.get_returns_iter())
            .flatten()
            .filter(|arg| {
                // To be a batch ouptut, a value must be produced by an operation that has users,
                // and which have at least one user outside of the batch.
                arg.get_origin()
                    .opref
                    .get_users_iter()
                    .any(|user| !self.ops.as_slice().contains(&user))
            })
            .dedup_by_key(|op| op.get_id())
            .covec();
        outputs.sort_unstable_by_key(|a| a.get_id());

        // Now we write the batch IR
        let mut batch = IR::empty();
        let mut batch_map = SmallMap::new();
        for (i, val) in inputs.iter().enumerate() {
            let (_, batch_arg) = batch.add_op(
                HpuInstructionSet::BatchArg {
                    pos: i.try_into().unwrap(),
                    ty: val.get_type(),
                },
                svec![],
            );
            batch_map.insert(val.get_id(), batch_arg[0]);
        }
        for (idx, op) in self.ops.iter().enumerate() {
            let instr = if idx == self.ops.len() - 1 {
                // Ensures the last is a flush...
                flush_pbs(op.get_instruction())
            } else {
                op.get_instruction()
            };
            let (_, batch_op_rets) = batch.add_op(
                instr,
                op.get_arg_valids()
                    .iter()
                    .map(|k| batch_map.get(k).unwrap())
                    .copied()
                    .collect(),
            );
            for (k, v) in (op.get_return_valids().iter(), batch_op_rets.into_iter()).mzip() {
                batch_map.insert(*k, v);
            }
        }
        for (i, val) in outputs.iter().enumerate() {
            batch.add_op(
                HpuInstructionSet::BatchRet {
                    pos: i.try_into().unwrap(),
                    ty: val.get_type(),
                },
                svec![*batch_map.get(&val.get_id()).unwrap()],
            );
        }

        (batch, inputs, outputs)
    }
}

#[derive(Clone)]
struct Batches<'a, 'b>(Vec<Batch<'a, 'b>>);

impl Dumpable for Batches<'_, '_> {
    fn dump_to_string(&self) -> String {
        let mut result = String::new();
        for (i, batch) in self.0.iter().enumerate() {
            result.push_str(&format!("{}: {}\n", i + 1, batch.dump_to_string()));
        }
        result
    }
}

impl<'a, 'b> Batches<'a, 'b> {
    pub fn new() -> Self {
        Batches(Vec::new())
    }

    pub fn push(&mut self, batch: Batch<'a, 'b>) {
        self.0.push(batch);
    }

    fn into_batch_iter(self) -> impl Iterator<Item = Batch<'a, 'b>> {
        self.0.into_iter()
    }

    fn batch_iter(&self) -> impl Iterator<Item = &Batch<'a, 'b>> {
        self.0.iter()
    }

    pub fn into_batch_map(self) -> FastMap<OpId, Rc<Batch<'a, 'b>>> {
        self.into_batch_iter()
            .map(Rc::new)
            .flat_map(|batch| (0..batch.len()).map(move |i| (batch.ops[i].get_id(), batch.clone())))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
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

fn forward_extract_batches<'a, 'b>(dir: &'b CritIR<'a>, batch_size: usize) -> Batches<'a, 'b> {
    // When MH, we can:
    //  + start by marking the transfer_in as not ready,
    //  + schedule as much as possible until stalled,
    //  + activate the transfer_in and

    let mut batches = Batches::new();
    let mut batch = Batch::new(batch_size);

    let mut states = dir.totally_mapped_opmap(|op| match op.get_predecessors_iter().count() {
        0 => State::Ready,
        n => State::Waiting(n),
    });

    let mut worklist = states
        .iter()
        .filter_map(|(opid, state)| match state {
            State::Ready => Some(opid),
            _ => None,
        })
        .covec();

    let mut ready_list = Vec::new();

    loop {
        while !worklist.is_empty() {
            let op = dir.get_op(worklist.pop().unwrap());
            states.get_mut(&op).unwrap().transition(|old| match old {
                State::Ready => State::Scheduled,
                _ => unreachable!(),
            });
            for user in op.get_users_iter() {
                states.get_mut(&user).unwrap().transition(|old| match old {
                    State::Waiting(0) => {
                        unreachable!()
                    }
                    State::Waiting(1) => {
                        if !user.get_instruction().is_pbs() {
                            worklist.push(user.get_id());
                        } else {
                            ready_list.push(user);
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

        while !batch.is_full() {
            // ready_list.shuffle(&mut rand::rng());
            ready_list.sort_by_key(|v| v.get_annotation().height);
            match ready_list.pop() {
                Some(v) => batch.push(v),
                None => break,
            }
        }

        // Batch is either full or there is not enough ready to schedule.
        worklist.extend(batch.iter_members().map(|op| op.get_id()));
        batches.push(batch.clone());
        batch = Batch::new(batch_size);
    }

    batches
}

fn backward_extract_batches<'a, 'b>(dir: &'b CritIR<'a>, batch_size: usize) -> Batches<'a, 'b> {
    let mut batches = Batches::new();
    let mut batch = Batch::new(batch_size);

    let mut states = dir.totally_mapped_opmap(|op| match op.get_users_iter().count() {
        0 => State::Ready,
        n => State::Waiting(n),
    });

    let mut worklist = states
        .iter()
        .filter_map(|(opid, state)| match state {
            State::Ready => Some(opid),
            _ => None,
        })
        .covec();

    let mut ready_list = Vec::new();

    loop {
        while !worklist.is_empty() {
            let op = dir.get_op(worklist.pop().unwrap());
            states.get_mut(&op).unwrap().transition(|old| match old {
                State::Ready => State::Scheduled,
                _ => unreachable!(),
            });
            for pred in op.get_predecessors_iter() {
                states.get_mut(&pred).unwrap().transition(|old| match old {
                    State::Waiting(0) => {
                        unreachable!()
                    }
                    State::Waiting(1) => {
                        if !pred.get_instruction().is_pbs() {
                            worklist.push(pred.get_id());
                        } else {
                            ready_list.push(pred);
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

        while !batch.is_full() {
            // ready_list.shuffle(&mut rand::rng());
            ready_list.sort_by_key(|v| v.get_annotation().depth);
            match ready_list.pop() {
                Some(op) => batch.push(op),
                None => break,
            }
        }

        worklist.extend(batch.iter_members().map(|op| op.get_id()));
        batches.push(batch.clone());
        batch = Batch::new(batch_size);
    }

    batches
}

pub struct PbsStatistics {
    pub depth_distribution: Histogram<u16>,
    pub height_distribution: Histogram<u16>,
    pub slack_distribution: Histogram<u16>,
    pub critical_path_length: u16,
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
    pub size_distribution: Histogram<u16>,
    pub min_slack_distribution: Histogram<u16>,
}

impl BatchingStatistics {
    fn extract<'a, 'b>(batches: &Batches<'a, 'b>) -> Self {
        let mut output = BatchingStatistics {
            size_distribution: Histogram::empty(),
            min_slack_distribution: Histogram::empty(),
        };
        for batch in batches.batch_iter() {
            output.size_distribution.count(&(batch.len().sas::<u16>()));
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

pub struct BatchStatistics {
    pub slacks: SmallVec<u16>,
}

pub struct Statistics {
    pub pbs: PbsStatistics,
    pub batching: BatchingStatistics,
}

pub fn batch<'a, 'b>(ir: &'a IR<HpuLang>, config: &'b HpuConfig) -> IR<HpuLang> {
    let air = analyze(ir);
    if TRACE_EXECUTION {
        let pbs_stats = PbsStatistics::extract(&air);
        pbs_stats.dump();
    }
    let forward_batches = forward_extract_batches(&air, config.pbs_max_batch_size);
    let backward_batches = backward_extract_batches(&air, config.pbs_max_batch_size);
    let batches = [forward_batches, backward_batches]
        .into_iter()
        .min_by_key(|batch| batch.len())
        .unwrap();
    if TRACE_EXECUTION {
        let batching_stats = BatchingStatistics::extract(&batches);
        batching_stats.dump();
    }
    let batchmap = batches.into_batch_map();
    let ir = lazy_translate(ir, move |opref, engine| {
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
                    .get_args_iter()
                    .map(|valref| engine.translate_val(valref))
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
                let batch = batchmap.get(&*opref).unwrap();
                let (batch_ir, inputs, outputs) = batch.gen_batch_ir();
                let block = Box::new(batch_ir);
                let new_args = inputs
                    .into_iter()
                    .map(|arg| engine.translate_val((*arg).clone()))
                    .collect();
                let new_rets = engine.add_op(Batch { block }, new_args);
                (outputs.into_iter(), new_rets.into_iter())
                    .mzip()
                    .for_each(|(old, new)| engine.register_translation(old.get_id(), new));
            }
            Batch { .. } | BatchArg { .. } | BatchRet { .. } => {
                panic!("Unexpected batch operations encountered.")
            }
        }
    });
    ir
}

#[cfg(test)]
mod test {
    use zhc_builder::{
        Builder, CiphertextSpec, add, bitwise_and, bitwise_or, bitwise_xor, if_then_else,
        if_then_zero, mul_lsb,
    };
    use zhc_ir::IR;
    use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
    use zhc_sim::hpu::{HpuConfig, PhysicalConfig};
    use zhc_utils::assert_display_is;

    use crate::{batcher::batch, test::check_iop_hpu_equivalence, translation::lower_iop_to_hpu};

    fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
        let ir = lower_iop_to_hpu(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        batch(&ir, &config)
    }

    #[test]
    fn test_batch_scheduler() {
        let ir = pipeline(&add(CiphertextSpec::new(16, 2, 2)).into_ir());
        assert_display_is!(
            ir.format().show_types(false),
            r#"
                %0 = src_ld<0.0_tsrc>();
                %1 = src_ld<1.0_tsrc>();
                %2 = add_ct(%0, %1);
                %3 = src_ld<0.1_tsrc>();
                %4 = src_ld<1.1_tsrc>();
                %5 = add_ct(%3, %4);
                %6 = src_ld<0.2_tsrc>();
                %7 = src_ld<1.2_tsrc>();
                %8 = add_ct(%6, %7);
                %9 = src_ld<0.3_tsrc>();
                %10 = src_ld<1.3_tsrc>();
                %11 = add_ct(%9, %10);
                %12 = src_ld<0.4_tsrc>();
                %13 = src_ld<1.4_tsrc>();
                %14 = add_ct(%12, %13);
                %15 = src_ld<0.5_tsrc>();
                %16 = src_ld<1.5_tsrc>();
                %17 = add_ct(%15, %16);
                %18 = src_ld<0.6_tsrc>();
                %19 = src_ld<1.6_tsrc>();
                %20 = add_ct(%18, %19);
                %21, %22, %23, %24, %25, %26, %27, %28 = batch {
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
                }(%2, %5, %8, %11, %14, %17, %20);
                %29 = add_ct(%22, %23);
                %30 = add_ct(%29, %24);
                %31 = add_ct(%30, %25);
                %32 = add_ct(%5, %22);
                %33, %34, %35, %36, %37 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3 = batch_arg<3, CtRegister>();
                    %a4 = batch_arg<4, CtRegister>();
                    %a5 = pbs<Lut@46>(%a3);
                    %a6 = pbs<Lut@45>(%a2);
                    %a7 = pbs<Lut@44>(%a1);
                    %a8 = pbs<Lut@1>(%a4);
                    %a9 = pbs_f<Lut@1>(%a0);
                    batch_ret<0, CtRegister>(%a5);
                    batch_ret<1, CtRegister>(%a7);
                    batch_ret<2, CtRegister>(%a6);
                    batch_ret<3, CtRegister>(%a9);
                    batch_ret<4, CtRegister>(%a8);
                }(%21, %29, %30, %31, %32);
                dst_st<0.0_tdst>(%36);
                dst_st<0.1_tdst>(%37);
                %38 = add_ct(%26, %33);
                %39 = add_ct(%26, %27);
                %40 = add_ct(%39, %33);
                %41 = add_ct(%39, %28);
                %42 = add_ct(%41, %33);
                %43 = add_ct(%8, %34);
                %44 = add_ct(%11, %35);
                %45 = add_ct(%14, %33);
                %46, %47, %48, %49, %50, %51 = batch {
                    %a0 = batch_arg<0, CtRegister>();
                    %a1 = batch_arg<1, CtRegister>();
                    %a2 = batch_arg<2, CtRegister>();
                    %a3 = batch_arg<3, CtRegister>();
                    %a4 = batch_arg<4, CtRegister>();
                    %a5 = batch_arg<5, CtRegister>();
                    %a6 = pbs<Lut@44>(%a0);
                    %a7 = pbs<Lut@45>(%a1);
                    %a8 = pbs<Lut@46>(%a2);
                    %a9 = pbs<Lut@1>(%a5);
                    %a10 = pbs<Lut@1>(%a4);
                    %a11 = pbs_f<Lut@1>(%a3);
                    batch_ret<0, CtRegister>(%a6);
                    batch_ret<1, CtRegister>(%a7);
                    batch_ret<2, CtRegister>(%a8);
                    batch_ret<3, CtRegister>(%a11);
                    batch_ret<4, CtRegister>(%a10);
                    batch_ret<5, CtRegister>(%a9);
                }(%38, %40, %42, %43, %44, %45);
                dst_st<0.2_tdst>(%49);
                dst_st<0.3_tdst>(%50);
                dst_st<0.4_tdst>(%51);
                %52 = add_ct(%17, %46);
                %53 = add_ct(%20, %47);
                %54 = src_ld<0.7_tsrc>();
                %55 = src_ld<1.7_tsrc>();
                %56 = add_ct(%54, %55);
                %57 = add_ct(%56, %48);
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
                }(%52, %53, %57);
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
            let iop_ir = b.into_ir();
            let hpu_ir = pipeline(&iop_ir);
            check_iop_hpu_equivalence(&iop_ir, &hpu_ir, spec, 100);
        };
        for size in 2..=64 {
            let spec = CiphertextSpec::new(size, 2, 2);
            check(add(spec));
            check(bitwise_and(spec));
            check(bitwise_or(spec));
            check(bitwise_xor(spec));
            check(if_then_else(spec));
            check(if_then_zero(spec));
            check(mul_lsb(spec));
        }
    }
}

// Notes
// =====
//
// [1]: We implemented both a forward and a backward list-scheduling algorithm. There is no clear winner in terms of
// performances, hence, we highlight some important matters here, in case this needs more thinking.
// The key to understanding batching performance is to notice that inspite of the fact that they
// perform a very similar processing; they both greedily schedule batches of pbses; both scheduler
// work on different rankings.
//
// The forward approach batches by increasing Pbs Depth (starting from input), while the backward
// approach batches by increasing Pbs Height (starting from effect). Some circuits will have pretty
// symmetric Depth/Height ranking, other will not. This imblance will greatly impacts how batching
// performs.
//
// Another point of importance is the priority scheme used to select the next element to be added.
// For now, we prioritise based on the opposite of the ranking naturally traversed by the scheduler.
// That is we prioritise deeper pbses when scheduling backward, and we prioritise higher pbses when
// scheduling forward. This gives better performances because essentially when traversing the
// operations in forward order, operations with bigger height will have more operations depending on
// it (it will be closer to the critical path). Hence, scheduling them as soon as possible will
// unlock more operations as we go through scheduling, and will prevent starving the scheduler.
