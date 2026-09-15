import Zhc.IopAlgebra.Traced.Instance
import Zhc.IopAlgebra.Model.Instance

namespace Zhc.IopAlgebra.Traced

open Zhc.Model

class HasModel (α : Type) (β : outParam Type) where
  model : α → β

export HasModel (model)

instance {H β : Type} : HasModel (Traced H β) β := ⟨Traced.model⟩

instance {sb : SpecBlock} : HasModel (IopAlgebra.CtBlock sb TraceM) (CiphertextBlock sb) :=
  ⟨Traced.model⟩

instance {sb : SpecBlock} : HasModel (IopAlgebra.PtBlock sb TraceM) (PlaintextBlock sb) :=
  ⟨Traced.model⟩

instance {sb : SpecBlock} : HasModel (IopAlgebra.CtBool sb TraceM) (CiphertextBool sb) :=
  ⟨Traced.model⟩

instance {sb : SpecBlock} {si : SpecInteger sb} :
    HasModel (IopAlgebra.CtInteger TraceM si) (CiphertextInteger si) :=
  ⟨Traced.model⟩

instance {sb : SpecBlock} {si : SpecInteger sb} :
    HasModel (IopAlgebra.PtInteger TraceM si) (PlaintextInteger si) :=
  ⟨Traced.model⟩

instance {α α' β β' : Type} [HasModel α α'] [HasModel β β'] : HasModel (α × β) (α' × β') :=
  ⟨fun p => (model p.1, model p.2)⟩

instance {α α' β β' : Type} [HasModel α α'] [HasModel β β'] : HasModel (MProd α β) (MProd α' β') :=
  ⟨fun p => ⟨model p.1, model p.2⟩⟩

instance : HasModel Unit Unit := ⟨id⟩

instance {α α' : Type} [HasModel α α'] : HasModel (ForInStep α) (ForInStep α') :=
  ⟨fun | .done a => .done (model a) | .yield a => .yield (model a)⟩

instance {α α' : Type} [HasModel α α'] : HasModel (List α) (List α') := ⟨List.map model⟩

instance {α α' : Type} [HasModel α α'] : HasModel (Option α) (Option α') := ⟨Option.map model⟩

instance : HasModel Nat Nat := ⟨id⟩

instance {n : Nat} : HasModel (Fin n) (Fin n) := ⟨id⟩

section ModelLemmas

variable {α α' : Type} [HasModel α α']

@[simp] theorem model_nil : model ([] : List α) = ([] : List α') := rfl

@[simp] theorem model_cons (a : α) (l : List α) : model (a :: l) = model a :: model l := rfl

@[simp] theorem model_none : model (none : Option α) = (none : Option α') := rfl

@[simp] theorem model_some (a : α) : model (some a) = some (model a) := rfl

@[simp] theorem model_nat (n : Nat) : model n = n := rfl

@[simp] theorem model_fin {n : Nat} (i : Fin n) : model i = i := rfl

@[simp] theorem model_pair {β β' : Type} [HasModel β β'] (a : α) (b : β) :
  model (a, b) = (model a, model b)
:=
  rfl

end ModelLemmas

section Projects

variable {α α' β β' ι : Type}

def Projects (m : TraceM α) (f : α → β) (v : β) : Prop :=
  ∀ b s r s', m.run b s = .ok r s' → f r = v

theorem Projects.pure (a : α) (f : α → β) : Projects (pure a : TraceM α) f (f a) := by
  intro b s r s' h
  cases h
  rfl

theorem Projects.bind {m : TraceM α} {k : α → TraceM β} {f : α → α'} {g : β → β'}
    {mId : Id α'} {kId : α' → Id β'}
    (hm : Projects m f mId) (hk : ∀ a, Projects (k a) g (kId (f a))) :
  Projects (m >>= k) g (mId >>= kId)
:= by
  intro b s r s' h
  change EST.bind (m.run b) (fun a => (k a).run b) s = .ok r s' at h
  unfold EST.bind at h
  split at h
  next a s1 hm' =>
    have := hk a b s1 r s' h
    rw [hm b s a s1 hm'] at this
    exact this
  next => cases h

theorem Projects.bindAny {m : TraceM α} {k : α → TraceM β} {g : β → β'} {v : β'}
    (hk : ∀ a, Projects (k a) g v) :
  Projects (m >>= k) g v
:= by
  intro b s r s' h
  change EST.bind (m.run b) (fun a => (k a).run b) s = .ok r s' at h
  unfold EST.bind at h
  split at h
  next a s1 _ => exact hk a b s1 r s' h
  next => cases h

theorem Projects.forIn {l : List ι} {init : α} {body : ι → α → TraceM (ForInStep α)}
    [HasModel α α'] {bodyId : ι → α' → Id (ForInStep α')}
    (h : ∀ i a, Projects (body i a) model (bodyId i (model a))) :
  Projects (forIn l init body) model (forIn (m := Id) l (model init) bodyId)
:= by
  induction l generalizing init with
  | nil => exact Projects.pure _ _
  | cons i l ih =>
    simp only [List.forIn_cons]
    refine Projects.bind (h i init) ?_
    intro step
    cases step with
    | done a => exact Projects.pure _ _
    | yield a => exact ih

theorem Projects.pure_eq {a : α} {f : α → β} {v : β} (h : f a = v) : Projects (Pure.pure a : TraceM α) f v :=
  h ▸ Projects.pure a f

theorem Projects.mapM_loop [HasModel ι ι'] [HasModel α α'] {f : ι → TraceM α}
    {g : ι' → Id α'} (h : ∀ i, Projects (f i) model (g (model i))) (l : List ι) (acc : List α) :
  Projects (List.mapM.loop f l acc) model (List.mapM.loop (m := Id) g (model l) (model acc))
:= by
  induction l generalizing acc with
  | nil => exact Projects.pure_eq (List.map_reverse (f := model) (l := acc))
  | cons i l ih => exact Projects.bind (h i) fun b => ih (b :: acc)

theorem Projects.mapM_loop_plain [HasModel α α'] {f : ι → TraceM α} {g : ι → Id α'}
    (h : ∀ i, Projects (f i) model (g i)) (l : List ι) (acc : List α) :
  Projects (List.mapM.loop f l acc) model (List.mapM.loop (m := Id) g l (model acc))
:= by
  induction l generalizing acc with
  | nil => exact Projects.pure_eq (List.map_reverse (f := model) (l := acc))
  | cons i l ih => exact Projects.bind (h i) fun b => ih (b :: acc)

theorem Projects.mapM_plain [HasModel α α'] {l : List ι} {f : ι → TraceM α} {g : ι → Id α'}
    (h : ∀ i, Projects (f i) model (g i)) :
  Projects (l.mapM f) model (l.mapM (m := Id) g)
:=
  Projects.mapM_loop_plain h l []

theorem Projects.mapM [HasModel ι ι'] [HasModel α α'] {l : List ι} {f : ι → TraceM α}
    {g : ι' → Id α'} (h : ∀ i, Projects (f i) model (g (model i))) :
  Projects (l.mapM f) model ((model l : List ι').mapM (m := Id) g)
:=
  Projects.mapM_loop h l []

theorem Projects.forIn' [HasModel ι ι'] [HasModel α α'] {l : List ι} {init : α}
    {body : ι → α → TraceM (ForInStep α)} {bodyId : ι' → α' → Id (ForInStep α')}
    (h : ∀ i a, Projects (body i a) model (bodyId (model i) (model a))) :
  Projects (ForIn.forIn l init body) model
    (ForIn.forIn (m := Id) (model l : List ι') (model init) bodyId)
:= by
  induction l generalizing init with
  | nil => exact Projects.pure _ _
  | cons i l ih =>
    simp only [model_cons, List.forIn_cons]
    refine Projects.bind (h i init) ?_
    intro step
    cases step with
    | done a => exact Projects.pure _ _
    | yield a => exact ih

end Projects

abbrev TraceM.ModelIs {α β : Type} [HasModel α β] (m : TraceM α) (v : β) : Prop :=
  Projects m model v

syntax "projects_op" : tactic

macro_rules
  | `(tactic| projects_op) => `(tactic| (repeat first
      | exact Projects.pure _ _
      | refine Projects.bindAny fun _ => ?_); done)

variable {sb : SpecBlock} {si : SpecInteger sb}

theorem Projects.declareInteger (si : SpecInteger sb) :
  Projects (IopAlgebra.declareInteger (M := TraceM) si) model (IopAlgebra.declareInteger (M := Id) si)
:= by projects_op

theorem Projects.letPtBlock (v : Nat) :
  Projects (IopAlgebra.letPtBlock (sb := sb) (M := TraceM) v) model
    (IopAlgebra.letPtBlock (sb := sb) (M := Id) v)
:= by projects_op

theorem Projects.letCtBlock (v : Nat) :
  Projects (IopAlgebra.letCtBlock (sb := sb) (M := TraceM) v) model
    (IopAlgebra.letCtBlock (sb := sb) (M := Id) v)
:= by projects_op

theorem Projects.addCt (a b : TrCtBlock sb) :
  Projects (IopAlgebra.addCt (M := TraceM) a b) model (IopAlgebra.addCt (M := Id) a.model b.model)
:= by projects_op

theorem Projects.subCt (a b : TrCtBlock sb) :
  Projects (IopAlgebra.subCt (M := TraceM) a b) model (IopAlgebra.subCt (M := Id) a.model b.model)
:= by projects_op

theorem Projects.shlCt (a : TrCtBlock sb) (amount : Nat) :
  Projects (IopAlgebra.shlCt (M := TraceM) a amount) model
    (IopAlgebra.shlCt (M := Id) a.model amount)
:= by projects_op

theorem Projects.packCt (a : TrCtBlock sb) (mul : Nat) (b : TrCtBlock sb) :
  Projects (IopAlgebra.packCt (M := TraceM) a mul b) model
    (IopAlgebra.packCt (M := Id) a.model mul b.model)
:= by projects_op

theorem Projects.addPt (a : TrCtBlock sb) (b : TrPtBlock sb) :
  Projects (IopAlgebra.addPt (M := TraceM) a b) model (IopAlgebra.addPt (M := Id) a.model b.model)
:= by projects_op

theorem Projects.subPt (a : TrCtBlock sb) (b : TrPtBlock sb) :
  Projects (IopAlgebra.subPt (M := TraceM) a b) model (IopAlgebra.subPt (M := Id) a.model b.model)
:= by projects_op

theorem Projects.ptSub (a : TrPtBlock sb) (b : TrCtBlock sb) :
  Projects (IopAlgebra.ptSub (M := TraceM) a b) model (IopAlgebra.ptSub (M := Id) a.model b.model)
:= by projects_op

theorem Projects.mulPt (a : TrCtBlock sb) (b : TrPtBlock sb) :
  Projects (IopAlgebra.mulPt (M := TraceM) a b) model (IopAlgebra.mulPt (M := Id) a.model b.model)
:= by projects_op

theorem Projects.extractCtBlock (x : TrCtInteger si) (i : Fin si.blockCount) :
  Projects (IopAlgebra.extractCtBlock (M := TraceM) x i) model
    (IopAlgebra.extractCtBlock (M := Id) x.model i)
:= by projects_op

theorem Projects.extractPtBlock (x : TrPtInteger si) (i : Fin si.blockCount) :
  Projects (IopAlgebra.extractPtBlock (M := TraceM) x i) model
    (IopAlgebra.extractPtBlock (M := Id) x.model i)
:= by projects_op

theorem Projects.storeCtBlock (x : TrCtInteger si) (i : Fin si.blockCount) (b : TrCtBlock sb) :
  Projects (IopAlgebra.storeCtBlock (M := TraceM) x i b) model
    (IopAlgebra.storeCtBlock (M := Id) x.model i b.model)
:= by projects_op

theorem Projects.boolFromBlock (b : TrCtBlock sb) :
  Projects (IopAlgebra.boolFromBlock (M := TraceM) b) model
    (IopAlgebra.boolFromBlock (M := Id) b.model)
:= by projects_op

theorem Projects.extractBoolBlock (b : TrCtBool sb) :
  Projects (IopAlgebra.extractBoolBlock (M := TraceM) b) model
    (IopAlgebra.extractBoolBlock (M := Id) b.model)
:= by projects_op

theorem Projects.pbs (lut : Lut1 sb) (a : TrCtBlock sb) :
  Projects (IopAlgebra.pbs (M := TraceM) lut a) model (IopAlgebra.pbs (M := Id) lut a.model)
:= by projects_op

theorem Projects.pbs2 (lut : Lut2 sb) (a : TrCtBlock sb) :
  Projects (IopAlgebra.pbs2 (M := TraceM) lut a) model (IopAlgebra.pbs2 (M := Id) lut a.model)
:= by projects_op

theorem Projects.pbs4 (lut : Lut4 sb) (a : TrCtBlock sb) :
  Projects (IopAlgebra.pbs4 (M := TraceM) lut a) model (IopAlgebra.pbs4 (M := Id) lut a.model)
:= by projects_op

theorem Projects.pbs8 (lut : Lut8 sb) (a : TrCtBlock sb) :
  Projects (IopAlgebra.pbs8 (M := TraceM) lut a) model (IopAlgebra.pbs8 (M := Id) lut a.model)
:= by projects_op

syntax "projects_prog" : tactic

macro_rules
  | `(tactic| projects_prog) => `(tactic| first
      | exact Projects.pure _ _
      | (refine Projects.bind (f := model) ?_ fun _ => ?_
         focus projects_op
         projects_prog)
      | (refine Projects.bind (Projects.forIn fun _ _ => ?_) fun _ => ?_ <;> projects_prog))

end Zhc.IopAlgebra.Traced
