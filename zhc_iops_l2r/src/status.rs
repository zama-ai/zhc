#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum zhc_status {
    ZHC_OK = 0,
    ZHC_ERR_NULL_POINTER = 1,
    ZHC_ERR_INVALID_ARGUMENT = 2,
}
