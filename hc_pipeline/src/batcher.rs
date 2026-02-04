use hc_ir::{IR, OpRef, ValId, ValMap};
use hc_langs::hpulang::{HpuInstructionSet, HpuLang};
use hc_utils::{
    iter::MultiZip,
    small::{SmallMap, SmallSet, SmallVec},
    svec,
};

struct Batcher<'a>(Vec<OpRef<'a, HpuLang>>);

impl<'a> Batcher<'a> {
    pub fn new() -> Self {
        Batcher(Vec::new())
    }

    pub fn push_op(&mut self, op: OpRef<'a, HpuLang>) {
        self.0.push(op);
    }

    pub fn flush(&mut self, output: &mut IR<HpuLang>, output_map: &mut ValMap<ValId>) {
        // We collect the inputs and outputs of the batch.
        let mut inputs = self
            .0
            .iter()
            .map(|op| op.get_args_iter())
            .flatten()
            .filter(|arg| {
                // To be a batch input, an op arg origin must not be in the batch.
                !self.0.as_slice().contains(&arg.get_origin().opref)
            })
            .collect::<SmallSet<_>>()
            .into_iter()
            .collect::<SmallVec<_>>();
        inputs.sort_unstable_by_key(|a| a.get_id());
        let mut outputs = self
            .0
            .iter()
            .map(|op| op.get_returns_iter())
            .flatten()
            .filter(|arg| {
                // To be a batch ouptut, an op ret must have one user outside of the batch.
                arg.get_users_iter()
                    .any(|user| !self.0.as_slice().contains(&user))
            })
            .collect::<SmallSet<_>>()
            .into_iter()
            .collect::<SmallVec<_>>();
        outputs.sort_unstable_by_key(|a| a.get_id());

        // Now we write the batch IR
        let mut batch = IR::empty();
        let mut batch_map = SmallMap::new();
        for (i, val) in inputs.iter().enumerate() {
            let (_, batch_arg) = batch
                .add_op(
                    HpuInstructionSet::BatchArg {
                        pos: i.try_into().unwrap(),
                        ty: val.get_type(),
                    },
                    svec![],
                )
                .unwrap();
            batch_map.insert(val.get_id(), batch_arg[0]);
        }
        for op in self.0.iter() {
            let (_, batch_op_rets) = batch
                .add_op(
                    op.get_operation(),
                    op.get_arg_valids()
                        .iter()
                        .map(|k| batch_map.get(k).unwrap())
                        .copied()
                        .collect(),
                )
                .unwrap();
            for (k, v) in (op.get_return_valids().iter(), batch_op_rets.into_iter()).mzip() {
                batch_map.insert(*k, v);
            }
        }
        for (i, val) in outputs.iter().enumerate() {
            batch
                .add_op(
                    HpuInstructionSet::BatchRet {
                        pos: i.try_into().unwrap(),
                        ty: val.get_type(),
                    },
                    svec![*batch_map.get(&val.get_id()).unwrap()],
                )
                .unwrap();
        }

        // We add the batch op in the new IR.
        let (_, valids) = output
            .add_op(
                HpuInstructionSet::Batch {
                    block: Box::new(batch),
                },
                inputs
                    .iter()
                    .map(|a| output_map.get(&a.get_id()).unwrap())
                    .copied()
                    .collect(),
            )
            .unwrap();
        for (k, v) in (outputs.into_iter().map(|a| a.get_id()), valids.into_iter()).mzip() {
            output_map.insert(k, v);
        }

        // We clean the batcher
        self.0.clear();
    }
}

pub fn batch(ir: &IR<HpuLang>) -> IR<HpuLang> {
    let mut output = IR::empty();
    let mut map = ir.empty_valmap::<ValId>();
    let mut batcher = Batcher::new();
    for op in ir.walk_ops_linear() {
        use hc_langs::hpulang::HpuInstructionSet::*;
        match op.get_operation() {
            AddCt | SubCt | Mac { .. } | AddPt | SubPt | PtSub | MulPt => {
                let (_, valids) = output
                    .add_op(
                        op.get_operation(),
                        svec![map[op.get_arg_valids()[0]], map[op.get_arg_valids()[1]]],
                    )
                    .unwrap();
                map.insert(op.get_return_valids()[0], valids[0]);
            }
            AddCst { .. } | SubCst { .. } | CstSub { .. } | MulCst { .. } => {
                let (_, valids) = output
                    .add_op(op.get_operation(), svec![map[op.get_arg_valids()[0]]])
                    .unwrap();
                map.insert(op.get_return_valids()[0], valids[0]);
            }
            ImmLd { .. } | SrcLd { .. } => {
                let (_, valids) = output.add_op(op.get_operation(), svec![]).unwrap();
                map.insert(op.get_return_valids()[0], valids[0]);
            }
            DstSt { .. } => {
                output
                    .add_op(op.get_operation(), svec![map[op.get_arg_valids()[0]]])
                    .unwrap();
            }
            Pbs { .. } | Pbs2 { .. } | Pbs4 { .. } | Pbs8 { .. } => {
                batcher.push_op(op);
            }
            PbsF { .. } | Pbs2F { .. } | Pbs4F { .. } | Pbs8F { .. } => {
                batcher.push_op(op);
                batcher.flush(&mut output, &mut map);
            }
            Batch { .. } | BatchArg { .. } | BatchRet { .. } => {
                unreachable!("Encountered a batch op while batching...")
            }
        }
    }
    output
}

#[cfg(test)]
mod test {
    use hc_ir::{IR, PrintWalker, translation::Translator};
    use hc_langs::{hpulang::HpuLang, ioplang::IopLang};
    use hc_sim::hpu::{HpuConfig, PhysicalConfig};
    use hc_utils::assert_display_is;

    use crate::{
        scheduler::schedule,
        test::{get_add_ir, get_cmp_ir},
        translation::IoplangToHpulang,
    };

    use super::batch;

    fn pipeline(ir: &IR<IopLang>) -> IR<HpuLang> {
        let ir = IoplangToHpulang.translate(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let scheduled = schedule(&ir, &config);
        let batch = batch(&scheduled);
        use hc_langs::hpulang::HpuInstructionSet::*;
        batch
            .walk_ops_linear()
            .for_each(|op| match op.get_operation() {
                Pbs { .. }
                | Pbs2 { .. }
                | Pbs4 { .. }
                | Pbs8 { .. }
                | PbsF { .. }
                | Pbs2F { .. }
                | Pbs4F { .. }
                | Pbs8F { .. } => panic!(),
                _ => {}
            });
        batch
    }

    #[test]
    fn test_batch_add_ir() {
        let ir = pipeline(&get_add_ir(16, 2, 2));
        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 : CtRegister = src_ld<0.0_tsrc>();
                %1 : CtRegister = src_ld<0.1_tsrc>();
                %2 : CtRegister = src_ld<0.2_tsrc>();
                %3 : CtRegister = src_ld<0.3_tsrc>();
                %4 : CtRegister = src_ld<0.4_tsrc>();
                %5 : CtRegister = src_ld<0.5_tsrc>();
                %6 : CtRegister = src_ld<0.6_tsrc>();
                %7 : CtRegister = src_ld<0.7_tsrc>();
                %8 : CtRegister = src_ld<1.0_tsrc>();
                %9 : CtRegister = add_ct(%0 : CtRegister, %8 : CtRegister);
                %10 : CtRegister = src_ld<1.1_tsrc>();
                %11 : CtRegister = src_ld<1.2_tsrc>();
                %12 : CtRegister = src_ld<1.3_tsrc>();
                %13 : CtRegister = src_ld<1.4_tsrc>();
                %14 : CtRegister = add_ct(%1 : CtRegister, %10 : CtRegister);
                %15 : CtRegister = src_ld<1.5_tsrc>();
                %16 : CtRegister = src_ld<1.6_tsrc>();
                %17 : CtRegister = src_ld<1.7_tsrc>();
                %18 : CtRegister = add_ct(%2 : CtRegister, %11 : CtRegister);
                %19 : CtRegister = add_ct(%3 : CtRegister, %12 : CtRegister);
                %20 : CtRegister = add_ct(%4 : CtRegister, %13 : CtRegister);
                %21 : CtRegister = add_ct(%5 : CtRegister, %15 : CtRegister);
                %22 : CtRegister = add_ct(%6 : CtRegister, %16 : CtRegister);
                %23 : CtRegister = add_ct(%7 : CtRegister, %17 : CtRegister);
                %24 : CtRegister, %25 : CtRegister, %26 : CtRegister, %27 : CtRegister, %28 : CtRegister, %29 : CtRegister, %30 : CtRegister, %31 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = batch_arg<3, CtRegister>();
                    %4 : CtRegister = batch_arg<4, CtRegister>();
                    %5 : CtRegister = batch_arg<5, CtRegister>();
                    %6 : CtRegister = batch_arg<6, CtRegister>();
                    %7 : CtRegister, %8 : CtRegister = pbs_2<Lut@26>(%0 : CtRegister);
                    %9 : CtRegister = pbs<Lut@47>(%1 : CtRegister);
                    %10 : CtRegister = pbs<Lut@48>(%2 : CtRegister);
                    %11 : CtRegister = pbs<Lut@49>(%3 : CtRegister);
                    %12 : CtRegister = pbs<Lut@47>(%4 : CtRegister);
                    %13 : CtRegister = pbs<Lut@48>(%5 : CtRegister);
                    %14 : CtRegister = pbs_f<Lut@49>(%6 : CtRegister);
                    batch_ret<0, CtRegister>(%7 : CtRegister);
                    batch_ret<1, CtRegister>(%8 : CtRegister);
                    batch_ret<2, CtRegister>(%9 : CtRegister);
                    batch_ret<3, CtRegister>(%10 : CtRegister);
                    batch_ret<4, CtRegister>(%11 : CtRegister);
                    batch_ret<5, CtRegister>(%12 : CtRegister);
                    batch_ret<6, CtRegister>(%13 : CtRegister);
                    batch_ret<7, CtRegister>(%14 : CtRegister);
                }(%9 : CtRegister, %14 : CtRegister, %18 : CtRegister, %19 : CtRegister, %20 : CtRegister, %21 : CtRegister, %22 : CtRegister);
                %32 : CtRegister = add_ct(%14 : CtRegister, %25 : CtRegister);
                %33 : CtRegister = add_ct(%25 : CtRegister, %26 : CtRegister);
                %34 : CtRegister = add_ct(%29 : CtRegister, %30 : CtRegister);
                %35 : CtRegister, %36 : CtRegister, %37 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = pbs<Lut@1>(%0 : CtRegister);
                    %4 : CtRegister = pbs<Lut@1>(%1 : CtRegister);
                    %5 : CtRegister = pbs_f<Lut@44>(%2 : CtRegister);
                    batch_ret<0, CtRegister>(%3 : CtRegister);
                    batch_ret<1, CtRegister>(%4 : CtRegister);
                    batch_ret<2, CtRegister>(%5 : CtRegister);
                }(%24 : CtRegister, %32 : CtRegister, %33 : CtRegister);
                %38 : CtRegister = add_ct(%33 : CtRegister, %27 : CtRegister);
                %39 : CtRegister = add_ct(%34 : CtRegister, %31 : CtRegister);
                dst_st<0.0_tdst>(%35 : CtRegister);
                dst_st<0.1_tdst>(%36 : CtRegister);
                %40 : CtRegister = add_ct(%38 : CtRegister, %28 : CtRegister);
                %41 : CtRegister = add_ct(%18 : CtRegister, %37 : CtRegister);
                %42 : CtRegister, %43 : CtRegister, %44 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = pbs<Lut@45>(%0 : CtRegister);
                    %4 : CtRegister = pbs<Lut@46>(%1 : CtRegister);
                    %5 : CtRegister = pbs_f<Lut@1>(%2 : CtRegister);
                    batch_ret<0, CtRegister>(%3 : CtRegister);
                    batch_ret<1, CtRegister>(%4 : CtRegister);
                    batch_ret<2, CtRegister>(%5 : CtRegister);
                }(%38 : CtRegister, %40 : CtRegister, %41 : CtRegister);
                %45 : CtRegister = add_ct(%19 : CtRegister, %42 : CtRegister);
                dst_st<0.2_tdst>(%44 : CtRegister);
                %46 : CtRegister = add_ct(%29 : CtRegister, %43 : CtRegister);
                %47 : CtRegister = add_ct(%34 : CtRegister, %43 : CtRegister);
                %48 : CtRegister = add_ct(%39 : CtRegister, %43 : CtRegister);
                %49 : CtRegister = add_ct(%20 : CtRegister, %43 : CtRegister);
                %50 : CtRegister, %51 : CtRegister, %52 : CtRegister, %53 : CtRegister, %54 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = batch_arg<3, CtRegister>();
                    %4 : CtRegister = batch_arg<4, CtRegister>();
                    %5 : CtRegister = pbs<Lut@1>(%0 : CtRegister);
                    %6 : CtRegister = pbs<Lut@44>(%1 : CtRegister);
                    %7 : CtRegister = pbs<Lut@45>(%2 : CtRegister);
                    %8 : CtRegister = pbs<Lut@46>(%3 : CtRegister);
                    %9 : CtRegister = pbs_f<Lut@1>(%4 : CtRegister);
                    batch_ret<0, CtRegister>(%5 : CtRegister);
                    batch_ret<1, CtRegister>(%6 : CtRegister);
                    batch_ret<2, CtRegister>(%7 : CtRegister);
                    batch_ret<3, CtRegister>(%8 : CtRegister);
                    batch_ret<4, CtRegister>(%9 : CtRegister);
                }(%45 : CtRegister, %46 : CtRegister, %47 : CtRegister, %48 : CtRegister, %49 : CtRegister);
                dst_st<0.3_tdst>(%50 : CtRegister);
                %55 : CtRegister = add_ct(%21 : CtRegister, %51 : CtRegister);
                dst_st<0.4_tdst>(%54 : CtRegister);
                %56 : CtRegister = add_ct(%22 : CtRegister, %52 : CtRegister);
                %57 : CtRegister = add_ct(%23 : CtRegister, %53 : CtRegister);
                %58 : CtRegister, %59 : CtRegister, %60 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = pbs<Lut@1>(%0 : CtRegister);
                    %4 : CtRegister = pbs<Lut@1>(%1 : CtRegister);
                    %5 : CtRegister = pbs_f<Lut@1>(%2 : CtRegister);
                    batch_ret<0, CtRegister>(%3 : CtRegister);
                    batch_ret<1, CtRegister>(%4 : CtRegister);
                    batch_ret<2, CtRegister>(%5 : CtRegister);
                }(%55 : CtRegister, %56 : CtRegister, %57 : CtRegister);
                dst_st<0.5_tdst>(%58 : CtRegister);
                dst_st<0.6_tdst>(%59 : CtRegister);
                dst_st<0.7_tdst>(%60 : CtRegister);
            "#
        );
    }

    #[test]
    fn test_batch_cmp_ir() {
        let ir = pipeline(&get_cmp_ir(16, 2, 2));
        assert_display_is!(
            ir.format().with_walker(PrintWalker::Linear),
            r#"
                %0 : CtRegister = src_ld<0.0_tsrc>();
                %1 : CtRegister = src_ld<0.1_tsrc>();
                %2 : CtRegister = mac<4_imm>(%1 : CtRegister, %0 : CtRegister);
                %3 : CtRegister = src_ld<0.2_tsrc>();
                %4 : CtRegister = src_ld<0.3_tsrc>();
                %5 : CtRegister = src_ld<0.4_tsrc>();
                %6 : CtRegister = src_ld<0.5_tsrc>();
                %7 : CtRegister = mac<4_imm>(%4 : CtRegister, %3 : CtRegister);
                %8 : CtRegister = src_ld<0.6_tsrc>();
                %9 : CtRegister = src_ld<0.7_tsrc>();
                %10 : CtRegister = src_ld<1.0_tsrc>();
                %11 : CtRegister = src_ld<1.1_tsrc>();
                %12 : CtRegister = mac<4_imm>(%6 : CtRegister, %5 : CtRegister);
                %13 : CtRegister = src_ld<1.2_tsrc>();
                %14 : CtRegister = src_ld<1.3_tsrc>();
                %15 : CtRegister = src_ld<1.4_tsrc>();
                %16 : CtRegister = src_ld<1.5_tsrc>();
                %17 : CtRegister = mac<4_imm>(%9 : CtRegister, %8 : CtRegister);
                %18 : CtRegister = src_ld<1.6_tsrc>();
                %19 : CtRegister = src_ld<1.7_tsrc>();
                %20 : CtRegister = mac<4_imm>(%11 : CtRegister, %10 : CtRegister);
                %21 : CtRegister = mac<4_imm>(%14 : CtRegister, %13 : CtRegister);
                %22 : CtRegister = mac<4_imm>(%16 : CtRegister, %15 : CtRegister);
                %23 : CtRegister = mac<4_imm>(%19 : CtRegister, %18 : CtRegister);
                %24 : CtRegister, %25 : CtRegister, %26 : CtRegister, %27 : CtRegister, %28 : CtRegister, %29 : CtRegister, %30 : CtRegister, %31 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = batch_arg<3, CtRegister>();
                    %4 : CtRegister = batch_arg<4, CtRegister>();
                    %5 : CtRegister = batch_arg<5, CtRegister>();
                    %6 : CtRegister = batch_arg<6, CtRegister>();
                    %7 : CtRegister = batch_arg<7, CtRegister>();
                    %8 : CtRegister = pbs<Lut@0>(%0 : CtRegister);
                    %9 : CtRegister = pbs<Lut@0>(%1 : CtRegister);
                    %10 : CtRegister = pbs<Lut@0>(%2 : CtRegister);
                    %11 : CtRegister = pbs<Lut@0>(%3 : CtRegister);
                    %12 : CtRegister = pbs<Lut@0>(%4 : CtRegister);
                    %13 : CtRegister = pbs<Lut@0>(%5 : CtRegister);
                    %14 : CtRegister = pbs<Lut@0>(%6 : CtRegister);
                    %15 : CtRegister = pbs_f<Lut@0>(%7 : CtRegister);
                    batch_ret<0, CtRegister>(%8 : CtRegister);
                    batch_ret<1, CtRegister>(%9 : CtRegister);
                    batch_ret<2, CtRegister>(%10 : CtRegister);
                    batch_ret<3, CtRegister>(%11 : CtRegister);
                    batch_ret<4, CtRegister>(%12 : CtRegister);
                    batch_ret<5, CtRegister>(%13 : CtRegister);
                    batch_ret<6, CtRegister>(%14 : CtRegister);
                    batch_ret<7, CtRegister>(%15 : CtRegister);
                }(%2 : CtRegister, %7 : CtRegister, %12 : CtRegister, %17 : CtRegister, %20 : CtRegister, %21 : CtRegister, %22 : CtRegister, %23 : CtRegister);
                %32 : CtRegister = sub_ct(%24 : CtRegister, %28 : CtRegister);
                %33 : CtRegister = sub_ct(%25 : CtRegister, %29 : CtRegister);
                %34 : CtRegister = sub_ct(%26 : CtRegister, %30 : CtRegister);
                %35 : CtRegister = sub_ct(%27 : CtRegister, %31 : CtRegister);
                %36 : CtRegister, %37 : CtRegister, %38 : CtRegister, %39 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = batch_arg<3, CtRegister>();
                    %4 : CtRegister = pbs<Lut@10>(%0 : CtRegister);
                    %5 : CtRegister = pbs<Lut@10>(%1 : CtRegister);
                    %6 : CtRegister = pbs<Lut@10>(%2 : CtRegister);
                    %7 : CtRegister = pbs_f<Lut@10>(%3 : CtRegister);
                    batch_ret<0, CtRegister>(%4 : CtRegister);
                    batch_ret<1, CtRegister>(%5 : CtRegister);
                    batch_ret<2, CtRegister>(%6 : CtRegister);
                    batch_ret<3, CtRegister>(%7 : CtRegister);
                }(%32 : CtRegister, %33 : CtRegister, %34 : CtRegister, %35 : CtRegister);
                %40 : CtRegister = add_cst<1_imm>(%36 : CtRegister);
                %41 : CtRegister = add_cst<1_imm>(%37 : CtRegister);
                %42 : CtRegister = add_cst<1_imm>(%38 : CtRegister);
                %43 : CtRegister = add_cst<1_imm>(%39 : CtRegister);
                %44 : CtRegister = mac<4_imm>(%41 : CtRegister, %40 : CtRegister);
                %45 : CtRegister = mac<4_imm>(%43 : CtRegister, %42 : CtRegister);
                %46 : CtRegister, %47 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = pbs<Lut@11>(%0 : CtRegister);
                    %3 : CtRegister = pbs_f<Lut@11>(%1 : CtRegister);
                    batch_ret<0, CtRegister>(%2 : CtRegister);
                    batch_ret<1, CtRegister>(%3 : CtRegister);
                }(%44 : CtRegister, %45 : CtRegister);
                %48 : CtRegister = mac<4_imm>(%47 : CtRegister, %46 : CtRegister);
                %49 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = pbs_f<Lut@27>(%0 : CtRegister);
                    batch_ret<0, CtRegister>(%1 : CtRegister);
                }(%48 : CtRegister);
                dst_st<0.0_tdst>(%49 : CtRegister);
            "#
        );
    }
}
