use std::cell::RefCell;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::Rc;

use zhc_utils::Dumpable;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Place(pub f64);

impl Eq for Place {}

impl Ord for Place {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Hash for Place {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        panic!("Not needed.")
    }
}

impl Dumpable for Place {
    fn dump_to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlaceCell(Rc<RefCell<Place>>);

impl Display for PlaceCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr_str = format!("{:p}", self.0.as_ptr());
        let last_4 = &ptr_str[ptr_str.len().saturating_sub(4)..];
        write!(f, "#{}({:.1})", last_4, self.0.borrow().0)
    }
}

impl Hash for PlaceCell {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        panic!("Not needed.")
    }
}

impl PlaceCell {
    pub fn new(val: u16) -> Self {
        PlaceCell(Rc::new(RefCell::new(Place(val as f64))))
    }

    pub fn get_val(&self) -> Place {
        *self.0.borrow()
    }

    pub fn set_val(&self, new: Place) {
        *self.0.borrow_mut() = new;
    }
}
