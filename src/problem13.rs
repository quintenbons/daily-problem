#![allow(dead_code)]
//! # Hard - Amazon
//! largest substring with max number of differing
//! characters

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let string0 = "abcba";
        let string1 = "aaabraaaaaa";
        assert_eq!(largest_substring(string0, 1), 1);
        assert_eq!(largest_substring(string0, 2), 3);
        assert_eq!(largest_substring(string0, 3), 5);
        assert_eq!(largest_substring(string0, 2), 3);

        assert_eq!(largest_substring(string1, 1), 6);
        assert_eq!(largest_substring(string1, 2), 7);
        assert_eq!(largest_substring(string1, 3), 11);
    }
}

use std::collections::HashMap;

fn largest_substring(string: &str, k: usize) -> usize {
    let mut hashmap: HashMap<char, usize> = HashMap::new();

    let mut start = 0;
    let mut biggest = 0;

    for (i, c) in string.chars().enumerate() {
        hashmap.insert(c, i);
        let length = i - start;

        if hashmap.len() <= k {
            // we can take this character
            biggest = usize::max(biggest, i - start + 1);
        } else {
            // we can't take this character
            biggest = usize::max(biggest, i - start);

            // SOOOO FRUSTRATING: I access c twice
            let (&c, &index) = hashmap.iter().min_by_key(|(_, k)| *k).unwrap();
            hashmap.remove(&c);
            start = index + 1;
        }
    }

    biggest
}
