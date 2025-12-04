use hpuc_ir::{IR, ValId};
use hpuc_langs::ioplang::{Ioplang, Litteral, LutGenerator, Operations, Types};
use hpuc_utils::{svec, ChunkIt, SmallVec};

#[derive(Debug, Clone)]
pub struct Config {
    pub integer_width: usize,
    pub message_width: usize,
    pub carry_width: usize,
    pub nu_msg: usize,  // Maximum computation that could be applied on a full message
    pub nu_bool: usize, // Maximum computation that could be applied on boolean
}

impl Config {
    pub fn block_count(&self) -> usize {
        (self.integer_width + self.message_width - 1) / self.message_width
    }
}

#[derive(Clone, Copy)]
pub struct CiphertextBlock(pub ValId);
#[derive(Clone, Copy)]
pub struct PlaintextBlock(pub ValId);
#[derive(Clone, Copy)]
pub struct Lut(pub ValId);
#[derive(Clone, Copy)]
pub struct Lut2(pub ValId);
#[derive(Clone, Copy)]
pub struct Lut4(pub ValId);
#[derive(Clone, Copy)]
pub struct Lut8(pub ValId);

pub struct Builder {
    config: Config,
    ir: IR<Ioplang>,
    input_ctr: usize,
    output_ctr: usize
}

impl Builder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            ir: IR::empty(),
            input_ctr: 0,
            output_ctr: 0
        }
    }

    pub fn into_ir(self) -> IR<Ioplang> {
        self.ir
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

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

    pub fn input_ct(&mut self) -> SmallVec<CiphertextBlock> {
        let pos = self.get_input_ctr();
        let (_, inp) = self.ir.add_op(Operations::Input { pos, typ: Types::Ciphertext }, svec![]).unwrap();
        let mut output = SmallVec::new();
        for i in 0..self.config().block_count() {
            let (_, index) = self.ir.add_op(Operations::Constant { value: Litteral::Index(i) }, svec![]).unwrap();
            let (_, ret) = self.ir.add_op(Operations::ExtractCtBlock, svec![inp[0], index[0]]).unwrap();
            output.push(CiphertextBlock(ret[0]));
        }
        output
    }

    pub fn input_pt(&mut self) -> SmallVec<PlaintextBlock> {
        let pos = self.get_input_ctr();
        let (_, inp) = self.ir.add_op(Operations::Input { pos, typ: Types::Plaintext }, svec![]).unwrap();
        let mut output = SmallVec::new();
        for i in 0..self.config().block_count() {
            let (_, index) = self.ir.add_op(Operations::Constant { value: Litteral::Index(i) }, svec![]).unwrap();
            let (_, ret) = self.ir.add_op(Operations::ExtractPtBlock, svec![inp[0], index[0]]).unwrap();
            output.push(PlaintextBlock(ret[0]));
        }
        output
    }

    pub fn output_ct(&mut self, blocks: SmallVec<CiphertextBlock>) {
        let (_, acc) = self.ir.add_op(Operations::Let { typ: Types::Ciphertext }, svec![]).unwrap();
        let mut acc = acc[0];
        for i in 0..blocks.len() {
            let (_, index) = self.ir.add_op(Operations::Constant { value: Litteral::Index(i) }, svec![]).unwrap();
            let (_, ret) = self.ir.add_op(Operations::StoreCtBlock, svec![blocks[i].0, acc, index[0]]).unwrap();
            acc = ret[0];
        }
        let pos = self.get_output_ctr();
        self.ir.add_op(Operations::Output { pos, typ: Types::Ciphertext }, svec![acc]).unwrap();
    }

    pub fn constant(&mut self, constant: usize) -> PlaintextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::Constant { value: Litteral::PlaintextBlock(constant) }, svec![])
            .unwrap();
        PlaintextBlock(ret[0])
    }

    pub fn pack(&mut self, blocks: SmallVec<CiphertextBlock>) -> SmallVec<CiphertextBlock> {
        let shift = self.constant(2usize.pow(self.config.message_width as u32));
        blocks.into_iter().chunk(2).map(|a| {
            match a {
                hpuc_utils::Chunk::Complete(sv) => {
                    self.mac(shift, sv[1], sv[0])
                },
                hpuc_utils::Chunk::Rest(sv) => {
                    sv[0]
                },
            }
        }).collect()
    }

    pub fn add(&mut self, src_a: CiphertextBlock, src_b: CiphertextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::AddCt, svec![src_a.0, src_b.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    pub fn adds(&mut self, src_a: CiphertextBlock, src_b: PlaintextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::AddPt, svec![src_a.0, src_b.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    pub fn sub(&mut self, src_a: CiphertextBlock, src_b: CiphertextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::SubCt, svec![src_a.0, src_b.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    pub fn subs(&mut self, src_a: CiphertextBlock, src_b: PlaintextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::SubPt, svec![src_a.0, src_b.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    pub fn ssub(&mut self, src_a: PlaintextBlock, src_b: CiphertextBlock) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::PtSub, svec![src_a.0, src_b.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

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

    pub fn pbs(&mut self, src: CiphertextBlock, lut: Lut) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::Pbs, svec![src.0, lut.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    pub fn pbs2(&mut self, src: CiphertextBlock, lut: Lut2) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::Pbs2, svec![src.0, lut.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    pub fn pbs4(&mut self, src: CiphertextBlock, lut: Lut4) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::Pbs4, svec![src.0, lut.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }

    pub fn pbs8(&mut self, src: CiphertextBlock, lut: Lut8) -> CiphertextBlock {
        let (_node, ret) = self
            .ir
            .add_op(Operations::Pbs8, svec![src.0, lut.0])
            .unwrap();
        CiphertextBlock(ret[0])
    }
}
