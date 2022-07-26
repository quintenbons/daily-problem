/// # Easy - Google
/// Target sum of two numbers of an array

#[cfg(test)]
mod tests {
    use super::*;

    fn general_test(f: fn(i32, &[i32]) -> bool) {
        assert!(f(17, &[10, 15, 3, 7]));
        assert!(!f(170, &[10, 15, 3, 7]));
        assert!(f(35, &[10, 15, 3, 7, 9, 32]));
        assert!(!f(36, &[10, 15, 3, 7]));

    }

    #[test]
    fn naive_test() {
        general_test(naive_sums_up);
    }

    #[test]
    fn optimal_test() {
        general_test(sums_up);
    }
}

use std::collections::HashSet;

/// # Recursive
/// Checks if any sum of elements of v (once per element)
/// sums up to k. Naive approach in N^2. This checks any
/// combination of numbers, so it does more than the exercice
/// tells us to
fn naive_sums_up(k: i32, v: &[i32]) -> bool {
    if v.len() == 0 {
        return k == 0;
    }

    let new = &v[1..];

    naive_sums_up(k, new) || naive_sums_up(k - v[0], new)
}

/// Checks if any sum of elements of v (once per element)
/// sums up to k. Optimal aproach in N
fn sums_up(k: i32, v: &[i32]) -> bool {
    let mut seen: HashSet<i32> = HashSet::new();

    for num in v {
        if seen.contains(&(k - *num)) {
            return true;
        }

        seen.insert(*num);
    }

    false
}
