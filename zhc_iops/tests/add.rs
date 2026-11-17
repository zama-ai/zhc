use zhc_builder::{Builder, CiphertextInteger, SpecInteger};
use zhc_iops::BuilderExt;
use zhc_langs::ioplang::IopValue;

type Iop = fn(&Builder, &CiphertextInteger, &CiphertextInteger) -> CiphertextInteger;

const ADDERS: [Iop; 2] = [
    Builder::proven_add_ripple_carry,
    Builder::proven_add_hillis_steele,
];

fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
    let [
        IopValue::CiphertextInteger(lhs),
        IopValue::CiphertextInteger(rhs),
    ] = inp
    else {
        unreachable!()
    };
    Some(vec![IopValue::CiphertextInteger(lhs.add(rhs))])
}

fn circuit(iop: Iop, spec: SpecInteger) -> Builder {
    let builder = Builder::new(spec.block_spec());
    let lhs = builder.ciphertext_integer_input(spec.int_size());
    let rhs = builder.ciphertext_integer_input(spec.int_size());
    let sum = iop(&builder, &lhs, &rhs);
    builder.ciphertext_integer_output(&sum);
    builder
}

fn check_correct(iop: Iop) {
    for size in (2..128).step_by(2) {
        circuit(iop, SpecInteger::new(size, 2, 2)).test_random(100, semantic);
    }
}

#[test]
fn correctness_add_ripple() {
    check_correct(Builder::proven_add_ripple_carry);
}

#[test]
fn correctness_add_hillis_steele() {
    check_correct(Builder::proven_add_hillis_steele);
}

#[test]
fn chains_in_one_circuit() {
    let spec = SpecInteger::new(16, 2, 2);
    let builder = Builder::new(spec.block_spec());
    let a = builder.ciphertext_integer_input(spec.int_size());
    let b = builder.ciphertext_integer_input(spec.int_size());
    let c = builder.ciphertext_integer_input(spec.int_size());
    let ab = builder.proven_add_ripple_carry(&a, &b);
    let abc = builder.proven_add_hillis_steele(&ab, &c);
    builder.ciphertext_integer_output(&abc);
    builder.test_random(100, |inp| {
        let [
            IopValue::CiphertextInteger(a),
            IopValue::CiphertextInteger(b),
            IopValue::CiphertextInteger(c),
        ] = inp
        else {
            unreachable!()
        };
        Some(vec![IopValue::CiphertextInteger(a.add(b).add(c))])
    });
}

#[test]
fn rejects_unsupported_block_spec() {
    for iop in ADDERS {
        let result = std::panic::catch_unwind(|| circuit(iop, SpecInteger::new(8, 1, 1)));
        let msg = result.unwrap_err();
        let msg = msg.downcast_ref::<String>().unwrap();
        assert!(
            msg.contains("does not match the algorithm block spec"),
            "{msg}"
        );
    }
}

#[test]
fn rejects_mismatched_operands() {
    for iop in ADDERS {
        let result = std::panic::catch_unwind(|| {
            let builder = Builder::new(SpecInteger::new(16, 2, 2).block_spec());
            let lhs = builder.ciphertext_integer_input(16);
            let rhs = builder.ciphertext_integer_input(8);
            iop(&builder, &lhs, &rhs)
        });
        let msg = result.unwrap_err();
        let msg = msg.downcast_ref::<String>().unwrap();
        assert!(
            msg.contains("handle int size does not match the signature"),
            "{msg}"
        );
    }
}

#[test]
fn rejects_foreign_builder() {
    for iop in ADDERS {
        let result = std::panic::catch_unwind(|| {
            let source = Builder::new(SpecInteger::new(16, 2, 2).block_spec());
            let lhs = source.ciphertext_integer_input(16);
            let rhs = source.ciphertext_integer_input(16);
            let other = Builder::new(SpecInteger::new(16, 1, 1).block_spec());
            iop(&other, &lhs, &rhs)
        });
        let msg = result.unwrap_err();
        let msg = msg.downcast_ref::<String>().unwrap();
        assert!(
            msg.contains("builder block spec does not match the algorithm block spec"),
            "{msg}"
        );
    }
}

#[test]
fn works_from_other_threads() {
    let handles: Vec<_> = ADDERS
        .into_iter()
        .map(|iop| {
            std::thread::spawn(move || {
                circuit(iop, SpecInteger::new(16, 2, 2)).test_random(10, semantic);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
