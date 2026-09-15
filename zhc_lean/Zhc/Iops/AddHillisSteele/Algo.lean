import Zhc.Iops.Algo
import Zhc.Iops.AddHillisSteele.Theorems

namespace Zhc.Iops

open Model (SpecInteger)

def hsAdd : Algo sb22 where
  Param := SpecInteger sb22
  ins si := [.ctInt si, .ctInt si]
  outs si := [.ctInt si]
  prog si := fun (x, y) => addHillisSteele si x y
  spec si := fun (x, y) r => r.toNat = (x.toNat + y.toNat) % 2 ^ si.intSize
  correct si := fun (x, y) ⟨hx, hy⟩ => addHillisSteele_correct si x y hx hy
  clean si := fun (x, y) ⟨hx, hy⟩ => addHillisSteele_isClean si x y hx hy
  traced_projects si := fun (x, y) => addHillisSteele_traced_projects si x y

end Zhc.Iops
