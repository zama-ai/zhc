use std::cell::{Ref, RefCell};

use hc_crypto::integer_semantics::{CiphertextBlockSpec, PlaintextBlockStorage};
use hc_ir::{IR, cse::eliminate_common_subexpressions, dce::eliminate_dead_code};
use hc_langs::ioplang::{
    IopInstructionSet, IopInterepreterContext, IopLang, IopTypeSystem, IopValue, Lut1Def, Lut2Def,
};
use hc_utils::{
    FastMap, iter::{Chunk, ChunkIt}, small::SmallVec, svec
};

use crate::builder::{Ciphertext, CiphertextBlock, Plaintext, PlaintextBlock};

/// Builder for constructing homomorphic encryption circuits.
pub struct Builder {
    pub(crate) spec: CiphertextBlockSpec,
    pub(crate) ir: RefCell<IR<IopLang>>,
    pub(crate) input_ctr: RefCell<usize>,
    pub(crate) output_ctr: RefCell<usize>,
}

impl Builder {
    fn get_input_ctr(&self) -> usize {
        let mut ctr = self.input_ctr.borrow_mut();
        let out = *ctr;
        *ctr += 1;
        out
    }

    fn get_output_ctr(&self) -> usize {
        let mut ctr = self.output_ctr.borrow_mut();
        let out = *ctr;
        *ctr += 1;
        out
    }
}

impl Builder {
    /// Creates a new builder with the specified block `spec`.
    pub fn new(spec: CiphertextBlockSpec) -> Self {
        Self {
            spec: spec,
            ir: RefCell::new(IR::empty()),
            input_ctr: RefCell::new(0),
            output_ctr: RefCell::new(0),
        }
    }

    /// Consumes the builder and returns the optimized IR.
    ///
    /// This method applies dead code elimination and common subexpression
    /// elimination before returning the final IR representation.
    pub fn into_ir(self) -> IR<IopLang> {
        let mut ir = self.ir.into_inner();
        eliminate_dead_code(&mut ir);
        eliminate_common_subexpressions(&mut ir);
        ir
    }

    /// Returns a reference to the integer spec.
    pub fn spec(&self) -> &CiphertextBlockSpec {
        &self.spec
    }

    /// Returns a reference to the current IR state.
    pub fn ir(&self) -> Ref<'_, IR<IopLang>> {
        self.ir.borrow()
    }

    /// Dumps the ir.
    pub fn dump_panic(&self) {
        println!("{:#}", self.ir.borrow());
        panic!()
    }

    /// Evaluates the IR with given inputs and panics with the interpretation-annotated graph.
    pub fn dump_eval_panic(&self, inputs: SmallVec<IopValue>) {
        let max_int_size = inputs
            .iter()
            .filter_map(|a| match a {
                IopValue::Ciphertext(ciphertext) => Some(ciphertext.spec().int_size()),
                _ => None,
            })
            .max()
            .unwrap();
        let context = IopInterepreterContext {
            spec: self.spec.ciphertext_spec(max_int_size),
            inputs: inputs.into_iter().enumerate().collect(),
            outputs: FastMap::new(),
        };
        let ir = self.ir.borrow();
        let (interpreted, _) = ir.interpret(context);
        println!("{:#}", interpreted);
        panic!()
    }

    /// Creates a ciphertext input and returns its blocks.
    pub fn eint_input(&self, int_size: u16) -> Ciphertext {
        let pos = self.get_input_ctr();
        let (_, inp) = self
            .ir
            .borrow_mut()
            .add_op(
                IopInstructionSet::Input {
                    pos,
                    typ: IopTypeSystem::Ciphertext,
                },
                svec![],
            )
            .unwrap();
        let mut output = SmallVec::new();
        let ct_spec = self.spec.ciphertext_spec(int_size);
        for index in 0..ct_spec.block_count() {
            let (_, ret) = self
                .ir
                .borrow_mut()
                .add_op(IopInstructionSet::ExtractCtBlock { index }, svec![inp[0]])
                .unwrap();
            output.push(CiphertextBlock {
                valid: ret[0],
                spec: self.spec,
            });
        }
        Ciphertext::from_blocks(output)
    }

    /// Creates a plaintext input and returns its blocks.
    pub fn int_input(&self, int_size: u16) -> Plaintext {
        let pos = self.get_input_ctr();
        let (_, inp) = self
            .ir
            .borrow_mut()
            .add_op(
                IopInstructionSet::Input {
                    pos,
                    typ: IopTypeSystem::Plaintext,
                },
                svec![],
            )
            .unwrap();
        let mut output = SmallVec::new();
        let pt_spec = self
            .spec()
            .matching_plaintext_block_spec()
            .plaintext_spec(int_size);
        for index in 0..pt_spec.block_count() {
            let (_, ret) = self
                .ir
                .borrow_mut()
                .add_op(IopInstructionSet::ExtractPtBlock { index }, svec![inp[0]])
                .unwrap();
            output.push(PlaintextBlock {
                valid: ret[0],
                spec: self.spec.matching_plaintext_block_spec(),
            });
        }
        Plaintext::from_blocks(output)
    }

    /// Creates a ciphertext output from the given `blocks`.
    pub fn eint_output(&self, ct: Ciphertext) {
        let (_, acc) = self
            .ir
            .borrow_mut()
            .add_op(IopInstructionSet::ZeroCiphertext, svec![])
            .unwrap();
        let mut acc = acc[0];
        for index in 0..TryInto::<u8>::try_into(ct.len()).unwrap() {
            let (_, ret) = self
                .ir
                .borrow_mut()
                .add_op(
                    IopInstructionSet::StoreCtBlock { index },
                    svec![ct.blocks()[index as usize].valid, acc],
                )
                .unwrap();
            acc = ret[0];
        }
        let pos = self.get_output_ctr();
        self.ir
            .borrow_mut()
            .add_op(
                IopInstructionSet::Output {
                    pos,
                    typ: IopTypeSystem::Ciphertext,
                },
                svec![acc],
            )
            .unwrap();
    }

    /// Creates a plaintext block containing the specified `constant`.
    pub fn block_constant(&self, constant: u8) -> PlaintextBlock {
        let (_node, ret) = self
            .ir
            .borrow_mut()
            .add_op(
                IopInstructionSet::LetPlaintextBlock {
                    value: constant as PlaintextBlockStorage,
                },
                svec![],
            )
            .unwrap();
        PlaintextBlock {
            valid: ret[0],
            spec: self.spec.matching_plaintext_block_spec(),
        }
    }

    /// Adds two ciphertext blocks `src_a` and `src_b`.
    pub fn block_add(&self, src_a: &CiphertextBlock, src_b: &CiphertextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .borrow_mut()
            .add_op(IopInstructionSet::AddCt, svec![src_a.valid, src_b.valid])
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Adds a ciphertext block `src_a` and a plaintext block `src_b`.
    pub fn block_adds(&self, src_a: &CiphertextBlock, src_b: &PlaintextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .borrow_mut()
            .add_op(IopInstructionSet::AddPt, svec![src_a.valid, src_b.valid])
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Subtracts ciphertext block `src_b` from `src_a`.
    pub fn block_sub(&self, src_a: &CiphertextBlock, src_b: &CiphertextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .borrow_mut()
            .add_op(IopInstructionSet::SubCt, svec![src_a.valid, src_b.valid])
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Subtracts plaintext block `src_b` from ciphertext block `src_a`.
    pub fn block_subs(&self, src_a: &CiphertextBlock, src_b: &PlaintextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .borrow_mut()
            .add_op(IopInstructionSet::SubPt, svec![src_a.valid, src_b.valid])
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Subtracts ciphertext block `src_b` from plaintext block `src_a`.
    pub fn block_ssub(&self, src_a: &PlaintextBlock, src_b: &CiphertextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .borrow_mut()
            .add_op(IopInstructionSet::PtSub, svec![src_a.valid, src_b.valid])
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    pub fn block_pack_ct(
        &self,
        src_a: &CiphertextBlock,
        src_b: &CiphertextBlock,
    ) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .borrow_mut()
            .add_op(
                IopInstructionSet::PackCt {
                    mul: 2u8.pow(self.spec().message_size() as u32) as PlaintextBlockStorage,
                },
                svec![src_a.valid, src_b.valid],
            )
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    pub fn block_pack_lut(
        &self,
        src_a: &CiphertextBlock,
        src_b: &CiphertextBlock,
        lut: Lut1Def,
    ) -> CiphertextBlock {
        let packed = self.block_pack_ct(src_a, src_b);
        self.block_pbs(&packed, lut)
    }

    /// Applies a 1-PBS to `src` using `lut`.
    pub fn block_pbs(&self, src: &CiphertextBlock, lut: Lut1Def) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .borrow_mut()
            .add_op(IopInstructionSet::Pbs { lut }, svec![src.valid])
            .unwrap();
        CiphertextBlock {
            valid: ret[0],
            spec: self.spec,
        }
    }

    /// Applies a 2-PBS to `src` using `lut`.
    pub fn block_pbs2(
        &self,
        src: &CiphertextBlock,
        lut: Lut2Def,
    ) -> (CiphertextBlock, CiphertextBlock) {
        let (_node, ret) = self
            .ir
            .borrow_mut()
            .add_op(IopInstructionSet::Pbs2 { lut }, svec![src.valid])
            .unwrap();
        (
            CiphertextBlock {
                valid: ret[0],
                spec: self.spec,
            },
            CiphertextBlock {
                valid: ret[1],
                spec: self.spec,
            },
        )
    }
}

impl Builder {
    pub fn vector_pack_one(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
    ) -> SmallVec<CiphertextBlock> {
        blocks
            .as_ref()
            .iter()
            .chunk(2)
            .map(|a| match a {
                Chunk::Complete(sv) => self.block_pack_ct(sv[1], sv[0]),
                Chunk::Rest(sv) => *sv[0],
            })
            .collect()
    }

    pub fn vector_pack_one_clean(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
    ) -> SmallVec<CiphertextBlock> {
        self.vector_pack_one_lut(blocks, Lut1Def::None)
    }

    pub fn vector_pack_one_lut(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
        lut: Lut1Def,
    ) -> SmallVec<CiphertextBlock> {
        blocks
            .as_ref()
            .iter()
            .chunk(2)
            .map(|a| match a {
                Chunk::Complete(sv) => {
                    let packed = self.block_pack_ct(sv[1], sv[0]);
                    self.block_pbs(&packed, lut)
                }
                Chunk::Rest(sv) => *sv[0],
            })
            .collect()
    }

    pub fn vector_pbs(
        &self,
        blocks: impl AsRef<[CiphertextBlock]>,
        lut: Lut1Def,
    ) -> SmallVec<CiphertextBlock> {
        blocks
            .as_ref()
            .iter()
            .map(|b| self.block_pbs(b, lut))
            .collect()
    }

    pub fn vector_add(
        &self,
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
