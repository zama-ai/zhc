import Zhc.Util
import Zhc.IopAlgebra.Class
import Zhc.IopAlgebra.Model.Instance

/-!
# Equations of the model instance

One equation per field of `IopAlgebra`, on the `Id` instance: the model function the operation
unfolds to. All hold by definition. They are `@[simp]`, so `simp` turns a program written
against `IopAlgebra` into plain model arithmetic.
-/

namespace Zhc.IopAlgebra.Model

open Zhc.Model

variable {sb : SpecBlock} {si : SpecInteger sb}

/-! ## Constants -/

@[simp] theorem declareInteger_eq (si : SpecInteger sb) :
  IopAlgebra.declareInteger (sb := sb) (M := Id) si = CiphertextInteger.ofNat 0
:=
  rfl

@[simp] theorem letCtBlock_eq (v : Nat) :
  IopAlgebra.letCtBlock (sb := sb) (M := Id) v = CiphertextBlock.ofNat v
:=
  rfl

@[simp] theorem letPtBlock_eq (v : Nat) :
  IopAlgebra.letPtBlock (sb := sb) (M := Id) v = PlaintextBlock.ofNat v
:=
  rfl

/-! ## Linear operators -/

@[simp] theorem addCt_eq (a b : CiphertextBlock sb) :
  IopAlgebra.addCt (sb := sb) (M := Id) a b = Operators.add a b
:=
  rfl

@[simp] theorem subCt_eq (a b : CiphertextBlock sb) :
  IopAlgebra.subCt (sb := sb) (M := Id) a b = Operators.sub a b
:=
  rfl

@[simp] theorem shlCt_eq (a : CiphertextBlock sb) (amount : Nat) :
  IopAlgebra.shlCt (sb := sb) (M := Id) a amount = Operators.shl a amount
:=
  rfl

@[simp] theorem packCt_eq (a : CiphertextBlock sb) (mul : Nat) (b : CiphertextBlock sb) :
  IopAlgebra.packCt (sb := sb) (M := Id) a mul b = Operators.mac a mul b
:=
  rfl

@[simp] theorem addPt_eq (a : CiphertextBlock sb) (b : PlaintextBlock sb) :
  IopAlgebra.addPt (sb := sb) (M := Id) a b = Operators.addPt a b
:=
  rfl

@[simp] theorem subPt_eq (a : CiphertextBlock sb) (b : PlaintextBlock sb) :
  IopAlgebra.subPt (sb := sb) (M := Id) a b = Operators.subPt a b
:=
  rfl

@[simp] theorem ptSub_eq (a : PlaintextBlock sb) (b : CiphertextBlock sb) :
  IopAlgebra.ptSub (sb := sb) (M := Id) a b = Operators.subCt a b
:=
  rfl

@[simp] theorem mulPt_eq (a : CiphertextBlock sb) (b : PlaintextBlock sb) :
  IopAlgebra.mulPt (sb := sb) (M := Id) a b = Operators.mulPt a b
:=
  rfl

/-! ## Integers and booleans -/

@[simp] theorem extractCtBlock_eq (x : CiphertextInteger si) (i : Fin si.blockCount) :
  IopAlgebra.extractCtBlock (sb := sb) (M := Id) x i = x.getBlock i
:=
  rfl

@[simp] theorem extractPtBlock_eq (x : PlaintextInteger si) (i : Fin si.blockCount) :
  IopAlgebra.extractPtBlock (sb := sb) (M := Id) x i = x.getBlock i
:=
  rfl

@[simp] theorem storeCtBlock_eq (x : CiphertextInteger si) (i : Fin si.blockCount)
    (b : CiphertextBlock sb) :
  IopAlgebra.storeCtBlock (sb := sb) (M := Id) x i b = x.setBlock i b
:=
  rfl

@[simp] theorem boolFromBlock_eq (b : CiphertextBlock sb) :
  IopAlgebra.boolFromBlock (sb := sb) (M := Id) b = (⟨b⟩ : CiphertextBool sb)
:=
  rfl

@[simp] theorem extractBoolBlock_eq (b : CiphertextBool sb) :
  IopAlgebra.extractBoolBlock (sb := sb) (M := Id) b = b.block
:=
  rfl

/-! ## Lookups

Output `j` is `ManyLut.output`, the raw semantics of the bootstrapping. On a clean input, with
the reserved bits clear, `ManyLut.output_of_clean` gives the table cell at the input. -/

@[simp] theorem pbs_eq (lut : Lut1 sb) (s : CiphertextBlock sb) :
  IopAlgebra.pbs (sb := sb) (M := Id) lut s = lut.output s 0
:=
  rfl

@[simp] theorem pbs2_eq (lut : Lut2 sb) (s : CiphertextBlock sb) :
  IopAlgebra.pbs2 (sb := sb) (M := Id) lut s = (lut.output s 0, lut.output s 1)
:=
  rfl

@[simp] theorem pbs4_eq (lut : Lut4 sb) (s : CiphertextBlock sb) :
  IopAlgebra.pbs4 (sb := sb) (M := Id) lut s
    = (lut.output s 0, lut.output s 1, lut.output s 2, lut.output s 3)
:=
  rfl

@[simp] theorem pbs8_eq (lut : Lut8 sb) (s : CiphertextBlock sb) :
  IopAlgebra.pbs8 (sb := sb) (M := Id) lut s
    = (lut.output s 0, lut.output s 1, lut.output s 2, lut.output s 3,
       lut.output s 4, lut.output s 5, lut.output s 6, lut.output s 7)
:=
  rfl

end Zhc.IopAlgebra.Model
