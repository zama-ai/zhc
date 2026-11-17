import Zhc.Ffi.L2r.Builder

namespace Zhc.Tests.Interpreter

open Ffi.L2r

/-- info: #[6, 2, 0, 4, 1] -/
#guard_msgs in
#eval show IO Unit from do
  let b ← Builder.new (← SpecBlock.new 2 2)
  let x ← b.ciphertextIntegerInput 8
  let mut acc ← b.ciphertextIntegerDeclare 8
  for i in [0:4] do
    let blk ← b.ciphertextIntegerGetBlock x i.toUInt8
    acc ← b.ciphertextIntegerStoreBlock acc i.toUInt8 (← b.blockAdd blk blk)
  b.ciphertextIntegerOutput acc
  b.ciphertextBoolOutput (← b.ciphertextBoolInput)
  IO.println (← b.interpret #[3, 1, 0, 2, 1] 5)

end Zhc.Tests.Interpreter
