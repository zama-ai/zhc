use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::{AnnIR, IR, ValId};
use zhc_langs::{
    doplang::{Argument, DopInstructionSet, DopLang},
    hpulang::{HpuInstructionSet, HpuLang},
};
use zhc_utils::{SafeAs, iter::MultiZip, small::SmallMap, svec};

use super::{
    allocator::{Alloc, Spill, Unspill},
    batch_map::BatchMap,
    register_file::RegId,
};

/// Lowers a register-allocated HPU program into a DOP instruction stream.
///
/// Walks `ir` — an HPU IR annotated with the allocator's [`Alloc`] decisions —
/// and emits, for each operation, the concrete DOP instructions realizing it:
/// the spills and unspills as `ST`/`LD`, the compute ops against their bound
/// registers, and inter-HPU transfers as the `WAIT`/`NOTIFY`/`LD_B2B` handshake
/// over the operation's reserved heap slots. The resulting stream is bracketed
/// between a leading `_START` and a trailing `_END` context marker.
pub fn translate<'ir>(ir: &AnnIR<'ir, HpuLang, Alloc, ()>, lut_reg: &LutRegistry) -> IR<DopLang> {
    use HpuInstructionSet::*;

    let mut output = IR::empty();
    let (_, val) = output.add_op(DopInstructionSet::_START, svec![]);
    let mut ctx = val[0];

    let mut add_op = |dop| {
        let (_, rets) = output.add_op(dop, svec![ctx]);
        ctx = rets[0];
    };

    for op in ir.walk_ops_linear() {
        let Alloc { slots, .. } = op.get_annotation();

        match op.get_instruction() {
            TransferIn { id, .. } => {
                add_op(DopInstructionSet::LD_B2B {
                    flag: Argument::UserFlag { flag: id.0 },
                    slot: Argument::ct_heap(slots[0].0 as usize),
                });
            }
            _ => {}
        }
    }

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
                dst: Argument::ct_heap(to.0.sas()),
                src: Argument::ct_reg(from.0),
            });
        }

        for Unspill { from, to } in unspills.iter() {
            add_op(DopInstructionSet::LD {
                dst: Argument::ct_reg(to.0),
                src: Argument::ct_heap(from.0.sas()),
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
            TransferOut { to, id, .. } => {
                add_op(DopInstructionSet::ST {
                    dst: Argument::ct_heap(slots[0].0 as usize),
                    src: Argument::ct_reg(srcs[0].0),
                });
                add_op(DopInstructionSet::NOTIFY {
                    virt_id: Argument::VirtId { id: to.0 },
                    flag: Argument::UserFlag { flag: id.0 },
                    slot: Argument::ct_heap(slots[0].0 as usize),
                });
            }
            TransferIn { id, .. } => {
                add_op(DopInstructionSet::WAIT {
                    flag: Argument::UserFlag { flag: id.0 },
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
            SubCt => {
                add_op(DopInstructionSet::SUB {
                    dst: Argument::ct_reg(dsts[0].0),
                    src1: Argument::ct_reg(srcs[0].0),
                    src2: Argument::ct_reg(srcs[1].0),
                });
            }
            Mac { cst } => {
                add_op(DopInstructionSet::MAC {
                    dst: Argument::ct_reg(dsts[0].0),
                    src1: Argument::ct_reg(srcs[0].0),
                    src2: Argument::ct_reg(srcs[1].0),
                    cst: Argument::pt_const(cst.0),
                });
            }
            AddPt => {
                let imm_ld_op = op
                    .get_args_iter()
                    .nth(1)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction()
                    .clone();
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
            SubPt => {
                let imm_ld_op = op
                    .get_args_iter()
                    .nth(1)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction()
                    .clone();
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
            PtSub => {
                let imm_ld_op = op
                    .get_args_iter()
                    .nth(0)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction()
                    .clone();
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
            MulPt => {
                let imm_ld_op = op
                    .get_args_iter()
                    .nth(1)
                    .unwrap()
                    .get_origin()
                    .opref
                    .get_instruction()
                    .clone();
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
            AddCst { cst } => {
                add_op(DopInstructionSet::ADDS {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_reg(srcs[0].0),
                    cst: Argument::pt_const(cst.0),
                });
            }
            SubCst { cst } => {
                add_op(DopInstructionSet::SUBS {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_reg(srcs[0].0),
                    cst: Argument::pt_const(cst.0),
                });
            }
            CstSub { cst } => {
                add_op(DopInstructionSet::SSUB {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_reg(srcs[0].0),
                    cst: Argument::pt_const(cst.0),
                });
            }
            MulCst { cst } => {
                add_op(DopInstructionSet::MULS {
                    dst: Argument::ct_reg(dsts[0].0),
                    src: Argument::ct_reg(srcs[0].0),
                    cst: Argument::pt_const(cst.0),
                });
            }
            CstCt { cst } => {
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
            Batch { block } => {
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
                        Pbs { lut } => {
                            add_op(DopInstructionSet::PBS {
                                dst: Argument::ct_reg(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut_reg.get_l1_lid(lut)),
                            });
                        }
                        PbsF { lut } => {
                            add_op(DopInstructionSet::PBS_F {
                                dst: Argument::ct_reg(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut_reg.get_l1_lid(lut)),
                            });
                        }
                        Pbs2 { lut } => {
                            add_op(DopInstructionSet::PBS_ML2 {
                                dst: Argument::ct_reg2(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut_reg.get_l2_lid(lut)),
                            });
                        }
                        Pbs2F { lut } => {
                            add_op(DopInstructionSet::PBS_ML2_F {
                                dst: Argument::ct_reg2(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut_reg.get_l2_lid(lut)),
                            });
                        }
                        Pbs4 { lut } => {
                            add_op(DopInstructionSet::PBS_ML4 {
                                dst: Argument::ct_reg4(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut_reg.get_l4_lid(lut)),
                            });
                        }
                        Pbs4F { lut } => {
                            add_op(DopInstructionSet::PBS_ML4_F {
                                dst: Argument::ct_reg4(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut_reg.get_l4_lid(lut)),
                            });
                        }
                        Pbs8 { lut } => {
                            add_op(DopInstructionSet::PBS_ML8 {
                                dst: Argument::ct_reg8(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut_reg.get_l8_lid(lut)),
                            });
                        }
                        Pbs8F { lut } => {
                            add_op(DopInstructionSet::PBS_ML8_F {
                                dst: Argument::ct_reg8(translate(rets[0]).0),
                                src: Argument::ct_reg(translate(args[0]).0),
                                lut: Argument::lut_id(lut_reg.get_l8_lid(lut)),
                            });
                        }
                        BatchArg { .. } | BatchRet { .. } => {}
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

    output.add_op(DopInstructionSet::_END, svec![ctx]);

    output
}
