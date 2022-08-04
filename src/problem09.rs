//! # Hard - Airbnb
//! Largest sum of non-adjacent numbers

#[cfg(test)]
mod tests {
    use super::*;

    fn general_test(f: fn(&[i32]) -> i32) {
        let vector1: Vec<i32> = vec![2, 4, 6, 2, 5];
        let vector2: Vec<i32> = vec![5, 1, 1, 5];

        assert_eq!(f(&vector1), 13, "largest for {:?}", vector1);
        assert_eq!(f(&vector2), 10, "largest for {:?}", vector2);
    }

    #[test]
    fn it_works() {
        general_test(largest_sum);
    }

    #[test]
    fn brute_works() {
        general_test(brute_sum);
    }
}

/// Gives the largest sum of non adjacent numbers
/// from a vector. It is easy to modify this function
fn largest_sum(vector: &[i32]) -> i32 {
    let mut biggest = 0; // this one is best for [..i+1]
    let mut no_last = 0; // this one is best for [..i]

    for num in vector {
        let temp = biggest;

        // the best for i+2 is either:
        // (we take num) [the biggest for i] + num
        // (we don't take num) the biggets for i+1
        biggest = biggest.max(no_last + num);
        no_last = temp;
    }

    biggest
}

/// # Recursive
/// Naive alternative
fn brute_sum(vector: &[i32]) -> i32 {
    // if len is 1 or 0
    if vector.len() == 0 {
        return 0;
    } else if vector.len() == 1 {
        return vector[0];
    }

    // either we take or we don't
    let take_it = vector[0] + brute_sum(&vector[2..]);
    let dont = brute_sum(&vector[1..]);
    take_it.max(dont)
}
