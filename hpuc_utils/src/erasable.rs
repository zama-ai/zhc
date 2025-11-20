#[derive(Debug, PartialEq, Eq)]
pub enum DeadOrAlive<T> {
    Live(T),
    Erased(T),
}

impl<T> DeadOrAlive<T> {
    pub fn is_live(&self) -> bool {
        matches!(self, DeadOrAlive::Live(_))
    }

    pub fn is_erased(&self) -> bool {
        matches!(self, DeadOrAlive::Erased(_))
    }

    pub fn as_ref(&self) -> DeadOrAlive<&T> {
        match self {
            DeadOrAlive::Live(v) => DeadOrAlive::Live(v),
            DeadOrAlive::Erased(v) => DeadOrAlive::Erased(v),
        }
    }

    pub fn as_mut(&mut self) -> DeadOrAlive<&mut T> {
        match self {
            DeadOrAlive::Live(v) => DeadOrAlive::Live(v),
            DeadOrAlive::Erased(v) => DeadOrAlive::Erased(v),
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            DeadOrAlive::Live(v) => v,
            DeadOrAlive::Erased(_) => panic!("Tried to unwrap an erased value"),
        }
    }

    pub fn unwrap_erased(self) -> T {
        match self {
            DeadOrAlive::Erased(v) => v,
            DeadOrAlive::Live(_) => panic!("Tried to unwrap_erased a live value"),
        }
    }

    pub fn unjail(self) -> T {
        match self {
            DeadOrAlive::Erased(v) => v,
            DeadOrAlive::Live(v) => v,
        }
    }

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
