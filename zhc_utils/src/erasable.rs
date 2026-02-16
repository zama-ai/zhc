/// A value that can be marked as live or erased while preserving the underlying data.
///
/// This enum allows tracking the state of a value without losing access to it.
/// Values can be moved between live and erased states, and the underlying data
/// can be extracted regardless of state using `unjail`.
#[derive(Debug, PartialEq, Eq)]
pub enum DeadOrAlive<T> {
    /// A live value that is actively being used.
    Live(T),
    /// An erased value that is marked as inactive but still accessible.
    Erased(T),
}

impl<T> DeadOrAlive<T> {
    /// Returns `true` if the value is in the live state.
    pub fn is_live(&self) -> bool {
        matches!(self, DeadOrAlive::Live(_))
    }

    /// Returns `true` if the value is in the erased state.
    pub fn is_erased(&self) -> bool {
        matches!(self, DeadOrAlive::Erased(_))
    }

    /// Converts from `&DeadOrAlive<T>` to `DeadOrAlive<&T>`.
    pub fn as_ref(&self) -> DeadOrAlive<&T> {
        match self {
            DeadOrAlive::Live(v) => DeadOrAlive::Live(v),
            DeadOrAlive::Erased(v) => DeadOrAlive::Erased(v),
        }
    }

    /// Converts from `&mut DeadOrAlive<T>` to `DeadOrAlive<&mut T>`.
    pub fn as_mut(&mut self) -> DeadOrAlive<&mut T> {
        match self {
            DeadOrAlive::Live(v) => DeadOrAlive::Live(v),
            DeadOrAlive::Erased(v) => DeadOrAlive::Erased(v),
        }
    }

    /// Extracts the value from a live state.
    ///
    /// # Panics
    ///
    /// Panics if the value is in the erased state.
    pub fn unwrap(self) -> T {
        match self {
            DeadOrAlive::Live(v) => v,
            DeadOrAlive::Erased(_) => panic!("Tried to unwrap an erased value"),
        }
    }

    /// Extracts the value from an erased state.
    ///
    /// # Panics
    ///
    /// Panics if the value is in the live state.
    pub fn unwrap_erased(self) -> T {
        match self {
            DeadOrAlive::Erased(v) => v,
            DeadOrAlive::Live(_) => panic!("Tried to unwrap_erased a live value"),
        }
    }

    /// Extracts the underlying value regardless of its state.
    pub fn unjail(self) -> T {
        match self {
            DeadOrAlive::Erased(v) => v,
            DeadOrAlive::Live(v) => v,
        }
    }

    /// Moves a live value to the erased state.
    ///
    /// If the value is already erased, this method has no effect.
    pub fn erase(&mut self) {
        if self.is_live() {
            let DeadOrAlive::Live(value) = std::mem::replace(self, unsafe { std::mem::zeroed() })
            else {
                unreachable!()
            };
            *self = DeadOrAlive::Erased(value);
        }
    }
}
