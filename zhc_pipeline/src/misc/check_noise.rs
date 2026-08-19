use zhc_crypto::integer_semantics::PlaintextBlockSpec;
use zhc_ir::{AnnIR, AnnOpRef, IR};
use zhc_langs::ioplang::{IopInstructionSet, IopLang};
use zhc_utils::{Dumpable, existential_enum, iter::CollectInSmallVec, svec};

pub const MAX_ALLOWED_NOISE: Noise = Noise(12);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoiseBudget(Noise);

impl NoiseBudget {
    pub fn overshoots(&self) -> bool {
        self.0 > MAX_ALLOWED_NOISE
    }

    pub fn as_percent(&self) -> f32 {
        (self.0.0 as f32 / MAX_ALLOWED_NOISE.0 as f32) * 100.
    }
}

impl std::fmt::Debug for NoiseBudget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max = MAX_ALLOWED_NOISE.0 as usize;
        if self.overshoots() {
            write!(f, "{}  {:.0}% ⚠", "🮽".repeat(max), self.as_percent())
        } else {
            let used = self.0.0 as usize;
            let free = max.saturating_sub(used);
            let fill = if self.0 == Noise::FRESH { "▓" } else { "█" };
            write!(
                f,
                "{}{}   {:.0}%",
                fill.repeat(used),
                "░".repeat(free),
                self.as_percent()
            )?;
            if self.0 == Noise::FRESH {
                write!(f, " (fresh)")?;
            }
            Ok(())
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Noise(u8);

impl std::fmt::Debug for Noise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Noise {
    pub const NONE: Self = Noise(0);
    pub const FRESH: Self = Noise(1);
    pub const MAX: Self = Noise(u8::MAX);

    pub const fn add(lhs: Noise, rhs: Noise) -> Noise {
        Noise(lhs.0.saturating_add(rhs.0))
    }

    pub const fn mul_constant(lhs: Noise, pt: u8) -> Noise {
        Noise(lhs.0.saturating_mul(pt))
    }
}

impl std::fmt::Display for Noise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Noise::MAX {
            write!(f, "∞σ")
        } else {
            write!(f, "{}σ", self.0)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[existential_enum]
enum Ann {
    UnknownPlaintext(()),
    PlaintextBlock(u8),
    Noised(Noise),
}

pub fn check_noise(ir: &IR<IopLang>, spec: &PlaintextBlockSpec) {
    let ann = analyze_noise(ir, spec);
    let overshoots = ann
        .walk_vals_linear()
        .filter(|v| v.get_annotation().overshoots())
        .cosvec();
    if !overshoots.is_empty() {
        panic!(
            "Noise overshoot for values:\n{}",
            overshoots.dump_to_string()
        );
    }
    let dirty_outputs = ann
        .walk_ops_linear()
        .filter(|op| {
            matches!(
                op.get_instruction(),
                IopInstructionSet::OutputCiphertext { .. }
            )
        })
        .map(|op| op.get_args_iter().next().unwrap())
        .filter(|v| v.get_annotation().0 != Noise::FRESH)
        .cosvec();
    if !dirty_outputs.is_empty() {
        panic!("Dirty outputs in circuit: \n{}",dirty_outputs.dump_to_string())
    }
}

pub fn analyze_noise<'a>(
    ir: &'a IR<IopLang>,
    spec: &PlaintextBlockSpec,
) -> AnnIR<'a, IopLang, (), NoiseBudget> {
    ir.forward_dataflow_analysis(
        |op: AnnOpRef<'_, '_, _, zhc_ir::Analysing<()>, zhc_ir::Analysing<Ann>>| {
            use IopInstructionSet::*;
            let noises = match op.get_instruction() {
                InputCiphertext { .. } => svec![Ann::Noised(Noise::FRESH)],
                InputPlaintext { .. } => svec![Ann::UnknownPlaintext(())],
                OutputCiphertext { .. } => svec![],
                _Consume { .. } => svec![],
                Inspect { .. } => svec![
                    op.get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .unwrap_analyzed()
                ],
                DeclareCiphertext { .. } => svec![Ann::Noised(Noise::NONE)],
                LetPlaintextBlock { value } => svec![Ann::PlaintextBlock(*value)],
                LetCiphertextBlock { .. } => svec![Ann::Noised(Noise::NONE)],
                AddCt | WrappingAddCt | TemperAddCt | SubCt | WrappingSubCt => {
                    svec![Ann::Noised(Noise::add(
                        op.get_args_iter()
                            .nth(0)
                            .unwrap()
                            .get_annotation()
                            .unwrap_analyzed()
                            .unwrap_noised(),
                        op.get_args_iter()
                            .nth(1)
                            .unwrap()
                            .get_annotation()
                            .unwrap_analyzed()
                            .unwrap_noised(),
                    ))]
                }
                PackCt { mul } => svec![Ann::Noised(Noise::add(
                    Noise::mul_constant(
                        op.get_args_iter()
                            .nth(0)
                            .unwrap()
                            .get_annotation()
                            .unwrap_analyzed()
                            .unwrap_noised(),
                        *mul
                    ),
                    op.get_args_iter()
                        .nth(1)
                        .unwrap()
                        .get_annotation()
                        .unwrap_analyzed()
                        .unwrap_noised(),
                ))],
                AddPt | WrappingAddPt | SubPt => svec![
                    op.get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .unwrap_analyzed()
                ],
                PtSub => svec![
                    op.get_args_iter()
                        .nth(1)
                        .unwrap()
                        .get_annotation()
                        .unwrap_analyzed()
                ],
                MulPt => {
                    let noise = op
                        .get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .unwrap_analyzed()
                        .unwrap_noised();
                    let mul = op
                        .get_args_iter()
                        .nth(1)
                        .unwrap()
                        .get_annotation()
                        .unwrap_analyzed()
                        .unwrap_plaintext_block();
                    svec![Ann::Noised(Noise::mul_constant(noise, mul))]
                }
                ExtractCtBlock { .. } => svec![
                    op.get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .unwrap_analyzed()
                ],
                ExtractPtBlock { .. } => {
                    assert!(
                        op.get_args_iter()
                            .nth(0)
                            .unwrap()
                            .get_annotation()
                            .unwrap_analyzed()
                            .is_unknown_plaintext()
                    );
                    svec![Ann::PlaintextBlock((1 << spec.message_size()) - 1)]
                }
                StoreCtBlock { .. } => svec![Ann::Noised(std::cmp::max(
                    op.get_args_iter()
                        .nth(0)
                        .unwrap()
                        .get_annotation()
                        .unwrap_analyzed()
                        .unwrap_noised(),
                    op.get_args_iter()
                        .nth(1)
                        .unwrap()
                        .get_annotation()
                        .unwrap_analyzed()
                        .unwrap_noised()
                ))],
                Pbs { .. } => svec![Ann::Noised(Noise::FRESH)],
                Pbs2 { .. } => svec![Ann::Noised(Noise::FRESH), Ann::Noised(Noise::FRESH)],
            };
            ((), noises)
        },
    )
    .map_valann(|op| match op.get_annotation() {
        Ann::UnknownPlaintext(_) => NoiseBudget(Noise::NONE),
        Ann::PlaintextBlock(_) => NoiseBudget(Noise::NONE),
        Ann::Noised(noise) => NoiseBudget(*noise),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use zhc_builder::CiphertextBlockSpec;
    use zhc_crypto::integer_semantics::lut::{LookupCheck, Lut1, Lut2};
    use zhc_langs::ioplang::IopTypeSystem;
    use zhc_utils::assert_display_is;

    #[test]
    fn add_is_exact_and_saturating() {
        assert_eq!(Noise::add(Noise(6), Noise(9)), Noise(15));
        assert_eq!(Noise::add(Noise::NONE, Noise::FRESH), Noise::FRESH);
        assert_eq!(Noise::add(Noise(254), Noise(2)), Noise::MAX);
        assert_eq!(Noise::add(Noise::MAX, Noise::NONE), Noise::MAX);
    }

    #[test]
    fn mul_constant_is_exact_and_saturating() {
        assert_eq!(Noise::mul_constant(Noise(3), 8), Noise(24));
        assert_eq!(Noise::mul_constant(Noise::FRESH, 0), Noise::NONE);
        assert_eq!(Noise::mul_constant(Noise(100), 3), Noise::MAX);
        assert_eq!(Noise::mul_constant(Noise::MAX, 2), Noise::MAX);
    }

    #[test]
    fn ordering_matches_values() {
        assert!(Noise::NONE < Noise::FRESH);
        assert!(Noise(12) < Noise::MAX);
    }

    #[test]
    fn display_prints_units() {
        assert_display_is!(Noise::NONE, "0σ");
        assert_display_is!(Noise(12), "12σ");
        assert_display_is!(Noise::MAX, "∞σ");
    }

    #[test]
    fn analyze_inputs_and_constants() {
        use IopInstructionSet::*;
        let mut ir: IR<IopLang> = IR::empty();
        ir.add_op(
            InputCiphertext {
                pos: 0,
                int_size: 4,
            },
            svec![],
        );
        ir.add_op(
            InputPlaintext {
                pos: 1,
                int_size: 4,
            },
            svec![],
        );
        ir.add_op(DeclareCiphertext { int_size: 4 }, svec![]);
        ir.add_op(LetCiphertextBlock { value: 2 }, svec![]);
        ir.add_op(LetPlaintextBlock { value: 3 }, svec![]);

        assert_display_is!(
            analyze_noise(&ir, &PlaintextBlockSpec(2)).format(),
            r#"
                %0 = input_ciphertext<0, 4>();
                    %0 -> ▓░░░░░░░░░░░   8% (fresh)
                %1 = input_plaintext<1, 4>();
                    %1 -> ░░░░░░░░░░░░   0%
                %2 = decl_ct<4>();
                    %2 -> ░░░░░░░░░░░░   0%
                %3 = let_ct_block<2>();
                    %3 -> ░░░░░░░░░░░░   0%
                %4 = let_pt_block<3>();
                    %4 -> ░░░░░░░░░░░░   0%
            "#
        );
    }

    #[test]
    fn analyze_extracts_and_inspect_forward_annotations() {
        use IopInstructionSet::*;
        let mut ir: IR<IopLang> = IR::empty();
        let (_, ct) = ir.add_op(
            InputCiphertext {
                pos: 0,
                int_size: 4,
            },
            svec![],
        );
        let (_, blk) = ir.add_op(ExtractCtBlock { index: 3 }, svec![ct[0]]);
        let (_, pt) = ir.add_op(
            InputPlaintext {
                pos: 1,
                int_size: 4,
            },
            svec![],
        );
        ir.add_op(ExtractPtBlock { index: 0 }, svec![pt[0]]);
        ir.add_op(
            Inspect {
                typ: IopTypeSystem::CiphertextBlock,
            },
            svec![blk[0]],
        );

        // Extracting keeps the ciphertext bound; an extracted plaintext block
        // is bounded by the 2-bit message maximum, 3.
        assert_display_is!(
            analyze_noise(&ir, &PlaintextBlockSpec(2)).format(),
            r#"
                %0 = input_ciphertext<0, 4>();
                    %0 -> ▓░░░░░░░░░░░   8% (fresh)
                %1 = extract_ct_block<3>(%0);
                    %1 -> ▓░░░░░░░░░░░   8% (fresh)
                %2 = input_plaintext<1, 4>();
                    %2 -> ░░░░░░░░░░░░   0%
                %3 = extract_pt_block<0>(%2);
                    %3 -> ░░░░░░░░░░░░   0%
                %4 = inspect(%1);
                    %4 -> ▓░░░░░░░░░░░   8% (fresh)
            "#
        );
    }

    #[test]
    fn analyze_ct_ct_ops_add_noises() {
        use IopInstructionSet::*;
        let mut ir: IR<IopLang> = IR::empty();
        let (_, a_ct) = ir.add_op(
            InputCiphertext {
                pos: 0,
                int_size: 4,
            },
            svec![],
        );
        let (_, a) = ir.add_op(ExtractCtBlock { index: 0 }, svec![a_ct[0]]);
        let (_, b_ct) = ir.add_op(
            InputCiphertext {
                pos: 1,
                int_size: 4,
            },
            svec![],
        );
        let (_, b) = ir.add_op(ExtractCtBlock { index: 0 }, svec![b_ct[0]]);
        let (_, s1) = ir.add_op(AddCt, svec![a[0], b[0]]);
        let (_, s2) = ir.add_op(WrappingAddCt, svec![s1[0], a[0]]);
        let (_, s3) = ir.add_op(TemperAddCt, svec![s2[0], s1[0]]);
        let (_, s4) = ir.add_op(SubCt, svec![s3[0], a[0]]);
        ir.add_op(WrappingSubCt, svec![s4[0], s3[0]]);

        // Every ciphertext-ciphertext flavor adds the two operand bounds.
        assert_display_is!(
            analyze_noise(&ir, &PlaintextBlockSpec(2)).format(),
            r#"
                %0 = input_ciphertext<0, 4>();
                    %0 -> ▓░░░░░░░░░░░   8% (fresh)
                %1 = extract_ct_block<0>(%0);
                    %1 -> ▓░░░░░░░░░░░   8% (fresh)
                %2 = input_ciphertext<1, 4>();
                    %2 -> ▓░░░░░░░░░░░   8% (fresh)
                %3 = extract_ct_block<0>(%2);
                    %3 -> ▓░░░░░░░░░░░   8% (fresh)
                %4 = add_ct(%1, %3);
                    %4 -> ██░░░░░░░░░░   17%
                %5 = wrapping_add_ct(%4, %1);
                    %5 -> ███░░░░░░░░░   25%
                %6 = temper_add_ct(%5, %4);
                    %6 -> █████░░░░░░░   42%
                %7 = sub_ct(%6, %1);
                    %7 -> ██████░░░░░░   50%
                %8 = wrapping_sub_ct(%7, %6);
                    %8 -> ███████████░   92%
            "#
        );
    }

    #[test]
    fn analyze_ct_pt_ops_forward_ciphertext_noise() {
        use IopInstructionSet::*;
        let mut ir: IR<IopLang> = IR::empty();
        let (_, ct) = ir.add_op(
            InputCiphertext {
                pos: 0,
                int_size: 4,
            },
            svec![],
        );
        let (_, blk) = ir.add_op(ExtractCtBlock { index: 0 }, svec![ct[0]]);
        let (_, noisy) = ir.add_op(AddCt, svec![blk[0], blk[0]]);
        let (_, pt) = ir.add_op(LetPlaintextBlock { value: 3 }, svec![]);
        let (_, r1) = ir.add_op(AddPt, svec![noisy[0], pt[0]]);
        let (_, r2) = ir.add_op(WrappingAddPt, svec![r1[0], pt[0]]);
        let (_, r3) = ir.add_op(SubPt, svec![r2[0], pt[0]]);
        ir.add_op(PtSub, svec![pt[0], r3[0]]);

        // Plaintext operands carry no noise: the 2σ bound passes through
        // every mixed flavor, including `pt_sub` where the ciphertext is the
        // second argument.
        assert_display_is!(
            analyze_noise(&ir, &PlaintextBlockSpec(2)).format(),
            r#"
                %0 = input_ciphertext<0, 4>();
                    %0 -> ▓░░░░░░░░░░░   8% (fresh)
                %1 = extract_ct_block<0>(%0);
                    %1 -> ▓░░░░░░░░░░░   8% (fresh)
                %2 = add_ct(%1, %1);
                    %2 -> ██░░░░░░░░░░   17%
                %3 = let_pt_block<3>();
                    %3 -> ░░░░░░░░░░░░   0%
                %4 = add_pt(%2, %3);
                    %4 -> ██░░░░░░░░░░   17%
                %5 = wrapping_add_pt(%4, %3);
                    %5 -> ██░░░░░░░░░░   17%
                %6 = sub_pt(%5, %3);
                    %6 -> ██░░░░░░░░░░   17%
                %7 = pt_sub(%3, %6);
                    %7 -> ██░░░░░░░░░░   17%
            "#
        );
    }

    #[test]
    fn analyze_mul_pt_scales_by_constant_or_worst_case() {
        use IopInstructionSet::*;
        let mut ir: IR<IopLang> = IR::empty();
        let (_, ct) = ir.add_op(
            InputCiphertext {
                pos: 0,
                int_size: 4,
            },
            svec![],
        );
        let (_, blk) = ir.add_op(ExtractCtBlock { index: 0 }, svec![ct[0]]);
        let (_, pt) = ir.add_op(LetPlaintextBlock { value: 2 }, svec![]);
        ir.add_op(MulPt, svec![blk[0], pt[0]]);
        let (_, unknown) = ir.add_op(
            InputPlaintext {
                pos: 1,
                int_size: 4,
            },
            svec![],
        );
        let (_, pt_blk) = ir.add_op(ExtractPtBlock { index: 0 }, svec![unknown[0]]);
        ir.add_op(MulPt, svec![blk[0], pt_blk[0]]);

        // A known constant scales exactly; an unknown multiplier is bounded
        // by the 2-bit message maximum, 3.
        assert_display_is!(
            analyze_noise(&ir, &PlaintextBlockSpec(2)).format(),
            r#"
                %0 = input_ciphertext<0, 4>();
                    %0 -> ▓░░░░░░░░░░░   8% (fresh)
                %1 = extract_ct_block<0>(%0);
                    %1 -> ▓░░░░░░░░░░░   8% (fresh)
                %2 = let_pt_block<2>();
                    %2 -> ░░░░░░░░░░░░   0%
                %3 = mul_pt(%1, %2);
                    %3 -> ██░░░░░░░░░░   17%
                %4 = input_plaintext<1, 4>();
                    %4 -> ░░░░░░░░░░░░   0%
                %5 = extract_pt_block<0>(%4);
                    %5 -> ░░░░░░░░░░░░   0%
                %6 = mul_pt(%1, %5);
                    %6 -> ███░░░░░░░░░   25%
            "#
        );
    }

    #[test]
    fn analyze_pack_ct_scales_high_block_and_adds_low_block() {
        use IopInstructionSet::*;
        let mut ir: IR<IopLang> = IR::empty();
        let (_, hi_ct) = ir.add_op(
            InputCiphertext {
                pos: 0,
                int_size: 4,
            },
            svec![],
        );
        let (_, hi_raw) = ir.add_op(ExtractCtBlock { index: 0 }, svec![hi_ct[0]]);
        let (_, hi) = ir.add_op(AddCt, svec![hi_raw[0], hi_raw[0]]);
        let (_, lo_ct) = ir.add_op(
            InputCiphertext {
                pos: 1,
                int_size: 4,
            },
            svec![],
        );
        let (_, lo) = ir.add_op(ExtractCtBlock { index: 0 }, svec![lo_ct[0]]);
        ir.add_op(PackCt { mul: 4 }, svec![hi[0], lo[0]]);

        // pack = hi * 4 + lo, so noise = 2σ * 4 + 1σ = 9σ.
        assert_display_is!(
            analyze_noise(&ir, &PlaintextBlockSpec(2)).format(),
            r#"
                %0 = input_ciphertext<0, 4>();
                    %0 -> ▓░░░░░░░░░░░   8% (fresh)
                %1 = extract_ct_block<0>(%0);
                    %1 -> ▓░░░░░░░░░░░   8% (fresh)
                %2 = add_ct(%1, %1);
                    %2 -> ██░░░░░░░░░░   17%
                %3 = input_ciphertext<1, 4>();
                    %3 -> ▓░░░░░░░░░░░   8% (fresh)
                %4 = extract_ct_block<0>(%3);
                    %4 -> ▓░░░░░░░░░░░   8% (fresh)
                %5 = pack_ct<4>(%2, %4);
                    %5 -> █████████░░░   75%
            "#
        );
    }

    #[test]
    fn analyze_store_ct_block_takes_max_noise() {
        use IopInstructionSet::*;
        let mut ir: IR<IopLang> = IR::empty();
        let (_, ct) = ir.add_op(
            InputCiphertext {
                pos: 0,
                int_size: 4,
            },
            svec![],
        );
        let (_, clean) = ir.add_op(LetCiphertextBlock { value: 0 }, svec![]);
        let (_, stored) = ir.add_op(StoreCtBlock { index: 0 }, svec![clean[0], ct[0]]);
        let (_, blk) = ir.add_op(ExtractCtBlock { index: 0 }, svec![ct[0]]);
        let (_, noisy) = ir.add_op(AddCt, svec![blk[0], blk[0]]);
        ir.add_op(StoreCtBlock { index: 1 }, svec![noisy[0], stored[0]]);

        // Storing a clean block keeps the composite bound; storing a noisier
        // block raises it.
        assert_display_is!(
            analyze_noise(&ir, &PlaintextBlockSpec(2)).format(),
            r#"
                %0 = input_ciphertext<0, 4>();
                    %0 -> ▓░░░░░░░░░░░   8% (fresh)
                %1 = let_ct_block<0>();
                    %1 -> ░░░░░░░░░░░░   0%
                %2 = store_ct_block<0>(%1, %0);
                    %2 -> ▓░░░░░░░░░░░   8% (fresh)
                %3 = extract_ct_block<0>(%0);
                    %3 -> ▓░░░░░░░░░░░   8% (fresh)
                %4 = add_ct(%3, %3);
                    %4 -> ██░░░░░░░░░░   17%
                %5 = store_ct_block<1>(%4, %2);
                    %5 -> ██░░░░░░░░░░   17%
            "#
        );
    }

    #[test]
    fn analyze_pbs_resets_noise() {
        use IopInstructionSet::*;
        let mut ir: IR<IopLang> = IR::empty();
        let (_, ct) = ir.add_op(
            InputCiphertext {
                pos: 0,
                int_size: 4,
            },
            svec![],
        );
        let (_, blk) = ir.add_op(ExtractCtBlock { index: 0 }, svec![ct[0]]);
        let (_, noisy) = ir.add_op(AddCt, svec![blk[0], blk[0]]);
        ir.add_op(
            Pbs {
                check: LookupCheck::Protect,
                lut: Lut1::from_fn("identity", CiphertextBlockSpec(2, 2), |b| b),
            },
            svec![noisy[0]],
        );
        ir.add_op(
            Pbs2 {
                check: LookupCheck::Protect,
                lut: Lut2::from_fn("identity2", CiphertextBlockSpec(2, 2), |b| b, |b| b),
            },
            svec![noisy[0]],
        );

        // Bootstrapping resets every output block to a fresh bound.
        assert_display_is!(
            analyze_noise(&ir, &PlaintextBlockSpec(2)).format(),
            r#"
                %0 = input_ciphertext<0, 4>();
                    %0 -> ▓░░░░░░░░░░░   8% (fresh)
                %1 = extract_ct_block<0>(%0);
                    %1 -> ▓░░░░░░░░░░░   8% (fresh)
                %2 = add_ct(%1, %1);
                    %2 -> ██░░░░░░░░░░   17%
                %3 = pbs<Protect, Lut1("identity")>(%2);
                    %3 -> ▓░░░░░░░░░░░   8% (fresh)
                %4, %5 = pbs2<Protect, Lut2("identity2")>(%2);
                    %4 -> ▓░░░░░░░░░░░   8% (fresh)
                    %5 -> ▓░░░░░░░░░░░   8% (fresh)
            "#
        );
    }

    #[test]
    fn analyze_diamond_reuses_shared_noise() {
        use IopInstructionSet::*;
        let mut ir: IR<IopLang> = IR::empty();
        let (_, ct) = ir.add_op(
            InputCiphertext {
                pos: 0,
                int_size: 4,
            },
            svec![],
        );
        let (_, x) = ir.add_op(ExtractCtBlock { index: 0 }, svec![ct[0]]);
        let (_, a) = ir.add_op(AddCt, svec![x[0], x[0]]);
        let (_, b) = ir.add_op(AddCt, svec![a[0], a[0]]);
        ir.add_op(AddCt, svec![a[0], b[0]]);

        assert_display_is!(
            analyze_noise(&ir, &PlaintextBlockSpec(2)).format(),
            r#"
                %0 = input_ciphertext<0, 4>();
                    %0 -> ▓░░░░░░░░░░░   8% (fresh)
                %1 = extract_ct_block<0>(%0);
                    %1 -> ▓░░░░░░░░░░░   8% (fresh)
                %2 = add_ct(%1, %1);
                    %2 -> ██░░░░░░░░░░   17%
                %3 = add_ct(%2, %2);
                    %3 -> ████░░░░░░░░   33%
                %4 = add_ct(%2, %3);
                    %4 -> ██████░░░░░░   50%
            "#
        );
    }

    #[test]
    fn analyze_counts_units_along_long_addition_chains() {
        use IopInstructionSet::*;
        let mut ir: IR<IopLang> = IR::empty();
        let (_, ct) = ir.add_op(
            InputCiphertext {
                pos: 0,
                int_size: 4,
            },
            svec![],
        );
        let (_, blk) = ir.add_op(ExtractCtBlock { index: 0 }, svec![ct[0]]);
        let mut acc = blk[0];
        for pos in 1..=30 {
            let (_, ct) = ir.add_op(InputCiphertext { pos, int_size: 4 }, svec![]);
            let (_, blk) = ir.add_op(ExtractCtBlock { index: 0 }, svec![ct[0]]);
            let (_, sum) = ir.add_op(AddCt, svec![acc, blk[0]]);
            acc = sum[0];
        }
        let analyzed = analyze_noise(&ir, &PlaintextBlockSpec(2));
        assert_display_is!(
            analyzed.format(),
            r#"
            %0 = input_ciphertext<0, 4>();
                %0 -> ▓░░░░░░░░░░░   8% (fresh)
            %1 = extract_ct_block<0>(%0);
                %1 -> ▓░░░░░░░░░░░   8% (fresh)
            %2 = input_ciphertext<1, 4>();
                %2 -> ▓░░░░░░░░░░░   8% (fresh)
            %3 = extract_ct_block<0>(%2);
                %3 -> ▓░░░░░░░░░░░   8% (fresh)
            %4 = add_ct(%1, %3);
                %4 -> ██░░░░░░░░░░   17%
            %5 = input_ciphertext<2, 4>();
                %5 -> ▓░░░░░░░░░░░   8% (fresh)
            %6 = extract_ct_block<0>(%5);
                %6 -> ▓░░░░░░░░░░░   8% (fresh)
            %7 = add_ct(%4, %6);
                %7 -> ███░░░░░░░░░   25%
            %8 = input_ciphertext<3, 4>();
                %8 -> ▓░░░░░░░░░░░   8% (fresh)
            %9 = extract_ct_block<0>(%8);
                %9 -> ▓░░░░░░░░░░░   8% (fresh)
            %10 = add_ct(%7, %9);
                %10 -> ████░░░░░░░░   33%
            %11 = input_ciphertext<4, 4>();
                %11 -> ▓░░░░░░░░░░░   8% (fresh)
            %12 = extract_ct_block<0>(%11);
                %12 -> ▓░░░░░░░░░░░   8% (fresh)
            %13 = add_ct(%10, %12);
                %13 -> █████░░░░░░░   42%
            %14 = input_ciphertext<5, 4>();
                %14 -> ▓░░░░░░░░░░░   8% (fresh)
            %15 = extract_ct_block<0>(%14);
                %15 -> ▓░░░░░░░░░░░   8% (fresh)
            %16 = add_ct(%13, %15);
                %16 -> ██████░░░░░░   50%
            %17 = input_ciphertext<6, 4>();
                %17 -> ▓░░░░░░░░░░░   8% (fresh)
            %18 = extract_ct_block<0>(%17);
                %18 -> ▓░░░░░░░░░░░   8% (fresh)
            %19 = add_ct(%16, %18);
                %19 -> ███████░░░░░   58%
            %20 = input_ciphertext<7, 4>();
                %20 -> ▓░░░░░░░░░░░   8% (fresh)
            %21 = extract_ct_block<0>(%20);
                %21 -> ▓░░░░░░░░░░░   8% (fresh)
            %22 = add_ct(%19, %21);
                %22 -> ████████░░░░   67%
            %23 = input_ciphertext<8, 4>();
                %23 -> ▓░░░░░░░░░░░   8% (fresh)
            %24 = extract_ct_block<0>(%23);
                %24 -> ▓░░░░░░░░░░░   8% (fresh)
            %25 = add_ct(%22, %24);
                %25 -> █████████░░░   75%
            %26 = input_ciphertext<9, 4>();
                %26 -> ▓░░░░░░░░░░░   8% (fresh)
            %27 = extract_ct_block<0>(%26);
                %27 -> ▓░░░░░░░░░░░   8% (fresh)
            %28 = add_ct(%25, %27);
                %28 -> ██████████░░   83%
            %29 = input_ciphertext<10, 4>();
                %29 -> ▓░░░░░░░░░░░   8% (fresh)
            %30 = extract_ct_block<0>(%29);
                %30 -> ▓░░░░░░░░░░░   8% (fresh)
            %31 = add_ct(%28, %30);
                %31 -> ███████████░   92%
            %32 = input_ciphertext<11, 4>();
                %32 -> ▓░░░░░░░░░░░   8% (fresh)
            %33 = extract_ct_block<0>(%32);
                %33 -> ▓░░░░░░░░░░░   8% (fresh)
            %34 = add_ct(%31, %33);
                %34 -> ████████████   100%
            %35 = input_ciphertext<12, 4>();
                %35 -> ▓░░░░░░░░░░░   8% (fresh)
            %36 = extract_ct_block<0>(%35);
                %36 -> ▓░░░░░░░░░░░   8% (fresh)
            %37 = add_ct(%34, %36);
                %37 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  108% ⚠
            %38 = input_ciphertext<13, 4>();
                %38 -> ▓░░░░░░░░░░░   8% (fresh)
            %39 = extract_ct_block<0>(%38);
                %39 -> ▓░░░░░░░░░░░   8% (fresh)
            %40 = add_ct(%37, %39);
                %40 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  117% ⚠
            %41 = input_ciphertext<14, 4>();
                %41 -> ▓░░░░░░░░░░░   8% (fresh)
            %42 = extract_ct_block<0>(%41);
                %42 -> ▓░░░░░░░░░░░   8% (fresh)
            %43 = add_ct(%40, %42);
                %43 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  125% ⚠
            %44 = input_ciphertext<15, 4>();
                %44 -> ▓░░░░░░░░░░░   8% (fresh)
            %45 = extract_ct_block<0>(%44);
                %45 -> ▓░░░░░░░░░░░   8% (fresh)
            %46 = add_ct(%43, %45);
                %46 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  133% ⚠
            %47 = input_ciphertext<16, 4>();
                %47 -> ▓░░░░░░░░░░░   8% (fresh)
            %48 = extract_ct_block<0>(%47);
                %48 -> ▓░░░░░░░░░░░   8% (fresh)
            %49 = add_ct(%46, %48);
                %49 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  142% ⚠
            %50 = input_ciphertext<17, 4>();
                %50 -> ▓░░░░░░░░░░░   8% (fresh)
            %51 = extract_ct_block<0>(%50);
                %51 -> ▓░░░░░░░░░░░   8% (fresh)
            %52 = add_ct(%49, %51);
                %52 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  150% ⚠
            %53 = input_ciphertext<18, 4>();
                %53 -> ▓░░░░░░░░░░░   8% (fresh)
            %54 = extract_ct_block<0>(%53);
                %54 -> ▓░░░░░░░░░░░   8% (fresh)
            %55 = add_ct(%52, %54);
                %55 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  158% ⚠
            %56 = input_ciphertext<19, 4>();
                %56 -> ▓░░░░░░░░░░░   8% (fresh)
            %57 = extract_ct_block<0>(%56);
                %57 -> ▓░░░░░░░░░░░   8% (fresh)
            %58 = add_ct(%55, %57);
                %58 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  167% ⚠
            %59 = input_ciphertext<20, 4>();
                %59 -> ▓░░░░░░░░░░░   8% (fresh)
            %60 = extract_ct_block<0>(%59);
                %60 -> ▓░░░░░░░░░░░   8% (fresh)
            %61 = add_ct(%58, %60);
                %61 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  175% ⚠
            %62 = input_ciphertext<21, 4>();
                %62 -> ▓░░░░░░░░░░░   8% (fresh)
            %63 = extract_ct_block<0>(%62);
                %63 -> ▓░░░░░░░░░░░   8% (fresh)
            %64 = add_ct(%61, %63);
                %64 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  183% ⚠
            %65 = input_ciphertext<22, 4>();
                %65 -> ▓░░░░░░░░░░░   8% (fresh)
            %66 = extract_ct_block<0>(%65);
                %66 -> ▓░░░░░░░░░░░   8% (fresh)
            %67 = add_ct(%64, %66);
                %67 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  192% ⚠
            %68 = input_ciphertext<23, 4>();
                %68 -> ▓░░░░░░░░░░░   8% (fresh)
            %69 = extract_ct_block<0>(%68);
                %69 -> ▓░░░░░░░░░░░   8% (fresh)
            %70 = add_ct(%67, %69);
                %70 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  200% ⚠
            %71 = input_ciphertext<24, 4>();
                %71 -> ▓░░░░░░░░░░░   8% (fresh)
            %72 = extract_ct_block<0>(%71);
                %72 -> ▓░░░░░░░░░░░   8% (fresh)
            %73 = add_ct(%70, %72);
                %73 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  208% ⚠
            %74 = input_ciphertext<25, 4>();
                %74 -> ▓░░░░░░░░░░░   8% (fresh)
            %75 = extract_ct_block<0>(%74);
                %75 -> ▓░░░░░░░░░░░   8% (fresh)
            %76 = add_ct(%73, %75);
                %76 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  217% ⚠
            %77 = input_ciphertext<26, 4>();
                %77 -> ▓░░░░░░░░░░░   8% (fresh)
            %78 = extract_ct_block<0>(%77);
                %78 -> ▓░░░░░░░░░░░   8% (fresh)
            %79 = add_ct(%76, %78);
                %79 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  225% ⚠
            %80 = input_ciphertext<27, 4>();
                %80 -> ▓░░░░░░░░░░░   8% (fresh)
            %81 = extract_ct_block<0>(%80);
                %81 -> ▓░░░░░░░░░░░   8% (fresh)
            %82 = add_ct(%79, %81);
                %82 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  233% ⚠
            %83 = input_ciphertext<28, 4>();
                %83 -> ▓░░░░░░░░░░░   8% (fresh)
            %84 = extract_ct_block<0>(%83);
                %84 -> ▓░░░░░░░░░░░   8% (fresh)
            %85 = add_ct(%82, %84);
                %85 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  242% ⚠
            %86 = input_ciphertext<29, 4>();
                %86 -> ▓░░░░░░░░░░░   8% (fresh)
            %87 = extract_ct_block<0>(%86);
                %87 -> ▓░░░░░░░░░░░   8% (fresh)
            %88 = add_ct(%85, %87);
                %88 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  250% ⚠
            %89 = input_ciphertext<30, 4>();
                %89 -> ▓░░░░░░░░░░░   8% (fresh)
            %90 = extract_ct_block<0>(%89);
                %90 -> ▓░░░░░░░░░░░   8% (fresh)
            %91 = add_ct(%88, %90);
                %91 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  258% ⚠
        "#
        );
    }

    #[test]
    fn analyze_saturates_on_repeated_packing() {
        use IopInstructionSet::*;
        let mut ir: IR<IopLang> = IR::empty();
        let (_, ct) = ir.add_op(
            InputCiphertext {
                pos: 0,
                int_size: 4,
            },
            svec![],
        );
        let (_, blk) = ir.add_op(ExtractCtBlock { index: 0 }, svec![ct[0]]);
        let mut acc = blk[0];
        // Each pack multiplies the bound by 4 and adds 1: 1, 5, 21, 85, then
        // 341 overflows u8 and saturates to the absorbing MAX.
        for pos in 1..=5 {
            let (_, ct) = ir.add_op(InputCiphertext { pos, int_size: 4 }, svec![]);
            let (_, blk) = ir.add_op(ExtractCtBlock { index: 0 }, svec![ct[0]]);
            let (_, packed) = ir.add_op(PackCt { mul: 4 }, svec![acc, blk[0]]);
            acc = packed[0];
        }
        let analyzed = analyze_noise(&ir, &PlaintextBlockSpec(2));
        assert_display_is!(
            analyzed.format(),
            r#"
            %0 = input_ciphertext<0, 4>();
                %0 -> ▓░░░░░░░░░░░   8% (fresh)
            %1 = extract_ct_block<0>(%0);
                %1 -> ▓░░░░░░░░░░░   8% (fresh)
            %2 = input_ciphertext<1, 4>();
                %2 -> ▓░░░░░░░░░░░   8% (fresh)
            %3 = extract_ct_block<0>(%2);
                %3 -> ▓░░░░░░░░░░░   8% (fresh)
            %4 = pack_ct<4>(%1, %3);
                %4 -> █████░░░░░░░   42%
            %5 = input_ciphertext<2, 4>();
                %5 -> ▓░░░░░░░░░░░   8% (fresh)
            %6 = extract_ct_block<0>(%5);
                %6 -> ▓░░░░░░░░░░░   8% (fresh)
            %7 = pack_ct<4>(%4, %6);
                %7 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  175% ⚠
            %8 = input_ciphertext<3, 4>();
                %8 -> ▓░░░░░░░░░░░   8% (fresh)
            %9 = extract_ct_block<0>(%8);
                %9 -> ▓░░░░░░░░░░░   8% (fresh)
            %10 = pack_ct<4>(%7, %9);
                %10 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  708% ⚠
            %11 = input_ciphertext<4, 4>();
                %11 -> ▓░░░░░░░░░░░   8% (fresh)
            %12 = extract_ct_block<0>(%11);
                %12 -> ▓░░░░░░░░░░░   8% (fresh)
            %13 = pack_ct<4>(%10, %12);
                %13 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  2125% ⚠
            %14 = input_ciphertext<5, 4>();
                %14 -> ▓░░░░░░░░░░░   8% (fresh)
            %15 = extract_ct_block<0>(%14);
                %15 -> ▓░░░░░░░░░░░   8% (fresh)
            %16 = pack_ct<4>(%13, %15);
                %16 -> 🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽🮽  2125% ⚠
        "#
        );
    }

    #[test]
    fn analyze_format_full_program() {
        use IopInstructionSet::*;
        let mut ir: IR<IopLang> = IR::empty();
        let (_, ct) = ir.add_op(
            InputCiphertext {
                pos: 0,
                int_size: 4,
            },
            svec![],
        );
        let (_, lo) = ir.add_op(ExtractCtBlock { index: 0 }, svec![ct[0]]);
        let (_, hi) = ir.add_op(ExtractCtBlock { index: 1 }, svec![ct[0]]);
        let (_, sum) = ir.add_op(AddCt, svec![lo[0], hi[0]]);
        let (_, pt) = ir.add_op(LetPlaintextBlock { value: 3 }, svec![]);
        let (_, scaled) = ir.add_op(MulPt, svec![sum[0], pt[0]]);
        let (_, stored) = ir.add_op(StoreCtBlock { index: 0 }, svec![scaled[0], ct[0]]);
        ir.add_op(OutputCiphertext { pos: 0 }, svec![stored[0]]);

        let analyzed = analyze_noise(&ir, &PlaintextBlockSpec(2));
        assert_display_is!(
            analyzed.format(),
            r#"
                %0 = input_ciphertext<0, 4>();
                    %0 -> ▓░░░░░░░░░░░   8% (fresh)
                %1 = extract_ct_block<0>(%0);
                    %1 -> ▓░░░░░░░░░░░   8% (fresh)
                %2 = extract_ct_block<1>(%0);
                    %2 -> ▓░░░░░░░░░░░   8% (fresh)
                %3 = add_ct(%1, %2);
                    %3 -> ██░░░░░░░░░░   17%
                %4 = let_pt_block<3>();
                    %4 -> ░░░░░░░░░░░░   0%
                %5 = mul_pt(%3, %4);
                    %5 -> ██████░░░░░░   50%
                %6 = store_ct_block<0>(%5, %0);
                    %6 -> ██████░░░░░░   50%
                output<0>(%6);
            "#
        );
    }
}
