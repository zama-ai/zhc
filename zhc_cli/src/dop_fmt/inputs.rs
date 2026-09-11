//! `--inputs` CLI values: whole-integer template inputs for a `--simulate` run.
//!
//! Each `--inputs` value is a whole integer (decimal, or hex with a `0x` prefix), positional: the
//! `id`-th `--inputs` value seeds the whole ciphertext template source `TS[<id>]` — `dop_fmt`
//! itself decomposes it into that argument's per-block slots (`TS[<id>].0`, `TS[<id>].1`, ...)
//! based on the block width declared in the file's `[signature]` (see `crate::sim`), so the
//! caller never names a block directly. Any `TS[<id>]` beyond the given `--inputs` values is
//! drawn randomly, matching `zhc_builder::Builder::test_random`'s "user value if given, random
//! otherwise" convention.

use std::str::FromStr;

/// One whole-integer `--inputs` value, decimal or `0x`-prefixed hex.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputValue(pub usize);

impl FromStr for InputValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = match s.strip_prefix("0x") {
            Some(hex) => usize::from_str_radix(hex, 16),
            None => s.parse::<usize>(),
        }
        .map_err(|err| format!("`{s}`: invalid value `{s}`: {err}"))?;
        Ok(InputValue(parsed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_decimal() {
        assert_eq!("24".parse::<InputValue>().unwrap(), InputValue(24));
    }

    #[test]
    fn parses_hex() {
        assert_eq!("0x55".parse::<InputValue>().unwrap(), InputValue(0x55));
    }

    #[test]
    fn rejects_garbage() {
        assert!("notanumber".parse::<InputValue>().is_err());
        assert!("0xzz".parse::<InputValue>().is_err());
    }
}
