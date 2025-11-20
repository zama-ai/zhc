/// Represents the operational state of an element that can be either active or inactive.
#[derive(Debug, Clone, Copy)]
pub enum State<T = ()> {
    /// The element is currently active and operational.
    Active(T),
    /// The element is currently inactive and not operational.
    Inactive(T),
}

#[allow(dead_code)]
impl<T> State<T> {
    /// Returns `true` if the state is active.
    pub fn is_active(&self) -> bool {
        matches!(self, State::Active(_))
    }

    /// Returns `true` if the state is inactive.
    pub fn is_inactive(&self) -> bool {
        matches!(self, State::Inactive(_))
    }

    /// Transitions an active state to inactive while preserving the contained value.
    ///
    /// # Panics
    ///
    /// Panics if the state is already inactive.
    pub fn shutdown(&mut self) {
        assert!(
            self.is_active(),
            "Tried to shut an already inactive element"
        );

        unsafe {
            let value = match std::ptr::read(self) {
                State::Active(v) => v,
                State::Inactive(_) => unreachable!(),
            };

            std::ptr::write(self, State::Inactive(value));
        }
    }

    /// Returns the contained value, consuming the state.
    ///
    /// # Panics
    ///
    /// Panics if the state is inactive.
    pub fn unwrap_active(self) -> T {
        match self {
            State::Active(t) => t,
            State::Inactive(_) => panic!("Tried top unwrap_active an inactive state"),
        }
    }

    /// Returns the contained value, consuming the state.
    ///
    /// # Panics
    ///
    /// Panics if the state is active.
    pub fn unwrap_inactive(self) -> T {
        match self {
            State::Active(t) => t,
            State::Inactive(_) => panic!("Tried top unwrap_active an inactive state"),
        }
    }

    /// Converts the state to contain references to the contained value instead of owned values.
    pub fn as_ref(&self) -> State<&T> {
        match self {
            State::Active(v) => State::Active(&v),
            State::Inactive(v) => State::Inactive(&v),
        }
    }

    /// Converts the state to contain mutable references to the contained value instead of owned
    /// values.
    pub fn as_mut_ref(&mut self) -> State<&mut T> {
        match self {
            State::Active(v) => State::Active(v),
            State::Inactive(v) => State::Inactive(v),
        }
    }
}
