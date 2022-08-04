#![allow(dead_code)]
//! # Medium - Twitter
//! Autocomplete system, good hashtable exercice
//! could be made more efficient since I use Strings
//! and use String::push_str a lot. I could just use
//! &mut Strings to make it more efficient

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_once() {
        let words = [
            "dog",
            "deer",
            "deal",
            "cat",
            "barbecue",
            "barbatos",
            "bar",
            "silent",
            "siren",
            "xylophone",
            "route",
            "program",
        ];

        let autocomplete = autocomplete_once(&words, "bar");

        for word in ["barbecue", "barbatos", "bar"] {
            assert!(
                autocomplete.contains(&word),
                "One word is missing '{}'",
                word
            );
        }

        for word in [
            "dog",
            "deer",
            "deal",
            "cat",
            "silent",
            "siren",
            "xylophone",
            "route",
            "program",
        ] {
            assert!(!autocomplete.contains(&word), "intruder '{}'", word);
        }
    }

    #[test]
    fn it_works() {
        let words = [
            "dog",
            "deer",
            "deal",
            "cat",
            "barbecue",
            "barbatos",
            "bar",
            "silent",
            "siren",
            "xylophone",
            "route",
            "program",
        ];

        let autocompleter = AC::new(&words);

        let auto = autocompleter.autocomplete("bar");

        for word in ["barbecue", "barbatos", "bar"] {
            assert!(
                auto.contains(&word.to_string()),
                "One word is missing '{}'",
                word
            );
        }

        for word in [
            "dog",
            "deer",
            "deal",
            "cat",
            "silent",
            "siren",
            "xylophone",
            "route",
            "program",
        ] {
            assert!(!auto.contains(&word.to_string()), "intruder '{}'", word);
        }
    }
}

use std::collections::HashMap;

#[derive(Debug)]
struct AC(HashMap<char, AC>, bool);

impl AC {
    fn new(words: &[&str]) -> AC {
        let mut auto = AC(HashMap::new(), false);

        for word in words {
            auto.add(word);
        }

        auto
    }

    fn add(&mut self, word: &str) {
        if word.is_empty() {
            self.1 = true;
            return;
        }

        let hash = &mut self.0;

        let c = word.chars().next().unwrap();

        if let Some(t) = hash.get_mut(&c) {
            t.add(&word[1..]);
        } else {
            hash.insert(c, AC::new(&[&word[1..]]));
        }
    }

    fn suffixes(&self, prefix: &str) -> Vec<String> {
        let hash = &self.0;

        if !prefix.is_empty() {
            let next_char = prefix.chars().next().unwrap();
            let a = hash.get(&next_char);

            match a {
                None => Vec::new(),
                Some(t) => t.suffixes(&prefix[1..]),
            }
        } else {
            let mut res = Vec::new();

            if self.1 {
                res.push("".to_string());
            }

            for (c, t) in hash {
                let mut suf: Vec<String> = t.suffixes(prefix);
                for i in 0..suf.len() {
                    let mut c = c.to_string();
                    c.push_str(&suf[i]);
                    suf[i] = c;
            }

                res.append(&mut suf);
            }

            res
        }
    }

    fn autocomplete(&self, prefix: &str) -> Vec<String> {
        let suffixes = self.suffixes(prefix);

        suffixes
            .iter()
            .map(|string| {
                let mut new_element = prefix.to_string();
                new_element.push_str(&string);
                new_element
            })
            .collect()
    }
}

/// If we are only going to autocomplete once,
/// there is no point in making a more efficient
/// data structure
fn autocomplete_once<'a>(words: &[&'a str], prefix: &str) -> Vec<&'a str> {
    let mut vec: Vec<&str> = Vec::new();

    'outer: for word in words {
        let chars = prefix.chars().zip(word.chars());

        for (c1, c2) in chars {
            if c1 != c2 {
                continue 'outer;
            }
        }

        vec.push(word);
    }

    vec
}
