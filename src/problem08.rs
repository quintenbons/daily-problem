//! # Easy - Google
//! Unival subtree count

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn general_test() {
        let tree = Node::combine(
            0,
            Some(Node::new(1)),
            Some(Node::combine(
                0,
                Some(Node::combine(1, Some(Node::new(1)), Some(Node::new(1)))),
                Some(Node::new(0)),
            )),
        );
        assert_eq!(count_univals(&tree), 5);
    }
}

struct Node {
    value: i32,
    left: Box<Option<Node>>,
    right: Box<Option<Node>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            left: Box::new(None),
            right: Box::new(None),
        }
    }

    fn combine(value: i32, left: Option<Node>, right: Option<Node>) -> Node {
        Node {
            value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

/// # Recursive
/// Counts the number of unival subtrees of tree
fn count_univals(tree: &Node) -> i32 {
    // we don't care about val this is not a sentinel
    // value
    aux_univals(tree, 0).0
}

/// Recursive auxilary function. Takes a number
/// gives the number of unival subrees and whether
/// the tree is unival with value val
fn aux_univals(tree: &Node, value: i32) -> (i32, bool) {
    let mut count: i32 = 0;
    let mut is_unival = true;

    let mut manage = |treeref: &Option<Node>| {
        if let Some(node) = treeref {
            let (number, unival) = aux_univals(node, tree.value);
            count += number;
            is_unival &= unival;
        }
    };

    manage(&tree.left);
    manage(&tree.right);

    (count + if is_unival {1} else {0}, is_unival && tree.value == value)
}

/// auxaux
fn deeper(treeref: &Option<Node>, expected_value: i32, count: &mut i32, is_unival: &mut bool) {}
