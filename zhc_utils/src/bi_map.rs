//! Bijective map between a domain and codomain type.
//!
//! [`BiMap`] enforces a strict one-to-one correspondence: every domain value
//! maps to exactly one codomain value and vice versa. Lookups are supported
//! in both directions. Duplicate domain or codomain values are rejected at
//! insertion time via panics.
//!
//! [`Domain`] and [`CoDomain`] are opaque newtype wrappers used as typed
//! index keys for the [`Index`](std::ops::Index) implementations on
//! [`BiMap`].

use std::fmt::Debug;
use std::ops::Index;

use crate::Dumpable;

/// Opaque wrapper tagging a value as a domain-side key for [`BiMap`]
/// indexing.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Domain<T>(T);

/// Opaque wrapper tagging a value as a codomain-side key for [`BiMap`]
/// indexing.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CoDomain<T>(T);

/// Bijective map enforcing one-to-one correspondence between domain and
/// codomain values.
///
/// Both the domain type `D` and codomain type `C` must implement
/// [`PartialEq`]. All entries are unique on both sides: no two pairs share
/// a domain value, and no two pairs share a codomain value. Equality
/// comparison between two [`BiMap`]s is order-independent.
#[derive(Clone, Debug)]
pub struct BiMap<D, C>(Vec<(Domain<D>, CoDomain<C>)>);

impl<D, C> BiMap<D, C>
where
    D: PartialEq,
    C: PartialEq,
{
    /// Creates an empty [`BiMap`].
    pub fn new() -> Self {
        BiMap(Vec::new())
    }

    /// Returns the number of pairs in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Inserts a domain-codomain pair into the map.
    ///
    /// The `dom` value is associated with `codom`, establishing a
    /// bidirectional mapping. Both values must be absent from the map;
    /// the bijection invariant is enforced at insertion time.
    ///
    /// # Panics
    ///
    /// Panics if `dom` is already present as a domain key, or if `codom`
    /// is already present as a codomain key.
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

    /// Returns a shared reference to the codomain value mapped to `dom`,
    /// or `None` if the domain key is absent.
    pub fn get_dom(&self, dom: &D) -> Option<&C> {
        self.0.iter().find(|(d, _)| &d.0 == dom).map(|(_, c)| &c.0)
    }

    /// Returns a mutable reference to the codomain value mapped to `dom`,
    /// or `None` if the domain key is absent.
    pub fn get_dom_mut(&mut self, dom: &D) -> Option<&mut C> {
        self.0
            .iter_mut()
            .find(|(d, _)| &d.0 == dom)
            .map(|(_, c)| &mut c.0)
    }

    /// Returns true if the map contains `dom` as a domain key.
    pub fn has_dom(&self, dom: &D) -> bool {
        self.get_dom(dom).is_some()
    }

    /// Returns a shared reference to the domain value mapped to `codom`,
    /// or `None` if the codomain key is absent.
    pub fn get_codom(&self, codom: &C) -> Option<&D> {
        self.0
            .iter()
            .find(|(_, c)| &c.0 == codom)
            .map(|(d, _)| &d.0)
    }

    /// Returns a mutable reference to the domain value mapped to `codom`,
    /// or `None` if the codomain key is absent.
    pub fn get_codom_mut(&mut self, codom: &C) -> Option<&mut D> {
        self.0
            .iter_mut()
            .find(|(_, c)| &c.0 == codom)
            .map(|(d, _)| &mut d.0)
    }

    /// Returns true if the map contains `codom` as a codomain key.
    pub fn has_codom(&self, codom: &C) -> bool {
        self.get_codom(codom).is_some()
    }

    /// Returns a consuming iterator over `(D, C)` pairs in insertion order.
    pub fn into_iter(self) -> impl Iterator<Item = (D, C)> {
        self.0.into_iter().map(|(d, c)| (d.0, c.0))
    }

    /// Returns a borrowing iterator over `(&D, &C)` pairs in insertion
    /// order.
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

impl<D, C> PartialEq for BiMap<D, C>
where
    D: PartialEq,
    C: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0
            .iter()
            .all(|(d, c)| other.get_dom(&d.0).map_or(false, |other_c| other_c == &c.0))
    }
}

impl<D, C> Eq for BiMap<D, C>
where
    D: Eq,
    C: Eq,
{
}
