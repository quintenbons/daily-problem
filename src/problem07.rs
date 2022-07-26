/// # Medium - Facebook
/// Decoding a message from integers to
/// the english alphabet

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(count_valid("1"), 1, "1"); // a
        assert_eq!(count_valid("11"), 2, "11"); // aa | k
        assert_eq!(count_valid("111"), 3, "111"); // aaa | ak | ka
        assert_eq!(count_valid("132"), 2, "132"); // acb | mc
    }
}

/// # Recursive
/// Gives the number of different valid interpretations
/// of the coded message
fn count_valid(input: &str) -> u32 {
    if input.len() <= 1 {
        return 1;
    }

    // 1 char + the rest
    let mut count = count_valid(&input[1..]);

    // or 2 chars + the rest if possible
    let i: u8 = input[..2]
        .parse()
        .expect("Could not parse string as unsigned");

    if i <= 26 {
        // useless to know the char but this is how we'd
        // do it
        let _new_char = (i + 96) as char;
        count += count_valid(&input[2..]);
    }

    count
}
