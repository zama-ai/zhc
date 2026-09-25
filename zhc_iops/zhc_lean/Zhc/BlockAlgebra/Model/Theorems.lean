import Zhc.Util
import Zhc.BlockAlgebra.Class
import Zhc.BlockAlgebra.Model.Instance

/-!
# Equations of the model instance

One equation per field of `BlockAlgebra`, on the `Id` instance: the model function the operation
unfolds to. All hold by definition. They are `@[simp]`, so `simp` turns a program written
against `BlockAlgebra` into plain model arithmetic.
-/

namespace Zhc.BlockAlgebra.Model

open Zhc.Model

variable {sb : SpecBlock} {si : SpecInteger sb}

/-! ## Constants -/

@[simp] theorem declareInteger_eq (si : SpecInteger sb) :
  BlockAlgebra.declareInteger (sb := sb) (M := Id) si = CiphertextInteger.ofNat 0
:=
  rfl

@[simp] theorem letCtBlock_eq (v : Nat) :
  BlockAlgebra.letCtBlock (sb := sb) (M := Id) v = CiphertextBlock.ofNat v
:=
  rfl

@[simp] theorem letPtBlock_eq (v : Nat) :
  BlockAlgebra.letPtBlock (sb := sb) (M := Id) v = PlaintextBlock.ofNat v
:=
  rfl

/-! ## Linear operators -/

@[simp] theorem addCt_eq (a b : CiphertextBlock sb) :
  BlockAlgebra.addCt (sb := sb) (M := Id) a b = Operators.add a b
:=
  rfl

@[simp] theorem subCt_eq (a b : CiphertextBlock sb) :
  BlockAlgebra.subCt (sb := sb) (M := Id) a b = Operators.sub a b
:=
  rfl

@[simp] theorem shlCt_eq (a : CiphertextBlock sb) (amount : Nat) :
  BlockAlgebra.shlCt (sb := sb) (M := Id) a amount = Operators.shl a amount
:=
  rfl

@[simp] theorem packCt_eq (a : CiphertextBlock sb) (mul : Nat) (b : CiphertextBlock sb) :
  BlockAlgebra.packCt (sb := sb) (M := Id) a mul b = Operators.mac a mul b
:=
  rfl

@[simp] theorem addPt_eq (a : CiphertextBlock sb) (b : PlaintextBlock sb) :
  BlockAlgebra.addPt (sb := sb) (M := Id) a b = Operators.addPt a b
:=
  rfl

@[simp] theorem subPt_eq (a : CiphertextBlock sb) (b : PlaintextBlock sb) :
  BlockAlgebra.subPt (sb := sb) (M := Id) a b = Operators.subPt a b
:=
  rfl

@[simp] theorem ptSub_eq (a : PlaintextBlock sb) (b : CiphertextBlock sb) :
  BlockAlgebra.ptSub (sb := sb) (M := Id) a b = Operators.subCt a b
:=
  rfl

@[simp] theorem mulPt_eq (a : CiphertextBlock sb) (b : PlaintextBlock sb) :
  BlockAlgebra.mulPt (sb := sb) (M := Id) a b = Operators.mulPt a b
:=
  rfl

/-! ## Integers and booleans -/

@[simp] theorem extractCtBlock_eq (x : CiphertextInteger si) (i : Fin si.blockCount) :
  BlockAlgebra.extractCtBlock (sb := sb) (M := Id) x i = x.getBlock i
:=
  rfl

@[simp] theorem extractPtBlock_eq (x : PlaintextInteger si) (i : Fin si.blockCount) :
  BlockAlgebra.extractPtBlock (sb := sb) (M := Id) x i = x.getBlock i
:=
  rfl

@[simp] theorem storeCtBlock_eq (x : CiphertextInteger si) (i : Fin si.blockCount)
    (b : CiphertextBlock sb) :
  BlockAlgebra.storeCtBlock (sb := sb) (M := Id) x i b = x.setBlock i b
:=
  rfl

@[simp] theorem boolFromBlock_eq (b : CiphertextBlock sb) :
  BlockAlgebra.boolFromBlock (sb := sb) (M := Id) b = (⟨b⟩ : CiphertextBool sb)
:=
  rfl

@[simp] theorem extractBoolBlock_eq (b : CiphertextBool sb) :
  BlockAlgebra.extractBoolBlock (sb := sb) (M := Id) b = b.block
:=
  rfl

/-! ## Lookups

Output `j` is `ManyLut.output`, the raw semantics of the bootstrapping. On a clean input, with
the reserved bits clear, `ManyLut.output_of_clean` gives the table cell at the input. -/

@[simp] theorem pbs_eq (lut : Lut1 sb) (s : CiphertextBlock sb) :
  BlockAlgebra.pbs (sb := sb) (M := Id) lut s = lut.output s 0
:=
  rfl

@[simp] theorem pbs2_eq (lut : Lut2 sb) (s : CiphertextBlock sb) :
  BlockAlgebra.pbs2 (sb := sb) (M := Id) lut s = (lut.output s 0, lut.output s 1)
:=
  rfl

@[simp] theorem pbs4_eq (lut : Lut4 sb) (s : CiphertextBlock sb) :
  BlockAlgebra.pbs4 (sb := sb) (M := Id) lut s
    = (lut.output s 0, lut.output s 1, lut.output s 2, lut.output s 3)
:=
  rfl

@[simp] theorem pbs8_eq (lut : Lut8 sb) (s : CiphertextBlock sb) :
  BlockAlgebra.pbs8 (sb := sb) (M := Id) lut s
    = (lut.output s 0, lut.output s 1, lut.output s 2, lut.output s 3,
       lut.output s 4, lut.output s 5, lut.output s 6, lut.output s 7)
:=
  rfl

end Zhc.BlockAlgebra.Model
