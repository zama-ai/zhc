import Zhc.IopAlgebra.Class
import Zhc.IopAlgebra.Model.Instance
import Zhc.IopAlgebra.Traced.Instance
import Zhc.IopAlgebra.Traced.Theorems

namespace Zhc

open Model (SpecBlock SpecInteger CiphertextInteger CiphertextBool PlaintextInteger)
open IopAlgebra.Traced (TraceM Projects)

inductive Ty (sb : SpecBlock) where
  | ctInt (si : SpecInteger sb)
  | ptInt (si : SpecInteger sb)
  | ctBool

variable {sb : SpecBlock}

def Ty.denote {M : Type → Type} [Monad M] (A : IopAlgebra sb M) : Ty sb → Type
  | .ctInt si => A.CtInteger si
  | .ptInt si => A.PtInteger si
  | .ctBool => A.CtBool

def Args {M : Type → Type} [Monad M] (A : IopAlgebra sb M) : List (Ty sb) → Type
  | [] => Unit
  | [t] => t.denote A
  | t :: ts => t.denote A × Args A ts

abbrev IdA (sb : SpecBlock) := IopAlgebra.Model.instIopAlgebraId sb

abbrev TrA (sb : SpecBlock) := IopAlgebra.Traced.instIopAlgebraTraced sb

def Ty.Clean : (t : Ty sb) → t.denote (IdA sb) → Prop
  | .ctInt _, x => CiphertextInteger.IsClean x
  | .ptInt _, x => PlaintextInteger.IsClean x
  | .ctBool, b => CiphertextBool.IsClean b

def Args.Clean : (ts : List (Ty sb)) → Args (IdA sb) ts → Prop
  | [], _ => True
  | [t], a => t.Clean a
  | t :: t' :: ts, (a, as) => t.Clean a ∧ Args.Clean (t' :: ts) as

def Ty.model : (t : Ty sb) → t.denote (TrA sb) → t.denote (IdA sb)
  | .ctInt _, x => x.model
  | .ptInt _, x => x.model
  | .ctBool, b => b.model

def Args.model : (ts : List (Ty sb)) → Args (TrA sb) ts → Args (IdA sb) ts
  | [], _ => ()
  | [t], a => t.model a
  | t :: t' :: ts, (a, as) => (t.model a, Args.model (t' :: ts) as)

def Ty.input : (t : Ty sb) → TraceM (t.denote (TrA sb))
  | .ctInt si => IopAlgebra.Traced.inputCtInteger si
  | .ptInt si => do
    let h ← (← read).integerPlaintextInput si.intSize.toUInt16
    pure ⟨h, PlaintextInteger.ofNat 0⟩
  | .ctBool => do
    let h ← (← read).boolCiphertextInput
    pure ⟨h, CiphertextBool.ofBool false⟩

def Ty.output : (t : Ty sb) → t.denote (TrA sb) → TraceM Unit
  | .ctInt _, x => IopAlgebra.Traced.outputCtInteger x
  | .ptInt _, _ => throw (IO.userError "plaintext integers cannot be circuit outputs")
  | .ctBool, b => do (← read).boolCiphertextOutput b.handle

def Args.input : (ts : List (Ty sb)) → TraceM (Args (TrA sb) ts)
  | [] => pure ()
  | [t] => t.input
  | t :: t' :: ts => do
    let a ← t.input
    let as ← Args.input (t' :: ts)
    pure (a, as)

def Args.output : (ts : List (Ty sb)) → Args (TrA sb) ts → TraceM Unit
  | [], _ => pure ()
  | [t], a => t.output a
  | t :: t' :: ts, (a, as) => do
    t.output a
    Args.output (t' :: ts) as

structure Algo (sb : SpecBlock) where
  Param : Type
  ins : Param → List (Ty sb)
  outs : Param → List (Ty sb)
  prog : ∀ {M : Type → Type} [Monad M] [A : IopAlgebra sb M] (p : Param), Args A (ins p) → M (Args A (outs p))
  spec : (p : Param) → Args (IdA sb) (ins p) → Args (IdA sb) (outs p) → Prop
  correct : ∀ p xs, Args.Clean _ xs → spec p xs (prog (M := Id) p xs)
  clean : ∀ p xs, Args.Clean _ xs → Args.Clean _ (prog (M := Id) p xs)
  traced_projects : ∀ p xs, Projects (prog (M := TraceM) p xs) (Args.model _) (prog (M := Id) p (Args.model _ xs))

def Algo.emit (a : Algo sb) (p : a.Param) : IO Ffi.Builder :=
  IopAlgebra.Traced.emit sb do
    let xs ← Args.input (a.ins p)
    let ys ← a.prog (M := TraceM) p xs
    Args.output (a.outs p) ys

end Zhc
