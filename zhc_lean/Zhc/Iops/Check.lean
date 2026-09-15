import Lean
import Zhc.Iops.Algo

namespace Zhc

open Lean Elab Command in
elab "#check_all_algos" : command => do
  let env ← getEnv
  let algos := env.constants.fold (init := #[]) fun acc n ci =>
    if ci.type.getAppFn.isConstOf ``Zhc.Algo then acc.push (n, ci) else acc
  if algos.isEmpty then
    throwError "no Algo found"
  for (n, ci) in algos do
    unless ci matches .defnInfo _ do
      throwError "{n} is not a definition"
    if isNoncomputable env n then
      throwError "{n} is noncomputable"
    if (← liftCoreM (collectAxioms n)).contains ``sorryAx then
      throwError "{n} uses sorry"
  logInfo m!"checked {algos.size} algorithms: {algos.map (·.1)}"

end Zhc
