use std::fmt;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use zhc_ir::translation::lazy_translate;
use zhc_ir::{AnnIR, AnnOpRef, AnnValRef, IR, OpId};
use zhc_langs::hpulang::{BatchStatistics, HpuInstructionSet, HpuLang};
use zhc_sim::hpu::HpuConfig;
use zhc_utils::FastMap;
use zhc_utils::iter::{CollectInSmallVec, CollectInVec, DedupedByKey, MultiZip};
use zhc_utils::small::SmallMap;
use zhc_utils::{iter::AllEq, svec};

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

type PbsDepth = u16;
type DepthedOpRef<'a, 'b> = AnnOpRef<'a, 'b, HpuLang, PbsDepth, ()>;
type DepthedValRef<'a, 'b> = AnnValRef<'a, 'b, HpuLang, PbsDepth, ()>;
type DepthedIR<'a> = AnnIR<'a, HpuLang, PbsDepth, ()>;

fn analyze_pbs_depth<'a>(ir: &'a IR<HpuLang>) -> DepthedIR<'a> {
    ir.forward_dataflow_analysis(|opref| {
        use zhc_langs::hpulang::HpuInstructionSet::*;
        match opref.get_instruction() {
            Pbs { .. }
            | Pbs2 { .. }
            | Pbs4 { .. }
            | Pbs8 { .. }
            | PbsF { .. }
            | Pbs2F { .. }
            | Pbs4F { .. }
            | Pbs8F { .. } => {
                let depth = opref
                    .get_predecessors_iter()
                    .map(|p| p.get_annotation().clone().unwrap_analyzed())
                    .max()
                    .unwrap()
                    + 1_u16;
                (depth, svec![(); opref.get_return_arity()])
            }
            Batch { .. } | BatchArg { .. } | BatchRet { .. } => {
                panic!()
            }
            _ => {
                let depth = opref
                    .get_predecessors_iter()
                    .map(|p| p.get_annotation().clone().unwrap_analyzed())
                    .max()
                    .unwrap_or(0_u16);
                (depth, svec![(); opref.get_return_arity()])
            }
        }
    })
}

#[derive(Clone)]
struct Batch<'a, 'b> {
    ops: Vec<DepthedOpRef<'a, 'b>>,
    depth: PbsDepth,
}

impl fmt::Debug for Batch<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // e.g. [2/4 @3 @7]  — fill/capacity then opids
        write!(f, "[{}/{}", self.ops.len(), self.ops.capacity())?;
        for op in &self.ops {
            write!(f, " {}", op.get_id())?;
        }
        if !self.is_iso_depth() {
            write!(f, " ⚠")?;
        }
        write!(f, "]")
    }
}

impl<'a, 'b> Batch<'a, 'b> {
    pub fn new(batch_size: usize, first: DepthedOpRef<'a, 'b>) -> Self {
        let mut output = Batch {
            ops: Vec::with_capacity(batch_size),
            depth: *first.get_annotation(),
        };
        output.try_push(first).unwrap();
        output
    }

    pub fn is_full(&self) -> bool {
        self.ops.len() == self.ops.capacity()
    }

    pub fn try_push(&mut self, op: DepthedOpRef<'a, 'b>) -> Result<(), DepthedOpRef<'a, 'b>> {
        if self.is_full() {
            panic!()
        }
        if self.may_receive(&op) {
            self.ops.push(op);
            Ok(())
        } else {
            Err(op)
        }
    }

    pub fn len(&self) -> usize {
        self.ops.len()
    }

    #[allow(unused)]
    pub fn batch_size(&self) -> usize {
        self.ops.capacity()
    }

    pub fn iter_members(&self) -> impl Iterator<Item = DepthedOpRef<'a, 'b>> {
        self.ops.iter().cloned()
    }

    pub fn is_iso_depth(&self) -> bool {
        self.iter_members()
            .map(|m| *m.get_annotation())
            .all_eq()
            .unwrap_or(true)
    }

    fn may_receive(&self, candidate: &DepthedOpRef<'a, 'b>) -> bool {
        let candidate_depth = *candidate.get_annotation();
        if candidate_depth > self.depth {
            // The candidate is after the batch. Not possible.
            return false;
        } else if candidate_depth == self.depth && self.is_iso_depth() {
            // The candidate is at the same as all existing members. Can not interfere.
            return true;
        } else {
            // The sad-path, we have to check with reachability analysis. See [1].
            self.iter_members().all(|m| !candidate.reaches(&m))
        }
    }

    pub fn gen_batch_ir(
        &self,
    ) -> (
        IR<HpuLang>,
        Vec<DepthedValRef<'a, 'b>>,
        Vec<DepthedValRef<'a, 'b>>,
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
struct DepthBatches<'a, 'b> {
    batches: Vec<Batch<'a, 'b>>,
    batch_size: usize,
}

impl fmt::Debug for DepthBatches<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Skip empty depth levels entirely
        if self.batches.is_empty() {
            return write!(f, "∅");
        }
        f.debug_list().entries(self.batches.iter()).finish()
    }
}

impl<'a, 'b> DepthBatches<'a, 'b> {
    pub fn new(batch_size: usize) -> Self {
        DepthBatches {
            batches: vec![],
            batch_size,
        }
    }

    pub fn try_push(&mut self, op: DepthedOpRef<'a, 'b>) -> Result<(), DepthedOpRef<'a, 'b>> {
        if self.batches.is_empty() || self.batches.last().unwrap().is_full() {
            self.batches.push(Batch::new(self.batch_size, op));
            Ok(())
        } else {
            self.batches.last_mut().unwrap().try_push(op)
        }
    }

    #[allow(unused)]
    pub fn needs_filling(&self) -> bool {
        self.batches.last().map(|l| !l.is_full()).unwrap_or(false)
    }

    pub fn into_batch_iter(self) -> impl Iterator<Item = Batch<'a, 'b>> {
        self.batches.into_iter()
    }
}

#[derive(Clone)]
struct Batches<'a, 'b>(Vec<DepthBatches<'a, 'b>>);

impl fmt::Debug for Batches<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // One line per non-empty depth: "d1: [2/4 @3 @7] [4/4 @1 @2 @5 @9]"
        for (i, db) in self.0.iter().enumerate() {
            write!(f, "d{}: {:?}\n", i + 1, db)?;
        }
        Ok(())
    }
}

impl<'a, 'b> Batches<'a, 'b> {
    pub fn new(batch_size: usize, pbs_depth: usize) -> Self {
        Batches(vec![DepthBatches::new(batch_size); pbs_depth + 1]) // +1 adds an extra depth batch but makes schedule loop simpler.
    }

    #[allow(unused)]
    pub fn statistics(&self) -> BatchStatistics {
        let mut stats = BatchStatistics::new();
        for db in &self.0 {
            for batch in &db.batches {
                stats.record(batch.ops.len() as u16);
            }
        }
        stats
    }

    fn into_batch_iter(self) -> impl Iterator<Item = Batch<'a, 'b>> {
        self.0
            .into_iter()
            .flat_map(|dbatches| dbatches.into_batch_iter())
    }

    pub fn into_batch_map(self) -> FastMap<OpId, Rc<Batch<'a, 'b>>> {
        self.into_batch_iter()
            .map(Rc::new)
            .flat_map(|batch| (0..batch.len()).map(move |i| (batch.ops[i].get_id(), batch.clone())))
            .collect()
    }
}

impl<'a, 'b> Index<PbsDepth> for Batches<'a, 'b> {
    type Output = DepthBatches<'a, 'b>;

    fn index(&self, index: PbsDepth) -> &Self::Output {
        &self.0[(index as usize).strict_sub(1)] // Depth indexing starts from 1
    }
}

impl<'a, 'b> IndexMut<PbsDepth> for Batches<'a, 'b> {
    fn index_mut(&mut self, index: PbsDepth) -> &mut Self::Output {
        &mut self.0[(index as usize).strict_sub(1)] // Depth indexing starts from 1
    }
}

fn extract_batches<'a, 'b>(dir: &'b DepthedIR<'a>, batch_size: usize) -> Batches<'a, 'b> {
    let max_depth = dir
        .walk_ops_linear()
        .map(|op| op.get_annotation().clone())
        .max()
        .unwrap();

    let mut batches = Batches::new(batch_size, max_depth as usize);
    for op in dir
        .walk_ops_topological()
        .rev()
        .filter(|op| op.get_instruction().is_pbs())
    {
        let depth = *op.get_annotation();
        batches[depth].try_push(op).unwrap();
    }
    batches
}

pub fn batch_schedule<'a, 'b>(ir: &'a IR<HpuLang>, config: &'b HpuConfig) -> IR<HpuLang> {
    let dir = analyze_pbs_depth(ir);
    let batches = extract_batches(&dir, config.pbs_max_batch_size);
    #[cfg(debug_assertions)]
    {
        eprintln!("{}", batches.statistics());
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
            | SrcLd { .. }
            | TransferIn { .. }
            | TransferOut { .. }
            => {
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
        Builder, CiphertextSpec, add, bitwise_and, bitwise_or, bitwise_xor, count_0, if_then_else,
        if_then_zero, mul_lsb,
    };
    use zhc_ir::IR;
    use zhc_langs::{hpulang::HpuLang, ioplang::IopLang};
    use zhc_sim::hpu::{HpuConfig, PhysicalConfig};
    use zhc_utils::assert_display_is;

    use crate::{
        batch_scheduler::batch_schedule, test::check_iop_hpu_equivalence,
        translation::lower_iop_to_hpu,
    };

    fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
        let ir = lower_iop_to_hpu(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        batch_schedule(&ir, &config)
    }

    #[test]
    fn test_broken() {
        let ir = pipeline(&count_0(CiphertextSpec::new(16, 2, 2)).into_ir());
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
                %8, %9, %10, %11, %12, %13, %14, %15, %16, %17, %18, %19, %20, %21, %22, %23 = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = batch_arg<3, CtRegister>();
                    %4 : CtRegister = batch_arg<4, CtRegister>();
                    %5 : CtRegister = batch_arg<5, CtRegister>();
                    %6 : CtRegister = batch_arg<6, CtRegister>();
                    %7 : CtRegister = batch_arg<7, CtRegister>();
                    %8 : CtRegister, %9 : CtRegister = pbs_2<Lut@71>(%7 : CtRegister);
                    %10 : CtRegister, %11 : CtRegister = pbs_2<Lut@71>(%6 : CtRegister);
                    %12 : CtRegister, %13 : CtRegister = pbs_2<Lut@71>(%5 : CtRegister);
                    %14 : CtRegister, %15 : CtRegister = pbs_2<Lut@71>(%4 : CtRegister);
                    %16 : CtRegister, %17 : CtRegister = pbs_2<Lut@71>(%3 : CtRegister);
                    %18 : CtRegister, %19 : CtRegister = pbs_2<Lut@71>(%2 : CtRegister);
                    %20 : CtRegister, %21 : CtRegister = pbs_2<Lut@71>(%1 : CtRegister);
                    %22 : CtRegister, %23 : CtRegister = pbs_2f<Lut@71>(%0 : CtRegister);
                    batch_ret<0, CtRegister>(%22 : CtRegister);
                    batch_ret<1, CtRegister>(%23 : CtRegister);
                    batch_ret<2, CtRegister>(%20 : CtRegister);
                    batch_ret<3, CtRegister>(%21 : CtRegister);
                    batch_ret<4, CtRegister>(%18 : CtRegister);
                    batch_ret<5, CtRegister>(%19 : CtRegister);
                    batch_ret<6, CtRegister>(%16 : CtRegister);
                    batch_ret<7, CtRegister>(%17 : CtRegister);
                    batch_ret<8, CtRegister>(%14 : CtRegister);
                    batch_ret<9, CtRegister>(%15 : CtRegister);
                    batch_ret<10, CtRegister>(%12 : CtRegister);
                    batch_ret<11, CtRegister>(%13 : CtRegister);
                    batch_ret<12, CtRegister>(%10 : CtRegister);
                    batch_ret<13, CtRegister>(%11 : CtRegister);
                    batch_ret<14, CtRegister>(%8 : CtRegister);
                    batch_ret<15, CtRegister>(%9 : CtRegister);
                }(%0, %1, %2, %3, %4, %5, %6, %7);
                %24 = add_ct(%22, %23);
                %25 = add_ct(%8, %9);
                %31 = add_ct(%15, %16);
                %26 = add_ct(%25, %10);
                %32 = add_ct(%31, %17);
                %27 = add_ct(%26, %11);
                %33 = add_ct(%32, %18);
                %28 = add_ct(%27, %12);
                %34 = add_ct(%33, %19);
                %29 = add_ct(%28, %13);
                %35 = add_ct(%34, %20);
                %30 = add_ct(%29, %14);
                %36 = add_ct(%35, %21);
                %37, %38, %39, %40, %41, %42 = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister, %4 : CtRegister = pbs_2<Lut@70>(%2 : CtRegister);
                    %5 : CtRegister, %6 : CtRegister = pbs_2<Lut@70>(%1 : CtRegister);
                    %7 : CtRegister, %8 : CtRegister = pbs_2f<Lut@65>(%0 : CtRegister);
                    batch_ret<0, CtRegister>(%7 : CtRegister);
                    batch_ret<1, CtRegister>(%8 : CtRegister);
                    batch_ret<2, CtRegister>(%5 : CtRegister);
                    batch_ret<3, CtRegister>(%6 : CtRegister);
                    batch_ret<4, CtRegister>(%3 : CtRegister);
                    batch_ret<5, CtRegister>(%4 : CtRegister);
                }(%24, %30, %36);
                %43 = add_ct(%39, %41);
                %45 = add_ct(%40, %42);
                %44 = add_ct(%43, %37);
                %46 = add_ct(%45, %38);
                %47, %48, %49, %50 = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister, %3 : CtRegister = pbs_2<Lut@26>(%1 : CtRegister);
                    %4 : CtRegister = pbs<Lut@3>(%0 : CtRegister);
                    %5 : CtRegister = pbs_f<Lut@1>(%0 : CtRegister);
                    batch_ret<0, CtRegister>(%5 : CtRegister);
                    batch_ret<1, CtRegister>(%4 : CtRegister);
                    batch_ret<2, CtRegister>(%2 : CtRegister);
                    batch_ret<3, CtRegister>(%3 : CtRegister);
                }(%44, %46);
                dst_st<0.0_tdst>(%47);
                %51 = add_ct(%48, %49);
                %52, %53 = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister, %2 : CtRegister = pbs_2f<Lut@26>(%0 : CtRegister);
                    batch_ret<0, CtRegister>(%1 : CtRegister);
                    batch_ret<1, CtRegister>(%2 : CtRegister);
                }(%51);
                dst_st<0.1_tdst>(%52);
                %54 = add_ct(%53, %50);
                %55, %56 = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister, %2 : CtRegister = pbs_2f<Lut@26>(%0 : CtRegister);
                    batch_ret<0, CtRegister>(%1 : CtRegister);
                    batch_ret<1, CtRegister>(%2 : CtRegister);
                }(%54);
                dst_st<0.2_tdst>(%55);
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
// [1]: The reachability analysis is less costly than it might appear at first. Recall that the IR holds its own Depth
// metric (largest distance to an input), which is equivalent in spirit to the PbsDepth computed
// here but accounts for every kind of operation along the paths, while the PbsDepth analysis only
// accounts for PBS operations. Given how these metrics are computed, we can assume that, given a
// candidate and a batch member, if PbsDepth(candidate) <= PbsDepth(member) then Depth(candidate) <=
// Depth(member). By default, the reachability analysis recursively exhausts the reached nodes,
// checking for equality of the reached node's opid with the queried node's. Fortunately, a
// depth-based cut-off is used to discard portions of the search space that we know can't contain
// the queried node. Initially, Depth(candidate) <= Depth(member), which means the opid will be
// checked. However, as the analysis recursively searches deeper in the IR,
// Depth(candidate_reachable_node) eventually becomes > Depth(member), at which point the
// search is cut off.
