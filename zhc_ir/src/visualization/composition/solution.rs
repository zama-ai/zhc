use std::{cell::RefCell, fmt::Display, rc::Rc};
use zhc_utils::{
    fsm,
    graphics::{Frame, Size},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableCell(Rc<RefCell<Variable>>);

impl VariableCell {
    pub fn fresh() -> Self {
        VariableCell(Rc::new(RefCell::new(Variable::Fresh)))
    }
}

impl VariableCell {
    pub fn set_size(&mut self, size: Size) {
        self.0.borrow_mut().set_size(size);
    }

    pub fn set_frame(&mut self, frame: Frame) {
        self.0.borrow_mut().set_frame(frame);
    }

    pub fn get_size(&self) -> Size {
        self.0.borrow().get_size()
    }

    pub fn get_frame(&self) -> Frame {
        self.0.borrow().get_frame()
    }
}

impl Display for VariableCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr_str = format!("{:p}", self.0.as_ptr());
        let last_4 = &ptr_str[ptr_str.len().saturating_sub(4)..];
        write!(f, "#{}", last_4)
    }
}

impl VariableCell {
    pub fn watch(&self) -> VariableWatch {
        VariableWatch(self.0.clone())
    }
}

/// Read-only reference to a variable cell.
#[derive(Debug, Clone)]
pub struct VariableWatch(Rc<RefCell<Variable>>);

impl VariableWatch {
    pub fn get_frame(&self) -> Frame {
        self.0.borrow().get_frame()
    }
}

/// Tracks the layout computation state of an element through the sizing and positioning phases.
#[fsm]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variable {
    Fresh,
    Sized { size: Size },
    Framed { size: Size, frame: Frame },
}

impl Variable {
    /// Transitions from Fresh to Sized state with the computed size.
    pub fn set_size(&mut self, size: Size) {
        self.transition(|old| {
            let Variable::Fresh = old else { panic!() };
            Variable::Sized { size }
        });
    }

    /// Transitions from Sized to Framed state with the final positioned frame.
    pub fn set_frame(&mut self, frame: Frame) {
        self.transition(|old| {
            let Variable::Sized { size } = old else {
                panic!()
            };
            Variable::Framed { size, frame }
        });
    }

    /// Returns the computed size after the sizing phase.
    pub fn get_size(&self) -> Size {
        match self {
            Self::Sized { size, .. } | Self::Framed { size, .. } => size.clone(),
            _ => panic!(),
        }
    }

    /// Returns the final positioned frame after the positioning phase.
    pub fn get_frame(&self) -> Frame {
        match self {
            Self::Framed { frame, .. } => frame.clone(),
            _ => panic!(),
        }
    }
}
