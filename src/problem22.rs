#![allow(dead_code)]
//! Medium - Microsoft
//! Untangle a sentence without spaces

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

fn untangle<'a>(dictionary: &Vec<String>, sentence: &str) -> Vec<&'a str> {
    unimplemented!();
}
