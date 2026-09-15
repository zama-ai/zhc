import Zhc.IopAlgebra.Class
import Zhc.Model.SpecBlock
import Zhc.Model.SpecInteger
import Zhc.Model.CiphertextBlock
import Zhc.Model.CiphertextInteger
import Zhc.Model.CiphertextBool
import Zhc.Model.PlaintextBlock
import Zhc.Model.PlaintextInteger
import Zhc.Model.Lookup
import Zhc.Model.Operators

namespace Zhc.IopAlgebra.Model

open Zhc.Model

instance instIopAlgebraId (sb : SpecBlock) : IopAlgebra sb Id where
  CtBlock := CiphertextBlock sb
  PtBlock := PlaintextBlock sb
  CtBool := CiphertextBool sb
  CtInteger si := CiphertextInteger si
  PtInteger si := PlaintextInteger si

  declareInteger _ := CiphertextInteger.ofNat 0
  letPtBlock := PlaintextBlock.ofNat
  letCtBlock := CiphertextBlock.ofNat

  addCt a b := Operators.add a b
  subCt a b := Operators.sub a b
  shlCt a amount := Operators.shl a amount
  packCt a mul b := Operators.mac a mul b
  addPt a b := Operators.addPt a b
  subPt a b := Operators.subPt a b
  ptSub a b := Operators.subCt a b
  mulPt a b := Operators.mulPt a b

  extractCtBlock x i := x.getBlock i
  extractPtBlock x i := x.getBlock i
  storeCtBlock x i b := x.setBlock i b

  boolFromBlock b := ⟨b⟩
  extractBoolBlock b := b.block

  pbs lut a := lut.output a 0
  pbs2 lut a := (lut.output a 0, lut.output a 1)
  pbs4 lut a := (lut.output a 0, lut.output a 1, lut.output a 2, lut.output a 3)
  pbs8 lut a := (lut.output a 0, lut.output a 1, lut.output a 2, lut.output a 3,
    lut.output a 4, lut.output a 5, lut.output a 6, lut.output a 7)

private def smoke : CiphertextBlock ⟨2, 2⟩ := Id.run do
  let a ← IopAlgebra.letCtBlock (sb := ⟨2, 2⟩) (M := Id) 3
  let b ← IopAlgebra.letCtBlock (sb := ⟨2, 2⟩) (M := Id) 2
  IopAlgebra.addCt a b

example : smoke = CiphertextBlock.ofNat 5 := by decide

end Zhc.IopAlgebra.Model
