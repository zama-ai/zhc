use hc_ir::{IR, OpRef, ValId, ValMap};
use hc_langs::hpulang::{Hpulang, Operations};
use hc_utils::{iter::MultiZip, small::{SmallMap, SmallSet, SmallVec}, svec};

struct Batcher<'a>(Vec<OpRef<'a, Hpulang>>);

impl<'a> Batcher<'a> {
    pub fn new() -> Self {
        Batcher(Vec::new())
    }

    pub fn push_op(&mut self, op: OpRef<'a, Hpulang>) {
        self.0.push(op);
    }

    pub fn flush(&mut self, output: &mut IR<Hpulang>, output_map: &mut ValMap<ValId>) {
        // We collect the inputs and outputs of the batch.
        let mut inputs = self
            .0
            .iter()
            .map(|op| op.get_args_iter())
            .flatten()
            .filter(|arg| {
                // To be a batch input, an op arg origin must not be in the batch.
                !self.0.as_slice().contains(&arg.get_origin())
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
                    Operations::BatchArg {
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
                    Operations::BatchRet {
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
                Operations::Batch {
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

pub fn batch(ir: &IR<Hpulang>) -> IR<Hpulang> {
    let mut output = IR::empty();
    let mut map = ir.empty_valmap::<ValId>();
    let mut batcher = Batcher::new();
    for op in ir.walk_ops_linear() {
        use hc_langs::hpulang::Operations::*;
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
    use hc_ir::{IR, translation::Translator};
    use hc_langs::{hpulang::Hpulang, ioplang::Ioplang};
    use hc_sim::hpu::{HpuConfig, PhysicalConfig};

    use crate::{
        scheduler::schedule,
        test::{get_add_ir, get_cmp_ir},
        translation::IoplangToHpulang,
    };

    use super::batch;

    fn pipeline(ir: &IR<Ioplang>) -> IR<Hpulang> {
        let ir = IoplangToHpulang.translate(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let scheduled = schedule(&ir, &config);
        let batch = batch(&scheduled);
        use hc_langs::hpulang::Operations::*;
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
        ir.check_ir_linear(
            "
            %0 : CtRegister = src_ld<0.0_tsrc>();
            %1 : CtRegister = src_ld<0.1_tsrc>();
            %2 : CtRegister = src_ld<0.2_tsrc>();
            %3 : CtRegister = src_ld<0.3_tsrc>();
            %4 : CtRegister = src_ld<0.4_tsrc>();
            %5 : CtRegister = src_ld<0.5_tsrc>();
            %6 : CtRegister = src_ld<0.6_tsrc>();
            %7 : CtRegister = src_ld<0.7_tsrc>();
            %8 : CtRegister = src_ld<1.0_tsrc>();
            %9 : CtRegister = add_ct(%0, %8);
            %10 : CtRegister = src_ld<1.1_tsrc>();
            %11 : CtRegister = src_ld<1.2_tsrc>();
            %12 : CtRegister = src_ld<1.3_tsrc>();
            %13 : CtRegister = src_ld<1.4_tsrc>();
            %14 : CtRegister = add_ct(%1, %10);
            %15 : CtRegister = src_ld<1.5_tsrc>();
            %16 : CtRegister = src_ld<1.6_tsrc>();
            %17 : CtRegister = src_ld<1.7_tsrc>();
            %18 : CtRegister = add_ct(%2, %11);
            %19 : CtRegister = add_ct(%3, %12);
            %20 : CtRegister = add_ct(%4, %13);
            %21 : CtRegister = add_ct(%5, %15);
            %22 : CtRegister = add_ct(%6, %16);
            %23 : CtRegister = add_ct(%7, %17);
            %24 : CtRegister, %25 : CtRegister, %26 : CtRegister, %27 : CtRegister, %28 : CtRegister, %29 : CtRegister, %30 : CtRegister, %31 : CtRegister, %32 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
            %1 : CtRegister = batch_arg<1, CtRegister>();
            %2 : CtRegister = batch_arg<2, CtRegister>();
            %3 : CtRegister = batch_arg<3, CtRegister>();
            %4 : CtRegister = batch_arg<4, CtRegister>();
            %5 : CtRegister = batch_arg<5, CtRegister>();
            %6 : CtRegister = batch_arg<6, CtRegister>();
            %7 : CtRegister = batch_arg<7, CtRegister>();
            %8 : CtRegister, %9 : CtRegister = pbs_2<Lut@26>(%0);
            %10 : CtRegister = pbs<Lut@47>(%1);
            %11 : CtRegister = pbs<Lut@48>(%2);
            %12 : CtRegister = pbs<Lut@49>(%3);
            %13 : CtRegister = pbs<Lut@47>(%4);
            %14 : CtRegister = pbs<Lut@48>(%5);
            %15 : CtRegister = pbs<Lut@49>(%6);
            %16 : CtRegister = pbs_f<Lut@50>(%7);
            batch_ret<0, CtRegister>(%8);
            batch_ret<1, CtRegister>(%9);
            batch_ret<2, CtRegister>(%10);
            batch_ret<3, CtRegister>(%11);
            batch_ret<4, CtRegister>(%12);
            batch_ret<5, CtRegister>(%13);
            batch_ret<6, CtRegister>(%14);
            batch_ret<7, CtRegister>(%15);
            batch_ret<8, CtRegister>(%16);}(%9, %14, %18, %19, %20, %21, %22, %23);
            %33 : CtRegister = add_ct(%23, %24);
            %34 : CtRegister = add_ct(%25, %26);
            %35 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
            %1 : CtRegister = pbs_f<Lut@44>(%0);
            batch_ret<0, CtRegister>(%1);}(%34);
            %36 : CtRegister = add_ct(%29, %30);
            dst_st<0.7_tdst>(%33);
            %37 : CtRegister = add_ct(%34, %27);
            %38 : CtRegister = add_ct(%36, %31);
            %39 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
            %1 : CtRegister = pbs_f<Lut@45>(%0);
            batch_ret<0, CtRegister>(%1);}(%37);
            %40 : CtRegister = add_ct(%37, %28);
            %41 : CtRegister = add_ct(%38, %32);
            %42 : CtRegister = add_ct(%9, %35);
            %43 : CtRegister, %44 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
            %1 : CtRegister = batch_arg<1, CtRegister>();
            %2 : CtRegister = pbs<Lut@46>(%0);
            %3 : CtRegister = pbs_f<Lut@21>(%1);
            batch_ret<0, CtRegister>(%2);
            batch_ret<1, CtRegister>(%3);}(%40, %41);
            %45 : CtRegister = add_ct(%14, %39);
            dst_st<0.0_tdst>(%42);
            dst_st<0.1_tdst>(%45);
            %46 : CtRegister = add_ct(%29, %43);
            %47 : CtRegister = add_cst<1_imm>(%44);
            %48 : CtRegister = add_ct(%36, %43);
            %49 : CtRegister = add_ct(%38, %43);
            %50 : CtRegister = add_ct(%18, %43);
            %51 : CtRegister = mac<4_imm>(%47, %43);
            dst_st<0.2_tdst>(%50);
            %52 : CtRegister, %53 : CtRegister, %54 : CtRegister, %55 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
            %1 : CtRegister = batch_arg<1, CtRegister>();
            %2 : CtRegister = batch_arg<2, CtRegister>();
            %3 : CtRegister = batch_arg<3, CtRegister>();
            %4 : CtRegister = pbs<Lut@44>(%0);
            %5 : CtRegister = pbs<Lut@45>(%1);
            %6 : CtRegister = pbs<Lut@46>(%2);
            %7 : CtRegister = pbs_f<Lut@52>(%3);
            batch_ret<0, CtRegister>(%4);
            batch_ret<1, CtRegister>(%5);
            batch_ret<2, CtRegister>(%6);
            batch_ret<3, CtRegister>(%7);}(%46, %48, %49, %51);
            %56 : CtRegister = add_ct(%19, %52);
            %57 : CtRegister = add_ct(%20, %53);
            dst_st<0.3_tdst>(%56);
            %58 : CtRegister = add_ct(%21, %54);
            dst_st<0.4_tdst>(%57);
            %59 : CtRegister = add_ct(%22, %55);
            dst_st<0.5_tdst>(%58);
            dst_st<0.6_tdst>(%59);
            ",
        );
    }

    #[test]
    fn test_batch_cmp_ir() {
        let ir = pipeline(&get_cmp_ir(16, 2, 2));
        ir.check_ir_linear(
            "
            %0 : CtRegister = src_ld<0.0_tsrc>();
            %1 : CtRegister = src_ld<0.1_tsrc>();
            %2 : CtRegister = mac<4_imm>(%1, %0);
            %3 : CtRegister = src_ld<0.2_tsrc>();
            %4 : CtRegister = src_ld<0.3_tsrc>();
            %5 : CtRegister = src_ld<0.4_tsrc>();
            %6 : CtRegister = src_ld<0.5_tsrc>();
            %7 : CtRegister = mac<4_imm>(%4, %3);
            %8 : CtRegister = src_ld<0.6_tsrc>();
            %9 : CtRegister = src_ld<0.7_tsrc>();
            %10 : CtRegister = src_ld<1.0_tsrc>();
            %11 : CtRegister = src_ld<1.1_tsrc>();
            %12 : CtRegister = mac<4_imm>(%6, %5);
            %13 : CtRegister = src_ld<1.2_tsrc>();
            %14 : CtRegister = src_ld<1.3_tsrc>();
            %15 : CtRegister = src_ld<1.4_tsrc>();
            %16 : CtRegister = src_ld<1.5_tsrc>();
            %17 : CtRegister = mac<4_imm>(%9, %8);
            %18 : CtRegister = src_ld<1.6_tsrc>();
            %19 : CtRegister = src_ld<1.7_tsrc>();
            %20 : CtRegister = mac<4_imm>(%11, %10);
            %21 : CtRegister = mac<4_imm>(%14, %13);
            %22 : CtRegister = mac<4_imm>(%16, %15);
            %23 : CtRegister = mac<4_imm>(%19, %18);
            %24 : CtRegister, %25 : CtRegister, %26 : CtRegister, %27 : CtRegister, %28 : CtRegister, %29 : CtRegister, %30 : CtRegister, %31 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = batch_arg<3, CtRegister>();
                    %4 : CtRegister = batch_arg<4, CtRegister>();
                    %5 : CtRegister = batch_arg<5, CtRegister>();
                    %6 : CtRegister = batch_arg<6, CtRegister>();
                    %7 : CtRegister = batch_arg<7, CtRegister>();
                    %8 : CtRegister = pbs<Lut@0>(%0);
                    %9 : CtRegister = pbs<Lut@0>(%1);
                    %10 : CtRegister = pbs<Lut@0>(%2);
                    %11 : CtRegister = pbs<Lut@0>(%3);
                    %12 : CtRegister = pbs<Lut@0>(%4);
                    %13 : CtRegister = pbs<Lut@0>(%5);
                    %14 : CtRegister = pbs<Lut@0>(%6);
                    %15 : CtRegister = pbs_f<Lut@0>(%7);
                    batch_ret<0, CtRegister>(%8);
                    batch_ret<1, CtRegister>(%9);
                    batch_ret<2, CtRegister>(%10);
                    batch_ret<3, CtRegister>(%11);
                    batch_ret<4, CtRegister>(%12);
                    batch_ret<5, CtRegister>(%13);
                    batch_ret<6, CtRegister>(%14);
                    batch_ret<7, CtRegister>(%15);
            }(%2, %7, %12, %17, %20, %21, %22, %23);
            %32 : CtRegister = sub_ct(%24, %28);
            %33 : CtRegister = sub_ct(%25, %29);
            %34 : CtRegister = sub_ct(%26, %30);
            %35 : CtRegister = sub_ct(%27, %31);
            %36 : CtRegister, %37 : CtRegister, %38 : CtRegister, %39 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = batch_arg<3, CtRegister>();
                    %4 : CtRegister = pbs<Lut@10>(%0);
                    %5 : CtRegister = pbs<Lut@10>(%1);
                    %6 : CtRegister = pbs<Lut@10>(%2);
                    %7 : CtRegister = pbs_f<Lut@10>(%3);
                    batch_ret<0, CtRegister>(%4);
                    batch_ret<1, CtRegister>(%5);
                    batch_ret<2, CtRegister>(%6);
                    batch_ret<3, CtRegister>(%7);
            }(%32, %33, %34, %35);
            %40 : CtRegister = add_cst<1_imm>(%36);
            %41 : CtRegister = add_cst<1_imm>(%37);
            %42 : CtRegister = add_cst<1_imm>(%38);
            %43 : CtRegister = add_cst<1_imm>(%39);
            %44 : CtRegister = mac<4_imm>(%41, %40);
            %45 : CtRegister = mac<4_imm>(%43, %42);
            %46 : CtRegister, %47 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = pbs<Lut@11>(%0);
                    %3 : CtRegister = pbs_f<Lut@11>(%1);
                    batch_ret<0, CtRegister>(%2);
                    batch_ret<1, CtRegister>(%3);
            }(%44, %45);
            %48 : CtRegister = mac<4_imm>(%47, %46);
            %49 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = pbs_f<Lut@27>(%0);
                    batch_ret<0, CtRegister>(%1);
            }(%48);
            dst_st<0.0_tdst>(%49);
            ",
        );
    }
}
