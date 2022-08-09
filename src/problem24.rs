#![allow(dead_code)]
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
    parent: Option<Weak<Tree>>,
}

enum Tree {
    Empty,
    Node(Node),
}

impl Node {
    fn is_locked(&self) -> bool {
        self.locked
    }

    fn lock(&mut self) -> bool {
        if !self.locked && self.can_toggle() {
            self.locked = true;
            self.increment_ancestors();
            true
        } else {
            false
        }
    }

    /// If you lock both father and son
    /// they will be locked forever...
    fn unlock(&mut self) -> bool {
        if self.locked && self.can_toggle() {
            self.locked = false;
            self.decrement_ancestors();
            true
        } else {
            false
        }
    }

    fn can_toggle(&self) -> bool {
        self.locked_children == 0 && !self.locked_ancestor()
    }

    fn locked_ancestor(&self) -> bool {
        match &self.parent {
            None => false,
            Some(weak_tree) => {
                if let Tree::Node(node) =
                    Rc::get_mut(&mut weak_tree.upgrade().unwrap()).unwrap()
                {
                    node.locked || node.locked_ancestor()
                } else {
                    panic!("Structural error in the tree");
                }
            }
        }
    }

    fn increment_ancestors(&mut self) {
        match &self.parent {
            None => (),
            Some(weak_tree) => {
                if let Tree::Node(node) =
                    Rc::get_mut(&mut weak_tree.upgrade().unwrap()).unwrap()
                {
                    node.locked_children += 1;
                    node.increment_ancestors();
                }
            }
        }
    }

    fn decrement_ancestors(&mut self) {
        match &self.parent {
            None => (),
            Some(weak_tree) => {
                if let Tree::Node(node) =
                    Rc::get_mut(&mut weak_tree.upgrade().unwrap()).unwrap()
                {
                    node.locked_children -= 1;
                    node.increment_ancestors();
                }
            }
        }
    }
}
