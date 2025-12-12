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

/// Enumeration of all available LUT types with their names and arities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LutType {
    None,
    MsgOnly,
    CarryOnly,
    CarryInMsg,
    MultCarryMsg,
    MultCarryMsgLsb,
    MultCarryMsgMsb,
    BwAnd,
    BwOr,
    BwXor,
    CmpSign,
    CmpReduce,
    CmpGt,
    CmpGte,
    CmpLt,
    CmpLte,
    CmpEq,
    CmpNeq,
    ManyGenProp,
    ReduceCarry2,
    ReduceCarry3,
    ReduceCarryPad,
    GenPropAdd,
    IfTrueZeroed,
    IfFalseZeroed,
    Ripple2GenProp,
    TestMany2,
    TestMany4,
    TestMany8,
    ManyCarryMsg,
    CmpGtMrg,
    CmpGteMrg,
    CmpLtMrg,
    CmpLteMrg,
    CmpEqMrg,
    CmpNeqMrg,
    IsSome,
    CarryIsSome,
    CarryIsNone,
    MultCarryMsgIsSome,
    MultCarryMsgMsbIsSome,
    IsNull,
    IsNullPos1,
    NotNull,
    MsgNotNull,
    MsgNotNullPos1,
    ManyMsgSplitShift1,
    SolvePropGroupFinal0,
    SolvePropGroupFinal1,
    SolvePropGroupFinal2,
    ExtractPropGroup0,
    ExtractPropGroup1,
    ExtractPropGroup2,
    ExtractPropGroup3,
    SolveProp,
    SolvePropCarry,
    SolveQuotient,
    SolveQuotientPos1,
    IfPos1FalseZeroed,
    IfPos1FalseZeroedMsgCarry1,
    ShiftLeftByCarryPos0Msg,
    ShiftLeftByCarryPos0MsgNext,
    ShiftRightByCarryPos0Msg,
    ShiftRightByCarryPos0MsgNext,
    IfPos0TrueZeroed,
    IfPos0FalseZeroed,
    IfPos1TrueZeroed,
    ManyInv1CarryMsg,
    ManyInv2CarryMsg,
    ManyInv3CarryMsg,
    ManyInv4CarryMsg,
    ManyInv5CarryMsg,
    ManyInv6CarryMsg,
    ManyInv7CarryMsg,
    ManyMsgSplit,
    Manym2lPropBit1MsgSplit,
    Manym2lPropBit0MsgSplit,
    Manyl2mPropBit1MsgSplit,
    Manyl2mPropBit0MsgSplit,
}

impl LutType {
    /// Returns the string name associated with this LUT type.
    pub fn name(&self) -> &'static str {
        match self {
            LutType::None => "None",
            LutType::MsgOnly => "MsgOnly",
            LutType::CarryOnly => "CarryOnly",
            LutType::CarryInMsg => "CarryInMsg",
            LutType::MultCarryMsg => "MultCarryMsg",
            LutType::MultCarryMsgLsb => "MultCarryMsgLsb",
            LutType::MultCarryMsgMsb => "MultCarryMsgMsb",
            LutType::BwAnd => "BwAnd",
            LutType::BwOr => "BwOr",
            LutType::BwXor => "BwXor",
            LutType::CmpSign => "CmpSign",
            LutType::CmpReduce => "CmpReduce",
            LutType::CmpGt => "CmpGt",
            LutType::CmpGte => "CmpGte",
            LutType::CmpLt => "CmpLt",
            LutType::CmpLte => "CmpLte",
            LutType::CmpEq => "CmpEq",
            LutType::CmpNeq => "CmpNeq",
            LutType::ManyGenProp => "ManyGenProp",
            LutType::ReduceCarry2 => "ReduceCarry2",
            LutType::ReduceCarry3 => "ReduceCarry3",
            LutType::ReduceCarryPad => "ReduceCarryPad",
            LutType::GenPropAdd => "GenPropAdd",
            LutType::IfTrueZeroed => "IfTrueZeroed",
            LutType::IfFalseZeroed => "IfFalseZeroed",
            LutType::Ripple2GenProp => "Ripple2GenProp",
            LutType::TestMany2 => "TestMany2",
            LutType::TestMany4 => "TestMany4",
            LutType::TestMany8 => "TestMany8",
            LutType::ManyCarryMsg => "ManyCarryMsg",
            LutType::CmpGtMrg => "CmpGtMrg",
            LutType::CmpGteMrg => "CmpGteMrg",
            LutType::CmpLtMrg => "CmpLtMrg",
            LutType::CmpLteMrg => "CmpLteMrg",
            LutType::CmpEqMrg => "CmpEqMrg",
            LutType::CmpNeqMrg => "CmpNeqMrg",
            LutType::IsSome => "IsSome",
            LutType::CarryIsSome => "CarryIsSome",
            LutType::CarryIsNone => "CarryIsNone",
            LutType::MultCarryMsgIsSome => "MultCarryMsgIsSome",
            LutType::MultCarryMsgMsbIsSome => "MultCarryMsgMsbIsSome",
            LutType::IsNull => "IsNull",
            LutType::IsNullPos1 => "IsNullPos1",
            LutType::NotNull => "NotNull",
            LutType::MsgNotNull => "MsgNotNull",
            LutType::MsgNotNullPos1 => "MsgNotNullPos1",
            LutType::ManyMsgSplitShift1 => "ManyMsgSplitShift1",
            LutType::SolvePropGroupFinal0 => "SolvePropGroupFinal0",
            LutType::SolvePropGroupFinal1 => "SolvePropGroupFinal1",
            LutType::SolvePropGroupFinal2 => "SolvePropGroupFinal2",
            LutType::ExtractPropGroup0 => "ExtractPropGroup0",
            LutType::ExtractPropGroup1 => "ExtractPropGroup1",
            LutType::ExtractPropGroup2 => "ExtractPropGroup2",
            LutType::ExtractPropGroup3 => "ExtractPropGroup3",
            LutType::SolveProp => "SolveProp",
            LutType::SolvePropCarry => "SolvePropCarry",
            LutType::SolveQuotient => "SolveQuotient",
            LutType::SolveQuotientPos1 => "SolveQuotientPos1",
            LutType::IfPos1FalseZeroed => "IfPos1FalseZeroed",
            LutType::IfPos1FalseZeroedMsgCarry1 => "IfPos1FalseZeroedMsgCarry1",
            LutType::ShiftLeftByCarryPos0Msg => "ShiftLeftByCarryPos0Msg",
            LutType::ShiftLeftByCarryPos0MsgNext => "ShiftLeftByCarryPos0MsgNext",
            LutType::ShiftRightByCarryPos0Msg => "ShiftRightByCarryPos0Msg",
            LutType::ShiftRightByCarryPos0MsgNext => "ShiftRightByCarryPos0MsgNext",
            LutType::IfPos0TrueZeroed => "IfPos0TrueZeroed",
            LutType::IfPos0FalseZeroed => "IfPos0FalseZeroed",
            LutType::IfPos1TrueZeroed => "IfPos1TrueZeroed",
            LutType::ManyInv1CarryMsg => "ManyInv1CarryMsg",
            LutType::ManyInv2CarryMsg => "ManyInv2CarryMsg",
            LutType::ManyInv3CarryMsg => "ManyInv3CarryMsg",
            LutType::ManyInv4CarryMsg => "ManyInv4CarryMsg",
            LutType::ManyInv5CarryMsg => "ManyInv5CarryMsg",
            LutType::ManyInv6CarryMsg => "ManyInv6CarryMsg",
            LutType::ManyInv7CarryMsg => "ManyInv7CarryMsg",
            LutType::ManyMsgSplit => "ManyMsgSplit",
            LutType::Manym2lPropBit1MsgSplit => "Manym2lPropBit1MsgSplit",
            LutType::Manym2lPropBit0MsgSplit => "Manym2lPropBit0MsgSplit",
            LutType::Manyl2mPropBit1MsgSplit => "Manyl2mPropBit1MsgSplit",
            LutType::Manyl2mPropBit0MsgSplit => "Manyl2mPropBit0MsgSplit",
        }
    }

    /// Returns the arity for this LUT type.
    pub fn arity(&self) -> usize {
        match self {
            LutType::None => 1,
            LutType::MsgOnly => 1,
            LutType::CarryOnly => 1,
            LutType::CarryInMsg => 1,
            LutType::MultCarryMsg => 1,
            LutType::MultCarryMsgLsb => 1,
            LutType::MultCarryMsgMsb => 1,
            LutType::BwAnd => 1,
            LutType::BwOr => 1,
            LutType::BwXor => 1,
            LutType::CmpSign => 1,
            LutType::CmpReduce => 1,
            LutType::CmpGt => 1,
            LutType::CmpGte => 1,
            LutType::CmpLt => 1,
            LutType::CmpLte => 1,
            LutType::CmpEq => 1,
            LutType::CmpNeq => 1,
            LutType::ManyGenProp => 2,
            LutType::ReduceCarry2 => 1,
            LutType::ReduceCarry3 => 1,
            LutType::ReduceCarryPad => 1,
            LutType::GenPropAdd => 1,
            LutType::IfTrueZeroed => 1,
            LutType::IfFalseZeroed => 1,
            LutType::Ripple2GenProp => 1,
            LutType::TestMany2 => 2,
            LutType::TestMany4 => 4,
            LutType::TestMany8 => 8,
            LutType::ManyCarryMsg => 2,
            LutType::CmpGtMrg => 1,
            LutType::CmpGteMrg => 1,
            LutType::CmpLtMrg => 1,
            LutType::CmpLteMrg => 1,
            LutType::CmpEqMrg => 1,
            LutType::CmpNeqMrg => 1,
            LutType::IsSome => 1,
            LutType::CarryIsSome => 1,
            LutType::CarryIsNone => 1,
            LutType::MultCarryMsgIsSome => 1,
            LutType::MultCarryMsgMsbIsSome => 1,
            LutType::IsNull => 1,
            LutType::IsNullPos1 => 1,
            LutType::NotNull => 1,
            LutType::MsgNotNull => 1,
            LutType::MsgNotNullPos1 => 1,
            LutType::ManyMsgSplitShift1 => 2,
            LutType::SolvePropGroupFinal0 => 1,
            LutType::SolvePropGroupFinal1 => 1,
            LutType::SolvePropGroupFinal2 => 1,
            LutType::ExtractPropGroup0 => 1,
            LutType::ExtractPropGroup1 => 1,
            LutType::ExtractPropGroup2 => 1,
            LutType::ExtractPropGroup3 => 1,
            LutType::SolveProp => 1,
            LutType::SolvePropCarry => 1,
            LutType::SolveQuotient => 1,
            LutType::SolveQuotientPos1 => 1,
            LutType::IfPos1FalseZeroed => 1,
            LutType::IfPos1FalseZeroedMsgCarry1 => 1,
            LutType::ShiftLeftByCarryPos0Msg => 1,
            LutType::ShiftLeftByCarryPos0MsgNext => 1,
            LutType::ShiftRightByCarryPos0Msg => 1,
            LutType::ShiftRightByCarryPos0MsgNext => 1,
            LutType::IfPos0TrueZeroed => 1,
            LutType::IfPos0FalseZeroed => 1,
            LutType::IfPos1TrueZeroed => 1,
            LutType::ManyInv1CarryMsg => 2,
            LutType::ManyInv2CarryMsg => 2,
            LutType::ManyInv3CarryMsg => 2,
            LutType::ManyInv4CarryMsg => 2,
            LutType::ManyInv5CarryMsg => 2,
            LutType::ManyInv6CarryMsg => 2,
            LutType::ManyInv7CarryMsg => 2,
            LutType::ManyMsgSplit => 2,
            LutType::Manym2lPropBit1MsgSplit => 2,
            LutType::Manym2lPropBit0MsgSplit => 2,
            LutType::Manyl2mPropBit1MsgSplit => 2,
            LutType::Manyl2mPropBit0MsgSplit => 2,
        }
    }
}
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

    /// Dumps the ir.
    pub fn dump(&self) {
        self.ir.dump();
    }

    fn get_input_ctr(&mut self) -> usize {
        let new = self.input_ctr + 1;
        std::mem::replace(&mut self.input_ctr, new)
    }

    fn get_output_ctr(&mut self) -> usize {
        let new = self.output_ctr + 1;
        std::mem::replace(&mut self.output_ctr, new)
    }

    /// Returns a handle to the specified 1-LUT in the circuit.
    pub fn get_lut(&mut self, lut: LutType) -> Lut {
        assert_eq!(lut.arity(), 1);
        self.decl_lut(lut.name())
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

    /// Returns a handle to the specified 2-LUT in the circuit.
    pub fn get_lut2(&mut self, lut: LutType) -> Lut2 {
        assert_eq!(lut.arity(), 2);
        self.decl_lut2(lut.name())
    }

    fn decl_lut2(
        &mut self,
        name: &str,
    ) -> Lut2 {
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

    /// Returns a handle to the specified 4-LUT in the circuit.
    pub fn get_lut4(&mut self, lut: LutType) -> Lut4 {
        assert_eq!(lut.arity(), 4);
        self.decl_lut4(lut.name())
    }

    fn decl_lut4(
        &mut self,
        name: &str,
    ) -> Lut4 {
        let (_node, ret) = self
            .ir
            .add_op(
                Operations::GenerateLut4 {
                    name: name.into(),
                    gene: [
                        LutGenerator::new(|a| a),
                        LutGenerator::new(|a| a),
                        LutGenerator::new(|a| a),
                        LutGenerator::new(|a| a),
                    ],
                },
                svec![],
            )
            .unwrap();
        Lut4(ret[0])
    }

    /// Returns a handle to the specified 8-LUT in the circuit.
    pub fn get_lut8(&mut self, lut: LutType) -> Lut8 {
        assert_eq!(lut.arity(), 8);
        self.decl_lut8(lut.name())
    }

    fn decl_lut8(
        &mut self,
        name: &str,
    ) -> Lut8 {
        let (_node, ret) = self
            .ir
            .add_op(
                Operations::GenerateLut8 {
                    name: name.into(),
                    gene: [
                        LutGenerator::new(|a| a),
                        LutGenerator::new(|a| a),
                        LutGenerator::new(|a| a),
                        LutGenerator::new(|a| a),
                        LutGenerator::new(|a| a),
                        LutGenerator::new(|a| a),
                        LutGenerator::new(|a| a),
                        LutGenerator::new(|a| a),
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
        let lut_none = self.get_lut(LutType::None);
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
