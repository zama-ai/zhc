//! Block-level API tests.
//!
//! Each test builds a tiny circuit, evaluates it on every relevant input and checks the raw
//! result block against the matching `zhc_crypto` primitive. Panicking cases assert that the
//! interpreter refuses the operation, by comparing with the panic of the primitive itself.

use std::panic::{AssertUnwindSafe, catch_unwind};

use zhc_crypto::integer_semantics::{
    CiphertextBlockSpec, EmulatedCiphertextBlock, EmulatedCiphertextBlockStorage, Flavor,
    lut::LookupCheck,
};
use zhc_langs::ioplang::{
    IopInterepreterContext, IopValue, Lut1Def, Lut2Def, Lut4Def, Lut8Def, LutFn,
};
use zhc_utils::FastMap;

use crate::{Builder, CiphertextBlock};

const SPEC: CiphertextBlockSpec = CiphertextBlockSpec(2, 2);
const FLAVORS: [Flavor; 3] = [Flavor::Protect, Flavor::Temper, Flavor::Wrapping];

/// Builds a constant block holding any complete-width value.
///
/// `LetCiphertextBlock` only accepts message values, so the upper bits are added with wrapping
/// plaintext additions, message-sized chunk by chunk.
fn let_complete(builder: &Builder, value: EmulatedCiphertextBlockStorage) -> CiphertextBlock {
    let msg = SPEC.message_size();
    let mut acc = builder.block_let_ciphertext((value & SPEC.message_mask()) as u8);
    let mut shift = msg;
    while shift < SPEC.complete_size() {
        let chunk = ((value >> shift) & SPEC.message_mask()) as u8;
        if chunk != 0 {
            let pt = builder.block_let_plaintext(chunk << shift);
            acc = builder.block_wrapping_add_plaintext(acc, pt);
        }
        shift += msg;
    }
    acc
}

/// Builds a circuit from raw complete-width constants, evaluates it and returns the raw output
/// blocks, or `None` if interpretation panicked.
fn run(
    inputs: &[EmulatedCiphertextBlockStorage],
    body: impl Fn(&Builder, &[CiphertextBlock]) -> Vec<CiphertextBlock>,
) -> Option<Vec<EmulatedCiphertextBlock>> {
    let builder = Builder::new(SPEC);
    let blocks: Vec<_> = inputs.iter().map(|v| let_complete(&builder, *v)).collect();
    let outs = body(&builder, &blocks);
    let mut context = IopInterepreterContext {
        spec: SPEC,
        inputs: FastMap::default(),
        outputs: FastMap::default(),
    };
    let ir = builder.ir();
    catch_unwind(AssertUnwindSafe(|| {
        let values = ir.evaluate::<IopValue>(&mut context).ok()?;
        Some(
            outs.iter()
                .map(|o| {
                    values
                        .get_val(o.valid)
                        .get_annotation()
                        .clone()
                        .unwrap_ciphertext_block()
                })
                .collect(),
        )
    }))
    .ok()
    .flatten()
}

/// Single-output variant of [`run`].
fn run1(
    inputs: &[EmulatedCiphertextBlockStorage],
    body: impl Fn(&Builder, &[CiphertextBlock]) -> CiphertextBlock,
) -> Option<EmulatedCiphertextBlock> {
    run(inputs, |b, x| vec![body(b, x)]).map(|mut v| v.remove(0))
}

/// A non-capturing closure adding `$k` to the data bits, usable as a [`LutFn`].
macro_rules! inc {
    ($k:literal) => {
        |b| {
            b.spec()
                .from_data((b.raw_data_bits() + $k) & b.spec().data_mask())
        }
    };
}

#[test]
fn let_complete_reaches_every_value() {
    for v in 0..=SPEC.complete_mask() {
        assert_eq!(run1(&[v], |_, b| b[0]), Some(SPEC.from_complete(v)));
    }
}

#[test]
fn add_flavors_match_crypto() {
    for a in 0..=SPEC.complete_mask() {
        for b in 0..=SPEC.complete_mask() {
            let (ea, eb) = (SPEC.from_complete(a), SPEC.from_complete(b));
            for flavor in FLAVORS {
                let expected = catch_unwind(|| ea.add(eb, flavor)).ok();
                let got = run1(&[a, b], |bd, x| bd.block_add_with(x[0], x[1], flavor));
                assert_eq!(got, expected, "add {flavor:?} on {a:#b} + {b:#b}");
            }
        }
    }
}

#[test]
fn sub_flavors_match_crypto() {
    for a in 0..=SPEC.complete_mask() {
        for b in 0..=SPEC.complete_mask() {
            let (ea, eb) = (SPEC.from_complete(a), SPEC.from_complete(b));
            for flavor in FLAVORS {
                let expected = catch_unwind(|| ea.sub(eb, flavor)).ok();
                let got = run1(&[a, b], |bd, x| bd.block_sub_with(x[0], x[1], flavor));
                assert_eq!(got, expected, "sub {flavor:?} on {a:#b} - {b:#b}");
            }
        }
    }
}

#[test]
fn named_shortcuts_pick_the_documented_flavor() {
    let (a, b) = (0b0_11_10, 0b0_01_11); // protect add overflows, temper does not
    let shortcuts: [(
        fn(&Builder, &[CiphertextBlock]) -> CiphertextBlock,
        Option<u16>,
    ); 6] = [
        (|bd, x| bd.block_add(x[0], x[1]), None),
        (|bd, x| bd.block_temper_add(x[0], x[1]), Some(0b1_01_01)),
        (|bd, x| bd.block_wrapping_add(x[0], x[1]), Some(0b1_01_01)),
        (|bd, x| bd.block_sub(x[1], x[0]), None),
        (|bd, x| bd.block_temper_sub(x[1], x[0]), None),
        (|bd, x| bd.block_wrapping_sub(x[1], x[0]), Some(0b1_10_01)),
    ];
    for (i, (f, expected)) in shortcuts.into_iter().enumerate() {
        let got = run1(&[a, b], f).map(|v| v.raw_complete_bits());
        assert_eq!(got, expected, "shortcut #{i}");
    }
}

#[test]
fn neg_matches_crypto() {
    for a in 0..=SPEC.complete_mask() {
        let expected = Some(SPEC.from_complete(a).neg());
        assert_eq!(run1(&[a], |bd, x| bd.block_neg(x[0])), expected);
    }
}

#[test]
fn shl_flavors_match_crypto() {
    for a in 0..=SPEC.complete_mask() {
        for amount in 0..SPEC.complete_size() {
            let ea = SPEC.from_complete(a);
            for flavor in FLAVORS {
                let expected = catch_unwind(|| ea.shl(amount, flavor)).ok();
                let got = run1(&[a], |bd, x| bd.block_shl_with(x[0], amount, flavor));
                assert_eq!(got, expected, "shl {flavor:?} on {a:#b} << {amount}");
            }
        }
    }
}

#[test]
fn mac_flavors_match_crypto() {
    for a in 0..=SPEC.data_mask() {
        for b in 0..=SPEC.message_mask() {
            let (ea, eb) = (SPEC.from_data(a), SPEC.from_message(b));
            for mul in [1u8, 2, 4] {
                for flavor in FLAVORS {
                    let expected = catch_unwind(|| ea.mac(mul, eb, flavor)).ok();
                    let got = run1(&[a, b], |bd, x| bd.block_mac_with(x[0], x[1], mul, flavor));
                    assert_eq!(got, expected, "mac {flavor:?} on {a:#b} * {mul} + {b:#b}");
                }
            }
        }
    }
}

#[test]
fn pack_is_mac_by_message_radix() {
    let (a, b) = (0b0_00_10, 0b0_00_01);
    for flavor in FLAVORS {
        let packed = run1(&[a, b], |bd, x| bd.block_pack_with(x[0], x[1], flavor));
        let mac = run1(&[a, b], |bd, x| bd.block_mac_with(x[0], x[1], 4, flavor));
        assert_eq!(packed, mac);
        assert_eq!(packed, Some(SPEC.from_data(0b10_01)));
    }
}

#[test]
fn pack_defaults_to_protect() {
    let (a, b) = (0b0_01_10, 0b0_00_01); // carry set on the high operand
    assert!(run1(&[a, b], |bd, x| bd.block_pack(x[0], x[1])).is_none());
    assert_eq!(
        run1(&[a, b], |bd, x| bd.block_temper_pack(x[0], x[1])),
        Some(SPEC.from_complete(0b1_10_01))
    );
}

#[test]
fn plaintext_ops_match_crypto() {
    let pt_spec = SPEC.complete_plaintext_block_spec();
    for a in 0..=SPEC.complete_mask() {
        for p in 0..=SPEC.complete_mask() {
            let ea = SPEC.from_complete(a);
            let ep = pt_spec.from_message(p);
            for flavor in FLAVORS {
                let plain = |bd: &Builder| bd.block_let_plaintext(p as u8);
                assert_eq!(
                    run1(&[a], |bd, x| bd.block_add_plaintext_with(
                        x[0],
                        plain(bd),
                        flavor
                    )),
                    catch_unwind(|| ea.add_pt(ep, flavor)).ok(),
                    "add_pt {flavor:?} {a:#b} + {p}"
                );
                assert_eq!(
                    run1(&[a], |bd, x| bd.block_sub_plaintext_with(
                        x[0],
                        plain(bd),
                        flavor
                    )),
                    catch_unwind(|| ea.sub_pt(ep, flavor)).ok(),
                    "sub_pt {flavor:?} {a:#b} - {p}"
                );
                assert_eq!(
                    run1(&[a], |bd, x| bd.block_plaintext_sub_with(
                        plain(bd),
                        x[0],
                        flavor
                    )),
                    catch_unwind(|| ep.sub_ct(ea, flavor)).ok(),
                    "pt_sub {flavor:?} {p} - {a:#b}"
                );
                assert_eq!(
                    run1(&[a], |bd, x| bd.block_mul_plaintext_with(
                        x[0],
                        plain(bd),
                        flavor
                    )),
                    catch_unwind(|| ea.mul_pt(ep, flavor)).ok(),
                    "mul_pt {flavor:?} {a:#b} * {p}"
                );
            }
        }
    }
}

#[test]
fn lookup_checks_match_crypto() {
    let def = Lut1Def::custom("shift_up", |b| {
        b.spec()
            .from_complete((b.raw_data_bits() << 1) & b.spec().complete_mask())
    });
    let table = def.into_lut(SPEC);
    for a in 0..=SPEC.complete_mask() {
        for check in [
            LookupCheck::Protect,
            LookupCheck::AllowInputPadding,
            LookupCheck::AllowOutputPadding,
            LookupCheck::AllowBothPadding,
        ] {
            let expected = catch_unwind(|| table.lookup(SPEC.from_complete(a), check)).ok();
            let got = run1(&[a], |bd, x| bd.block_lookup_with(x[0], def.clone(), check));
            assert_eq!(got, expected, "lookup {check:?} on {a:#b}");
        }
    }
}

#[test]
fn lookup_shortcuts_pick_the_documented_checks() {
    // Padding set on the input: only the wrapping shortcut accepts it.
    let a = 0b1_00_01;
    assert!(run1(&[a], |bd, x| bd.block_lookup(x[0], Lut1Def::None)).is_none());
    assert!(run1(&[a], |bd, x| bd.block_padding_lookup(x[0], Lut1Def::None)).is_none());
    assert_eq!(
        run1(&[a], |bd, x| bd.block_wrapping_lookup(x[0], Lut1Def::None)),
        // The table is indexed by the data bits only, then negated because of the padding bit.
        Some(SPEC.from_data(a & SPEC.data_mask()).neg())
    );
    // Padding clear on the input, table writes the padding bit.
    let a = 0b0_01_01;
    let set_padding = || {
        Lut1Def::custom("set_padding", |b| {
            b.spec().from_complete(b.spec().padding_mask())
        })
    };
    assert!(run1(&[a], |bd, x| bd.block_lookup(x[0], set_padding())).is_none());
    assert_eq!(
        run1(&[a], |bd, x| bd.block_padding_lookup(x[0], set_padding())),
        Some(SPEC.from_complete(SPEC.padding_mask()))
    );
}

#[test]
fn lookup2_matches_crypto() {
    let def = Lut2Def::custom("id_inc", [|b| b, inc!(1)]);
    let table = def.into_lut(SPEC);
    for a in 0..=SPEC.data_mask() {
        let expected = catch_unwind(|| table.lookup(SPEC.from_data(a), LookupCheck::Protect))
            .ok()
            .map(|(o0, o1)| vec![o0, o1]);
        let got = run(&[a], |bd, x| {
            let (o0, o1) = bd.block_lookup2(x[0], def.clone());
            vec![o0, o1]
        });
        assert_eq!(got, expected, "lookup2 on {a:#b}");
    }
}

#[test]
fn lookup4_matches_crypto() {
    let def = Lut4Def::custom("plus_k", [|b| b, inc!(1), inc!(2), inc!(3)]);
    let table = def.into_lut(SPEC);
    for a in 0..=SPEC.data_mask() {
        let expected = catch_unwind(|| table.lookup(SPEC.from_data(a), LookupCheck::Protect))
            .ok()
            .map(|(o0, o1, o2, o3)| vec![o0, o1, o2, o3]);
        let got = run(&[a], |bd, x| bd.block_lookup4(x[0], def.clone()).to_vec());
        assert_eq!(got, expected, "lookup4 on {a:#b}");
    }
}

#[test]
fn lookup8_matches_crypto() {
    let fs: [LutFn; 8] = [
        |b| b,
        inc!(1),
        inc!(2),
        inc!(3),
        inc!(4),
        inc!(5),
        inc!(6),
        inc!(7),
    ];
    let def = Lut8Def::custom("plus_k", fs);
    let table = def.into_lut(SPEC);
    for a in 0..=SPEC.data_mask() {
        let expected = catch_unwind(|| table.lookup(SPEC.from_data(a), LookupCheck::Protect))
            .ok()
            .map(|(o0, o1, o2, o3, o4, o5, o6, o7)| vec![o0, o1, o2, o3, o4, o5, o6, o7]);
        let got = run(&[a], |bd, x| bd.block_lookup8(x[0], def.clone()).to_vec());
        assert_eq!(got, expected, "lookup8 on {a:#b}");
    }
}

#[test]
fn many_lut_rejects_input_padding_checks() {
    let def = Lut2Def::custom("id", [|b| b, |b| b]);
    for check in [
        LookupCheck::AllowInputPadding,
        LookupCheck::AllowBothPadding,
    ] {
        let got = run(&[0], |bd, x| {
            let (o0, o1) = bd.block_lookup2_with(x[0], def.clone(), check);
            vec![o0, o1]
        });
        assert!(got.is_none(), "{check:?} should be rejected by a many-LUT");
    }
}

#[test]
fn flavored_instructions_format_with_prefix() {
    let builder = Builder::new(SPEC);
    let a = builder.block_let_ciphertext(1);
    let b = builder.block_let_ciphertext(2);
    let p = builder.block_let_plaintext(1);
    builder.block_add(a, b);
    builder.block_temper_sub(a, b);
    builder.block_wrapping_mul_plaintext(a, p);
    builder.block_neg(a);
    builder.block_temper_shl(a, 1);
    builder.block_wrapping_pack(a, b);
    let text = builder.ir().format().to_string();
    for needle in [
        "add_ct(",
        "temper_sub_ct(",
        "wrapping_mul_pt(",
        "neg_ct(",
        "temper_shl_ct<1>(",
        "wrapping_pack_ct<4>(",
    ] {
        assert!(text.contains(needle), "missing {needle:?} in:\n{text}");
    }
}
