use hc_ir::{IR, cse::eliminate_common_subexpressions, dce::eliminate_dead_code};
use hc_langs::ioplang::{Ioplang, Litteral, LutGenerator, Operations, Types};
use hc_utils::{iter::{Chunk, ChunkIt}, small::SmallVec, svec};

use crate::builder::{
    BlockConfig, CiphertextBlock, EncryptedInteger, Lut, Lut1Type, Lut2, Lut2Type, PlaintextBlock, PlaintextInteger, blocks_count
};

/// Builder for constructing homomorphic encryption circuits.
pub struct Builder {
    pub(crate) config: BlockConfig,
    pub(crate) ir: IR<Ioplang>,
    pub(crate) input_ctr: usize,
    pub(crate) output_ctr: usize,
}

impl Builder {
    fn get_input_ctr(&mut self) -> usize {
        let new = self.input_ctr + 1;
        std::mem::replace(&mut self.input_ctr, new)
    }

    fn get_output_ctr(&mut self) -> usize {
        let new = self.output_ctr + 1;
        std::mem::replace(&mut self.output_ctr, new)
    }

    fn decl_lut(&mut self, name: &str) -> Lut {
        let (_node, ret) = self
            .ir
            .add_op(
                Operations::GenerateLut {
                    name: name.into(),
                    gene: LutGenerator::new(|a| a),
                },
                svec![],
            )
            .unwrap();
        Lut(ret[0])
    }

    fn decl_lut2(&mut self, name: &str) -> Lut2 {
        let (_node, ret) = self
            .ir
            .add_op(
                Operations::GenerateLut2 {
                    name: name.into(),
                    gene: [LutGenerator::new(|a| a), LutGenerator::new(|a| a)],
                },
                svec![],
            )
            .unwrap();
        Lut2(ret[0])
    }

}

impl Builder {
    /// Creates a new builder with the specified integer `config`.
    pub fn new(config: &BlockConfig) -> Self {
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
    pub fn config(&self) -> &BlockConfig {
        &self.config
    }

    /// Returns a reference to the current IR state.
    pub fn ir(&self) -> &IR<Ioplang> {
        &self.ir
    }

    /// Dumps the ir.
    pub fn dump(&self) {
        println!("{:#}", self.ir);
        panic!()
    }

    /// Returns a handle to the specified 1-LUT in the circuit.
    pub fn lut(&mut self, lut: Lut1Type) -> Lut {
        self.decl_lut(lut.name())
    }

    /// Returns a handle to the specified 2-LUT in the circuit.
    pub fn lut2(&mut self, lut: Lut2Type) -> Lut2 {
        self.decl_lut2(lut.name())
    }

    /// Creates a ciphertext input and returns its blocks.
    pub fn eint_input(&mut self, width: u8) -> EncryptedInteger {
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
        for i in 0..blocks_count(width, &self.config) {
            let (_, index) = self
                .ir
                .add_op(
                    Operations::Constant {
                        value: Litteral::Index(i as usize),
                    },
                    svec![],
                )
                .unwrap();
            let (_, ret) = self
                .ir
                .add_op(Operations::ExtractCtBlock, svec![inp[0], index[0]])
                .unwrap();
            output.push(CiphertextBlock {
                valid: ret[0],
                config: self.config,
            });
        }
        EncryptedInteger::from_blocks(width, output)
    }

    /// Creates a plaintext input and returns its blocks.
    pub fn int_input(&mut self, width: u8) -> PlaintextInteger {
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
        for i in 0..blocks_count(width, &self.config) {
            let (_, index) = self
                .ir
                .add_op(
                    Operations::Constant {
                        value: Litteral::Index(i as usize),
                    },
                    svec![],
                )
                .unwrap();
            let (_, ret) = self
                .ir
                .add_op(Operations::ExtractPtBlock, svec![inp[0], index[0]])
                .unwrap();
            output.push(PlaintextBlock {
                valid: ret[0],
                config: self.config,
            });
        }
        PlaintextInteger::from_blocks(width, output)
    }

    /// Creates a ciphertext output from the given `blocks`.
    pub fn eint_output(&mut self, ct: EncryptedInteger) {
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
        for i in 0..ct.len() {
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
                .add_op(
                    Operations::StoreCtBlock,
                    svec![ct.blocks()[i].valid, acc, index[0]],
                )
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
    pub fn block_constant(&mut self, constant: usize) -> PlaintextBlock {
        let (_node, ret) = self
            .ir
            .add_op(
                Operations::Constant {
                    value: Litteral::PlaintextBlock(constant),
                },
                svec![],
            )
            .unwrap();
        PlaintextBlock {
            valid: ret[0],
            config: self.config,
        }
    }

    /// Adds two ciphertext blocks `src_a` and `src_b`.
    pub fn block_add(
        &mut self,
        src_a: &CiphertextBlock,
        src_b: &CiphertextBlock,
    ) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::AddCt, svec![src_a.valid, src_b.valid])
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            config: self.config,
        }
    }

    /// Adds a ciphertext block `src_a` and a plaintext block `src_b`.
    pub fn block_adds(
        &mut self,
        src_a: &CiphertextBlock,
        src_b: &PlaintextBlock,
    ) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::AddPt, svec![src_a.valid, src_b.valid])
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            config: self.config,
        }
    }

    /// Subtracts ciphertext block `src_b` from `src_a`.
    pub fn block_sub(
        &mut self,
        src_a: &CiphertextBlock,
        src_b: &CiphertextBlock,
    ) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::SubCt, svec![src_a.valid, src_b.valid])
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            config: self.config,
        }
    }

    /// Subtracts plaintext block `src_b` from ciphertext block `src_a`.
    pub fn block_subs(
        &mut self,
        src_a: &CiphertextBlock,
        src_b: &PlaintextBlock,
    ) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::SubPt, svec![src_a.valid, src_b.valid])
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            config: self.config,
        }
    }

    /// Subtracts ciphertext block `src_b` from plaintext block `src_a`.
    pub fn block_ssub(
        &mut self,
        src_a: &PlaintextBlock,
        src_b: &CiphertextBlock,
    ) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::PtSub, svec![src_a.valid, src_b.valid])
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            config: self.config,
        }
    }

    /// Performs multiply-accumulate with constant `cst_a` and blocks `src_a`, `src_b`.
    ///
    /// Computes `cst_a * src_a + src_b` where `cst_a` is plaintext and
    /// `src_a`, `src_b` are ciphertext blocks.
    pub fn block_mac(
        &mut self,
        cst_a: &PlaintextBlock,
        src_a: &CiphertextBlock,
        src_b: &CiphertextBlock,
    ) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(
                Operations::Mac,
                svec![cst_a.valid, src_a.valid, src_b.valid],
            )
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            config: self.config,
        }
    }

    pub fn block_pack(
        &mut self,
        src_a: &CiphertextBlock,
        src_b: &CiphertextBlock,
    ) -> CiphertextBlock {
        let shift = self.block_constant(2usize.pow(self.config.message_width as u32));
        self.block_mac(&shift, src_a, src_b)
    }

    pub fn block_pack_lut(
        &mut self,
        src_a: &CiphertextBlock,
        src_b: &CiphertextBlock,
        lut: &Lut,
    ) -> CiphertextBlock {
        let shift = self.block_constant(2usize.pow(self.config.message_width as u32));
        let maced = self.block_mac(&shift, src_a, src_b);
        self.block_pbs(&maced, lut)
    }

    /// Applies a 1-PBS to `src` using `lut`.
    pub fn block_pbs(&mut self, src: &CiphertextBlock, lut: &Lut) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::Pbs, svec![src.valid, lut.0])
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            config: self.config,
        }
    }

    /// Applies a 2-PBS to `src` using `lut`.
    pub fn block_pbs2(&mut self, src: &CiphertextBlock, lut: &Lut2) -> (CiphertextBlock, CiphertextBlock) {
        let (_node, ret) = self
            .ir
            .add_op(Operations::Pbs2, svec![src.valid, lut.0])
            .unwrap();
        (CiphertextBlock {
            valid: ret[0],
            config: self.config,
        },
        CiphertextBlock {
            valid: ret[1],
            config: self.config,
        }
        )
    }
}

impl Builder {
    pub fn vector_pack_one(
        &mut self,
        blocks: impl AsRef<[CiphertextBlock]>,
    ) -> SmallVec<CiphertextBlock> {
        let shift = self.block_constant(2usize.pow(self.config.message_width as u32));
        blocks
            .as_ref()
            .iter()
            .chunk(2)
            .map(|a| match a {
                Chunk::Complete(sv) => self.block_mac(&shift, sv[1], sv[0]),
                Chunk::Rest(sv) => *sv[0],
            })
            .collect()
    }

    pub fn vector_pack_one_clean(
        &mut self,
        blocks: impl AsRef<[CiphertextBlock]>,
    ) -> SmallVec<CiphertextBlock> {
        let lut_none = self.lut(Lut1Type::None);
        self.vector_pack_one_lut(blocks, &lut_none)
    }

    pub fn vector_pack_one_lut(
        &mut self,
        blocks: impl AsRef<[CiphertextBlock]>,
        lut: &Lut
    ) -> SmallVec<CiphertextBlock> {
        let shift = self.block_constant(2usize.pow(self.config.message_width as u32));
        blocks
            .as_ref()
            .iter()
            .chunk(2)
            .map(|a| match a {
                Chunk::Complete(sv) => {
                    let maced = self.block_mac(&shift, sv[1], sv[0]);
                    self.block_pbs(&maced, lut)
                }
                Chunk::Rest(sv) => *sv[0],
            })
            .collect()
    }

    pub fn vector_pbs(
        &mut self,
        blocks: impl AsRef<[CiphertextBlock]>,
        lut: &Lut,
    ) -> SmallVec<CiphertextBlock> {
        blocks
            .as_ref()
            .iter()
            .map(|b| self.block_pbs(b, lut))
            .collect()
    }

    pub fn vector_add(
        &mut self,
        lhs: impl AsRef<[CiphertextBlock]>,
        rhs: impl AsRef<[CiphertextBlock]>,
        extension: ExtensionBehavior,
    ) -> SmallVec<CiphertextBlock> {
        let mut output = SmallVec::new();
        let mut lhs_i = lhs.as_ref().iter();
        let mut rhs_i = rhs.as_ref().iter();
        loop {
            match (&extension, lhs_i.next(), rhs_i.next()) {
                (_, Some(li), Some(ri)) => output.push(self.block_add(li, ri)),
                (_, None, None) => break,
                (ExtensionBehavior::Crash, _, _) => panic!(),
                (ExtensionBehavior::Limit, _, _) => break,
                (ExtensionBehavior::Passthrough, None, Some(v)) => output.push(*v),
                (ExtensionBehavior::Passthrough, Some(v), None) => output.push(*v),
            }
        }
        return output;
    }
}

/// What to do when performing element wise operations on incompatible sizes
pub enum ExtensionBehavior {
    /// Crash the program
    Crash,
    /// Limit the output size to the short one
    Limit,
    /// Pass the blocks of largest
    Passthrough,
}
