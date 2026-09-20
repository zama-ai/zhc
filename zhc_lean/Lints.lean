import Lean

open Lean Elab Command

register_option linter.zhc.noPanic : Bool := {
  defValue := true
  descr := "reject code that can panic at run time"
}

register_option linter.zhc.exports : Bool := {
  defValue := true
  descr := "reject exported functions that do not go through zhc_export"
}

namespace Lints

def panicking : List Name :=
  [``panicCore, ``panic, ``panicWithPos, ``panicWithPosWithDecl]

def isPanicking : Name → Bool
  | n@(.str _ s) => panicking.contains n || s.endsWith "!"
  | n => panicking.contains n

def isTheorem (stx : Syntax) : Bool :=
  stx.getKind == ``Lean.Parser.Command.declaration &&
    stx[1].getKind == ``Lean.Parser.Command.theorem

def isLogical (ctx : ContextInfo) (ti : TermInfo) : IO Bool :=
  try ctx.runMetaM ti.lctx (Meta.isProp ti.expr <||> Meta.isProof ti.expr)
  catch _ => pure false

partial def collectPanics (ctx? : Option ContextInfo) (t : InfoTree) (acc : Array (Syntax × Name)) :
    IO (Array (Syntax × Name)) := do
  match t with
  | .context pci t => collectPanics (pci.mergeIntoOuter? ctx?) t acc
  | .hole _ => pure acc
  | .node info children =>
    let mut acc := acc
    if let (.ofTermInfo ti, some ctx) := (info, ctx?) then
      if ← isLogical ctx ti then return acc
      if !ti.isBinder then
        if let some n := ti.expr.getAppFn.constName? then
          if isPanicking n then acc := acc.push (ti.stx, n)
    for c in children do
      acc ← collectPanics ctx? c acc
    pure acc

def noPanicLinter : Linter where
  run := withSetOptionIn fun stx => do
    unless Linter.getLinterValue linter.zhc.noPanic (← Linter.getLinterOptions) do return
    if isTheorem stx then return
    let mut hits : Array (Syntax.Range × Syntax × Name) := #[]
    for t in ← getInfoTrees do
      for (ref, n) in ← collectPanics none t #[] do
        if let some r := ref.getRange? then hits := hits.push (r, ref, n)
    let inside (a b : Syntax.Range) := b.start ≤ a.start && a.stop ≤ b.stop
    let mut seen : Array Syntax.Range := #[]
    for (r, ref, n) in hits do
      if hits.any (fun (r', _) => inside r' r && r' != r) then continue
      if seen.contains r then continue
      seen := seen.push r
      let what := if panicking.contains n then m!"`panic!`" else m!"`{.ofConstName n}`"
      logErrorAt ref m!"{what} can panic at run time. Use `Option`, `Except` or `IO` errors \
        instead, or disable this check with `set_option linter.zhc.noPanic false in`."

def newDecls (t : InfoTree) : Array (Syntax × Name) :=
  t.foldInfo (init := #[]) fun _ info acc =>
    match info with
    | .ofTermInfo ti =>
      if ti.isBinder && ti.expr.isConst then acc.push (ti.stx, ti.expr.constName!) else acc
    | _ => acc

def exportsLinter : Linter where
  run := withSetOptionIn fun _ => do
    unless Linter.getLinterValue linter.zhc.exports (← Linter.getLinterOptions) do return
    if (← get).messages.hasErrors then return
    let env ← getEnv
    let mut seen : NameSet := {}
    for t in ← getInfoTrees do
      for (ref, n) in newDecls t do
        if seen.contains n then continue
        seen := seen.insert n
        unless (getExportNameFor? env n).isSome do continue
        if isNoncomputable env n then
          logErrorAt ref m!"{n} is noncomputable"
        let base := match n with
          | .str p "export" => p
          | _ => n
        for suffix in [`spec, `correct, `clean, `projects] do
          let companion := base ++ suffix
          if !env.contains companion then
            logErrorAt ref m!"{n} is exported without {companion}: exports must go through zhc_export"
          else if (← liftCoreM (collectAxioms companion)).contains ``sorryAx then
            logErrorAt ref m!"{companion} uses sorry"

initialize
  addLinter noPanicLinter
  addLinter exportsLinter

end Lints
