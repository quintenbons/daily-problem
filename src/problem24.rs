#[allow(dead_code)]
//! Medium - Google
//! Locked binary trees
//! No test this time

use std::rc::Rc;
use std::rc::Weak;

struct Node {
    locked: bool,
    locked_children: usize,
    child0: Rc<Tree>,
    child1: Rc<Tree>,
    parent: Weak<Tree>,
}

enum Tree {
    Empty,
    Node(Node),
}

impl Tree {
    fn is_locked(&self) -> bool {
        unimplemented!();
    }

    fn lock(&mut self) -> bool {
        unimplemented!();
    }

    /// If you lock both father and son
    /// they will be locked forever...
    fn unlock(&mut self) -> bool {
        unimplemented!();
    }

    fn can_toggle(&self) -> bool {
        unimplemented!();
    }
}
