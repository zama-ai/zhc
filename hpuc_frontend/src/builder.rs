//! Defined a Builder
//! Extend IR with some context and a set of utilities function

use std::sync::{Arc, Mutex, MutexGuard};

use hpuc_ir::{IR, IRError, ValId};
use hpuc_langs::ioplang::{Ioplang, Operations, Types};
use hpuc_utils::SmallVec;
use hpuc_utils::svec;

/// Context structure use to spread constant in the builder environnement
#[derive(Debug, Clone)]
pub struct BuilderContext {
    pub integer_w: i64,
    pub msg_w: i64,
    pub carry_w: i64,
    pub nu_msg: i64,  // Maximum computation that could be applied on a full message
    pub nu_bool: i64, // Maximum computation that could be applied on boolean
}

impl BuilderContext {
    pub fn blk_nb(&self) -> i64 {
        (self.integer_w + self.msg_w - 1) / self.msg_w
    }
}

#[derive(Clone)]
pub struct IopBuilder {
    context: BuilderContext,
    ir: Arc<Mutex<IR<Ioplang>>>,
}

impl IopBuilder {
    pub fn new(context: BuilderContext) -> Self {
        Self {
            context,
            ir: Arc::new(Mutex::new(IR::empty())),
        }
    }

    pub fn context(&self) -> &BuilderContext {
        &self.context
    }

    pub fn ir(&self) -> MutexGuard<'_, IR<Ioplang>> {
        self.ir.lock().unwrap()
    }
}

impl TryFrom<IopBuilder> for IR<Ioplang> {
    type Error = Arc<Mutex<IR<Ioplang>>>;

    fn try_from(value: IopBuilder) -> Result<Self, Self::Error> {
        let mtx = Arc::try_unwrap(value.ir)?;
        Ok(mtx.into_inner().unwrap())
    }
}

/// Implement basics operation as function for ease binding with frontend
impl IopBuilder {
    pub fn add(&self, src_a: ValId, src_b: ValId) -> Result<ValId, IRError<Ioplang>> {
        let mut ir = self.ir.lock().unwrap();
        let (_node, ret) = ir.add_op(Operations::AddCt, svec![src_a, src_b])?;
        Ok(ret[0])
    }
    pub fn adds(&self, src_a: ValId, src_b: ValId) -> Result<ValId, IRError<Ioplang>> {
        let mut ir = self.ir.lock().unwrap();
        let (_node, ret) = ir.add_op(Operations::AddPt, svec![src_a, src_b])?;
        Ok(ret[0])
    }
    pub fn addx(&self, src_a: ValId, src_b: ValId) -> Result<ValId, IRError<Ioplang>> {
        // Extract type in sub-scope to correctly handle ir mutex
        let (typ_a, typ_b) = {
            let ir = self.ir.lock().unwrap();
            let typ_a = ir.get_val(src_a).get_type();
            let typ_b = ir.get_val(src_b).get_type();
            (typ_a, typ_b)
        };
        match (typ_a, typ_b) {
            (Types::CiphertextBlock, Types::PlaintextBlock) => self.adds(src_a, src_b),
            (Types::PlaintextBlock, Types::CiphertextBlock) => self.adds(src_b, src_a),
            _ => self.add(src_a, src_b),
        }
    }

    pub fn sub(&self, src_a: ValId, src_b: ValId) -> Result<ValId, IRError<Ioplang>> {
        let mut ir = self.ir.lock().unwrap();
        let (_node, ret) = ir.add_op(Operations::SubCt, svec![src_a, src_b])?;
        Ok(ret[0])
    }

    pub fn subs(&self, src_a: ValId, src_b: ValId) -> Result<ValId, IRError<Ioplang>> {
        let mut ir = self.ir.lock().unwrap();
        let (_node, ret) = ir.add_op(Operations::SubPt, svec![src_a, src_b])?;
        Ok(ret[0])
    }
    pub fn ssub(&self, src_a: ValId, src_b: ValId) -> Result<ValId, IRError<Ioplang>> {
        let mut ir = self.ir.lock().unwrap();
        let (_node, ret) = ir.add_op(Operations::PtSub, svec![src_a, src_b])?;
        Ok(ret[0])
    }
    pub fn subx(&self, src_a: ValId, src_b: ValId) -> Result<ValId, IRError<Ioplang>> {
        // Extract type in sub-scope to correctly handle ir mutex
        let (typ_a, typ_b) = {
            let ir = self.ir.lock().unwrap();
            let typ_a = ir.get_val(src_a).get_type();
            let typ_b = ir.get_val(src_b).get_type();
            (typ_a, typ_b)
        };

        match (typ_a, typ_b) {
            (Types::CiphertextBlock, Types::PlaintextBlock) => self.subs(src_a, src_b),
            (Types::PlaintextBlock, Types::CiphertextBlock) => self.ssub(src_a, src_b),
            _ => self.sub(src_a, src_b),
        }
    }

    pub fn mac(&self, cst_a: ValId, src_a: ValId, src_b: ValId) -> Result<ValId, IRError<Ioplang>> {
        let mut ir = self.ir.lock().unwrap();
        let (_node, ret) = ir.add_op(Operations::Mac, svec![cst_a, src_a, src_b])?;
        Ok(ret[0])
    }

    pub fn pbs_ml(&self, src: ValId, lut: ValId) -> Result<SmallVec<ValId>, IRError<Ioplang>> {
        let mut ir = self.ir.lock().unwrap();
        let (_node, ret) = match ir.get_val(lut).get_type() {
            Types::Lut2 => ir.add_op(Operations::Pbs2, svec![src, lut]),
            Types::Lut4 => ir.add_op(Operations::Pbs4, svec![src, lut]),
            Types::Lut8 => ir.add_op(Operations::Pbs8, svec![src, lut]),
            _ => ir.add_op(Operations::Pbs, svec![src, lut]),
        }?;
        Ok(ret)
    }

    pub fn pbs(&self, src: ValId, lut: ValId) -> Result<ValId, IRError<Ioplang>> {
        let mut ir = self.ir.lock().unwrap();
        let (_node, ret) = ir.add_op(Operations::Pbs, svec![src, lut])?;
        Ok(ret[0])
    }
}
