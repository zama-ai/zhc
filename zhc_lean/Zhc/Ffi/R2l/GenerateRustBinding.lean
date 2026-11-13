import Lean

open Lean

inductive Kind where
  | u16
  | ct
  | pt
  | bool

def rustKeywords : List String :=
  ["as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
   "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
   "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
   "use", "where", "while", "abstract", "become", "box", "do", "final", "gen", "macro", "override",
   "priv", "try", "typeof", "unsized", "virtual", "yield"]

partial def binders (e : Expr) (acc : Array (Name × Expr)) : Array (Name × Expr) × Expr :=
  match e with
  | .forallE n t b _ => binders b (acc.push (n, t))
  | _ => (acc, e)

partial def products (e : Expr) (acc : Array Expr) : Array Expr :=
  match e.app2? ``Prod with
  | some (a, b) => products b (acc.push a)
  | none => acc.push e

def main (args : List String) : IO UInt32 := do
  let [moduleName, outPath] := args
    | throw (IO.userError "usage: lake env lean --run Zhc/Ffi/R2l/GenerateRustBinding.lean <module> <output.rs>")
  initSearchPath (← findSysroot)
  let env ← importModules #[{ module := moduleName.toName }] {}
  let exports := env.constants.fold (init := #[]) fun acc e _ =>
    match e with
    | .str n "export" =>
      if (getExportNameFor? env e).isSome && env.contains (n ++ `spec) then acc.push n else acc
    | _ => acc
  let exports := exports.qsort (·.toString < ·.toString)
  let mut funcs := ""
  let mut entries := ""
  for n in exports do
    let some symbol := getExportNameFor? env (n ++ `export) | throw (IO.userError s!"{n}: no export name")
    let .str _ func := n | throw (IO.userError s!"{n}: not a string name")
    let method := "proven_" ++ func
    let some run := env.find? (n ++ `run) | throw (IO.userError s!"{n}: no {n ++ `run}")
    let (bs, result) := binders run.type #[]
    let some (_, builderTy) := bs[0]? | throw (IO.userError s!"{n}: no builder argument")
    unless builderTy.isConstOf `Zhc.Ffi.L2r.Builder do
      throw (IO.userError s!"{n}: the first argument is not a builder")
    let mut params : Array (String × Kind) := #[]
    for (b, t) in bs.extract 1 bs.size do
      let name := b.eraseMacroScopes.toString
      let valid := !name.isEmpty && !name.front.isDigit
        && name.all (fun c => c.isAlphanum || c == '_') && !rustKeywords.contains name
        && name != "builder"
      unless valid do
        throw (IO.userError s!"{n}: argument name {name} is not a valid Rust name")
      let kind ← match t.constName? with
        | some `UInt16 => pure Kind.u16
        | some `Zhc.Ffi.L2r.CiphertextInteger => pure Kind.ct
        | some `Zhc.Ffi.L2r.PlaintextInteger => pure Kind.pt
        | some `Zhc.Ffi.L2r.CiphertextBool => pure Kind.bool
        | _ => throw (IO.userError s!"{n}: argument {name} has an unsupported type {t}")
      params := params.push (name, kind)
    let some outTy := result.app1? ``IO | throw (IO.userError s!"{n}: the result type is not IO _")
    let mut outputs : Array Kind := #[]
    for t in products outTy #[] do
      let kind ← match t.constName? with
        | some `Zhc.Ffi.L2r.CiphertextInteger => pure Kind.ct
        | some `Zhc.Ffi.L2r.CiphertextBool => pure Kind.bool
        | _ => throw (IO.userError s!"{n}: output has an unsupported type {t}")
      outputs := outputs.push kind
    let rustParams := params.map fun (name, kind) => match kind with
      | .u16 => s!"{name}: u16"
      | .ct => s!"{name}: &::zhc_builder::CiphertextInteger"
      | .pt => s!"{name}: &::zhc_builder::PlaintextInteger"
      | .bool => s!"{name}: &::zhc_builder::CiphertextBool"
    let rustOutputs := outputs.map fun kind => match kind with
      | .ct => "::zhc_builder::CiphertextInteger"
      | _ => "::zhc_builder::CiphertextBool"
    let ret := match rustOutputs with
      | #[o] => o
      | _ => "(" ++ ", ".intercalate rustOutputs.toList ++ ")"
    let funcParams := String.join (rustParams.map (", " ++ ·)).toList
    let macroParams := ", ".intercalate rustParams.toList
    let externParams := params.map fun (name, kind) => match kind with
      | .u16 => s!", {name}: u16"
      | _ => s!", {name}: *mut c_void"
    let slotTypes := outputs.map fun kind => match kind with
      | .ct => "zhc_ciphertext_integer"
      | _ => "zhc_ciphertext_bool"
    let externSlots := (Array.range outputs.size).map fun i => s!", out{i}: *mut *mut c_void"
    let slotDecls := (slotTypes.zipIdx).map fun (ty, i) =>
      s!"    let mut out{i}: *mut {ty} = std::ptr::null_mut();\n"
    let slotArgs := (Array.range outputs.size).map fun i => s!", (&raw mut out{i}).cast()"
    let takes := (Array.range outputs.size).map fun i =>
      s!"    let out{i} = unsafe \{ take(out{i}) };\n"
    let callArgs := params.map fun (name, kind) => match kind with
      | .u16 => s!", {name}"
      | .ct => s!", box_ciphertext_integer({name})"
      | .pt => s!", box_plaintext_integer({name})"
      | .bool => s!", box_ciphertext_bool({name})"
    let reads := (Array.range outputs.size).map fun i => s!"out{i}.unwrap().0"
    let value := match reads with
      | #[r] => r
      | _ => "(" ++ ", ".intercalate reads.toList ++ ")"
    entries := entries ++ s!"            fn {method} = {func}({macroParams}) -> {ret};\n"
    funcs := funcs ++ s!"pub fn {func}(builder: &::zhc_builder::Builder{funcParams}) -> {ret} \{
    unsafe extern \"C\" \{
        fn {symbol}(builder: *mut c_void{String.join externParams.toList}{String.join externSlots.toList}) -> *mut c_void;
    }
    prepare();
{String.join slotDecls.toList}    let res = unsafe \{ {symbol}(box_builder(builder){String.join callArgs.toList}{String.join slotArgs.toList}) };
{String.join takes.toList}    finish(res);
    {value}
}

"
  IO.FS.writeFile outPath s!"// Generated from `{moduleName}` by `Zhc/Ffi/R2l/GenerateRustBinding.lean`. Do not edit by hand.

use super::*;

{funcs}#[macro_export]
macro_rules! for_each_export \{
    ($m:ident) => \{
        $m! \{
{entries}        }
    };
}
"
  pure 0
