//! `--simulate`: interpretation of a loaded `DopLang` program.
//!
//! Draws inspiration from `zhc_builder::Builder::test_random`'s pattern of giving every declared
//! input a value before evaluation — a user-supplied one when given, or a random one otherwise —
//! then running the whole thing once through the interpreter. Unlike `test_random`'s
//! whole-integer `Ciphertext`/`Plaintext` values keyed by signature position, a raw
//! `dop.asm`/hex file has no such positional input signature at the instruction level; its true
//! inputs are the *template* slots the microcontroller patches at load time (`TS[<id>].<block>`,
//! `TI[<id>].<block>`). `--inputs` (see [`crate::inputs`]) instead names a whole ciphertext by
//! its `TS[<id>]` position — `dop_fmt` decomposes that whole value into the per-block slots
//! itself, using the block count each `Ciphertext` argument declares in the file's `[signature]`
//! — and unnamed `TS[<id>]`s, along with every `TI[<id>]`, are drawn randomly, block by block.
//! Heap and I/O memory are internal staging storage rather than caller inputs, so they always
//! start at zero, matching the interpreted program's initial hardware state rather than standing
//! in for unknown data.
//!
//! The run is a structural smoke test — does the instruction stream evaluate to completion
//! without a runtime panic (wrong argument kind, out-of-range LUT id, unpopulated slot, ...) —
//! not a numeric check against expected outputs (there's nothing here to compare against, unlike
//! `test_random`'s `gen_expect`). On success, the whole-integer values used for every `TS[<id>]`
//! and produced for every `TD[<id>]` are reassembled from their blocks and reported, so a run
//! reads like `5, 24 -> 29`.

use std::collections::{BTreeSet, HashSet};

use zhc_crypto::integer_semantics::{CiphertextBlockSpec, EmulatedCiphertextBlock, Type};
use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::evaluation::OpState;
use zhc_ir::{IR, Signature};
use zhc_langs::doplang::{
    CtMem, CtReg, DopInstructionSet, DopInterpreterContext, DopLang, DopValue, MASK_PBS2,
    MASK_PBS4, MASK_PBS8, PtArg,
};

/// The number of consecutive registers a many-LUT destination's mask spans.
fn reg_span(mask: usize) -> usize {
    match mask {
        MASK_PBS2 => 2,
        MASK_PBS4 => 4,
        MASK_PBS8 => 8,
        _ => 1,
    }
}

#[derive(Default)]
struct Addrs {
    max_reg: Option<usize>,
    heap: HashSet<usize>,
    io: HashSet<usize>,
    ct_src: HashSet<(usize, usize)>,
    pt_src: HashSet<(usize, usize)>,
}

impl Addrs {
    fn visit_reg(&mut self, a: &CtReg) {
        let top = a.addr + reg_span(a.mask) - 1;
        self.max_reg = Some(self.max_reg.map_or(top, |m| m.max(top)));
    }

    fn visit_mem(&mut self, a: &CtMem) {
        match a {
            CtMem::Heap(inner) => {
                self.heap.insert(inner.addr);
            }
            CtMem::Io(inner) => {
                self.io.insert(inner.addr);
            }
            CtMem::Src(inner) => {
                self.ct_src.insert((inner.id, inner.block));
            }
            // A destination template is written by the program itself (`ST`); nothing to seed.
            CtMem::Dst(_) => {}
        }
    }

    fn visit_pt(&mut self, a: &PtArg) {
        if let PtArg::Var(inner) = a {
            self.pt_src.insert((inner.id, inner.block));
        }
    }
}

fn scan(ir: &IR<DopLang>) -> Addrs {
    use DopInstructionSet::*;
    let mut addrs = Addrs::default();
    for op in ir.walk_ops_linear() {
        match op.get_instruction() {
            ADD { dst, src1, src2 } | SUB { dst, src1, src2 } => {
                addrs.visit_reg(dst);
                addrs.visit_reg(src1);
                addrs.visit_reg(src2);
            }
            MAC { dst, src1, src2, cst } => {
                addrs.visit_reg(dst);
                addrs.visit_reg(src1);
                addrs.visit_reg(src2);
                addrs.visit_pt(cst);
            }
            ADDS { dst, src, cst }
            | SUBS { dst, src, cst }
            | SSUB { dst, src, cst }
            | MULS { dst, src, cst } => {
                addrs.visit_reg(dst);
                addrs.visit_reg(src);
                addrs.visit_pt(cst);
            }
            LD { dst, src } => {
                addrs.visit_reg(dst);
                addrs.visit_mem(src);
            }
            ST { dst, src } => {
                addrs.visit_mem(dst);
                addrs.visit_reg(src);
            }
            PBS { dst, src, .. }
            | PBS_ML2 { dst, src, .. }
            | PBS_ML4 { dst, src, .. }
            | PBS_ML8 { dst, src, .. }
            | PBS_F { dst, src, .. }
            | PBS_ML2_F { dst, src, .. }
            | PBS_ML4_F { dst, src, .. }
            | PBS_ML8_F { dst, src, .. } => {
                addrs.visit_reg(dst);
                addrs.visit_reg(src);
            }
            WAIT { slot, .. } => {
                if let Some(slot) = slot {
                    addrs.visit_mem(slot);
                }
            }
            NOTIFY { slot, .. } | LD_B2B { slot, .. } => addrs.visit_mem(slot),
            _START | _END | SYNC => {}
        }
    }
    addrs
}

/// Splits a whole integer into `num_blocks` message-width chunks, block 0 holding the
/// least-significant chunk (matching how `dop.asm` stores a multi-block integer: `TD[i].0` is
/// the low block, `TD[i].1` the next, and so on).
fn decompose(value: usize, message_size: u8, num_blocks: usize) -> Vec<u16> {
    let mask = (1usize << message_size) - 1;
    (0..num_blocks)
        .map(|i| ((value >> (i * message_size as usize)) & mask) as u16)
        .collect()
}

/// Reassembles a whole integer from its per-block message values, the inverse of [`decompose`].
fn recompose(blocks: &[EmulatedCiphertextBlock], message_size: u8) -> usize {
    blocks
        .iter()
        .enumerate()
        .fold(0usize, |acc, (i, b)| {
            acc | ((b.raw_message_bits() as usize) << (i * message_size as usize))
        })
}

/// The declared block count for `Ciphertext` argument/return `id` of `signature`'s given side.
fn num_blocks(types: &[Type], id: usize, side: &str) -> Result<usize, String> {
    match types.get(id) {
        Some(Type::Ciphertext(spec)) => {
            Ok(spec.int_size() as usize / spec.block_spec().message_size() as usize)
        }
        Some(Type::Plaintext(_)) => Err(format!("{side}[{id}] is a Plaintext, expected Ciphertext")),
        None => Err(format!("{side}[{id}] has no matching entry in [signature]")),
    }
}

/// The result of a completed [`simulate`] run.
pub enum SimOutcome {
    /// Every instruction evaluated without panicking; holds the number evaluated.
    Ok(usize),
    /// One `(instruction, panic message)` pair per operation that panicked during evaluation (an
    /// operation whose arguments were poisoned by an earlier failure, rather than one that
    /// panicked itself, is not included).
    Failed(Vec<(String, String)>),
}

/// A completed [`simulate`] run: the whole-integer `TS[<id>]` values actually used (ascending by
/// id), the whole-integer `TD[<id>]` results produced (ascending by id, empty unless `outcome` is
/// [`SimOutcome::Ok`]), and the raw evaluation [`SimOutcome`].
pub struct SimReport {
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
    pub outcome: SimOutcome,
}

/// Runs `ir` end-to-end: heap and I/O memory start at zero. Every `TS[<id>]` named by `inputs`
/// (`inputs[id]`, if present) is decomposed into its blocks per `signature`'s declared block
/// count; every other `TS[<id>]` and every `TI[<id>]` is drawn randomly, block by block.
///
/// # Errors
///
/// Returns an error (rather than panicking) if an `--inputs` value doesn't fit the ciphertext it
/// names, or if a template id has no matching `Ciphertext` entry in `signature` — this is checked
/// before evaluation starts, so it's distinct from a [`SimOutcome`], which only reports what
/// happened *during* evaluation.
pub fn simulate(
    ir: &IR<DopLang>,
    block_spec: CiphertextBlockSpec,
    lut_reg: &LutRegistry,
    signature: &Signature<Type>,
    inputs: &[usize],
) -> Result<SimReport, String> {
    let addrs = scan(ir);
    let num_registers = addrs.max_reg.map_or(1, |m| m + 1);
    let message_size = block_spec.message_size();

    let mut context = DopInterpreterContext::new(block_spec, num_registers, lut_reg);
    for addr in &addrs.heap {
        context.heap.insert(*addr, block_spec.from_data(0));
    }
    for addr in &addrs.io {
        context.io.insert(*addr, block_spec.from_data(0));
    }

    let ts_ids: BTreeSet<usize> = addrs.ct_src.iter().map(|&(id, _)| id).collect();
    let mut used_inputs = Vec::with_capacity(ts_ids.len());
    for id in ts_ids {
        let n = num_blocks(signature.get_args(), id, "TS")?;
        let values = match inputs.get(id) {
            Some(&value) => {
                if n < usize::BITS as usize && value >> (n * message_size as usize) != 0 {
                    return Err(format!(
                        "TS[{id}]: value {value} doesn't fit {n} block(s) of {message_size} bit(s)"
                    ));
                }
                decompose(value, message_size, n)
            }
            None => (0..n).map(|_| block_spec.random().raw_message_bits()).collect(),
        };
        let blocks: Vec<_> = values
            .iter()
            .enumerate()
            .map(|(block, &v)| {
                let ct = block_spec.from_message(v);
                context.sources.insert((id, block), ct);
                ct
            })
            .collect();
        used_inputs.push(recompose(&blocks, message_size));
    }

    let pt_block_spec = block_spec.matching_plaintext_block_spec();
    for &(id, block) in &addrs.pt_src {
        context
            .pt_sources
            .insert((id, block), pt_block_spec.random());
    }

    let outcome = match ir.evaluate::<DopValue>(&mut context) {
        Ok(evaluated) => SimOutcome::Ok(evaluated.walk_ops_linear().count()),
        Err(failed) => SimOutcome::Failed(
            failed
                .walk_ops_linear()
                .filter_map(|op| match op.get_annotation() {
                    OpState::Panicked(msg) => Some((op.get_instruction().to_string(), msg.clone())),
                    _ => None,
                })
                .collect(),
        ),
    };

    let outputs = if matches!(outcome, SimOutcome::Ok(_)) {
        let td_ids: BTreeSet<usize> = context.destinations.keys().map(|&(id, _)| id).collect();
        td_ids
            .into_iter()
            .map(|id| {
                let n = num_blocks(signature.get_returns(), id, "TD")?;
                let blocks: Vec<_> = (0..n)
                    .map(|block| {
                        context
                            .destinations
                            .get(&(id, block))
                            .copied()
                            .unwrap_or_else(|| block_spec.from_message(0))
                    })
                    .collect();
                Ok(recompose(&blocks, message_size))
            })
            .collect::<Result<Vec<_>, String>>()?
    } else {
        Vec::new()
    };

    Ok(SimReport {
        inputs: used_inputs,
        outputs,
        outcome,
    })
}
