/// Medium - Google
/// Serializing data structures into strings

#[cfg(test)]
mod tests {
    use super::*;
    use Tree::*;

    #[test]
    fn it_verifies() {
        let my_tree: Tree<i32> = Node(SNode {
            value: 1,
            left: Box::new(Empty),
            right: Box::new(Node(SNode {
                value: 2,
                left: Box::new(Node(SNode {
                    value: 4,
                    left: Box::new(Empty),
                    right: Box::new(Empty),
                })),
                right: Box::new(Node(SNode {
                    value: 3,
                    left: Box::new(Empty),
                    right: Box::new(Empty),
                })),
            })),
        });

        let again: Tree<i32> = Node(SNode {
            value: 1,
            left: Box::new(Empty),
            right: Box::new(Node(SNode {
                value: 2,
                left: Box::new(Node(SNode {
                    value: 4,
                    left: Box::new(Empty),
                    right: Box::new(Empty),
                })),
                right: Box::new(Node(SNode {
                    value: 3,
                    left: Box::new(Empty),
                    right: Box::new(Empty),
                })),
            })),
        });

        assert_eq!(my_tree, again);
    }

    #[test]
    fn it_serializes() {
        let empty_tree: Tree<i32> = Empty;
        let my_tree: Tree<i32> = Node(SNode {
            value: 1,
            left: Box::new(Empty),
            right: Box::new(Node(SNode {
                value: 2,
                left: Box::new(Node(SNode {
                    value: 4,
                    left: Box::new(Empty),
                    right: Box::new(Empty),
                })),
                right: Box::new(Node(SNode {
                    value: 3,
                    left: Box::new(Empty),
                    right: Box::new(Empty),
                })),
            })),
        });

        assert_eq!(serialize(&empty_tree), "N");
        assert_eq!(serialize(&empty_tree), "N");
        assert_eq!(serialize(&my_tree), "1-N-2-4-N-N-3-N-N");
        assert_eq!(serialize(&my_tree), "1-N-2-4-N-N-3-N-N");
    }

    #[test]
    fn it_deserializes() {
        let empty_tree = deserialize("N");
        let tree = deserialize("1-N-2-4-N-N-3-N-N");
        let my_tree: Tree<i32> = Node(SNode {
            value: 1,
            left: Box::new(Empty),
            right: Box::new(Node(SNode {
                value: 2,
                left: Box::new(Node(SNode {
                    value: 4,
                    left: Box::new(Empty),
                    right: Box::new(Empty),
                })),
                right: Box::new(Node(SNode {
                    value: 3,
                    left: Box::new(Empty),
                    right: Box::new(Empty),
                })),
            })),
        });

        assert_eq!(Tree::Empty, empty_tree);
        assert_eq!(my_tree.value(), Some(&1));
        assert_eq!(tree, my_tree);
    }
}

/// Implement binary tree struct would be cheating
#[derive(Debug)]
#[derive(PartialEq)]
enum Tree<T> {
    Empty,
    Node(SNode<T>),
}

#[derive(Debug)]
#[derive(PartialEq)]
struct SNode<T> {
    value: T,
    left: Box<Tree<T>>,
    right: Box<Tree<T>>,
}

impl<T> Tree<T> {
    fn value(&self) -> Option<&T> {
        match self {
            Tree::Empty => None,
            Tree::Node(n) => Some(&n.value),
        }
    }
}

/// Serializes an i32 tree. This could also work with
/// the Into<String> trait. Note that a string tree
/// will make some problems because the N and -
/// characters are being used
fn serialize(tree: &Tree<i32>) -> String {
    match tree {
        Tree::Empty => "N".to_string(),
        Tree::Node(node) => {
            let left_ser = serialize(&*node.left);
            let right_ser = serialize(&*node.right);

            format!("{}-{}-{}", node.value, left_ser, right_ser)
        }
    }
}

/// Unserializes an i32 tree.
fn deserialize(tree: &str) -> Tree<i32> {
    let mut vec: Vec<&str> = tree.split("-").collect();
    let mut start = 0;

    aux_deserialize(&vec, &mut start)
}

fn aux_deserialize(vec: &Vec<&str>, start: &mut usize) -> Tree<i32> {
    let content = vec.get(*start).expect("Unexpected end of serialization");
    *start += 1;

    if *content == "N" {
        return Tree::Empty
    }

    let value: i32 = content.parse().expect("Could not parse into i32");
    let left = Box::new(aux_deserialize(&vec, start));
    let right = Box::new(aux_deserialize(&vec, start));

    let snode = SNode::<i32> {
        value,
        left,
        right,
    };

    Tree::Node(snode)
}
