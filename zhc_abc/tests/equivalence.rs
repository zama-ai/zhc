use std::collections::HashMap;

use zhc_abc::{CheckError, CheckOptions, Circuit, Lit, Verdict, check_equivalence};
use zhc_utils::assert_display_is;

struct Recorded {
    circuit: Circuit,
    inputs: Vec<Lit>,
    ands: Vec<(Lit, Lit, Lit)>,
    outputs: Vec<Lit>,
}

impl Recorded {
    fn new() -> Self {
        Self {
            circuit: Circuit::new(),
            inputs: Vec::new(),
            ands: Vec::new(),
            outputs: Vec::new(),
        }
    }

    fn input(&mut self) -> Lit {
        let l = self.circuit.input();
        self.inputs.push(l);
        l
    }

    fn not(&self, a: Lit) -> Lit {
        self.circuit.not(a)
    }

    fn and(&mut self, a: Lit, b: Lit) -> Lit {
        let l = self.circuit.and(a, b);
        let na = self.not(a);
        let nb = self.not(b);
        if !l.is_constant() && l != a && l != na && l != b && l != nb {
            self.ands.push((l, a, b));
        }
        l
    }

    fn or(&mut self, a: Lit, b: Lit) -> Lit {
        let na = self.not(a);
        let nb = self.not(b);
        let n = self.and(na, nb);
        self.not(n)
    }

    fn xor(&mut self, a: Lit, b: Lit) -> Lit {
        let na = self.not(a);
        let nb = self.not(b);
        let t = self.and(a, nb);
        let u = self.and(na, b);
        self.or(t, u)
    }

    fn mux(&mut self, s: Lit, t: Lit, f: Lit) -> Lit {
        let ns = self.not(s);
        let x = self.and(s, t);
        let y = self.and(ns, f);
        self.or(x, y)
    }

    fn output(&mut self, l: Lit) {
        self.circuit.output(l);
        self.outputs.push(l);
    }

    fn eval(&self, inputs: &[bool]) -> Vec<bool> {
        let mut vals: HashMap<Lit, bool> = HashMap::new();
        vals.insert(Lit::FALSE, false);
        for (l, v) in self.inputs.iter().zip(inputs) {
            vals.insert(*l, *v);
        }
        let get = |vals: &HashMap<Lit, bool>, l: Lit| match vals.get(&l) {
            Some(v) => *v,
            None => !vals[&self.circuit.not(l)],
        };
        for (l, a, b) in &self.ands {
            let v = get(&vals, *a) && get(&vals, *b);
            vals.insert(*l, v);
        }
        self.outputs.iter().map(|l| get(&vals, *l)).collect()
    }
}

fn ripple_adder(r: &mut Recorded, a: &[Lit], b: &[Lit]) -> Vec<Lit> {
    let mut c = Lit::FALSE;
    let mut out = Vec::new();
    for i in 0..a.len() {
        let axb = r.xor(a[i], b[i]);
        out.push(r.xor(axb, c));
        let t = r.and(a[i], b[i]);
        let u = r.and(c, axb);
        c = r.or(t, u);
    }
    out
}

fn majority_adder(r: &mut Recorded, a: &[Lit], b: &[Lit]) -> Vec<Lit> {
    let mut c = Lit::FALSE;
    let mut out = Vec::new();
    for i in 0..a.len() {
        let axb = r.xor(a[i], b[i]);
        let naxb = r.not(axb);
        out.push(r.mux(c, naxb, axb));
        let ab = r.and(a[i], b[i]);
        let ac = r.and(a[i], c);
        let bc = r.and(b[i], c);
        let t = r.or(ab, ac);
        c = r.or(t, bc);
    }
    out
}

fn build(width: usize, adder: fn(&mut Recorded, &[Lit], &[Lit]) -> Vec<Lit>) -> Recorded {
    let mut r = Recorded::new();
    let a: Vec<Lit> = (0..width).map(|_| r.input()).collect();
    let b: Vec<Lit> = (0..width).map(|_| r.input()).collect();
    let s = adder(&mut r, &a, &b);
    for l in s {
        r.output(l);
    }
    r
}

#[test]
fn different_adders_are_equivalent() {
    let left = build(2, ripple_adder);
    let right = build(2, majority_adder);
    assert_display_is!(
        left.circuit,
        r#"
            n1 = input 0
            n2 = input 1
            n3 = input 2
            n4 = input 3
            n5 = and(n1, !n3)
            n6 = and(!n1, n3)
            n7 = and(!n5, !n6)
            n8 = and(n1, n3)
            n9 = and(n2, !n4)
            n10 = and(!n2, n4)
            n11 = and(!n9, !n10)
            n12 = and(!n8, !n11)
            n13 = and(n8, n11)
            n14 = and(!n12, !n13)
            n15 = and(n2, n4)
            n16 = and(n8, !n11)
            n17 = and(!n15, !n16)
            output 0 = !n7
            output 1 = !n14
        "#
    );
    assert_display_is!(
        right.circuit,
        r#"
            n1 = input 0
            n2 = input 1
            n3 = input 2
            n4 = input 3
            n5 = and(n1, !n3)
            n6 = and(!n1, n3)
            n7 = and(!n5, !n6)
            n8 = and(n1, n3)
            n9 = and(n2, !n4)
            n10 = and(!n2, n4)
            n11 = and(!n9, !n10)
            n12 = and(n8, n11)
            n13 = and(!n8, !n11)
            n14 = and(!n12, !n13)
            n15 = and(n2, n4)
            n16 = and(n2, n8)
            n17 = and(n4, n8)
            n18 = and(!n15, !n16)
            n19 = and(!n17, n18)
            output 0 = !n7
            output 1 = !n14
        "#
    );
    let verdict =
        check_equivalence(&left.circuit, &right.circuit, &CheckOptions::default()).unwrap();
    assert_eq!(verdict, Verdict::Equivalent);
}

#[test]
fn broken_adder_gives_a_valid_counterexample() {
    let left = build(2, ripple_adder);
    let mut right = Recorded::new();
    let a: Vec<Lit> = (0..2).map(|_| right.input()).collect();
    let b: Vec<Lit> = (0..2).map(|_| right.input()).collect();
    let mut s = majority_adder(&mut right, &a, &b);
    s[1] = right.xor(s[1], a[0]);
    for l in s {
        right.output(l);
    }
    assert_display_is!(
        left.circuit,
        r#"
            n1 = input 0
            n2 = input 1
            n3 = input 2
            n4 = input 3
            n5 = and(n1, !n3)
            n6 = and(!n1, n3)
            n7 = and(!n5, !n6)
            n8 = and(n1, n3)
            n9 = and(n2, !n4)
            n10 = and(!n2, n4)
            n11 = and(!n9, !n10)
            n12 = and(!n8, !n11)
            n13 = and(n8, n11)
            n14 = and(!n12, !n13)
            n15 = and(n2, n4)
            n16 = and(n8, !n11)
            n17 = and(!n15, !n16)
            output 0 = !n7
            output 1 = !n14
        "#
    );
    assert_display_is!(
        right.circuit,
        r#"
            n1 = input 0
            n2 = input 1
            n3 = input 2
            n4 = input 3
            n5 = and(n1, !n3)
            n6 = and(!n1, n3)
            n7 = and(!n5, !n6)
            n8 = and(n1, n3)
            n9 = and(n2, !n4)
            n10 = and(!n2, n4)
            n11 = and(!n9, !n10)
            n12 = and(n8, n11)
            n13 = and(!n8, !n11)
            n14 = and(!n12, !n13)
            n15 = and(n2, n4)
            n16 = and(n2, n8)
            n17 = and(n4, n8)
            n18 = and(!n15, !n16)
            n19 = and(!n17, n18)
            n20 = and(!n1, !n14)
            n21 = and(n1, n14)
            n22 = and(!n20, !n21)
            output 0 = !n7
            output 1 = !n22
        "#
    );
    let verdict =
        check_equivalence(&left.circuit, &right.circuit, &CheckOptions::default()).unwrap();
    let Verdict::NotEquivalent(cex) = verdict else {
        panic!("expected a counterexample, got {verdict:?}");
    };
    assert_eq!(cex.inputs.len(), 4);
    let l = left.eval(&cex.inputs);
    let r = right.eval(&cex.inputs);
    assert_ne!(l, r);
    assert_ne!(l[cex.output], r[cex.output]);
}

#[test]
fn shape_mismatch_is_an_error() {
    let left = build(2, ripple_adder);
    let right = build(1, ripple_adder);
    assert_display_is!(
        left.circuit,
        r#"
            n1 = input 0
            n2 = input 1
            n3 = input 2
            n4 = input 3
            n5 = and(n1, !n3)
            n6 = and(!n1, n3)
            n7 = and(!n5, !n6)
            n8 = and(n1, n3)
            n9 = and(n2, !n4)
            n10 = and(!n2, n4)
            n11 = and(!n9, !n10)
            n12 = and(!n8, !n11)
            n13 = and(n8, n11)
            n14 = and(!n12, !n13)
            n15 = and(n2, n4)
            n16 = and(n8, !n11)
            n17 = and(!n15, !n16)
            output 0 = !n7
            output 1 = !n14
        "#
    );
    assert_display_is!(
        right.circuit,
        r#"
            n1 = input 0
            n2 = input 1
            n3 = and(n1, !n2)
            n4 = and(!n1, n2)
            n5 = and(!n3, !n4)
            n6 = and(n1, n2)
            output 0 = !n5
        "#
    );
    let err =
        check_equivalence(&left.circuit, &right.circuit, &CheckOptions::default()).unwrap_err();
    assert_eq!(err, CheckError::InputCountMismatch { left: 4, right: 2 });
}

#[test]
fn constants_fold() {
    let mut c = Circuit::new();
    assert_eq!(c.constant(false), Lit::FALSE);
    assert_eq!(c.constant(true), Lit::TRUE);
    assert_eq!(c.not(Lit::FALSE), Lit::TRUE);
    let x = c.input();
    assert_eq!(c.and(x, Lit::FALSE), Lit::FALSE);
    assert_eq!(c.and(x, Lit::TRUE), x);
    let nx = c.not(x);
    assert_eq!(c.and(x, nx), Lit::FALSE);
    assert_eq!(c.and(x, x), x);
    assert_eq!(c.n_ands(), 0);
    c.output(x);
    c.output(nx);
    assert_display_is!(
        c,
        r#"
            n1 = input 0
            output 0 = n1
            output 1 = !n1
        "#
    );
}

#[test]
fn parallel_checks_agree_with_serial_ones() {
    assert_display_is!(
        build(2, ripple_adder).circuit,
        r#"
            n1 = input 0
            n2 = input 1
            n3 = input 2
            n4 = input 3
            n5 = and(n1, !n3)
            n6 = and(!n1, n3)
            n7 = and(!n5, !n6)
            n8 = and(n1, n3)
            n9 = and(n2, !n4)
            n10 = and(!n2, n4)
            n11 = and(!n9, !n10)
            n12 = and(!n8, !n11)
            n13 = and(n8, n11)
            n14 = and(!n12, !n13)
            n15 = and(n2, n4)
            n16 = and(n8, !n11)
            n17 = and(!n15, !n16)
            output 0 = !n7
            output 1 = !n14
        "#
    );
    let expected: Vec<(usize, bool)> = (0..64).map(|i| (4 + i % 13, i % 3 == 0)).collect();
    let run = |(width, broken): (usize, bool)| -> bool {
        let left = build(width, ripple_adder);
        let mut right = Recorded::new();
        let a: Vec<Lit> = (0..width).map(|_| right.input()).collect();
        let b: Vec<Lit> = (0..width).map(|_| right.input()).collect();
        let mut s = majority_adder(&mut right, &a, &b);
        if broken {
            s[width / 2] = right.xor(s[width / 2], b[0]);
        }
        for l in s {
            right.output(l);
        }
        let v = check_equivalence(&left.circuit, &right.circuit, &CheckOptions::default()).unwrap();
        v == Verdict::Equivalent
    };
    let serial: Vec<bool> = expected.iter().map(|c| run(*c)).collect();
    assert!(
        serial
            .iter()
            .zip(&expected)
            .all(|(ok, (_, broken))| *ok != *broken)
    );
    let handles: Vec<_> = (0..8)
        .map(|t| {
            let cases = expected.clone();
            std::thread::spawn(move || {
                cases
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i % 8 == t)
                    .map(|(i, c)| (i, run(*c)))
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    for h in handles {
        for (i, ok) in h.join().unwrap() {
            assert_eq!(ok, serial[i], "verdict differs in parallel for case {i}");
        }
    }
}

#[test]
fn display_lists_inputs_gates_and_outputs() {
    let mut c = Circuit::new();
    let a = c.input();
    let b = c.input();
    let x = c.xor(a, b);
    c.output(x);
    let na = c.not(a);
    c.output(na);
    c.output(Lit::TRUE);
    assert_display_is!(
        c,
        r#"
            n1 = input 0
            n2 = input 1
            n3 = and(n1, !n2)
            n4 = and(!n1, n2)
            n5 = and(!n3, !n4)
            output 0 = !n5
            output 1 = !n1
            output 2 = true
        "#
    );
}
