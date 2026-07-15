use zhc_utils::{SafeAs, StoreIndex};

#[derive(Debug, Copy, Clone)]
pub struct RegId(pub u16);

impl StoreIndex for RegId {
    type Raw = u16;
    fn as_raw(&self) -> Self::Raw {
        self.0
    }
    fn as_usize(&self) -> usize {
        self.0 as usize
    }
    fn raw_from_usize(val: usize) -> Self::Raw {
        val.sas()
    }
    fn from_usize(val: usize) -> Self {
        RegId(Self::raw_from_usize(val))
    }
}
