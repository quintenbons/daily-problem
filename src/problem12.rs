#![allow(dead_code)]
/// # Hard - Amazon
/// Unique ways to go up stairs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_works() {
        assert_eq!(combinations_naive(4), 5);
    }

    #[test]
    fn optimal_works() {
        assert_eq!(combinations(4, 2), 5);
        assert_eq!(combinations(4, 3), 7);
        assert_eq!(combinations(4, 4), 8);
        assert_eq!(combinations(4, 10), 8);
        assert_eq!(combinations(3, 10), 4);
    }
}

/// Naive solution
fn combinations_naive(num: u32) -> u32 {
    match num {
        0 | 1 => 1,
        n => combinations_naive(n - 1) + combinations_naive(n - 2),
    }
}

/// Optimal solution. you just have ot notice
/// this is fibonacci's sequence. So the way
/// to go is ascending or descending programmation
/// here to save stack depth in rust, it's better
/// ascending. We could also cache the results using
/// a struct instead
fn combinations(num: u32, max_step: u32) -> u32 {
    let mut past_solutions: Vec<u32> = Vec::with_capacity(num as usize);

    let sub_or_clamp = |i: u32, max_step: u32| {
        (if i < max_step { 0 } else { i - max_step }) as usize
    };

    past_solutions.push(1);
    past_solutions.push(1);

    for i in 2..num {
        let sum: u32 = past_solutions
            .iter()
            .skip(sub_or_clamp(i, max_step))
            .sum();
        past_solutions.push(sum);
    }

    past_solutions.iter().skip(sub_or_clamp(num, max_step)).sum()
}
