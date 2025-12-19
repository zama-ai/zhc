use hpuc_ir::{IR, OpRef, ValId, ValMap};
use hpuc_langs::hpulang::{Hpulang, Operations};
use hpuc_utils::{svec, MultiZip, SmallMap, SmallSet, SmallVec};

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
        use hpuc_langs::hpulang::Operations::*;
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
    use hpuc_ir::{IR, translation::Translator};
    use hpuc_langs::{hpulang::Hpulang, ioplang::Ioplang};
    use hpuc_sim::hpu::{HpuConfig, PhysicalConfig};

    use crate::{
        scheduler::schedule,
        test::{get_add_ir, get_cmp_ir, get_sub_ir},
        translation::IoplangToHpulang,
    };

    use super::batch;

    fn pipeline(ir: &IR<Ioplang>) -> IR<Hpulang> {
        let ir = IoplangToHpulang.translate(&ir);
        let config = HpuConfig::from(PhysicalConfig::gaussian_64b());
        let scheduled = schedule(&ir, &config);
        let batch = batch(&scheduled);
        use hpuc_langs::hpulang::Operations::*;
        batch.walk_ops_linear().for_each(|op| match op.get_operation() {
            Pbs { .. } | Pbs2 { .. } | Pbs4 { .. } | Pbs8 { .. } | PbsF { .. } | Pbs2F { .. } | Pbs4F { .. } | Pbs8F { .. } => panic!(),
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
            %7 : CtRegister = src_ld<1.0_tsrc>();
            %8 : CtRegister = add_ct(%0, %7);
            %9 : CtRegister = src_ld<1.1_tsrc>();
            %10 : CtRegister = src_ld<1.2_tsrc>();
            %11 : CtRegister = src_ld<1.3_tsrc>();
            %12 : CtRegister = src_ld<1.4_tsrc>();
            %13 : CtRegister = add_ct(%1, %9);
            %14 : CtRegister = src_ld<1.5_tsrc>();
            %15 : CtRegister = src_ld<1.6_tsrc>();
            %16 : CtRegister = add_ct(%2, %10);
            %17 : CtRegister = add_ct(%3, %11);
            %18 : CtRegister = add_ct(%4, %12);
            %19 : CtRegister = add_ct(%5, %14);
            %20 : CtRegister = add_ct(%6, %15);
            %21 : CtRegister, %22 : CtRegister, %23 : CtRegister, %24 : CtRegister, %25 : CtRegister, %26 : CtRegister, %27 : CtRegister, %28 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = batch_arg<3, CtRegister>();
                    %4 : CtRegister = batch_arg<4, CtRegister>();
                    %5 : CtRegister = batch_arg<5, CtRegister>();
                    %6 : CtRegister = batch_arg<6, CtRegister>();
                    %7 : CtRegister, %8 : CtRegister = pbs_2<Lut@26>(%0);
                    %9 : CtRegister = pbs<Lut@47>(%1);
                    %10 : CtRegister = pbs<Lut@48>(%2);
                    %11 : CtRegister = pbs<Lut@49>(%3);
                    %12 : CtRegister = pbs<Lut@47>(%4);
                    %13 : CtRegister = pbs<Lut@48>(%5);
                    %14 : CtRegister = pbs_f<Lut@49>(%6);
                    batch_ret<0, CtRegister>(%7);
                    batch_ret<1, CtRegister>(%8);
                    batch_ret<2, CtRegister>(%9);
                    batch_ret<3, CtRegister>(%10);
                    batch_ret<4, CtRegister>(%11);
                    batch_ret<5, CtRegister>(%12);
                    batch_ret<6, CtRegister>(%13);
                    batch_ret<7, CtRegister>(%14);
            }(%8, %13, %16, %17, %18, %19, %20);
            %29 : CtRegister = add_ct(%13, %22);
            dst_st<0.0_tdst>(%21);
            %30 : CtRegister = add_ct(%23, %22);
            dst_st<0.1_tdst>(%29);
            %31 : CtRegister = add_ct(%27, %26);
            %32 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = pbs_f<Lut@44>(%0);
                    batch_ret<0, CtRegister>(%1);
            }(%30);
            %33 : CtRegister = add_ct(%24, %30);
            %34 : CtRegister = add_ct(%28, %31);
            %35 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = pbs_f<Lut@45>(%0);
                    batch_ret<0, CtRegister>(%1);
            }(%33);
            %36 : CtRegister = add_ct(%25, %33);
            %37 : CtRegister = add_ct(%16, %32);
            %38 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = pbs_f<Lut@46>(%0);
                    batch_ret<0, CtRegister>(%1);
            }(%36);
            %39 : CtRegister = add_ct(%17, %35);
            dst_st<0.2_tdst>(%37);
            dst_st<0.3_tdst>(%39);
            %40 : CtRegister = add_ct(%26, %38);
            %41 : CtRegister = add_ct(%31, %38);
            %42 : CtRegister = add_ct(%34, %38);
            %43 : CtRegister, %44 : CtRegister, %45 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = pbs<Lut@46>(%0);
                    %4 : CtRegister = pbs<Lut@44>(%1);
                    %5 : CtRegister = pbs_f<Lut@45>(%2);
                    batch_ret<0, CtRegister>(%3);
                    batch_ret<1, CtRegister>(%4);
                    batch_ret<2, CtRegister>(%5);
            }(%40, %41, %42);
            %46 : CtRegister = add_ct(%18, %43);
            %47 : CtRegister = add_ct(%19, %44);
            dst_st<0.4_tdst>(%46);
            %48 : CtRegister = add_ct(%20, %45);
            dst_st<0.5_tdst>(%47);
            dst_st<0.6_tdst>(%48);
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

    #[test]
    fn test_batch_sub_ir() {
        let ir = pipeline(&get_sub_ir(16, 2, 2));
        ir.check_ir_linear(
            "
            %0 : CtRegister = src_ld<0.0_tsrc>();
            %1 : CtRegister = src_ld<0.1_tsrc>();
            %2 : CtRegister = src_ld<0.2_tsrc>();
            %3 : CtRegister = src_ld<0.3_tsrc>();
            %4 : CtRegister = src_ld<0.4_tsrc>();
            %5 : CtRegister = src_ld<0.5_tsrc>();
            %6 : CtRegister = src_ld<0.6_tsrc>();
            %7 : CtRegister = src_ld<1.0_tsrc>();
            %8 : CtRegister = cst_sub<3_imm>(%7);
            %9 : CtRegister = src_ld<1.1_tsrc>();
            %10 : CtRegister = src_ld<1.2_tsrc>();
            %11 : CtRegister = src_ld<1.3_tsrc>();
            %12 : CtRegister = src_ld<1.4_tsrc>();
            %13 : CtRegister = cst_sub<3_imm>(%9);
            %14 : CtRegister = src_ld<1.5_tsrc>();
            %15 : CtRegister = src_ld<1.6_tsrc>();
            %16 : CtRegister = cst_sub<3_imm>(%10);
            %17 : CtRegister = cst_sub<3_imm>(%11);
            %18 : CtRegister = cst_sub<3_imm>(%12);
            %19 : CtRegister = cst_sub<3_imm>(%14);
            %20 : CtRegister = cst_sub<3_imm>(%15);
            %21 : CtRegister = add_ct(%0, %8);
            %22 : CtRegister = add_ct(%1, %13);
            %23 : CtRegister = add_ct(%2, %16);
            %24 : CtRegister = add_ct(%3, %17);
            %25 : CtRegister = add_ct(%4, %18);
            %26 : CtRegister = add_ct(%5, %19);
            %27 : CtRegister = add_ct(%6, %20);
            %28 : CtRegister, %29 : CtRegister, %30 : CtRegister, %31 : CtRegister, %32 : CtRegister, %33 : CtRegister, %34 : CtRegister, %35 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = batch_arg<3, CtRegister>();
                    %4 : CtRegister = batch_arg<4, CtRegister>();
                    %5 : CtRegister = batch_arg<5, CtRegister>();
                    %6 : CtRegister = batch_arg<6, CtRegister>();
                    %7 : CtRegister, %8 : CtRegister = pbs_2<Lut@26>(%0);
                    %9 : CtRegister = pbs<Lut@47>(%1);
                    %10 : CtRegister = pbs<Lut@48>(%2);
                    %11 : CtRegister = pbs<Lut@49>(%3);
                    %12 : CtRegister = pbs<Lut@47>(%4);
                    %13 : CtRegister = pbs<Lut@48>(%5);
                    %14 : CtRegister = pbs_f<Lut@49>(%6);
                    batch_ret<0, CtRegister>(%7);
                    batch_ret<1, CtRegister>(%8);
                    batch_ret<2, CtRegister>(%9);
                    batch_ret<3, CtRegister>(%10);
                    batch_ret<4, CtRegister>(%11);
                    batch_ret<5, CtRegister>(%12);
                    batch_ret<6, CtRegister>(%13);
                    batch_ret<7, CtRegister>(%14);
            }(%21, %22, %23, %24, %25, %26, %27);
            %36 : CtRegister = add_ct(%22, %29);
            %37 : CtRegister = add_ct(%30, %29);
            %38 : CtRegister = add_ct(%34, %33);
            %39 : CtRegister, %40 : CtRegister, %41 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = pbs<Lut@1>(%0);
                    %4 : CtRegister = pbs<Lut@1>(%1);
                    %5 : CtRegister = pbs_f<Lut@44>(%2);
                    batch_ret<0, CtRegister>(%3);
                    batch_ret<1, CtRegister>(%4);
                    batch_ret<2, CtRegister>(%5);
            }(%28, %36, %37);
            %42 : CtRegister = add_ct(%31, %37);
            %43 : CtRegister = add_ct(%35, %38);
            dst_st<0.0_tdst>(%39);
            dst_st<0.1_tdst>(%40);
            %44 : CtRegister = add_ct(%32, %42);
            %45 : CtRegister = add_ct(%23, %41);
            %46 : CtRegister, %47 : CtRegister, %48 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = pbs<Lut@45>(%0);
                    %4 : CtRegister = pbs<Lut@46>(%1);
                    %5 : CtRegister = pbs_f<Lut@1>(%2);
                    batch_ret<0, CtRegister>(%3);
                    batch_ret<1, CtRegister>(%4);
                    batch_ret<2, CtRegister>(%5);
            }(%42, %44, %45);
            %49 : CtRegister = add_ct(%24, %46);
            dst_st<0.2_tdst>(%48);
            %50 : CtRegister = add_ct(%33, %47);
            %51 : CtRegister = add_ct(%38, %47);
            %52 : CtRegister = add_ct(%43, %47);
            %53 : CtRegister, %54 : CtRegister, %55 : CtRegister, %56 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = batch_arg<3, CtRegister>();
                    %4 : CtRegister = pbs<Lut@1>(%0);
                    %5 : CtRegister = pbs<Lut@46>(%1);
                    %6 : CtRegister = pbs<Lut@44>(%2);
                    %7 : CtRegister = pbs_f<Lut@45>(%3);
                    batch_ret<0, CtRegister>(%4);
                    batch_ret<1, CtRegister>(%5);
                    batch_ret<2, CtRegister>(%6);
                    batch_ret<3, CtRegister>(%7);
            }(%49, %50, %51, %52);
            dst_st<0.3_tdst>(%53);
            %57 : CtRegister = add_ct(%25, %54);
            %58 : CtRegister = add_ct(%26, %55);
            %59 : CtRegister = add_ct(%27, %56);
            %60 : CtRegister, %61 : CtRegister, %62 : CtRegister = batch {
                    %0 : CtRegister = batch_arg<0, CtRegister>();
                    %1 : CtRegister = batch_arg<1, CtRegister>();
                    %2 : CtRegister = batch_arg<2, CtRegister>();
                    %3 : CtRegister = pbs<Lut@1>(%0);
                    %4 : CtRegister = pbs<Lut@1>(%1);
                    %5 : CtRegister = pbs_f<Lut@1>(%2);
                    batch_ret<0, CtRegister>(%3);
                    batch_ret<1, CtRegister>(%4);
                    batch_ret<2, CtRegister>(%5);
            }(%57, %58, %59);
            dst_st<0.4_tdst>(%60);
            dst_st<0.5_tdst>(%61);
            dst_st<0.6_tdst>(%62);
            ",
        );
    }
}
