use zhc_crypto::integer_semantics::CiphertextSpec;
use zhc_langs::ioplang::{Lut1Def, Lut2Def};
use zhc_utils::iter::{ChunkIt, UnwrapChunks};

use crate::{
    CiphertextBlock, NU,
    builder::{Builder, Ciphertext},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountKind {
    Zeros,
    Ones,
}

pub fn count_0(spec: CiphertextSpec) -> Builder {
    let mut builder = Builder::new(spec.block_spec());
    let src_a = builder.input_ciphertext(spec.int_size());
    let res = builder.iop_count(&src_a, CountKind::Zeros);
    builder.output_ciphertext(res);
    builder
}

type Column = Vec<CiphertextBlock>;

impl Builder {
    fn countx_reduce_rec(&mut self, inp: impl AsRef<[Column]>, kind: CountKind) -> Vec<Column> {
        // The workhorse recursive reduction implementation.
        let inp = inp.as_ref();

        if inp.iter().all(|col| col.len() <= 1) {
            // Reduction is finished, can return.
            return inp.to_vec();
        } else {
            let op_nb = NU;
            let op_nb_bool = op_nb.next_power_of_two();
            let op_nb_single = op_nb_bool - 1;
            let is_first_reduction_iteration = inp.len() == 1;

            let mut output: Vec<Column> = vec![Column::new(); inp.len() + 1];

            for (col_idx, col) in inp.iter().enumerate() {
                if col.len() == 1 {
                    output[col_idx].push(col[0]);
                } else if col_idx == inp.len() - 1 {
                    col.iter()
                        .cloned()
                        .chunk(op_nb_single)
                        .unwrap_chunks()
                        .map(|chunk| {
                            let sum = self.vector_add_reduce(&chunk);
                            if kind == CountKind::Zeros && is_first_reduction_iteration {
                                match chunk.len() {
                                    1 => self.block_lookup2(sum, Lut2Def::ManyInv1CarryMsg),
                                    2 => self.block_lookup2(sum, Lut2Def::ManyInv2CarryMsg),
                                    3 => self.block_lookup2(sum, Lut2Def::ManyInv3CarryMsg),
                                    4 => self.block_lookup2(sum, Lut2Def::ManyInv4CarryMsg),
                                    5 => self.block_lookup2(sum, Lut2Def::ManyInv5CarryMsg),
                                    6 => self.block_lookup2(sum, Lut2Def::ManyInv6CarryMsg),
                                    7 => self.block_lookup2(sum, Lut2Def::ManyInv7CarryMsg),
                                    _ => unreachable!(),
                                }
                            } else {
                                self.block_lookup2(sum, Lut2Def::ManyCarryMsg)
                            }
                        })
                        .for_each(|(msg, carry)| {
                            output[col_idx].push(msg);
                            output[col_idx + 1].push(carry);
                        });
                } else {
                    col.iter()
                        .cloned()
                        .chunk(op_nb)
                        .unwrap_chunks()
                        .map(|chunk| {
                            let sum = self.vector_add_reduce(&chunk);
                            if chunk.len() <= 2 {
                                // We have enough room to use a 2lookup in this case
                                self.block_lookup2(sum, Lut2Def::ManyCarryMsg)
                            } else {
                                // We don't have enough room. We must do two pbses
                                (
                                    self.block_lookup(sum, Lut1Def::MsgOnly),
                                    self.block_lookup(sum, Lut1Def::CarryOnly),
                                )
                            }
                        })
                        .for_each(|(msg, carry)| {
                            output[col_idx].push(msg);
                            output[col_idx + 1].push(carry);
                        });
                }
            }

            self.countx_reduce_rec(output, kind)
        }
    }

    pub fn iop_count(&mut self, inp: &Ciphertext, kind: CountKind) -> Ciphertext {
        let blocks = self.split_ciphertext(inp);
        let bits = self
            .vector_lookup2(blocks, Lut2Def::ManyMsgSplit)
            .into_iter()
            .flat_map(|(l, r)| [l, r].into_iter())
            .collect::<Vec<_>>();
        let res: Vec<Column> = self.countx_reduce_rec(vec![bits], kind);
        let res: Vec<CiphertextBlock> = res
            .into_iter()
            .filter(|col| !col.is_empty())
            .map(|col| col[0])
            .collect();
        self.join_ciphertext(res, None)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zhc_langs::ioplang::IopValue;
    use zhc_utils::assert_display_is;

    #[test]
    fn test_count0() {
        let spec = CiphertextSpec::new(18, 2, 2);
        let ir = count_0(spec).into_ir();
        assert_display_is!(
            ir.format()
                .with_walker(zhc_ir::PrintWalker::Linear)
                .show_comments(true)
                .show_types(false),
            r#"
                %0 = input_ciphertext<0, 18>();
                %1 = extract_ct_block<0>(%0);
                %2 = extract_ct_block<1>(%0);
                %3 = extract_ct_block<2>(%0);
                %4 = extract_ct_block<3>(%0);
                %5 = extract_ct_block<4>(%0);
                %6 = extract_ct_block<5>(%0);
                %7 = extract_ct_block<6>(%0);
                %8 = extract_ct_block<7>(%0);
                %9 = extract_ct_block<8>(%0);
                %10, %11 = pbs2<ManyMsgSplit>(%1);
                %12, %13 = pbs2<ManyMsgSplit>(%2);
                %14, %15 = pbs2<ManyMsgSplit>(%3);
                %16, %17 = pbs2<ManyMsgSplit>(%4);
                %18, %19 = pbs2<ManyMsgSplit>(%5);
                %20, %21 = pbs2<ManyMsgSplit>(%6);
                %22, %23 = pbs2<ManyMsgSplit>(%7);
                %24, %25 = pbs2<ManyMsgSplit>(%8);
                %26, %27 = pbs2<ManyMsgSplit>(%9);
                %28 = add_ct(%10, %11);
                %29 = add_ct(%28, %12);
                %30 = add_ct(%29, %13);
                %31 = add_ct(%30, %14);
                %32 = add_ct(%31, %15);
                %33 = add_ct(%32, %16);
                %34, %35 = pbs2<ManyInv7CarryMsg>(%33);
                %36 = add_ct(%17, %18);
                %37 = add_ct(%36, %19);
                %38 = add_ct(%37, %20);
                %39 = add_ct(%38, %21);
                %40 = add_ct(%39, %22);
                %41 = add_ct(%40, %23);
                %42, %43 = pbs2<ManyInv7CarryMsg>(%41);
                %44 = add_ct(%24, %25);
                %45 = add_ct(%44, %26);
                %46 = add_ct(%45, %27);
                %47, %48 = pbs2<ManyInv4CarryMsg>(%46);
                %49 = add_ct(%34, %42);
                %50 = add_ct(%49, %47);
                %51 = pbs<Protect, MsgOnly>(%50);
                %52 = pbs<Protect, CarryOnly>(%50);
                %53 = add_ct(%35, %43);
                %54 = add_ct(%53, %48);
                %55, %56 = pbs2<ManyCarryMsg>(%54);
                %57 = add_ct(%52, %55);
                %58, %59 = pbs2<ManyCarryMsg>(%57);
                %60 = add_ct(%59, %56);
                %61, %62 = pbs2<ManyCarryMsg>(%60);
                %63 = decl_ct<8>();
                %64 = store_ct_block<0>(%51, %63);
                %65 = store_ct_block<1>(%58, %64);
                %66 = store_ct_block<2>(%61, %65);
                %67 = store_ct_block<3>(%62, %66);
                output<0>(%67);
            "#
        );
    }

    #[test]
    fn correctness() {
        fn semantic(inp: &[IopValue]) -> Vec<IopValue> {
            let [IopValue::Ciphertext(inp)] = inp else {
                unreachable!()
            };
            let res = inp.as_storage().count_zeros() - (u128::BITS - inp.spec().int_size() as u32);
            vec![IopValue::Ciphertext(inp.spec().from_int(res as u128))]
        }
        count_0(CiphertextSpec::new(3, 2, 2)).test_random(100, semantic);

        // for size in (2..128).step_by(2) {
        //     // count_0(CiphertextSpec::new(size, 2, 2)).test_random(100, semantic);
        // }
    }
}
