use hc_ir::ValId;

/// A handle to a 1-LUT in the IR.
#[derive(Clone, Copy)]
pub struct Lut1(pub ValId);

/// A handle to 2-LUT in the IR.
#[derive(Clone, Copy)]
pub struct Lut2(pub ValId);
