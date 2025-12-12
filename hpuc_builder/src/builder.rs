use hpuc_ir::{IR, ValId, cse::eliminate_common_subexpressions, dce::eliminate_dead_code};
use hpuc_langs::ioplang::{Ioplang, Litteral, LutGenerator, Operations, Types};
use hpuc_utils::{ChunkIt, SmallVec, svec};

/// Configuration parameters for homomorphic integer operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntegerConfig {
    pub integer_width: usize,
    pub message_width: usize,
    pub carry_width: usize,
    pub nu_msg: usize, // Maximum computation that could be applied on a full message
    pub nu_bool: usize, // Maximum computation that could be applied on boolean
}

impl IntegerConfig {
    /// Returns the number of blocks needed to represent an integer.
    pub fn block_count(&self) -> usize {
        (self.integer_width + self.message_width - 1) / self.message_width
    }
}

/// A handle to an encrypted message block in the IR.
#[derive(Clone, Copy)]
pub struct CiphertextBlock(pub ValId);

/// A handle to a plaintext message block in the IR.
#[derive(Clone, Copy)]
pub struct PlaintextBlock(pub ValId);

/// A handle to a 1-LUT in the IR.
#[derive(Clone, Copy)]
pub struct Lut(pub ValId);

/// A handle to 2-LUT in the IR.
#[derive(Clone, Copy)]
pub struct Lut2(pub ValId);

/// A handle to 4-LUT in the IR.
#[derive(Clone, Copy)]
pub struct Lut4(pub ValId);

/// A handle to 8-LUT in the IR.
#[derive(Clone, Copy)]
pub struct Lut8(pub ValId);

/// Builder for constructing homomorphic encryption circuits.
pub struct Builder {
    config: IntegerConfig,
    ir: IR<Ioplang>,
    input_ctr: usize,
    output_ctr: usize,
}

impl Builder {
    /// Creates a new builder with the specified integer `config`.
    pub fn new(config: &IntegerConfig) -> Self {
        Self {
            config: config.to_owned(),
            ir: IR::empty(),
            input_ctr: 0,
            output_ctr: 0,
        }
    }

    /// Consumes the builder and returns the optimized IR.
    ///
    /// This method applies dead code elimination and common subexpression
    /// elimination before returning the final IR representation.
    pub fn into_ir(self) -> IR<Ioplang> {
        let mut ir = self.ir;
        eliminate_dead_code(&mut ir);
        eliminate_common_subexpressions(&mut ir);
        ir
    }

    /// Returns a reference to the integer configuration.
    pub fn config(&self) -> &IntegerConfig {
        &self.config
    }

    /// Returns a reference to the current IR state.
    pub fn ir(&self) -> &IR<Ioplang> {
        &self.ir
    }

    fn get_input_ctr(&mut self) -> usize {
        let new = self.input_ctr + 1;
        std::mem::replace(&mut self.input_ctr, new)
    }

    fn get_output_ctr(&mut self) -> usize {
        let new = self.output_ctr + 1;
        std::mem::replace(&mut self.output_ctr, new)
    }

    /// Declares a 1-LUT with the given `name` and function `f`.
    pub fn decl_lut(&mut self, name: &str, f: impl Fn(u8) -> u8 + 'static) -> Lut {
        let (_node, ret) = self
            .ir
            .add_op(
                Operations::GenerateLut {
                    name: name.into(),
                    gene: LutGenerator::new(f),
                },
                svec![],
            )
            .unwrap();
        Lut(ret[0])
    }

    /// Declares a 2-LUT with the given `name` and functions `f1`, `f2`.
    pub fn decl_lut2(
        &mut self,
        name: &str,
        f1: impl Fn(u8) -> u8 + 'static,
        f2: impl Fn(u8) -> u8 + 'static,
    ) -> Lut2 {
        let (_node, ret) = self
            .ir
            .add_op(
                Operations::GenerateLut2 {
                    name: name.into(),
                    gene: [LutGenerator::new(f1), LutGenerator::new(f2)],
                },
                svec![],
            )
            .unwrap();
        Lut2(ret[0])
    }

    /// Declares a 4-LUT with the given `name` and functions.
    pub fn decl_lut4(
        &mut self,
        name: &str,
        f1: impl Fn(u8) -> u8 + 'static,
        f2: impl Fn(u8) -> u8 + 'static,
        f3: impl Fn(u8) -> u8 + 'static,
        f4: impl Fn(u8) -> u8 + 'static,
    ) -> Lut4 {
        let (_node, ret) = self
            .ir
            .add_op(
                Operations::GenerateLut4 {
                    name: name.into(),
                    gene: [
                        LutGenerator::new(f1),
                        LutGenerator::new(f2),
                        LutGenerator::new(f3),
                        LutGenerator::new(f4),
                    ],
                },
                svec![],
            )
            .unwrap();
        Lut4(ret[0])
    }

    /// Declares an 8-LUT with the given `name` and functions.
    pub fn decl_lut8(
        &mut self,
        name: &str,
        f1: impl Fn(u8) -> u8 + 'static,
        f2: impl Fn(u8) -> u8 + 'static,
        f3: impl Fn(u8) -> u8 + 'static,
        f4: impl Fn(u8) -> u8 + 'static,
        f5: impl Fn(u8) -> u8 + 'static,
        f6: impl Fn(u8) -> u8 + 'static,
        f7: impl Fn(u8) -> u8 + 'static,
        f8: impl Fn(u8) -> u8 + 'static,
    ) -> Lut8 {
        let (_node, ret) = self
            .ir
            .add_op(
                Operations::GenerateLut8 {
                    name: name.into(),
                    gene: [
                        LutGenerator::new(f1),
                        LutGenerator::new(f2),
                        LutGenerator::new(f3),
                        LutGenerator::new(f4),
                        LutGenerator::new(f5),
                        LutGenerator::new(f6),
                        LutGenerator::new(f7),
                        LutGenerator::new(f8),
                    ],
                },
                svec![],
            )
            .unwrap();
        Lut8(ret[0])
    }

    /// Creates a ciphertext input and returns its blocks.
    pub fn input_ct(&mut self) -> SmallVec<CiphertextBlock> {
        let pos = self.get_input_ctr();
        let (_, inp) = self
            .ir
            .add_op(
                Operations::Input {
                    pos,
                    typ: Types::Ciphertext,
                },
                svec![],
            )
            .unwrap();
        let mut output = SmallVec::new();
        for i in 0..self.config().block_count() {
            let (_, index) = self
                .ir
                .add_op(
                    Operations::Constant {
                        value: Litteral::Index(i),
                    },
                    svec![],
                )
                .unwrap();
            let (_, ret) = self
                .ir
                .add_op(Operations::ExtractCtBlock, svec![inp[0], index[0]])
                .unwrap();
            output.push(CiphertextBlock(ret[0]));
        }
        output
    }

    /// Creates a plaintext input and returns its blocks.
    pub fn input_pt(&mut self) -> SmallVec<PlaintextBlock> {
        let pos = self.get_input_ctr();
        let (_, inp) = self
            .ir
            .add_op(
                Operations::Input {
                    pos,
                    typ: Types::Plaintext,
                },
                svec![],
            )
            .unwrap();
        let mut output = SmallVec::new();
        for i in 0..self.config().block_count() {
            let (_, index) = self
                .ir
                .add_op(
                    Operations::Constant {
                        value: Litteral::Index(i),
                    },
                    svec![],
                )
                .unwrap();
            let (_, ret) = self
                .ir
                .add_op(Operations::ExtractPtBlock, svec![inp[0], index[0]])
                .unwrap();
            output.push(PlaintextBlock(ret[0]));
        }
        output
    }

    /// Creates a ciphertext output from the given `blocks`.
    pub fn output_ct(&mut self, blocks: SmallVec<CiphertextBlock>) {
        let (_, acc) = self
            .ir
            .add_op(
                Operations::Let {
                    typ: Types::Ciphertext,
                },
                svec![],
            )
            .unwrap();
        let mut acc = acc[0];
        for i in 0..blocks.len() {
            let (_, index) = self
                .ir
                .add_op(
                    Operations::Constant {
                        value: Litteral::Index(i),
                    },
                    svec![],
                )
                .unwrap();
            let (_, ret) = self
                .ir
                .add_op(Operations::StoreCtBlock, svec![blocks[i].0, acc, index[0]])
                .unwrap();
            acc = ret[0];
        }
        let pos = self.get_output_ctr();
        self.ir
            .add_op(
                Operations::Output {
                    pos,
                    typ: Types::Ciphertext,
                },
                svec![acc],
            )
            .unwrap();
    }

    /// Creates a plaintext block containing the specified `constant`.
    pub fn constant(&mut self, constant: usize) -> PlaintextBlock {
        let (_node, ret) = self
            .ir
            .add_op(
                Operations::Constant {
                    value: Litteral::PlaintextBlock(constant),
                },
                svec![],
            )
            .unwrap();
        PlaintextBlock(ret[0])
    }

    /// Packs ciphertext blocks by combining pairs into single blocks.
    ///
    /// This operation reduces the number of blocks by half by packing two
    /// consecutive blocks into one using multiply-accumulate and PBS operations.
    pub fn pack(&mut self, blocks: SmallVec<CiphertextBlock>) -> SmallVec<CiphertextBlock> {
        let shift = self.constant(2usize.pow(self.config.message_width as u32));
        let lut_none = self.decl_lut("None", |a| a);
        blocks
            .into_iter()
            .chunk(2)
            .map(|a| match a {
                hpuc_utils::Chunk::Complete(sv) => {
                    let maced = self.mac(shift, sv[1], sv[0]);
                    self.pbs(maced, lut_none)
                }
                hpuc_utils::Chunk::Rest(sv) => sv[0],
            })
            .collect()
    }

    /// Adds two ciphertext blocks `src_a` and `src_b`.
    pub fn add(&mut self, src_a: CiphertextBlock, src_b: CiphertextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::AddCt, svec![src_a.0, src_b.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    /// Adds a ciphertext block `src_a` and a plaintext block `src_b`.
    pub fn adds(&mut self, src_a: CiphertextBlock, src_b: PlaintextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::AddPt, svec![src_a.0, src_b.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    /// Subtracts ciphertext block `src_b` from `src_a`.
    pub fn sub(&mut self, src_a: CiphertextBlock, src_b: CiphertextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::SubCt, svec![src_a.0, src_b.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    /// Subtracts plaintext block `src_b` from ciphertext block `src_a`.
    pub fn subs(&mut self, src_a: CiphertextBlock, src_b: PlaintextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::SubPt, svec![src_a.0, src_b.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    /// Subtracts ciphertext block `src_b` from plaintext block `src_a`.
    pub fn ssub(&mut self, src_a: PlaintextBlock, src_b: CiphertextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::PtSub, svec![src_a.0, src_b.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    /// Performs multiply-accumulate with constant `cst_a` and blocks `src_a`, `src_b`.
    ///
    /// Computes `cst_a * src_a + src_b` where `cst_a` is plaintext and
    /// `src_a`, `src_b` are ciphertext blocks.
    pub fn mac(
        &mut self,
        cst_a: PlaintextBlock,
        src_a: CiphertextBlock,
        src_b: CiphertextBlock,
    ) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::Mac, svec![cst_a.0, src_a.0, src_b.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    /// Applies a 1-PBS to `src` using `lut`.
    pub fn pbs(&mut self, src: CiphertextBlock, lut: Lut) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::Pbs, svec![src.0, lut.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    /// Applies a 2-PBS to `src` using `lut`.
    pub fn pbs2(&mut self, src: CiphertextBlock, lut: Lut2) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::Pbs2, svec![src.0, lut.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    /// Applies a 4-PBS to `src` using `lut`.
    pub fn pbs4(&mut self, src: CiphertextBlock, lut: Lut4) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::Pbs4, svec![src.0, lut.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    /// Applies an 8-PBS to `src` using `lut`.
    pub fn pbs8(&mut self, src: CiphertextBlock, lut: Lut8) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::Pbs8, svec![src.0, lut.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }
}
