use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::{Lut1Def, Lut2Def};
use zhc_utils::n_bits_to_encode;

use crate::{
    BitType, Ciphertext, CiphertextBlock, NU, NU_BOOL, PropagationDirection, builder::Builder,
};

pub fn lead0(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let output = builder.iop_lead0(&src_a);
    builder.ciphertext_output(output);
    builder
}

pub fn lead1(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let output = builder.iop_lead1(&src_a);
    builder.ciphertext_output(output);
    builder
}

pub fn trail0(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let output = builder.iop_trail0(&src_a);
    builder.ciphertext_output(output);
    builder
}

pub fn trail1(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let output = builder.iop_trail1(&src_a);
    builder.ciphertext_output(output);
    builder
}

pub fn ilog2(spec: CiphertextSpec) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let src_a = builder.ciphertext_input(spec.int_size());
    let output = builder.iop_ilog2(&src_a);
    builder.ciphertext_output(output);
    builder
}

impl Builder {
    pub fn iop_ilog2(&self, src: impl AsRef<Ciphertext>) -> Ciphertext {
        let src = src.as_ref();
        assert!(
            src.spec().int_size().is_multiple_of(2),
            "Non-multiple-of-two integer size not supported."
        );
        let blocks = self.ciphertext_split(src);
        let bits = self.propagate_bits(blocks, BitType::One, PropagationDirection::MsbToLsb);
        let output_blocks =
            self.count_from_bits(&bits[1..src.spec().int_size() as usize], BitType::One);
        let output_size: u16 = n_bits_to_encode(src.spec().int_size());
        let n_blocks = output_size.div_ceil(src.spec().block_spec().message_size() as u16) as usize;
        self.ciphertext_join(&output_blocks[..n_blocks], Some(output_size))
    }

    pub fn iop_trail0(&self, src: impl AsRef<Ciphertext>) -> Ciphertext {
        let src = src.as_ref();
        assert!(
            src.spec().int_size().is_multiple_of(2),
            "Non-multiple-of-two integer size not supported."
        );
        let blocks = self.ciphertext_split(src);
        let bits = self.propagate_bits(blocks, BitType::One, PropagationDirection::LsbToMsb);
        let output_blocks =
            self.count_from_bits(&bits[0..src.spec().int_size() as usize], BitType::Zero);
        let output_size: u16 = n_bits_to_encode(src.spec().int_size());
        let n_blocks = output_size.div_ceil(src.spec().block_spec().message_size() as u16) as usize;
        self.ciphertext_join(&output_blocks[..n_blocks], Some(output_size))
    }

    pub fn iop_trail1(&self, src: impl AsRef<Ciphertext>) -> Ciphertext {
        let src = src.as_ref();
        assert!(
            src.spec().int_size().is_multiple_of(2),
            "Non-multiple-of-two integer size not supported."
        );
        let blocks = self.ciphertext_split(src);
        let bits = self.propagate_bits(blocks, BitType::Zero, PropagationDirection::LsbToMsb);
        let output_blocks =
            self.count_from_bits(&bits[0..src.spec().int_size() as usize], BitType::One);
        let output_size: u16 = n_bits_to_encode(src.spec().int_size());
        let n_blocks = output_size.div_ceil(src.spec().block_spec().message_size() as u16) as usize;
        self.ciphertext_join(&output_blocks[..n_blocks], Some(output_size))
    }

    pub fn iop_lead0(&self, src: impl AsRef<Ciphertext>) -> Ciphertext {
        let src = src.as_ref();
        assert!(
            src.spec().int_size().is_multiple_of(2),
            "Non-multiple-of-two integer size not supported."
        );
        let blocks = self.ciphertext_split(src);
        let bits = self.propagate_bits(blocks, BitType::One, PropagationDirection::MsbToLsb);
        let output_blocks =
            self.count_from_bits(&bits[0..src.spec().int_size() as usize], BitType::Zero);
        let output_size: u16 = n_bits_to_encode(src.spec().int_size());
        let n_blocks = output_size.div_ceil(src.spec().block_spec().message_size() as u16) as usize;
        self.ciphertext_join(&output_blocks[..n_blocks], Some(output_size))
    }

    pub fn iop_lead1(&self, src: impl AsRef<Ciphertext>) -> Ciphertext {
        let src = src.as_ref();
        assert!(
            src.spec().int_size().is_multiple_of(2),
            "Non-multiple-of-two integer size not supported."
        );
        let blocks = self.ciphertext_split(src);
        let bits = self.propagate_bits(blocks, BitType::Zero, PropagationDirection::MsbToLsb);
        let output_blocks =
            self.count_from_bits(&bits[0..src.spec().int_size() as usize], BitType::One);
        let output_size: u16 = n_bits_to_encode(src.spec().int_size());
        let n_blocks = output_size.div_ceil(src.spec().block_spec().message_size() as u16) as usize;
        self.ciphertext_join(&output_blocks[..n_blocks], Some(output_size))
    }

    pub(crate) fn propagate_bits(
        &self,
        src_a: impl AsRef<[CiphertextBlock]>,
        bit_type: BitType,
        direction: PropagationDirection,
    ) -> Vec<CiphertextBlock> {
        // Propagates bits of the given type

        self.push_comment("propagate_bits");
        let src_a = src_a.as_ref();

        let propagate_block = self.propagate_blocks(src_a, bit_type, direction, false);

        let mut res_v = Vec::new();
        for (idx, ct) in src_a.iter().enumerate() {
            // propagation start point
            let start_idx = if direction == PropagationDirection::LsbToMsb {
                0
            } else {
                src_a.len() - 1
            };
            let m = if idx == start_idx {
                ct.clone()
            } else {
                let neigh_idx = if direction == PropagationDirection::LsbToMsb {
                    idx - 1
                } else {
                    idx + 1
                };
                self.block_pack(propagate_block[neigh_idx], ct)
            };
            let v = if bit_type == BitType::One {
                if direction == PropagationDirection::LsbToMsb {
                    self.block_lookup2(m, Lut2Def::Manyl2mPropBit1MsgSplit)
                } else {
                    self.block_lookup2(m, Lut2Def::Manym2lPropBit1MsgSplit)
                }
            } else if direction == PropagationDirection::LsbToMsb {
                self.block_lookup2(m, Lut2Def::Manyl2mPropBit0MsgSplit)
            } else {
                self.block_lookup2(m, Lut2Def::Manym2lPropBit0MsgSplit)
            };
            res_v.push(v.0.clone());
            res_v.push(v.1.clone());
        }

        let output = self.comment("output").vector_inspect(res_v);

        self.pop_comment();

        output
    }

    pub(crate) fn propagate_blocks(
        &self,
        src_a: impl AsRef<[CiphertextBlock]>,
        bit_type: BitType,
        direction: PropagationDirection,
        inverse_output: bool,
    ) -> Vec<CiphertextBlock> {
        // Propagate bits of bit_type in a given direction, on a per-block basis.
        self.push_comment("propagate_blocks");

        let op_nb = NU;
        let op_nb_bool = NU_BOOL;

        let src_a = src_a.as_ref();

        let mut proc_nb = op_nb;
        let mut src = if bit_type.clone() == BitType::One {
            src_a.to_vec()
        } else {
            // Bitwise not
            // Do not clean the ct, but reduce the nb of sequential operations
            // in next step (reducing proc_nb).
            proc_nb -= 1;
            let cst_msg_max = self.block_let_plaintext(self.spec().message_mask() as u8);
            src_a
                .iter()
                .map(|ct| self.block_plaintext_sub(cst_msg_max, ct))
                .collect()
        };

        if direction == PropagationDirection::LsbToMsb {
            src.reverse();
        }

        // First step
        // Work within each group of proc_nb blocks.
        // For <i> get a boolean not null status of current block and the MSB ones.
        // within this group.
        let mut g_a: Vec<CiphertextBlock> = Vec::new();
        for (c_id, c) in src.chunks(proc_nb).enumerate() {
            c.iter().rev().fold(None, |acc, elt| {
                let is_not_null;
                let tmp;
                if let Some(x) = acc {
                    tmp = self.block_add(x, elt);
                    is_not_null = self.block_lookup(tmp, Lut1Def::NotNull);
                } else {
                    tmp = elt.clone();
                    is_not_null = self.block_lookup(elt, Lut1Def::NotNull);
                };
                g_a.insert(c_id * proc_nb, is_not_null); // Reverse insertion per chunk
                Some(tmp)
            });
        }

        // Second step
        // Proparate the not null status from MSB to LSB, with stride of
        // (op_nb_bool**k)*proc_nb
        // assert_eq!(g_a.len(),props.blk_w());
        let grp_nb = g_a.len().div_ceil(proc_nb);
        let mut level_nb = 0;
        let mut stride_size: usize = 1; // in group unit
        while stride_size < grp_nb {
            for chk in g_a.chunks_mut(op_nb_bool * stride_size * proc_nb) {
                chk.chunks_mut(stride_size * proc_nb)
                    .rev()
                    .fold(None, |acc, sub_chk| {
                        if let Some(x) = acc {
                            let tmp = self.block_add(x, sub_chk[0]);
                            sub_chk[0] = self.block_lookup(tmp, Lut1Def::NotNull);
                            Some(tmp)
                        } else {
                            Some(sub_chk[0].clone())
                        }
                    });
            }

            stride_size *= op_nb_bool;
            level_nb += 1;
        }

        // This code was written for a limited size, due the following
        // leveled additions.
        assert!(level_nb < op_nb_bool);

        // Third step
        // Apply
        let mut neigh_a: Vec<CiphertextBlock> = Vec::new();
        for _i in 1..level_nb {
            neigh_a.push(self.block_let_ciphertext(0));
        }

        let mut neigh = self.block_let_ciphertext(0);
        let mut prev = None;
        g_a.chunks_mut(proc_nb)
            .enumerate()
            .rev()
            .for_each(|(chk_idx, chk)| {
                let keep_v0 = chk[0].clone();

                let all_neigh = if let Some(x) = &prev {
                    self.block_add(neigh, x)
                } else {
                    neigh.clone()
                };

                for (idx, v) in chk.iter_mut().enumerate() {
                    if idx == 0 {
                        // [0] is already complete with prev.
                        // do not need to add prev
                        *v = self.block_add(&v, neigh);
                    } else {
                        *v = self.block_add(&v, all_neigh);
                    }
                    // Need to inverse it for 0 if needed
                    if inverse_output {
                        *v = self.block_lookup(&v, Lut1Def::IsNull);
                    } else {
                        *v = self.block_lookup(&v, Lut1Def::NotNull);
                    }
                }

                // For next chunk
                prev = Some(keep_v0.clone());

                // Update neighbors for next iteration
                let mut do_update_neigh = false;
                for i in 1..(level_nb as u32) {
                    if (chk_idx % op_nb_bool.pow(i)) == 0 {
                        // Update the corresponding neigh value
                        neigh_a[(i - 1) as usize] = keep_v0.clone();
                        do_update_neigh = true;
                    }
                }
                if do_update_neigh {
                    neigh = neigh_a[0].clone();
                    for n in neigh_a.iter().skip(1) {
                        neigh = self.block_add(neigh, n);
                    }
                }
            });

        if direction == PropagationDirection::LsbToMsb {
            g_a.reverse();
        }

        let output = self.comment("output").vector_inspect(g_a);

        self.pop_comment();

        output
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::n_bits_to_encode;

    #[test]
    fn correctness_lead0() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(inp)] = inp else {
                unreachable!()
            };
            let res =
                inp.as_storage().leading_zeros() - (u128::BITS - inp.spec().int_size() as u32);
            let output_size: u16 = n_bits_to_encode(inp.spec().int_size());
            vec![IopValue::Ciphertext(
                inp.spec()
                    .block_spec()
                    .ciphertext_spec(output_size)
                    .from_int(res as u128),
            )]
        }

        for size in (2..128).step_by(2) {
            lead0(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_lead1() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(inp)] = inp else {
                unreachable!()
            };
            let res =
                (inp.as_storage() << (u128::BITS - inp.spec().int_size() as u32)).leading_ones();
            let output_size: u16 = n_bits_to_encode(inp.spec().int_size());
            vec![IopValue::Ciphertext(
                inp.spec()
                    .block_spec()
                    .ciphertext_spec(output_size)
                    .from_int(res as u128),
            )]
        }

        for size in (2..128).step_by(2) {
            lead1(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_trail0() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(inp)] = inp else {
                unreachable!()
            };
            let res = inp
                .as_storage()
                .trailing_zeros()
                .min(inp.spec().int_size() as u32);
            let output_size: u16 = n_bits_to_encode(inp.spec().int_size());
            vec![IopValue::Ciphertext(
                inp.spec()
                    .block_spec()
                    .ciphertext_spec(output_size)
                    .from_int(res as u128),
            )]
        }

        for size in (2..128).step_by(2) {
            trail0(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_trail1() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(inp)] = inp else {
                unreachable!()
            };
            let res = inp.as_storage().trailing_ones();
            let output_size: u16 = n_bits_to_encode(inp.spec().int_size());
            vec![IopValue::Ciphertext(
                inp.spec()
                    .block_spec()
                    .ciphertext_spec(output_size)
                    .from_int(res as u128),
            )]
        }

        for size in (2..128).step_by(2) {
            trail1(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }

    #[test]
    fn correctness_ilog2() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(inp)] = inp else {
                unreachable!()
            };
            let res = inp.as_storage();
            let res = if res == 0 { 0 } else { res.ilog2() };
            let output_size: u16 = n_bits_to_encode(inp.spec().int_size());
            vec![IopValue::Ciphertext(
                inp.spec()
                    .block_spec()
                    .ciphertext_spec(output_size)
                    .from_int(res as u128),
            )]
        }

        for size in (2..128).step_by(2) {
            ilog2(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        }
    }
}
