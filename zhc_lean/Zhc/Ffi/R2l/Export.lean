import Zhc.Ffi.R2l.Macro
import Zhc.Iops.AddRippleCarry.Theorems
import Zhc.Iops.AddHillisSteele.Theorems

namespace Zhc.Ffi.R2l

open Iops

zhc_export add_ripple_carry (si : SpecInteger sb22) (x y : CiphertextInteger si) :
    (r : CiphertextInteger si) where
  spec := r.toNat = (x.toNat + y.toNat) % 2 ^ si.intSize
  prog := addRippleCarry
  correct := addRippleCarry_correct
  clean := addRippleCarry_isClean
  projects := addRippleCarry_traced_projects

zhc_export add_hillis_steele (si : SpecInteger sb22) (x y : CiphertextInteger si) :
    (r : CiphertextInteger si) where
  spec := r.toNat = (x.toNat + y.toNat) % 2 ^ si.intSize
  prog := addHillisSteele
  correct := addHillisSteele_correct
  clean := addHillisSteele_clean
  projects := addHillisSteele_projects

end Zhc.Ffi.R2l
