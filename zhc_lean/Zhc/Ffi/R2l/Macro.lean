import Zhc.Model.Clean
import Zhc.BlockAlgebra.Class
import Zhc.BlockAlgebra.Model.Instance
import Zhc.BlockAlgebra.Tracer.Instance
import Zhc.BlockAlgebra.Tracer.Theorems

namespace Zhc.Ffi.R2l

open Model (SpecBlock SpecInteger CiphertextInteger PlaintextInteger CiphertextBool)
open BlockAlgebra.Tracer (TrCtInteger TrPtInteger TrCtBool narrow16)

variable {sb : SpecBlock}

def specOf (sb : SpecBlock) (spec : L2r.SpecInteger) : IO (SpecInteger sb) := do
  unless (← (← spec.block).same (← L2r.SpecBlock.ofModel sb)) do
    throw (IO.userError "handle block spec does not match the algorithm block spec")
  match SpecInteger.ofIntSize? (sb := sb) (← spec.intSize).toNat with
  | some si => pure si
  | none => throw (IO.userError "handle int size is not a multiple of the block message size")

def checkSame (sb : SpecBlock) (si : SpecInteger sb) (spec : L2r.SpecInteger) : IO Unit := do
  if (← specOf sb spec) != si then
    throw (IO.userError "handle int size does not match the signature")

def bindCt (sb : SpecBlock) (x : L2r.CiphertextInteger) : IO (SpecInteger sb) := do
  specOf sb (← x.spec)

def bindPt (sb : SpecBlock) (x : L2r.PlaintextInteger) : IO (SpecInteger sb) := do
  specOf sb (← x.spec)

def checkCt (sb : SpecBlock) (si : SpecInteger sb) (x : L2r.CiphertextInteger) : IO Unit := do
  checkSame sb si (← x.spec)

def checkPt (sb : SpecBlock) (si : SpecInteger sb) (x : L2r.PlaintextInteger) : IO Unit := do
  checkSame sb si (← x.spec)

def checkBool (sb : SpecBlock) (x : L2r.CiphertextBool) : IO Unit := do
  unless (← (← x.spec).same (← L2r.SpecBlock.ofModel sb)) do
    throw (IO.userError "handle block spec does not match the algorithm block spec")

def decodeSpec (sb : SpecBlock) (intSize : UInt16) : IO (SpecInteger sb) :=
  match SpecInteger.ofIntSize? (sb := sb) intSize.toNat with
  | some si => pure si
  | none => throw (IO.userError "int size is not a multiple of the block message size")

def encodeSpec (si : SpecInteger sb) : IO UInt16 :=
  narrow16 si.intSize

def checkBuilder (b : L2r.Builder) (sb : SpecBlock) : IO Unit := do
  unless (← (← b.spec).same (← L2r.SpecBlock.ofModel sb)) do
    throw (IO.userError "builder block spec does not match the algorithm block spec")

def tracedCt (si : SpecInteger sb) (h : L2r.CiphertextInteger) : TrCtInteger si :=
  ⟨h, CiphertextInteger.ofNat 0⟩

def tracedPt (si : SpecInteger sb) (h : L2r.PlaintextInteger) : TrPtInteger si :=
  ⟨h, PlaintextInteger.ofNat 0⟩

def tracedBool (sb : SpecBlock) (h : L2r.CiphertextBool) : TrCtBool sb :=
  ⟨h, CiphertextBool.ofBool false⟩

def inputCt (b : L2r.Builder) (si : SpecInteger sb) : IO L2r.CiphertextInteger := do
  b.ciphertextIntegerInput (← narrow16 si.intSize)

def inputPt (b : L2r.Builder) (si : SpecInteger sb) : IO L2r.PlaintextInteger := do
  b.plaintextIntegerInput (← narrow16 si.intSize)

def inputBool (b : L2r.Builder) : IO L2r.CiphertextBool :=
  b.ciphertextBoolInput


section Macro

open Lean

declare_syntax_cat zhcType (behavior := symbol)
syntax &"SpecInteger" term:max : zhcType
syntax &"CiphertextInteger" ident : zhcType
syntax &"PlaintextInteger" ident : zhcType
syntax &"CiphertextBool" : zhcType

declare_syntax_cat zhcBinder
syntax "(" ident+ " : " zhcType ")" : zhcBinder

declare_syntax_cat zhcOut
syntax "(" ident " : " &"CiphertextInteger" ident ")" : zhcOut
syntax "(" ident " : " &"CiphertextBool" ")" : zhcOut

syntax (name := zhcExport) "zhc_export " ident zhcBinder* " : " zhcOut (" × " zhcOut)* " where"
  withPosition(&"spec" " := " term)
  &"prog" " := " ident
  &"correct" " := " ident
  &"clean" " := " ident
  &"projects" " := " ident : command

syntax "zhc_prod(" term,+ ")" : term
syntax "zhc_tuple(" term,+ ")" : term

macro_rules
  | `(zhc_prod($t)) => `($t)
  | `(zhc_prod($t, $ts,*)) => `($t × zhc_prod($ts,*))

macro_rules
  | `(zhc_tuple($t)) => `($t)
  | `(zhc_tuple($t, $ts,*)) => `(($t, zhc_tuple($ts,*)))

inductive Kind where
  | ctInt (si : Ident)
  | ptInt (si : Ident)
  | ctBool
  deriving Inhabited

def Kind.isCtInt : Kind → Bool
  | .ctInt _ => true
  | _ => false

def Kind.hasSpecInteger (k : Kind) (p : Ident) : Bool :=
  match k with
  | .ctInt si | .ptInt si => si == p
  | .ctBool => false

structure Arg where
  name : Ident
  kind : Kind

structure Sig where
  name : Ident
  specB : Term
  specIs : Array Ident
  args : Array Arg
  rets : Array Kind
  retNames : Array Ident

def parseSig (name : Ident) (binders : Array (TSyntax `zhcBinder)) (outs : Array (TSyntax `zhcOut)) :
    MacroM Sig := do
  let mut specB? : Option Term := none
  let mut specIs := #[]
  let mut args := #[]
  for b in binders do
    match b with
    | `(zhcBinder| ($xs* : SpecInteger $s)) =>
      match specB? with
      | none => specB? := some s
      | some specB =>
        unless specB.raw.structEq s.raw do
          Macro.throwErrorAt s "zhc_export: all SpecInteger binders must use the same block spec"
      specIs := specIs ++ xs
    | `(zhcBinder| ($xs* : CiphertextInteger $si)) =>
      for x in xs do
        args := args.push ⟨x, .ctInt si⟩
    | `(zhcBinder| ($xs* : PlaintextInteger $si)) =>
      for x in xs do
        args := args.push ⟨x, .ptInt si⟩
    | `(zhcBinder| ($xs* : CiphertextBool)) =>
      for x in xs do
        args := args.push ⟨x, .ctBool⟩
    | _ => Macro.throwErrorAt b "zhc_export: this binder form is in the syntax but not in parseSig"
  let checkDeclared (k : Kind) : MacroM Unit :=
    match k with
    | .ctInt si | .ptInt si =>
      unless specIs.any (·.getId == si.getId) do
        Macro.throwErrorAt si s!"zhc_export: {si.getId} is not declared: \
          the signature has no ({si.getId} : SpecInteger _) binder"
    | .ctBool => pure ()
  for a in args do
    checkDeclared a.kind
  let mut rets := #[]
  let mut retNames : Array Ident := #[]
  for o in outs do
    let (r, k) ← match o with
      | `(zhcOut| ($r : CiphertextInteger $si)) => pure (r, Kind.ctInt si)
      | `(zhcOut| ($r : CiphertextBool)) => pure (r, Kind.ctBool)
      | _ => Macro.throwErrorAt o "zhc_export: this output form is in the syntax but not in parseSig"
    checkDeclared k
    let used := specIs.any (· == r) || args.any (·.name == r) || retNames.any (· == r)
    if used then
      Macro.throwErrorAt r "zhc_export: the name of an output must be different from all the other names"
    rets := rets.push k
    retNames := retNames.push r
  let some specB := specB?
    | Macro.throwErrorAt name "zhc_export: the signature needs a (si : SpecInteger sb) binder"
  pure ⟨name, specB, specIs, args, rets, retNames⟩

def genCheck (s : Sig) (prog : Ident) : MacroM Command := do
  let M := mkIdent `M
  let specIs := s.specIs
  let specITys ← s.specIs.mapM fun _ => `(Zhc.Model.SpecInteger $(s.specB))
  let args : Array Ident := s.args.map (·.name)
  let argTys ← s.args.mapM fun a => match a.kind with
    | .ctInt si => `(Zhc.BlockAlgebra.CtInteger (M := $M) $si)
    | .ptInt si => `(Zhc.BlockAlgebra.PtInteger (M := $M) $si)
    | .ctBool => `(Zhc.BlockAlgebra.CtBool (sb := $(s.specB)) (M := $M))
  let rets ← s.rets.mapM fun k => match k with
    | .ctInt si => `(Zhc.BlockAlgebra.CtInteger (M := $M) $si)
    | .ptInt si => `(Zhc.BlockAlgebra.PtInteger (M := $M) $si)
    | .ctBool => `(Zhc.BlockAlgebra.CtBool (sb := $(s.specB)) (M := $M))
  `(example {$M : Type → Type} [Monad $M] [Zhc.BlockAlgebra $(s.specB) $M]
      $[($specIs : $specITys)]* $[($args : $argTys)]* : $M (zhc_prod($rets,*)) :=
      $prog $specIs* $args*)

def genSpec (s : Sig) (spec : Term) : MacroM Command := do
  let specIs := s.specIs
  let specITys ← s.specIs.mapM fun _ => `(Zhc.Model.SpecInteger $(s.specB))
  let args : Array Ident := s.args.map (·.name)
  let argTys ← s.args.mapM fun a => match a.kind with
    | .ctInt si => `(Zhc.Model.CiphertextInteger $si)
    | .ptInt si => `(Zhc.Model.PlaintextInteger $si)
    | .ctBool => `(Zhc.Model.CiphertextBool $(s.specB))
  let rets ← s.rets.mapM fun k => match k with
    | .ctInt si => `(Zhc.Model.CiphertextInteger $si)
    | .ptInt si => `(Zhc.Model.PlaintextInteger $si)
    | .ctBool => `(Zhc.Model.CiphertextBool $(s.specB))
  let retNames : Array Term := s.retNames.map fun r => r
  let specId := mkIdentFrom s.name (s.name.getId ++ `spec)
  `(abbrev $specId $[($specIs : $specITys)]* $[($args : $argTys)]* :
      zhc_prod($rets,*) → Prop := fun zhc_tuple($retNames,*) => $spec)

def genCorrect (s : Sig) (prog correct : Ident) : MacroM Command := do
  let specIs := s.specIs
  let specITys ← s.specIs.mapM fun _ => `(Zhc.Model.SpecInteger $(s.specB))
  let args : Array Ident := s.args.map (·.name)
  let argTys ← s.args.mapM fun a => match a.kind with
    | .ctInt si => `(Zhc.Model.CiphertextInteger $si)
    | .ptInt si => `(Zhc.Model.PlaintextInteger $si)
    | .ctBool => `(Zhc.Model.CiphertextBool $(s.specB))
  let hyps ← s.args.mapM fun a => `(Zhc.Model.Clean.isClean $(a.name))
  let run ← `($prog (M := Id) $specIs* $args*)
  let specId := mkIdentFrom s.name (s.name.getId ++ `spec)
  let concl ← `($specId $specIs* $args* $run)
  let ty ← hyps.foldrM (fun h acc => `($h → $acc)) concl
  let name := mkIdentFrom s.name (s.name.getId ++ `correct)
  `(theorem $name $[($specIs : $specITys)]* $[($args : $argTys)]* : $ty := $correct $specIs* $args*)

def genClean (s : Sig) (prog clean : Ident) : MacroM Command := do
  let specIs := s.specIs
  let specITys ← s.specIs.mapM fun _ => `(Zhc.Model.SpecInteger $(s.specB))
  let args : Array Ident := s.args.map (·.name)
  let argTys ← s.args.mapM fun a => match a.kind with
    | .ctInt si => `(Zhc.Model.CiphertextInteger $si)
    | .ptInt si => `(Zhc.Model.PlaintextInteger $si)
    | .ctBool => `(Zhc.Model.CiphertextBool $(s.specB))
  let hyps ← s.args.mapM fun a => `(Zhc.Model.Clean.isClean $(a.name))
  let concl ← `(Zhc.Model.Clean.isClean ($prog (M := Id) $specIs* $args*))
  let ty ← hyps.foldrM (fun h acc => `($h → $acc)) concl
  let name := mkIdentFrom s.name (s.name.getId ++ `clean)
  `(theorem $name $[($specIs : $specITys)]* $[($args : $argTys)]* : $ty := $clean $specIs* $args*)

def genProjects (s : Sig) (prog projects : Ident) : MacroM Command := do
  let specIs := s.specIs
  let specITys ← s.specIs.mapM fun _ => `(Zhc.Model.SpecInteger $(s.specB))
  let args : Array Ident := s.args.map (·.name)
  let argTys ← s.args.mapM fun a => match a.kind with
    | .ctInt si => `(Zhc.BlockAlgebra.Tracer.TrCtInteger $si)
    | .ptInt si => `(Zhc.BlockAlgebra.Tracer.TrPtInteger $si)
    | .ctBool => `(Zhc.BlockAlgebra.Tracer.TrCtBool $(s.specB))
  let models ← s.args.mapM fun a => `(Zhc.BlockAlgebra.Tracer.model $(a.name))
  let trRun ← `($prog (M := Zhc.BlockAlgebra.Tracer.TraceM) $specIs* $args*)
  let idRun ← `($prog (M := Id) $specIs* $models*)
  let ty ← `(Zhc.BlockAlgebra.Tracer.TraceM.ModelIs $trRun $idRun)
  let name := mkIdentFrom s.name (s.name.getId ++ `projects)
  `(theorem $name $[($specIs : $specITys)]* $[($args : $argTys)]* : $ty := $projects $specIs* $args*)

def genRun (s : Sig) (prog : Ident) : MacroM Command := do
  let b := mkIdent (← Macro.addMacroScope `b)
  -- When a SpecInteger is used in output position but in no input, it must be exposed as an argument.
  let rawSpecIs := s.specIs.filter fun p => !(s.args.any fun a => match a.kind with
    | .ctInt si | .ptInt si => si == p
    | .ctBool => false)
  let rawTys ← rawSpecIs.mapM fun _ => `(UInt16)
  let args : Array Ident := s.args.map (·.name)
  let argTys ← s.args.mapM fun a => match a.kind with
    | .ctInt _ => `(Zhc.Ffi.L2r.CiphertextInteger)
    | .ptInt _ => `(Zhc.Ffi.L2r.PlaintextInteger)
    | .ctBool => `(Zhc.Ffi.L2r.CiphertextBool)
  let retTys ← s.rets.mapM fun k => match k with
    | .ctInt _ => `(Zhc.Ffi.L2r.CiphertextInteger)
    | .ptInt _ => `(Zhc.Ffi.L2r.PlaintextInteger)
    | .ctBool => `(Zhc.Ffi.L2r.CiphertextBool)
  let outs : Array Term ← (Array.range s.rets.size).mapM fun i =>
    return mkIdent (← Macro.addMacroScope (Name.mkSimple s!"o{i}"))
  let handles ← outs.mapM fun o => `(Zhc.BlockAlgebra.Tracer.Traced.handle $o)
  let wrapped ← s.args.mapM fun a => match a.kind with
    | .ctInt si => `(Zhc.Ffi.R2l.tracedCt $si $(a.name))
    | .ptInt si => `(Zhc.Ffi.R2l.tracedPt $si $(a.name))
    | .ctBool => `(Zhc.Ffi.R2l.tracedBool $(s.specB) $(a.name))

  let mut steps : Array (TSyntax `doElem) := #[]
  for p in s.specIs do
    let source := s.args.find? (·.kind.hasSpecInteger p)
    let step ← match source with
      | some ⟨x, .ctInt _⟩ => `(doElem| let $p ← Zhc.Ffi.R2l.bindCt $(s.specB) $x)
      | some ⟨x, .ptInt _⟩ => `(doElem| let $p ← Zhc.Ffi.R2l.bindPt $(s.specB) $x)
      | _ => `(doElem| let $p ← Zhc.Ffi.R2l.decodeSpec $(s.specB) $p)
    steps := steps.push step
  for a in s.args do
    let step ← match a.kind with
      | .ctInt si => `(doElem| Zhc.Ffi.R2l.checkCt $(s.specB) $si $(a.name))
      | .ptInt si => `(doElem| Zhc.Ffi.R2l.checkPt $(s.specB) $si $(a.name))
      | .ctBool => `(doElem| Zhc.Ffi.R2l.checkBool $(s.specB) $(a.name))
    steps := steps.push step
  steps := steps.push (← `(doElem| Zhc.Ffi.R2l.checkBuilder $b $(s.specB)))
  let run ← `(ReaderT.run ($prog (M := Zhc.BlockAlgebra.Tracer.TraceM) $(s.specIs)* $wrapped*) $b)
  steps := steps.push (← `(doElem| let zhc_tuple($outs,*) ← $run:term))
  let runId := mkIdentFrom s.name (s.name.getId ++ `run)
  `(def $runId ($b : Zhc.Ffi.L2r.Builder) $[($rawSpecIs : $rawTys)]* $[($args : $argTys)]* : IO (zhc_prod($retTys,*)) := do
      $[$steps:doElem]*
      pure (zhc_tuple($handles,*)))

def genExport (s : Sig) : MacroM Command := do
  let b := mkIdent (← Macro.addMacroScope `b)
  let rawSpecIs := s.specIs.filter fun p => !s.args.any (·.kind.hasSpecInteger p)
  let rawTys ← rawSpecIs.mapM fun _ => `(UInt16)
  let args : Array Ident := s.args.map (·.name)
  let argTys ← s.args.mapM fun a => match a.kind with
    | .ctInt _ => `(Zhc.Ffi.L2r.CiphertextInteger)
    | .ptInt _ => `(Zhc.Ffi.L2r.PlaintextInteger)
    | .ctBool => `(Zhc.Ffi.L2r.CiphertextBool)
  let outs : Array Term ← (Array.range s.rets.size).mapM fun i =>
    return mkIdent (← Macro.addMacroScope (Name.mkSimple s!"o{i}"))
  let slots : Array Ident ← (Array.range s.rets.size).mapM fun i =>
    return mkIdent (← Macro.addMacroScope (Name.mkSimple s!"out{i}"))
  let slotTys ← slots.mapM fun _ => `(USize)
  let runId := mkIdentFrom s.name (s.name.getId ++ `run)
  let mut steps : Array (TSyntax `doElem) := #[]
  let call ← `($runId $b $rawSpecIs* $args*)
  steps := steps.push (← `(doElem| let zhc_tuple($outs,*) ← $call:term))
  for ((k, o), slot) in (s.rets.zip outs).zip slots do
    let write ← match k with
      | .ctInt _ => `(doElem| Zhc.Ffi.L2r.CiphertextInteger.unbox $o $slot)
      | .ptInt _ => Macro.throwError "zhc_export: plaintext integers cannot be outputs"
      | .ctBool => `(doElem| Zhc.Ffi.L2r.CiphertextBool.unbox $o $slot)
    steps := steps.push write
  let sym := mkIdent (Name.mkSimple ("zhc_lean_export_" ++ s.name.getId.toString))
  let exportId := mkIdentFrom s.name (s.name.getId ++ `export)
  `(@[export $sym:ident]
    def $exportId ($b : Zhc.Ffi.L2r.Builder) $[($rawSpecIs : $rawTys)]* $[($args : $argTys)]* $[($slots : $slotTys)]* : IO Unit := do
      $[$steps:doElem]*)

def genCircuit (s : Sig) : MacroM Command := do
  let b := mkIdent (← Macro.addMacroScope `b)
  let specIs := s.specIs
  let specITys ← s.specIs.mapM fun _ => `(Zhc.Model.SpecInteger $(s.specB))
  let args : Array Ident := s.args.map (·.name)
  let rawSpecIs := s.specIs.filter fun p => !s.args.any (·.kind.hasSpecInteger p)
  let rawIds ← rawSpecIs.mapM fun p => return mkIdent (← Macro.addMacroScope p.getId)
  let outs : Array Term ← (Array.range s.rets.size).mapM fun i =>
    return mkIdent (← Macro.addMacroScope (Name.mkSimple s!"o{i}"))

  let mut steps : Array (TSyntax `doElem) := #[]
  steps := steps.push (← `(doElem| let $b ← Zhc.Ffi.L2r.SpecBlock.ofModel $(s.specB) >>= Zhc.Ffi.L2r.Builder.new))
  for (p, raw) in rawSpecIs.zip rawIds do
    steps := steps.push (← `(doElem| let $raw ← Zhc.Ffi.R2l.encodeSpec $p))
  for a in s.args do
    let input ← match a.kind with
      | .ctInt si => `(Zhc.Ffi.R2l.inputCt $b $si)
      | .ptInt si => `(Zhc.Ffi.R2l.inputPt $b $si)
      | .ctBool => `(Zhc.Ffi.R2l.inputBool $b)
    steps := steps.push (← `(doElem| let $(a.name) ← $input:term))
  let runId := mkIdentFrom s.name (s.name.getId ++ `run)
  let call ← `($runId $b $rawIds* $args*)
  steps := steps.push (← `(doElem| let zhc_tuple($outs,*) ← $call:term))
  for (k, o) in s.rets.zip outs do
    let output ← match k with
      | .ctInt _ => `(Zhc.Ffi.L2r.Builder.ciphertextIntegerOutput $b $o)
      | .ptInt _ => Macro.throwError "zhc_export: plaintext integers cannot be circuit outputs"
      | .ctBool => `(Zhc.Ffi.L2r.Builder.ciphertextBoolOutput $b $o)
    steps := steps.push (← `(doElem| $output:term))
  let name := mkIdentFrom s.name (s.name.getId ++ `circuit)
  `(def $name $[($specIs : $specITys)]* : IO Zhc.Ffi.L2r.Builder := do
      $[$steps:doElem]*
      pure $b)

macro_rules
  | `(zhc_export $name $binders* : $out $[× $outs]* where
      spec := $spec
      prog := $prog
      correct := $correct
      clean := $clean
      projects := $projects) => do
    let s ← parseSig name binders (#[out] ++ outs)
    let cmds := #[
      ← genCheck s prog,
      ← genSpec s spec,
      ← genCorrect s prog correct,
      ← genClean s prog clean,
      ← genProjects s prog projects,
      ← genRun s prog,
      ← genExport s,
      ← genCircuit s
    ]
    return mkNullNode (cmds.map (·.raw))

end Macro

end Zhc.Ffi.R2l
