import Zhc.BlockAlgebra.Class
import Zhc.Model.CiphertextBlock
import Zhc.Model.CiphertextInteger
import Zhc.Model.CiphertextBool
import Zhc.Model.PlaintextBlock
import Zhc.Model.PlaintextInteger
import Zhc.Model.Lookup
import Zhc.Model.Operators
import Zhc.Ffi.L2r.Builder

namespace Zhc.BlockAlgebra.Tracer

open Zhc.Model

structure Traced (H : Type) (β : Type) where
  handle : H
  model : β

abbrev TraceM := ReaderT Ffi.L2r.Builder IO
abbrev TrCtBlock (sb : SpecBlock) := Traced Ffi.L2r.CiphertextBlock (CiphertextBlock sb)
abbrev TrPtBlock (sb : SpecBlock) := Traced Ffi.L2r.PlaintextBlock (PlaintextBlock sb)
abbrev TrCtBool (sb : SpecBlock) := Traced Ffi.L2r.CiphertextBool (CiphertextBool sb)
abbrev TrCtInteger {sb : SpecBlock} (si : SpecInteger sb) := Traced Ffi.L2r.CiphertextInteger (CiphertextInteger si)
abbrev TrPtInteger {sb : SpecBlock} (si : SpecInteger sb) := Traced Ffi.L2r.PlaintextInteger (PlaintextInteger si)

variable {sb : SpecBlock} {si : SpecInteger sb}

def narrow8 (v : Nat) : IO UInt8 :=
  if v < 2 ^ 8 then pure v.toUInt8
  else throw (IO.userError s!"{v} does not fit in 8 bits")

def narrow16 (v : Nat) : IO UInt16 :=
  if v < 2 ^ 16 then pure v.toUInt16
  else throw (IO.userError s!"{v} does not fit in 16 bits")

def declareInteger (si : SpecInteger sb) : TraceM (TrCtInteger si) := do
  let h ← (← read).ciphertextIntegerDeclare (← narrow16 si.intSize)
  pure ⟨h, CiphertextInteger.ofNat 0⟩

def letPtBlock (v : Nat) : TraceM (TrPtBlock sb) := do
  let h ← (← read).blockLetPlaintext (← narrow8 v)
  pure ⟨h, PlaintextBlock.ofNat v⟩

def letCtBlock (v : Nat) : TraceM (TrCtBlock sb) := do
  let h ← (← read).blockLetCiphertext (← narrow8 v)
  pure ⟨h, CiphertextBlock.ofNat v⟩

def addCt (a b : TrCtBlock sb) : TraceM (TrCtBlock sb) := do
  let h ← (← read).blockAdd a.handle b.handle
  pure ⟨h, Operators.add a.model b.model⟩

def subCt (a b : TrCtBlock sb) : TraceM (TrCtBlock sb) := do
  let h ← (← read).blockSub a.handle b.handle
  pure ⟨h, Operators.sub a.model b.model⟩

def shlCt (a : TrCtBlock sb) (amount : Nat) : TraceM (TrCtBlock sb) := do
  let h ← (← read).blockShl a.handle (← narrow8 amount)
  pure ⟨h, Operators.shl a.model amount⟩

def packCt (a : TrCtBlock sb) (mul : Nat) (b : TrCtBlock sb) : TraceM (TrCtBlock sb) := do
  let h ← (← read).blockMac a.handle b.handle (← narrow8 mul)
  pure ⟨h, Operators.mac a.model mul b.model⟩

def addPt (a : TrCtBlock sb) (b : TrPtBlock sb) : TraceM (TrCtBlock sb) := do
  let h ← (← read).blockAddPlaintext a.handle b.handle
  pure ⟨h, Operators.addPt a.model b.model⟩

def subPt (a : TrCtBlock sb) (b : TrPtBlock sb) : TraceM (TrCtBlock sb) := do
  let h ← (← read).blockSubPlaintext a.handle b.handle
  pure ⟨h, Operators.subPt a.model b.model⟩

def ptSub (a : TrPtBlock sb) (b : TrCtBlock sb) : TraceM (TrCtBlock sb) := do
  let h ← (← read).blockPlaintextSub a.handle b.handle
  pure ⟨h, Operators.subCt a.model b.model⟩

def mulPt (a : TrCtBlock sb) (b : TrPtBlock sb) : TraceM (TrCtBlock sb) := do
  let h ← (← read).blockMulPlaintext a.handle b.handle
  pure ⟨h, Operators.mulPt a.model b.model⟩

def extractCtBlock (x : TrCtInteger si) (i : Fin si.blockCount) : TraceM (TrCtBlock sb) := do
  let h ← (← read).ciphertextIntegerGetBlock x.handle (← narrow8 i.val)
  pure ⟨h, x.model.getBlock i⟩

def extractPtBlock (x : TrPtInteger si) (i : Fin si.blockCount) : TraceM (TrPtBlock sb) := do
  let h ← (← read).plaintextIntegerGetBlock x.handle (← narrow8 i.val)
  pure ⟨h, x.model.getBlock i⟩

def storeCtBlock (x : TrCtInteger si) (i : Fin si.blockCount) (b : TrCtBlock sb) :
    TraceM (TrCtInteger si) := do
  let h ← (← read).ciphertextIntegerStoreBlock x.handle (← narrow8 i.val) b.handle
  pure ⟨h, x.model.setBlock i b.model⟩

def boolFromBlock (b : TrCtBlock sb) : TraceM (TrCtBool sb) := do
  let h ← (← read).ciphertextBoolStoreBlock b.handle
  pure ⟨h, ⟨b.model⟩⟩

def extractBoolBlock (b : TrCtBool sb) : TraceM (TrCtBlock sb) := do
  let h ← (← read).ciphertextBoolGetBlock b.handle
  pure ⟨h, b.model.block⟩

private def unpack2 (r : Array Ffi.L2r.CiphertextBlock) :
    IO (Ffi.L2r.CiphertextBlock × Ffi.L2r.CiphertextBlock) :=
  match r with
  | #[a, b] => pure (a, b)
  | _ => throw (IO.userError "lookup2 returned an unexpected number of blocks")

private def unpack4 (r : Array Ffi.L2r.CiphertextBlock) :
    IO (Ffi.L2r.CiphertextBlock × Ffi.L2r.CiphertextBlock × Ffi.L2r.CiphertextBlock × Ffi.L2r.CiphertextBlock) :=
  match r with
  | #[a, b, c, d] => pure (a, b, c, d)
  | _ => throw (IO.userError "lookup4 returned an unexpected number of blocks")

private def unpack8 (r : Array Ffi.L2r.CiphertextBlock) :
    IO (Ffi.L2r.CiphertextBlock × Ffi.L2r.CiphertextBlock × Ffi.L2r.CiphertextBlock × Ffi.L2r.CiphertextBlock ×
        Ffi.L2r.CiphertextBlock × Ffi.L2r.CiphertextBlock × Ffi.L2r.CiphertextBlock × Ffi.L2r.CiphertextBlock) :=
  match r with
  | #[a, b, c, d, e, f, g, h] => pure (a, b, c, d, e, f, g, h)
  | _ => throw (IO.userError "lookup8 returned an unexpected number of blocks")

def pbs (lut : Lut1 sb) (a : TrCtBlock sb) : TraceM (TrCtBlock sb) := do
  let h ← (← read).blockLookup a.handle (← Ffi.L2r.Lut1.ofModel lut)
  pure ⟨h, lut.output a.model 0⟩

def pbs2 (lut : Lut2 sb) (a : TrCtBlock sb) : TraceM (TrCtBlock sb × TrCtBlock sb) := do
  let h ← unpack2 (← (← read).blockLookup2 a.handle (← Ffi.L2r.Lut2.ofModel lut))
  pure (⟨h.1, lut.output a.model 0⟩, ⟨h.2, lut.output a.model 1⟩)

def pbs4 (lut : Lut4 sb) (a : TrCtBlock sb) :
    TraceM (TrCtBlock sb × TrCtBlock sb × TrCtBlock sb × TrCtBlock sb) := do
  let h ← unpack4 (← (← read).blockLookup4 a.handle (← Ffi.L2r.Lut4.ofModel lut))
  pure (⟨h.1, lut.output a.model 0⟩, ⟨h.2.1, lut.output a.model 1⟩,
    ⟨h.2.2.1, lut.output a.model 2⟩, ⟨h.2.2.2, lut.output a.model 3⟩)

def pbs8 (lut : Lut8 sb) (a : TrCtBlock sb) :
    TraceM (TrCtBlock sb × TrCtBlock sb × TrCtBlock sb × TrCtBlock sb ×
      TrCtBlock sb × TrCtBlock sb × TrCtBlock sb × TrCtBlock sb) := do
  let h ← unpack8 (← (← read).blockLookup8 a.handle (← Ffi.L2r.Lut8.ofModel lut))
  pure (⟨h.1, lut.output a.model 0⟩, ⟨h.2.1, lut.output a.model 1⟩,
    ⟨h.2.2.1, lut.output a.model 2⟩, ⟨h.2.2.2.1, lut.output a.model 3⟩,
    ⟨h.2.2.2.2.1, lut.output a.model 4⟩, ⟨h.2.2.2.2.2.1, lut.output a.model 5⟩,
    ⟨h.2.2.2.2.2.2.1, lut.output a.model 6⟩, ⟨h.2.2.2.2.2.2.2, lut.output a.model 7⟩)

instance instBlockAlgebraTracer (sb : SpecBlock) : BlockAlgebra sb TraceM where
  CtBlock := TrCtBlock sb
  PtBlock := TrPtBlock sb
  CtBool := TrCtBool sb
  CtInteger := TrCtInteger
  PtInteger := TrPtInteger

  declareInteger := declareInteger
  letPtBlock := letPtBlock
  letCtBlock := letCtBlock
  addCt := addCt
  subCt := subCt
  shlCt := shlCt
  packCt := packCt
  addPt := addPt
  subPt := subPt
  ptSub := ptSub
  mulPt := mulPt
  extractCtBlock := extractCtBlock
  extractPtBlock := extractPtBlock
  storeCtBlock := storeCtBlock
  boolFromBlock := boolFromBlock
  extractBoolBlock := extractBoolBlock
  pbs := pbs
  pbs2 := pbs2
  pbs4 := pbs4
  pbs8 := pbs8

def inputCtInteger (si : SpecInteger sb) : TraceM (TrCtInteger si) := do
  let h ← (← read).ciphertextIntegerInput (← narrow16 si.intSize)
  pure ⟨h, CiphertextInteger.ofNat 0⟩

def outputCtInteger (x : TrCtInteger si) : TraceM Unit := do
  (← read).ciphertextIntegerOutput x.handle

def emit (sb : SpecBlock) (prog : TraceM Unit) : IO Ffi.L2r.Builder := do
  let builder ← Ffi.L2r.Builder.new (← Ffi.L2r.SpecBlock.ofModel sb)
  prog.run builder
  return builder

end Zhc.BlockAlgebra.Tracer
