import Lake
open System Lake DSL

def workspaceDir (pkg : Package) : FilePath := pkg.dir / ".."

package zhc_lean

lean_lib Lints

@[default_target]
lean_lib Zhc where
  plugins := #[`@/Lints:shared]

@[test_driver]
lean_lib Tests where
  precompileModules := true
  plugins := #[`@/Lints:shared]

target zhc_iops_l2r_lib pkg : FilePath := Job.async do
  let targetDir := pkg.buildDir / "cargo"
  let libFile := targetDir / "release" / "libzhc_iops_l2r.a"
  proc {
    cmd := "cargo"
    args := #["build", "-p", "zhc_iops_l2r", "--lib", "--release",
      "--target-dir", targetDir.toString]
    cwd := workspaceDir pkg
  }
  addTrace (← computeTrace libFile)
  return libFile

target zhc_lean_shim_o pkg : FilePath := do
  let oFile := pkg.buildDir / "ffi" / "zhc_l2r.o"
  let srcJob ← inputTextFile <| pkg.dir / "ffi" / "zhc_l2r.c"
  let weakArgs := #[
    "-I", (← getLeanIncludeDir).toString,
    "-I", (workspaceDir pkg / "zhc_iops_l2r" / "include").toString
  ]
  buildO oFile srcJob weakArgs #["-fPIC", "-Wall", "-Wextra", "-Werror", "-std=c11"] "cc"
    getLeanTrace

extern_lib libzhc_lean_shim pkg := do
  let oJob ← zhc_lean_shim_o.fetch
  buildStaticLib (pkg.staticLibDir / nameToStaticLib "zhc_lean_shim") #[oJob]

extern_lib libzhc_iops_l2r _pkg := zhc_iops_l2r_lib.fetch
