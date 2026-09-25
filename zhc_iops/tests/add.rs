use zhc_builder::{Builder, IntegerCiphertextSpec};
use zhc_iops::{Error, add_hillis_steele, add_ripple_carry, add_tree};
use zhc_langs::ioplang::IopValue;

type Emit = fn(IntegerCiphertextSpec) -> Result<Builder, Error>;

const ADDERS: [Emit; 3] = [add_ripple_carry, add_hillis_steele, add_tree];

fn semantic(inp: &[IopValue]) -> Option<Vec<IopValue>> {
    let [
        IopValue::IntegerCiphertext(lhs),
        IopValue::IntegerCiphertext(rhs),
    ] = inp
    else {
        unreachable!()
    };
    Some(vec![IopValue::IntegerCiphertext(lhs.add(*rhs))])
}

fn check_correct(emit: Emit) {
    for size in (2..128).step_by(2) {
        emit(IntegerCiphertextSpec::new(size, 2, 2))
            .unwrap()
            .test_random(100, semantic);
    }
}

#[test]
fn correctness_add_ripple() {
    check_correct(add_ripple_carry);
}

#[test]
fn correctness_add_hillis_steele() {
    check_correct(add_hillis_steele);
}

#[test]
fn correctness_add_tree() {
    check_correct(add_tree);
}

#[test]
fn rejects_unsupported_block_spec() {
    for emit in ADDERS {
        assert!(emit(IntegerCiphertextSpec::new(8, 1, 1)).is_err());
    }
}

#[test]
fn works_from_other_threads() {
    let handles: Vec<_> = ADDERS
        .into_iter()
        .map(|emit| {
            std::thread::spawn(move || {
                emit(IntegerCiphertextSpec::new(16, 2, 2))
                    .unwrap()
                    .test_random(10, semantic);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
