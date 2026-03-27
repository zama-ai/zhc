use std::fmt::{Debug, Display};
use std::rc::Rc;

use zhc_utils::Dumpable;
use zhc_utils::small::SmallVec;

#[derive(Debug, Clone, PartialEq, Eq)]
enum HierarchyInner {
    Root,
    Leaf {
        comment: String,
        depth: u8,
        parent: Rc<HierarchyInner>,
    },
}

impl HierarchyInner {
    pub fn is_child(self: &Rc<Self>, maybe_parent: &Rc<Self>) -> bool {
        if Rc::ptr_eq(self, maybe_parent) {
            true
        } else {
            match self.as_ref() {
                HierarchyInner::Root => false,
                HierarchyInner::Leaf { parent, .. } => parent.is_child(maybe_parent),
            }
        }
    }

    pub fn format(self: &Rc<Self>) -> String {
        match self.as_ref() {
            HierarchyInner::Root => "".into(),
            HierarchyInner::Leaf {
                comment, parent, ..
            } => {
                if parent.is_root() {
                    format!("{}", comment)
                } else {
                    format!("{} / {}", parent.comment(), comment)
                }
            }
        }
    }

    pub fn is_root(self: &Rc<Self>) -> bool {
        matches!(self.as_ref(), HierarchyInner::Root)
    }

    pub fn get_depth(self: &Rc<Self>) -> u8 {
        match self.as_ref() {
            HierarchyInner::Root => 0,
            HierarchyInner::Leaf { depth, .. } => *depth,
        }
    }

    pub fn comment(self: &Rc<Self>) -> String {
        match self.as_ref() {
            HierarchyInner::Root => "".into(),
            HierarchyInner::Leaf { comment, .. } => comment.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Hierarchy(Rc<HierarchyInner>);

impl Hierarchy {
    pub fn new() -> Self {
        Hierarchy(Rc::new(HierarchyInner::Root))
    }

    pub fn push(&mut self, comment: impl Into<String>) {
        self.0 = Rc::new(HierarchyInner::Leaf {
            comment: comment.into(),
            depth: self.0.get_depth() + 1,
            parent: self.0.clone(),
        });
    }

    pub fn pop(&mut self) {
        self.0 = match self.0.as_ref() {
            HierarchyInner::Root => panic!(),
            HierarchyInner::Leaf { parent, .. } => parent.clone(),
        };
    }

    pub fn is_root(&self) -> bool {
        self.0.is_root()
    }

    /// Returns the parent hierarchy, or None if this is the root.
    pub fn parent(&self) -> Option<Hierarchy> {
        match self.0.as_ref() {
            HierarchyInner::Root => None,
            HierarchyInner::Leaf { parent, .. } => Some(Hierarchy(parent.clone())),
        }
    }

    pub fn common_ancestor(&self, other: &Self) -> Option<Hierarchy> {
        // If they're the same, return self
        if self == other {
            return Some(self.clone());
        }

        // Bring both to the same depth by walking up from the deeper one
        let mut self_cursor = self.clone();
        let mut other_cursor = other.clone();

        while self_cursor.0.get_depth() > other_cursor.0.get_depth() {
            self_cursor.pop();
        }

        while other_cursor.0.get_depth() > self_cursor.0.get_depth() {
            other_cursor.pop();
        }

        // Now walk up both until we find a common ancestor
        while self_cursor != other_cursor {
            if self_cursor.is_root() || other_cursor.is_root() {
                // If we've reached root without finding common ancestor, return None
                return None;
            }
            self_cursor.pop();
            other_cursor.pop();
        }

        Some(self_cursor)
    }

    pub fn get_root(&self) -> Hierarchy {
        let mut current = self.clone();
        while let Some(parent) = current.parent() {
            current = parent;
        }
        current
    }

    pub fn range(from_exc: &Self, mut to_inc: Self) -> impl DoubleEndedIterator<Item = Hierarchy> {
        assert!(*from_exc >= to_inc);
        let mut output = SmallVec::new();
        while to_inc != *from_exc {
            output.push(to_inc.clone());
            to_inc.pop();
        }
        output.into_iter().rev()
    }

    pub fn comment(&self) -> String {
        self.0.comment()
    }
}

impl Debug for Hierarchy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hierarchy({})", &self.0.format())
    }
}

impl Display for Hierarchy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0.format())
    }
}

impl PartialEq for Hierarchy {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Hierarchy {}

impl PartialOrd for Hierarchy {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if Rc::ptr_eq(&self.0, &other.0) {
            Some(std::cmp::Ordering::Equal)
        } else if other.0.is_child(&self.0) {
            Some(std::cmp::Ordering::Greater)
        } else if self.0.is_child(&other.0) {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

impl Dumpable for Hierarchy {
    fn dump_to_string(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_hierarchy_is_root() {
        let h = Hierarchy::new();
        assert!(h.is_root());
    }

    #[test]
    fn test_push_pop() {
        let mut h = Hierarchy::new();
        h.push("level1");
        assert!(!h.is_root());
        h.push("level2");
        h.pop();
        h.pop();
        assert!(h.is_root());
    }

    #[test]
    #[should_panic]
    fn test_pop_root_panics() {
        let mut h = Hierarchy::new();
        h.pop();
    }

    #[test]
    fn test_parent() {
        let mut h = Hierarchy::new();
        assert_eq!(h.parent(), None);
        h.push("level1");
        assert!(h.parent().is_some());
        assert!(h.parent().unwrap().is_root());
    }

    #[test]
    fn test_get_root() {
        let mut h = Hierarchy::new();
        let root = h.get_root();
        assert!(root.is_root());

        h.push("level1");
        h.push("level2");
        h.push("level3");
        let root = h.get_root();
        assert!(root.is_root());
    }

    #[test]
    fn test_equality() {
        let mut h1 = Hierarchy::new();
        let h2 = Hierarchy::new();

        // Different root instances are not equal
        assert_ne!(h1, h2);

        // Same instance is equal
        let h3 = h1.clone();
        assert_eq!(h1, h3);

        h1.push("test");
        let h4 = h1.clone();
        assert_eq!(h1, h4);
    }

    #[test]
    fn test_partial_ord() {
        let mut h1 = Hierarchy::new();
        let h_root = h1.clone();

        h1.push("level1");
        let h_level1 = h1.clone();

        h1.push("level2");
        let h_level2 = h1.clone();

        // Child is less than parent
        assert!(h_level1 < h_root);
        assert!(h_level2 < h_level1);
        assert!(h_level2 < h_root);

        // Parent is greater than child
        assert!(h_root > h_level1);
        assert!(h_level1 > h_level2);

        // Unrelated hierarchies
        let mut h2 = Hierarchy::new();
        h2.push("other");
        assert_eq!(h2.partial_cmp(&h_level1), None);
    }

    #[test]
    fn test_common_ancestor_same() {
        let mut h = Hierarchy::new();
        h.push("level1");
        let ancestor = h.common_ancestor(&h);
        assert_eq!(ancestor, Some(h.clone()));
    }

    #[test]
    fn test_common_ancestor_parent_child() {
        let mut h = Hierarchy::new();
        h.push("level1");
        let parent = h.clone();
        h.push("level2");
        let child = h.clone();

        let ancestor = child.common_ancestor(&parent);
        assert_eq!(ancestor, Some(parent.clone()));
    }

    #[test]
    fn test_common_ancestor_siblings() {
        let mut h = Hierarchy::new();
        h.push("level1");
        let parent = h.clone();

        h.push("child1");
        let child1 = h.clone();
        h.pop();

        h.push("child2");
        let child2 = h.clone();

        let ancestor = child1.common_ancestor(&child2);
        assert_eq!(ancestor, Some(parent));
    }

    #[test]
    fn test_common_ancestor_different_roots() {
        let mut h1 = Hierarchy::new();
        h1.push("level1");

        let mut h2 = Hierarchy::new();
        h2.push("level1");

        let ancestor = h1.common_ancestor(&h2);
        assert_eq!(ancestor, None);
    }

    #[test]
    fn test_range() {
        let mut h = Hierarchy::new();
        let root = h.clone();
        h.push("level1");
        let level1 = h.clone();
        h.push("level2");
        let level2 = h.clone();
        h.push("level3");
        let level3 = h.clone();

        let range: Vec<_> = Hierarchy::range(&root, level3.clone()).collect();
        assert_eq!(range.len(), 3);
        assert_eq!(range[0], level1);
        assert_eq!(range[1], level2);
        assert_eq!(range[2], level3);
    }

    #[test]
    fn test_depth() {
        let mut h = Hierarchy::new();
        assert_eq!(h.0.get_depth(), 0);

        h.push("level1");
        assert_eq!(h.0.get_depth(), 1);

        h.push("level2");
        assert_eq!(h.0.get_depth(), 2);

        h.pop();
        assert_eq!(h.0.get_depth(), 1);
    }
}
