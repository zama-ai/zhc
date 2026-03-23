use zhc_ir::{AnnIR, IR, ValId};
use zhc_langs::{
    doplang::{Argument, DopInstructionSet, DopLang},
    hpulang::{HpuInstructionSet, HpuLang},
};
use zhc_utils::{iter::MultiZip, small::SmallMap, svec};

use crate::allocator::{
    allocator::{Alloc, Spill, Unspill},
    batch_map::BatchMap,
    register_file::RegId,
};

pub fn translate<'ir>(ir: &AnnIR<'ir, HpuLang, Alloc, ()>) -> IR<DopLang> {
    use HpuInstructionSet::*;

    let mut output = IR::empty();
    let (_, val) = output.add_op(DopInstructionSet::_INIT, svec![]);
    let mut ctx = val[0];

    let mut add_op = |dop| {
        let (_, rets) = output.add_op(dop, svec![ctx]);
        ctx = rets[0];
    };

    for op in ir.walk_ops_linear() {
        let Alloc {
            spills,
            unspills,
            srcs,
            dsts,
            slots,
        } = op.get_annotation();

        for Spill { from, to } in spills.iter() {
            add_op(DopInstructionSet::ST {
                dst: Argument::ct_heap(to.0 as usize),
                src: Argument::ct_reg(from.0),
            });
        }

        for Unspill { from, to } in unspills.iter() {
            add_op(DopInstructionSet::LD {
                dst: Argument::ct_reg(to.0),
                src: Argument::ct_heap(from.0 as usize),
            })
        }

        match op.get_instruction() {
            SrcLd { from } => {
                add_op(DopInstructionSet::LD {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_src_var(
                        from.src_pos.try_into().unwrap(),
                        from.block_pos.try_into().unwrap(),
                    ),
                });
            }
            TransferOut { tid } => {
                add_op(DopInstructionSet::ST {
                    dst: Argument::ct_heap(slots[0].0 as usize),
                    src: Argument::ct_reg(srcs[0].0),
                });
                add_op(DopInstructionSet::NOTIFY {
                    virt_id: Argument::VirtId { id: 0 },
                    flag: Argument::UserFlag { flag: tid },
                    slot: Argument::ct_heap(slots[0].0 as usize),
                });
            }
            TransferIn { tid } => {
                // Ajouter un load_b2b
                // Ajouter le wait et voila
                add_op(DopInstructionSet::LD_B2B {
                    flag: Argument::UserFlag { flag: tid },
                    slot: Argument::ct_heap(slots[0].0 as usize),
                });
                add_op(DopInstructionSet::WAIT {
                    flag: Argument::UserFlag { flag: tid },
                    slot: Some(Argument::ct_heap(slots[0].0 as usize)),
                });
                add_op(DopInstructionSet::LD {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_heap(slots[0].0 as usize),
                })
            }
            DstSt { to } => {
                add_op(DopInstructionSet::ST {
                    src: Argument::ct_reg(srcs[0].0),
                    dst: Argument::ct_dst_var(
                        to.dst_pos.try_into().unwrap(),
                        to.block_pos.try_into().unwrap(),
                    ),
                });
            }
            ImmLd { .. } => {
                // This is a no-op in the doplang dialect.
                // Handled in Pt operations.
            }
            AddCt => {
                add_op(DopInstructionSet::ADD {
                    dst: Argument::ct_reg(dsts[0].0),
                    src1: Argument::ct_reg(srcs[0].0),
                    src2: Argument::ct_reg(srcs[1].0),
                });
            }
            HpuInstructionSet::SubCt => {
                add_op(DopInstructionSet::SUB {
                    dst: Argument::ct_reg(dsts[0].0),
                    src1: Argument::ct_reg(srcs[0].0),
                    src2: Argument::ct_reg(srcs[1].0),
                });
            }
            HpuInstructionSet::Mac { cst } => {
                add_op(DopInstructionSet::MAC {
                    dst: Argument::ct_reg(dsts[0].0),
                    src1: Argument::ct_reg(srcs[0].0),
                    src2: Argument::ct_reg(srcs[1].0),
                    cst: Argument::pt_const(cst.0),
                });
            }
            HpuInstructionSet::AddPt => {
                let imm_ld_op = op
                    .get_args_iter()
                    .nth(1)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction();
                let HpuInstructionSet::ImmLd { from } = imm_ld_op else {
                    unreachable!()
                };
                add_op(DopInstructionSet::ADDS {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_reg(srcs[0].0),
                    cst: Argument::pt_src_var(
                        from.imm_pos.try_into().unwrap(),
                        from.block_pos.try_into().unwrap(),
                    ),
                });
            }
            HpuInstructionSet::SubPt => {
                let imm_ld_op = op
                    .get_args_iter()
                    .nth(1)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction();
                let HpuInstructionSet::ImmLd { from } = imm_ld_op else {
                    unreachable!()
                };
                add_op(DopInstructionSet::SUBS {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_reg(srcs[0].0),
                    cst: Argument::pt_src_var(
                        from.imm_pos.try_into().unwrap(),
                        from.block_pos.try_into().unwrap(),
                    ),
                });
            }
            HpuInstructionSet::PtSub => {
                let imm_ld_op = op
                    .get_args_iter()
                    .nth(0)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction();
                let HpuInstructionSet::ImmLd { from } = imm_ld_op else {
                    unreachable!()
                };
                add_op(DopInstructionSet::SSUB {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_reg(srcs[0].0),
                    cst: Argument::pt_src_var(
                        from.imm_pos.try_into().unwrap(),
                        from.block_pos.try_into().unwrap(),
                    ),
                });
            }
            HpuInstructionSet::MulPt => {
                let imm_ld_op = op
                    .get_args_iter()
                    .nth(1)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction();
                let HpuInstructionSet::ImmLd { from } = imm_ld_op else {
                    unreachable!()
                };
                add_op(DopInstructionSet::MULS {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_reg(srcs[0].0),
                    cst: Argument::pt_src_var(
                        from.imm_pos.try_into().unwrap(),
                        from.block_pos.try_into().unwrap(),
                    ),
                });
            }
            HpuInstructionSet::AddCst { cst } => {
                add_op(DopInstructionSet::ADDS {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_reg(srcs[0].0),
                    cst: Argument::pt_const(cst.0),
                });
            }
            HpuInstructionSet::SubCst { cst } => {
                add_op(DopInstructionSet::SUBS {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_reg(srcs[0].0),
                    cst: Argument::pt_const(cst.0),
                });
            }
            HpuInstructionSet::CstSub { cst } => {
                add_op(DopInstructionSet::SSUB {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_reg(srcs[0].0),
                    cst: Argument::pt_const(cst.0),
                });
            }
            HpuInstructionSet::MulCst { cst } => {
                add_op(DopInstructionSet::MULS {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_reg(srcs[0].0),
                    cst: Argument::pt_const(cst.0),
                });
            }
            HpuInstructionSet::CstCt { cst } => {
                add_op(DopInstructionSet::SUB {
                    dst: Argument::ct_reg(dsts[0].0),
                    src1: Argument::ct_reg(dsts[0].0),
                    src2: Argument::ct_reg(dsts[0].0),
                });
                if cst.0 != 0 {
                    add_op(DopInstructionSet::ADDS {
                        dst: Argument::ct_reg(dsts[0].0),
                        src: Argument::ct_reg(dsts[0].0),
                        cst: Argument::pt_const(cst.0),
                    });
                }
            }
            HpuInstructionSet::Batch { block } => {
                let batch_map = BatchMap::from_op(&op);
                let reg_map: SmallMap<ValId, RegId> =
                    ((op.get_arg_valids().iter().cloned(), srcs.iter().cloned()).mzip())
                        .chain(
                            (op.get_return_valids().iter().cloned(), dsts.iter().cloned()).mzip(),
                        )
                        .collect();
                let translate = |v: ValId| -> RegId { *reg_map.get(&batch_map[v]).unwrap() };
                for op in block.walk_ops_linear() {
                    let args = op.get_arg_valids();
                    let rets = op.get_return_valids();
                    match op.get_instruction() {
                        HpuInstructionSet::Pbs { lut } => {
                            add_op(DopInstructionSet::PBS {
                                dst: Argument::ct_reg(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut),
                            });
                        }
                        HpuInstructionSet::PbsF { lut } => {
                            add_op(DopInstructionSet::PBS_F {
                                dst: Argument::ct_reg(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut),
                            });
                        }
                        HpuInstructionSet::Pbs2 { lut } => {
                            add_op(DopInstructionSet::PBS_ML2 {
                                dst: Argument::ct_reg2(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut),
                            });
                        }
                        HpuInstructionSet::Pbs2F { lut } => {
                            add_op(DopInstructionSet::PBS_ML2_F {
                                dst: Argument::ct_reg2(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut),
                            });
                        }
                        HpuInstructionSet::Pbs4 { lut } => {
                            add_op(DopInstructionSet::PBS_ML4 {
                                dst: Argument::ct_reg4(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut),
                            });
                        }
                        HpuInstructionSet::Pbs4F { lut } => {
                            add_op(DopInstructionSet::PBS_ML4_F {
                                dst: Argument::ct_reg4(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut),
                            });
                        }
                        HpuInstructionSet::Pbs8 { lut } => {
                            add_op(DopInstructionSet::PBS_ML8 {
                                dst: Argument::ct_reg8(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut),
                            });
                        }
                        HpuInstructionSet::Pbs8F { lut } => {
                            add_op(DopInstructionSet::PBS_ML8_F {
                                dst: Argument::ct_reg8(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut),
                            });
                        }
                        HpuInstructionSet::BatchArg { .. } | HpuInstructionSet::BatchRet { .. } => {
                        }
                        _ => unreachable!(
                            "Encountered unexpected operation while allocating: {}",
                            op.get_instruction()
                        ),
                    }
                }
            }
            _ => unreachable!(
                "Encountered unexpected operation while allocating: {}",
                op.get_instruction()
            ),
        }
    }

    output
}
