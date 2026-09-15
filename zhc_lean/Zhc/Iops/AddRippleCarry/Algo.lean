import Zhc.Iops.Algo
import Zhc.Iops.AddRippleCarry.Theorems

namespace Zhc.Iops

open Model (SpecInteger)

def rippleAdd : Algo sb22 where
  Param := SpecInteger sb22
  ins si := [.ctInt si, .ctInt si]
  outs si := [.ctInt si]
  prog si := fun (x, y) => addRippleCarry si x y
  spec si := fun (x, y) r => r.toNat = (x.toNat + y.toNat) % 2 ^ si.intSize
  correct si := fun (x, y) ⟨hx, hy⟩ => addRippleCarry_correct si x y hx hy
  clean si := fun (x, y) ⟨hx, hy⟩ => addRippleCarry_isClean si x y hx hy
  traced_projects si := fun (x, y) => addRippleCarry_traced_projects si x y

end Zhc.Iops
