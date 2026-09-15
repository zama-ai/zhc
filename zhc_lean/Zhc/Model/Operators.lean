import Zhc.Model.SpecBlock
import Zhc.Model.CiphertextBlock
import Zhc.Model.PlaintextBlock

namespace Zhc.Model.Operators variable {sb : SpecBlock}

open CiphertextBlock (ofNat)

def add (a b : CiphertextBlock sb) : CiphertextBlock sb :=
  ofNat (a.toCompleteNat + b.toCompleteNat)

def sub (a b : CiphertextBlock sb) : CiphertextBlock sb :=
  ofNat (a.toCompleteNat + 2 ^ sb.completeSize - b.toCompleteNat)

def shl (a : CiphertextBlock sb) (amount : Nat) : CiphertextBlock sb :=
  ofNat (a.toCompleteNat * 2 ^ amount)

def mac (a : CiphertextBlock sb) (mul : Nat) (b : CiphertextBlock sb) : CiphertextBlock sb :=
  ofNat (a.toCompleteNat * mul + b.toCompleteNat)

def addPt (a : CiphertextBlock sb) (b : PlaintextBlock sb) : CiphertextBlock sb :=
  ofNat (a.toCompleteNat + b.toCompleteNat)

def subPt (a : CiphertextBlock sb) (b : PlaintextBlock sb) : CiphertextBlock sb :=
  ofNat (a.toCompleteNat + 2 ^ sb.completeSize - b.toCompleteNat)

def subCt (a : PlaintextBlock sb) (b : CiphertextBlock sb) : CiphertextBlock sb :=
  ofNat (a.toCompleteNat + 2 ^ sb.completeSize - b.toCompleteNat)

def mulPt (a : CiphertextBlock sb) (b : PlaintextBlock sb) : CiphertextBlock sb :=
  ofNat (a.toCompleteNat * b.toCompleteNat)

end Zhc.Model.Operators
