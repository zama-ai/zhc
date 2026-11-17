namespace Zhc.Model

class Clean (α : Type) where
  isClean : α → Prop

variable {α β : Type}

instance [Clean α] [Clean β] : Clean (α × β) := ⟨fun p => Clean.isClean p.1 ∧ Clean.isClean p.2⟩

instance [Clean α] : Clean (Id α) := inferInstanceAs (Clean α)

end Zhc.Model
