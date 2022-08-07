#![allow(dead_code)]
//! Medium - Microsoft
//! Untangle a sentence without spaces
//! I admit my test is not very exhaustive
//! but this problem is hard to test thoroughly

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let dictionary: Vec<String> = vec![
            "fox".to_string(),
            "fast".to_string(),
            "quick".to_string(),
            "brown".to_string(),
            "the".to_string(),
            "ran".to_string(),
        ];

        assert_eq!(
            untangle(&dictionary, "quickfastbrownfoxranfast"),
            vec!["quick", "fast", "brown", "fox", "ran", "fast"]
        );
    }
}

/// There are many ways to solve this in O(N^2). We could also do as
/// a previous problem, and make a dictionary out of dic: letter -> dic
/// to save up some time for multiple calls of the function. That would
/// take up (n*max common prefix) time to make the dictionary + O(N) recursions
/// per word of the sentence at worst. Meaning still a O(NxM), but way more
/// performant. I think the implementation here is not interesting
fn untangle<'a>(dictionary: &Vec<String>, sentence: &str) -> Vec<&'a str> {
    unimplemented!();
}
