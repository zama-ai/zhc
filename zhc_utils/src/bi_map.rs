use std::fmt::Debug;
use std::ops::Index;

use crate::Dumpable;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Domain<T>(T);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CoDomain<T>(T);

pub struct BiMap<D, C>(Vec<(Domain<D>, CoDomain<C>)>);

impl<D, C> BiMap<D, C>
where
    D: PartialEq,
    C: PartialEq,
{
    pub fn new() -> Self {
        BiMap(Vec::new())
    }

    pub fn insert(&mut self, dom: D, codom: C) {
        for (existing_dom, existing_codom) in &self.0 {
            if existing_dom.0 == dom {
                panic!("Domain value already mapped");
            }
            if existing_codom.0 == codom {
                panic!("CoDomain value already mapped");
            }
        }
        self.0.push((Domain(dom), CoDomain(codom)));
    }

    pub fn get_dom(&self, dom: &D) -> Option<&C> {
        self.0.iter().find(|(d, _)| &d.0 == dom).map(|(_, c)| &c.0)
    }

    pub fn get_dom_mut(&mut self, dom: &D) -> Option<&mut C> {
        self.0
            .iter_mut()
            .find(|(d, _)| &d.0 == dom)
            .map(|(_, c)| &mut c.0)
    }

    pub fn has_dom(&self, dom: &D) -> bool {
        self.get_dom(dom).is_some()
    }

    pub fn get_codom(&self, codom: &C) -> Option<&D> {
        self.0
            .iter()
            .find(|(_, c)| &c.0 == codom)
            .map(|(d, _)| &d.0)
    }

    pub fn get_codom_mut(&mut self, codom: &C) -> Option<&mut D> {
        self.0
            .iter_mut()
            .find(|(_, c)| &c.0 == codom)
            .map(|(d, _)| &mut d.0)
    }

    pub fn has_codom(&self, codom: &C) -> bool {
        self.get_codom(codom).is_some()
    }

    pub fn into_iter(self) -> impl Iterator<Item = (D, C)> {
        self.0.into_iter().map(|(d, c)| (d.0, c.0))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&D, &C)> {
        self.0.iter().map(|(d, c)| (&d.0, &c.0))
    }
}

impl<D, C> Index<Domain<&D>> for BiMap<D, C>
where
    D: PartialEq,
    C: PartialEq,
{
    type Output = C;

    fn index(&self, index: Domain<&D>) -> &Self::Output {
        self.get_dom(index.0)
            .expect("Domain key not found in BiMap")
    }
}

impl<D, C> Index<CoDomain<&C>> for BiMap<D, C>
where
    D: PartialEq,
    C: PartialEq,
{
    type Output = D;

    fn index(&self, index: CoDomain<&C>) -> &Self::Output {
        self.get_codom(index.0)
            .expect("CoDomain key not found in BiMap")
    }
}

impl<D: Dumpable + PartialEq, C: Dumpable + PartialEq> Dumpable for BiMap<D, C> {
    fn dump_to_string(&self) -> String {
        let mut result = String::from("{ ");
        for (dom, codom) in self.iter() {
            result.push_str(&format!(
                "  {:?} <-> {:?}, ",
                dom.dump_to_string(),
                codom.dump_to_string()
            ));
        }
        result.push_str("}");
        result
    }
}
