import Lake
open System Lake DSL

package zhc_lean

/-- `precompileModules` loads the compiled modules, and with them the C shim and the Rust
library, into the Lean frontend. This is what lets `#eval` call the FFI. -/
@[default_target]
lean_lib Zhc where
  precompileModules := true

/-- End-to-end FFI check: `lake exe zhc_smoke`. -/
lean_exe zhc_smoke where
  root := `Smoke

/-! ## C API

The Rust workspace root, holding `zhc_c_api`. -/
def workspaceDir (pkg : Package) : FilePath := pkg.dir / ".."

/-- Builds the Rust static library with cargo. Cargo does its own change tracking. -/
target zhc_c_api_lib pkg : FilePath := Job.async do
  let libFile := workspaceDir pkg / "target" / "release" / "libzhc_c_api.a"
  proc {
    cmd := "cargo"
    args := #["build", "-p", "zhc_c_api", "--lib", "--release"]
    cwd := workspaceDir pkg
  }
  addTrace (← computeTrace libFile)
  return libFile

/-- The Lean-ABI shim over the C API. -/
target zhc_lean_shim_o pkg : FilePath := do
  let oFile := pkg.buildDir / "ffi" / "zhc_lean.o"
  let srcJob ← inputTextFile <| pkg.dir / "ffi" / "zhc_lean.c"
  let weakArgs := #[
    "-I", (← getLeanIncludeDir).toString,
    "-I", (workspaceDir pkg / "zhc_c_api" / "include").toString
  ]
  buildO oFile srcJob weakArgs #["-fPIC", "-Wall", "-Wextra", "-Werror", "-std=c11"] "cc"
    getLeanTrace

/-! Link order matters for static archives: the shim refers to `zhc_*`, so it comes first. -/

extern_lib libzhc_lean_shim pkg := do
  let oJob ← zhc_lean_shim_o.fetch
  buildStaticLib (pkg.staticLibDir / nameToStaticLib "zhc_lean_shim") #[oJob]

extern_lib libzhc_c_api _pkg := zhc_c_api_lib.fetch
