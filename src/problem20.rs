#![allow(dead_code)]
//! # Easy - Google
//! Find common node in chained lists
//! Since rust is not designed with pointers in
//! mind, we will use ids

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let chain0 = Chain::from_vec(&vec![1, 3, 5, 4, 10, 8]);
        let chain1 = Chain::from_vec(&vec![2, 4, 11, 9, 2, 19]);

        assert_eq!(find_link(&chain0, &chain1), Some(4));
    }
}

use std::collections::HashSet;

struct Node {
    id: usize,
    next: Box<Chain>,
}

enum Chain {
    Empty,
    Node(Node),
}

impl Chain {
    fn new() -> Chain {
        Chain::Empty
    }

    fn new_node(id: usize, next: Self) -> Self {
        Chain::Node(Node {
            id,
            next: Box::new(next),
        })
    }

    fn from_vec(vector: &Vec<usize>) -> Self {
        if vector.is_empty() {
            return Chain::new();
        }

        let mut res = Chain::new_node(*vector.last().unwrap(), Chain::Empty);

        for id in vector.iter().rev().skip(1) {
            res = Chain::new_node(*id, res);
        }

        res
    }

    fn next<'a>(&'a self) -> &'a Chain {
        match self {
            Self::Empty => self,
            Self::Node(node) => node.next.as_ref(),
        }
    }
}

/// Gives the common node of both chains. Expects
/// there is only one, but gives the on closest to
/// chain1's head if there are multiple
fn find_link(chain0: &Chain, chain1: &Chain) -> Option<usize> {
    // Make a set containing all ids of chain0
    let mut seen: HashSet<usize> = HashSet::new();
    let mut actual = chain0;

    loop {
        match actual {
            Chain::Node(node) => {
                seen.insert(node.id);
            }
            _ => break,
        }

        actual = actual.next();
    }

    // Find first node of chain1 with seen id
    actual = chain1;

    loop {
        match actual {
            Chain::Node(node) => {
                if seen.contains(&node.id) {
                    return Some(node.id);
                }
            }
            _ => break,
        }

        actual = actual.next();
    }

    None
}
