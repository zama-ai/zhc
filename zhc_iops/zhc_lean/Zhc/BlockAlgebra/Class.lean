import Zhc.Model.SpecBlock
import Zhc.Model.SpecInteger
import Zhc.Model.Lookup

namespace Zhc

open Model (SpecBlock SpecInteger)

-- An interface describing a DSL for blocks. The IopLang language in Lean
class BlockAlgebra (sb : SpecBlock) (M : Type → Type) [Monad M] where

  -- Allowed Types
  CtBlock : Type
  PtBlock : Type
  CtBool : Type
  CtInteger : SpecInteger sb → Type
  PtInteger : SpecInteger sb → Type

  -- Operators Sigatures
  declareInteger : (si : SpecInteger sb) → M (CtInteger si)
  letPtBlock : Nat → M PtBlock
  letCtBlock : Nat → M CtBlock
  addCt : CtBlock → CtBlock → M CtBlock
  subCt : CtBlock → CtBlock → M CtBlock
  shlCt : CtBlock → (amount : Nat) → M CtBlock
  packCt : CtBlock → (mul : Nat) → CtBlock → M CtBlock
  addPt : CtBlock → PtBlock → M CtBlock
  subPt : CtBlock → PtBlock → M CtBlock
  ptSub : PtBlock → CtBlock → M CtBlock
  mulPt : CtBlock → PtBlock → M CtBlock
  extractCtBlock {si : SpecInteger sb} : CtInteger si → Fin si.blockCount → M CtBlock
  extractPtBlock {si : SpecInteger sb} : PtInteger si → Fin si.blockCount → M PtBlock
  storeCtBlock {si : SpecInteger sb} : CtInteger si → Fin si.blockCount → CtBlock → M (CtInteger si)
  boolFromBlock : CtBlock → M CtBool
  extractBoolBlock : CtBool → M CtBlock
  pbs : Model.Lut1 sb → CtBlock → M CtBlock
  pbs2 : Model.Lut2 sb → CtBlock → M (CtBlock × CtBlock)
  pbs4 : Model.Lut4 sb → CtBlock → M (CtBlock × CtBlock × CtBlock × CtBlock)
  pbs8 : Model.Lut8 sb → CtBlock → M (CtBlock × CtBlock × CtBlock × CtBlock × CtBlock × CtBlock × CtBlock × CtBlock)

end Zhc
