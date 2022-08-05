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
        unimplemented!();
    }

    fn new_node(id: usize, next: Self) -> Self {
        unimplemented!();
    }

    fn from_vec(vector: &Vec<usize>) -> Self {
        unimplemented!();
    }
}

/// Gives the common node of both chains. Expects
/// there is only one, but gives the on closest to
/// chain1's head if there are multiple
fn find_link(chain0: &Chain, chain1: &Chain) -> Option<usize> {
    unimplemented!();
}
