//! Hard - Stripe
//! Finding lowest positive integer missing from vector
//!
//! # Complexity goal
//! Time: O(N)
//! Space: O(1)

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn naive_test() {
        let ex1 = vec![3, 4, -1, 1];
        let ex2 = vec![1, 2, 0];
        let ex3: Vec<i32> = (0..40).map(|_| thread_rng().gen_range(0..300)).collect();

        assert_eq!(naive_missing(&ex1), 2);
        assert_eq!(naive_missing(&ex2), 3);

        let lowest = naive_missing(&ex3);

        for i in 1..lowest {
            assert!(ex3.contains(&i), "{} was not in ex3", i);
        }

        assert!(!ex3.contains(&lowest));
    }

    #[test]
    fn optimal_test() {
        let mut ex1 = vec![3, 4, -1, 1];
        let mut ex2 = vec![1, 2, 0];
        let mut ex3 = (0..40)
            .map(|_| thread_rng().gen_range(0..300))
            .collect::<std::collections::HashSet<i32>>()
            .iter()
            .map(|i| *i)
            .collect();

        assert_eq!(lowest_missing(&mut ex1), naive_missing(&ex1));
        assert_eq!(lowest_missing(&mut ex2), naive_missing(&ex2));
        assert_eq!(lowest_missing(&mut ex3), naive_missing(&ex3));
    }
}

/// Checks lowest missing strictly positive integer. Naive
///
/// # Complexity
/// Time: O(N^2)
/// Space: O(1)
fn naive_missing(vec: &Vec<i32>) -> i32 {
    let mut lowest = 1;

    loop {
        for num in vec {
            if *num == lowest {
                lowest += 1;
                continue;
            }
        }

        break lowest;
    }
}

/// Checks lowest missing strictly positive integer
/// Modifies the array in place. Only works if the
/// Array is deduped
///
/// # Complexity
/// Time: O(N)
/// Space: O(1)
fn lowest_missing(vec: &mut Vec<i32>) -> i32 {
    for i in 0..vec.len() {
        // println!("{} -> {:?}", i, vec);
        swap_it(vec, i);
    }

    for i in 0..vec.len() {
        let val = vec[i];
        let expected = i as i32 + 1;

        if expected != val {
            return expected;
        }
    }

    (vec.len() + 1) as i32
}

fn swap_it(vec: &mut Vec<i32>, i: usize) {
    let val = vec[i];

    if val > 0 && val <= vec.len() as i32 {
        let new_position = val as usize - 1;

        if i != new_position {
            vec.swap(new_position, i);
            swap_it(vec, i);
        }
    }
}
