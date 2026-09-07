use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ValueSet {
    set: u128,
    n_bits: u8,
}

impl ValueSet {
    pub fn wrapping_add(lhs: Self, rhs: Self) -> Self {
        assert_eq!(lhs.n_bits, rhs.n_bits);
        rhs.iter_values().fold(ValueSet::new_empty(lhs.n_bits), |acc, v| {
            acc.union(lhs.wrapping_add_scalar(v))
        })
    }

    pub fn wrapping_sub(lhs: Self, rhs: Self) -> Self {
        assert_eq!(lhs.n_bits, rhs.n_bits);
        rhs.iter_values().fold(ValueSet::new_empty(lhs.n_bits), |acc, v| {
            acc.union(lhs.wrapping_sub_scalar(v))
        })
    }

    pub fn wrapping_mul(lhs: Self, rhs: Self) -> Self {
        assert_eq!(lhs.n_bits, rhs.n_bits);
        rhs.iter_values().fold(ValueSet::new_empty(lhs.n_bits), |acc, v| {
            acc.union(lhs.wrapping_mul_scalar(v))
        })
    }
}

impl ValueSet {
    pub fn wrapping_add_scalar(mut self, value: u8) -> Self {
        assert!(value <= self.domain_max());
        let size = self.domain_size();
        if value == 0 {
            return self;
        }
        let up = self.set << value;
        let down = self.set >> (size - value as u32);
        self.set = (up | down) & self.mask();
        self
    }

    pub fn wrapping_sub_scalar(self, value: u8) -> Self {
        assert!(value <= self.domain_max());
        if value == 0 {
            return self;
        }
        self.wrapping_add_scalar((self.domain_size() - value as u32) as u8)
    }

    pub fn wrapping_mul_scalar(self, value: u8) -> Self {
        assert!(value <= self.domain_max());
        if value == 1 {
            return self;
        }
        let mask = self.domain_max() as u16;
        let mut out = ValueSet::new_empty(self.n_bits);
        for v in self.iter_values() {
            out.insert(((v as u16 * value as u16) & mask) as u8);
        }
        out
    }


    pub fn apply(self, func: impl Fn(u8) -> u8) -> Self {
        let mut out = ValueSet::new_empty(self.n_bits);
        for v in self.iter_values() {
            let nv = func(v);
            assert!(nv <= self.domain_max());
            out.insert(nv);
        }
        out
    }
}

impl ValueSet {
    pub fn new_empty(n_bits: u8) -> Self {
        assert!(n_bits <= 7);
        ValueSet { set: 0, n_bits }
    }

    pub fn new_full(n_bits: u8) -> Self {
        let empty = Self::new_empty(n_bits);
        ValueSet { set: empty.mask(), n_bits }
    }

    pub fn from_single(n_bits: u8, value: u8) -> Self {
        let mut vs = Self::new_empty(n_bits);
        vs.insert(value);
        vs
    }

    /// Number of values in the domain, `2^n_bits`.
    fn domain_size(&self) -> u32 {
        1u32 << self.n_bits
    }

    /// Bitmask covering the whole domain.
    fn mask(&self) -> u128 {
        let size = self.domain_size();
        if size == 128 { u128::MAX } else { (1u128 << size) - 1 }
    }

    pub fn n_bits(&self) -> u8 {
        self.n_bits
    }

    pub fn domain_max(&self) -> u8 {
        ((1u16 << self.n_bits) - 1) as u8
    }

    pub fn insert(&mut self, value: u8) {
        assert!(value <= self.domain_max());
        self.set |= 1u128 << value;
    }

    pub fn remove(&mut self, value: u8) {
        assert!(value <= self.domain_max());
        self.set &= !(1u128 << value);
    }

    pub fn contains(&self, value: u8) -> bool {
        assert!(value <= self.domain_max());
        (self.set >> value) & 1 != 0
    }

    pub fn is_empty(&self) -> bool {
        self.set == 0
    }

    pub fn count(&self) -> u32 {
        self.set.count_ones()
    }

    pub fn min(&self) -> Option<u8> {
        if self.is_empty() {
            None
        } else {
            Some(self.set.trailing_zeros() as u8)
        }
    }

    pub fn max(&self) -> Option<u8> {
        if self.is_empty() {
            None
        } else {
            Some(127 - self.set.leading_zeros() as u8)
        }
    }

    pub fn iter_values(&self) -> impl Iterator<Item = u8> {
        let set = self.set;
        (0..=self.domain_max()).filter(move |&v| (set >> v) & 1 != 0)
    }

    pub fn union(self, other: Self) -> Self {
        assert_eq!(self.n_bits, other.n_bits);
        ValueSet { set: self.set | other.set, n_bits: self.n_bits }
    }

    pub fn intersection(self, other: Self) -> Self {
        assert_eq!(self.n_bits, other.n_bits);
        ValueSet { set: self.set & other.set, n_bits: self.n_bits }
    }
}

impl fmt::Debug for ValueSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.n_bits as u32;
        let total = 1u32 << n;
        let count = self.count();

        if f.alternate() {
            let width = 1u32 << ((n + 1) / 2);
            let height = 1u32 << (n / 2);
            for row in 0..height {
                if row > 0 {
                    writeln!(f)?;
                }
                for col in 0..width {
                    let val = row * width + col;
                    if val < total && self.contains(val as u8) {
                        write!(f, "\u{2593}\u{2593}")?;
                    } else {
                        write!(f, "\u{2591}\u{2591}")?;
                    }
                }
            }
            Ok(())
        } else {
            write!(f, "ValueSet({}b {}/{} ", n, count, total)?;
            for v in 0..total as u8 {
                if self.contains(v) {
                    write!(f, "\u{2593}")?;
                } else {
                    write!(f, "\u{2591}")?;
                }
            }
            write!(f, ")")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_empty_has_no_values() {
        for n_bits in 0..=7 {
            let vs = ValueSet::new_empty(n_bits);
            assert!(vs.is_empty());
            assert_eq!(vs.count(), 0);
            assert_eq!(vs.n_bits(), n_bits);
            assert_eq!(vs.min(), None);
            assert_eq!(vs.max(), None);
            assert_eq!(vs.iter_values().count(), 0);
            for v in 0..=vs.domain_max() {
                assert!(!vs.contains(v));
            }
        }
    }

    #[test]
    fn new_full_has_all_values() {
        for n_bits in 0..=7 {
            let vs = ValueSet::new_full(n_bits);
            assert!(!vs.is_empty());
            assert_eq!(vs.count(), 1 << n_bits);
            assert_eq!(vs.min(), Some(0));
            assert_eq!(vs.max(), Some(vs.domain_max()));
            for v in 0..=vs.domain_max() {
                assert!(vs.contains(v));
            }
            assert!(vs.iter_values().eq(0..=vs.domain_max()));
        }
    }

    #[test]
    fn from_single_contains_only_that_value() {
        for n_bits in 0..=7 {
            let max = ValueSet::new_empty(n_bits).domain_max();
            for value in 0..=max {
                let vs = ValueSet::from_single(n_bits, value);
                assert_eq!(vs.count(), 1);
                assert_eq!(vs.min(), Some(value));
                assert_eq!(vs.max(), Some(value));
                for v in 0..=max {
                    assert_eq!(vs.contains(v), v == value);
                }
            }
        }
    }

    #[test]
    fn domain_max_is_two_pow_n_minus_one() {
        for n_bits in 0..=7u8 {
            assert_eq!(ValueSet::new_empty(n_bits).domain_max() as u32, (1u32 << n_bits) - 1);
        }
    }

    #[test]
    #[should_panic]
    fn new_empty_rejects_more_than_seven_bits() {
        ValueSet::new_empty(8);
    }

    #[test]
    fn insert_remove_contains_roundtrip() {
        for n_bits in 0..=7 {
            let mut vs = ValueSet::new_empty(n_bits);
            let max = vs.domain_max();
            for v in 0..=max {
                vs.insert(v);
                assert!(vs.contains(v));
                assert_eq!(vs.count(), v as u32 + 1);
            }
            vs.insert(0);
            assert_eq!(vs.count(), max as u32 + 1);
            for v in 0..=max {
                vs.remove(v);
                assert!(!vs.contains(v));
                assert_eq!(vs.count(), (max - v) as u32);
            }
            vs.remove(0);
            assert!(vs.is_empty());
        }
    }

    #[test]
    #[should_panic]
    fn insert_rejects_value_outside_domain() {
        ValueSet::new_empty(3).insert(8);
    }

    #[test]
    #[should_panic]
    fn remove_rejects_value_outside_domain() {
        ValueSet::new_empty(3).remove(8);
    }

    #[test]
    #[should_panic]
    fn contains_rejects_value_outside_domain() {
        ValueSet::new_empty(3).contains(8);
    }

    #[test]
    fn min_max_count_match_iter_values() {
        for n_bits in 0..=7 {
            for _ in 0..50 {
                let mut vs = ValueSet::new_empty(n_bits);
                for _ in 0..rand::random_range(0..=vs.domain_max()) {
                    vs.insert(rand::random_range(0..=vs.domain_max()));
                }
                let values: Vec<u8> = vs.iter_values().collect();
                assert_eq!(vs.min(), values.first().copied());
                assert_eq!(vs.max(), values.last().copied());
                assert_eq!(vs.count() as usize, values.len());
                assert_eq!(vs.is_empty(), values.is_empty());
                assert!(values.windows(2).all(|w| w[0] < w[1]));
            }
        }
    }

    #[test]
    fn iter_values_is_sorted_and_deduplicated() {
        let mut vs = ValueSet::new_empty(4);
        for v in [9, 3, 3, 15, 0, 9] {
            vs.insert(v);
        }
        assert_eq!(vs.iter_values().collect::<Vec<_>>(), vec![0, 3, 9, 15]);
    }

    #[test]
    fn union_and_intersection_match_per_value_check() {
        for n_bits in 0..=7 {
            for _ in 0..50 {
                let mut a = ValueSet::new_empty(n_bits);
                let mut b = ValueSet::new_empty(n_bits);
                for _ in 0..rand::random_range(0..=a.domain_max()) {
                    a.insert(rand::random_range(0..=a.domain_max()));
                }
                for _ in 0..rand::random_range(0..=b.domain_max()) {
                    b.insert(rand::random_range(0..=b.domain_max()));
                }
                let u = a.union(b);
                let i = a.intersection(b);
                for v in 0..=a.domain_max() {
                    assert_eq!(u.contains(v), a.contains(v) || b.contains(v));
                    assert_eq!(i.contains(v), a.contains(v) && b.contains(v));
                }
                assert_eq!(a.union(ValueSet::new_empty(n_bits)), a);
                assert_eq!(a.union(ValueSet::new_full(n_bits)), ValueSet::new_full(n_bits));
                assert_eq!(a.intersection(ValueSet::new_empty(n_bits)), ValueSet::new_empty(n_bits));
                assert_eq!(a.intersection(ValueSet::new_full(n_bits)), a);
            }
        }
    }

    #[test]
    #[should_panic]
    fn union_rejects_mismatched_widths() {
        let _ = ValueSet::new_empty(3).union(ValueSet::new_empty(4));
    }

    #[test]
    #[should_panic]
    fn intersection_rejects_mismatched_widths() {
        let _ = ValueSet::new_empty(3).intersection(ValueSet::new_empty(4));
    }

    #[test]
    fn wrapping_add_scalar_matches_bruteforce() {
        for n_bits in 0..=7 {
            let modulo = 1u32 << n_bits;
            for _ in 0..30 {
                let mut vs = ValueSet::new_empty(n_bits);
                for _ in 0..rand::random_range(0..=vs.domain_max()) {
                    vs.insert(rand::random_range(0..=vs.domain_max()));
                }
                for k in 0..=vs.domain_max() {
                    let mut expected = ValueSet::new_empty(n_bits);
                    for v in vs.iter_values() {
                        expected.insert(((v as u32 + k as u32) % modulo) as u8);
                    }
                    assert_eq!(vs.wrapping_add_scalar(k), expected, "n_bits={n_bits} k={k}");
                }
            }
        }
    }

    #[test]
    fn wrapping_add_scalar_examples() {
        let mut vs = ValueSet::new_empty(3);
        vs.insert(0);
        vs.insert(6);
        vs.insert(7);
        let mut plus_one = ValueSet::new_empty(3);
        plus_one.insert(1);
        plus_one.insert(7);
        plus_one.insert(0);
        assert_eq!(vs.wrapping_add_scalar(1), plus_one);
        assert_eq!(vs.wrapping_add_scalar(0), vs);
        assert_eq!(ValueSet::new_full(3).wrapping_add_scalar(5), ValueSet::new_full(3));
        assert_eq!(ValueSet::new_empty(3).wrapping_add_scalar(5), ValueSet::new_empty(3));
    }

    #[test]
    fn wrapping_sub_scalar_matches_bruteforce() {
        for n_bits in 0..=7 {
            let modulo = 1u32 << n_bits;
            for _ in 0..30 {
                let mut vs = ValueSet::new_empty(n_bits);
                for _ in 0..rand::random_range(0..=vs.domain_max()) {
                    vs.insert(rand::random_range(0..=vs.domain_max()));
                }
                for k in 0..=vs.domain_max() {
                    let mut expected = ValueSet::new_empty(n_bits);
                    for v in vs.iter_values() {
                        expected.insert(((v as u32 + modulo - k as u32) % modulo) as u8);
                    }
                    assert_eq!(vs.wrapping_sub_scalar(k), expected, "n_bits={n_bits} k={k}");
                }
            }
        }
    }

    #[test]
    fn wrapping_sub_scalar_inverts_add_scalar() {
        for n_bits in 0..=7 {
            for _ in 0..30 {
                let mut vs = ValueSet::new_empty(n_bits);
                for _ in 0..rand::random_range(0..=vs.domain_max()) {
                    vs.insert(rand::random_range(0..=vs.domain_max()));
                }
                for k in 0..=vs.domain_max() {
                    assert_eq!(vs.wrapping_add_scalar(k).wrapping_sub_scalar(k), vs);
                    assert_eq!(vs.wrapping_sub_scalar(k).wrapping_add_scalar(k), vs);
                }
            }
        }
    }

    #[test]
    fn wrapping_sub_scalar_examples() {
        let mut vs = ValueSet::new_empty(3);
        vs.insert(0);
        vs.insert(1);
        vs.insert(7);
        let mut minus_one = ValueSet::new_empty(3);
        minus_one.insert(7);
        minus_one.insert(0);
        minus_one.insert(6);
        assert_eq!(vs.wrapping_sub_scalar(1), minus_one);
        assert_eq!(vs.wrapping_sub_scalar(0), vs);
    }

    #[test]
    fn wrapping_mul_scalar_matches_bruteforce() {
        for n_bits in 0..=7 {
            let modulo = 1u32 << n_bits;
            for _ in 0..30 {
                let mut vs = ValueSet::new_empty(n_bits);
                for _ in 0..rand::random_range(0..=vs.domain_max()) {
                    vs.insert(rand::random_range(0..=vs.domain_max()));
                }
                for k in 0..=vs.domain_max() {
                    let mut expected = ValueSet::new_empty(n_bits);
                    for v in vs.iter_values() {
                        expected.insert(((v as u32 * k as u32) % modulo) as u8);
                    }
                    assert_eq!(vs.wrapping_mul_scalar(k), expected, "n_bits={n_bits} k={k}");
                }
            }
        }
    }

    #[test]
    fn wrapping_mul_scalar_examples() {
        let mut vs = ValueSet::new_empty(3);
        vs.insert(1);
        vs.insert(3);
        vs.insert(5);
        assert_eq!(vs.wrapping_mul_scalar(0), ValueSet::from_single(3, 0));
        assert_eq!(vs.wrapping_mul_scalar(1), vs);
        let mut times_two = ValueSet::new_empty(3);
        times_two.insert(2);
        times_two.insert(6);
        assert_eq!(vs.wrapping_mul_scalar(2), times_two);
        let mut times_three = ValueSet::new_empty(3);
        times_three.insert(3);
        times_three.insert(1);
        times_three.insert(7);
        assert_eq!(vs.wrapping_mul_scalar(3), times_three);
        assert_eq!(ValueSet::new_empty(3).wrapping_mul_scalar(0), ValueSet::new_empty(3));
    }

    #[test]
    #[should_panic]
    fn wrapping_add_scalar_rejects_value_outside_domain() {
        ValueSet::new_full(3).wrapping_add_scalar(8);
    }

    #[test]
    #[should_panic]
    fn wrapping_sub_scalar_rejects_value_outside_domain() {
        ValueSet::new_full(3).wrapping_sub_scalar(8);
    }

    #[test]
    #[should_panic]
    fn wrapping_mul_scalar_rejects_value_outside_domain() {
        ValueSet::new_full(3).wrapping_mul_scalar(8);
    }

    #[test]
    fn wrapping_add_matches_bruteforce() {
        for n_bits in 0..=7 {
            let modulo = 1u32 << n_bits;
            for _ in 0..40 {
                let mut a = ValueSet::new_empty(n_bits);
                let mut b = ValueSet::new_empty(n_bits);
                for _ in 0..rand::random_range(0..=a.domain_max()) {
                    a.insert(rand::random_range(0..=a.domain_max()));
                }
                for _ in 0..rand::random_range(0..=b.domain_max()) {
                    b.insert(rand::random_range(0..=b.domain_max()));
                }
                let mut expected = ValueSet::new_empty(n_bits);
                for l in a.iter_values() {
                    for r in b.iter_values() {
                        expected.insert(((l as u32 + r as u32) % modulo) as u8);
                    }
                }
                assert_eq!(ValueSet::wrapping_add(a, b), expected, "n_bits={n_bits}");
                assert_eq!(ValueSet::wrapping_add(b, a), expected, "n_bits={n_bits}");
            }
        }
    }

    #[test]
    fn wrapping_sub_matches_bruteforce() {
        for n_bits in 0..=7 {
            let modulo = 1u32 << n_bits;
            for _ in 0..40 {
                let mut a = ValueSet::new_empty(n_bits);
                let mut b = ValueSet::new_empty(n_bits);
                for _ in 0..rand::random_range(0..=a.domain_max()) {
                    a.insert(rand::random_range(0..=a.domain_max()));
                }
                for _ in 0..rand::random_range(0..=b.domain_max()) {
                    b.insert(rand::random_range(0..=b.domain_max()));
                }
                let mut expected = ValueSet::new_empty(n_bits);
                for l in a.iter_values() {
                    for r in b.iter_values() {
                        expected.insert(((l as u32 + modulo - r as u32) % modulo) as u8);
                    }
                }
                assert_eq!(ValueSet::wrapping_sub(a, b), expected, "n_bits={n_bits}");
            }
        }
    }

    #[test]
    fn wrapping_mul_matches_bruteforce() {
        for n_bits in 0..=7 {
            let modulo = 1u32 << n_bits;
            for _ in 0..40 {
                let mut a = ValueSet::new_empty(n_bits);
                let mut b = ValueSet::new_empty(n_bits);
                for _ in 0..rand::random_range(0..=a.domain_max()) {
                    a.insert(rand::random_range(0..=a.domain_max()));
                }
                for _ in 0..rand::random_range(0..=b.domain_max()) {
                    b.insert(rand::random_range(0..=b.domain_max()));
                }
                let mut expected = ValueSet::new_empty(n_bits);
                for l in a.iter_values() {
                    for r in b.iter_values() {
                        expected.insert(((l as u32 * r as u32) % modulo) as u8);
                    }
                }
                assert_eq!(ValueSet::wrapping_mul(a, b), expected, "n_bits={n_bits}");
                assert_eq!(ValueSet::wrapping_mul(b, a), expected, "n_bits={n_bits}");
            }
        }
    }

    #[test]
    fn set_arithmetic_with_empty_operand_is_empty() {
        for n_bits in 0..=7 {
            let full = ValueSet::new_full(n_bits);
            let empty = ValueSet::new_empty(n_bits);
            assert_eq!(ValueSet::wrapping_add(full, empty), empty);
            assert_eq!(ValueSet::wrapping_add(empty, full), empty);
            assert_eq!(ValueSet::wrapping_sub(full, empty), empty);
            assert_eq!(ValueSet::wrapping_sub(empty, full), empty);
            assert_eq!(ValueSet::wrapping_mul(full, empty), empty);
            assert_eq!(ValueSet::wrapping_mul(empty, full), empty);
        }
    }

    #[test]
    fn set_arithmetic_examples() {
        let mut a = ValueSet::new_empty(3);
        a.insert(1);
        a.insert(2);
        let mut b = ValueSet::new_empty(3);
        b.insert(6);
        b.insert(7);

        let mut sum = ValueSet::new_empty(3);
        sum.insert(7);
        sum.insert(0);
        sum.insert(1);
        assert_eq!(ValueSet::wrapping_add(a, b), sum);

        let mut diff = ValueSet::new_empty(3);
        diff.insert(3);
        diff.insert(2);
        diff.insert(4);
        assert_eq!(ValueSet::wrapping_sub(a, b), diff);

        let mut prod = ValueSet::new_empty(3);
        prod.insert(6);
        prod.insert(7);
        prod.insert(4);
        assert_eq!(ValueSet::wrapping_mul(a, b), prod);
    }

    #[test]
    #[should_panic]
    fn wrapping_add_rejects_mismatched_widths() {
        ValueSet::wrapping_add(ValueSet::new_full(3), ValueSet::new_full(4));
    }

    #[test]
    #[should_panic]
    fn wrapping_sub_rejects_mismatched_widths() {
        ValueSet::wrapping_sub(ValueSet::new_full(3), ValueSet::new_full(4));
    }

    #[test]
    #[should_panic]
    fn wrapping_mul_rejects_mismatched_widths() {
        ValueSet::wrapping_mul(ValueSet::new_full(3), ValueSet::new_full(4));
    }

    #[test]
    fn apply_maps_every_value() {
        let mut vs = ValueSet::new_empty(4);
        for v in 0..4 {
            vs.insert(v);
        }
        let mut doubled = ValueSet::new_empty(4);
        for v in [0, 2, 4, 6] {
            doubled.insert(v);
        }
        assert_eq!(vs.apply(|v| v * 2), doubled);
        let mut flipped = ValueSet::new_empty(4);
        for v in [15, 14, 13, 12] {
            flipped.insert(v);
        }
        assert_eq!(vs.apply(|v| 15 - v), flipped);
        assert_eq!(vs.apply(|_| 7), ValueSet::from_single(4, 7));
        assert_eq!(ValueSet::new_empty(4).apply(|v| v), ValueSet::new_empty(4));
    }

    #[test]
    fn apply_matches_scalar_ops() {
        for n_bits in 0..=7 {
            let mask = ValueSet::new_empty(n_bits).domain_max();
            for _ in 0..30 {
                let mut vs = ValueSet::new_empty(n_bits);
                for _ in 0..rand::random_range(0..=mask) {
                    vs.insert(rand::random_range(0..=mask));
                }
                for k in 0..=mask {
                    assert_eq!(vs.apply(|v| v.wrapping_add(k) & mask), vs.wrapping_add_scalar(k));
                    assert_eq!(vs.apply(|v| v.wrapping_sub(k) & mask), vs.wrapping_sub_scalar(k));
                    assert_eq!(
                        vs.apply(|v| ((v as u16 * k as u16) & mask as u16) as u8),
                        vs.wrapping_mul_scalar(k)
                    );
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn apply_rejects_result_outside_domain() {
        ValueSet::new_full(3).apply(|v| v + 8);
    }

    #[test]
    fn debug_compact_format() {
        let mut vs = ValueSet::new_empty(2);
        vs.insert(0);
        vs.insert(3);
        assert_eq!(format!("{vs:?}"), "ValueSet(2b 2/4 \u{2593}\u{2591}\u{2591}\u{2593})");
        assert_eq!(format!("{:?}", ValueSet::new_empty(0)), "ValueSet(0b 0/1 \u{2591})");
    }

    #[test]
    fn debug_alternate_format_grid() {
        // 3 bits: width 4, height 2.
        let mut vs = ValueSet::new_empty(3);
        vs.insert(0);
        vs.insert(5);
        let expected = format!(
            "{f}{e}{e}{e}\n{e}{f}{e}{e}",
            f = "\u{2593}\u{2593}",
            e = "\u{2591}\u{2591}"
        );
        assert_eq!(format!("{vs:#?}"), expected);
    }

    #[test]
    fn debug_alternate_grid_covers_whole_domain() {
        for n_bits in 0..=7u8 {
            let s = format!("{:#?}", ValueSet::new_full(n_bits));
            assert_eq!(s.chars().filter(|&c| c == '\u{2593}').count() / 2, 1 << n_bits);
            assert_eq!(s.lines().count(), 1 << (n_bits / 2));
        }
    }
}
