#![allow(dead_code)]
/// Hard - Google
/// FS longest path

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let filesystem = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";

        assert_eq!(get_longest(&filesystem), 32);
        assert_eq!(get_longest(""), 0);
    }
}

use std::iter::Peekable;

enum Element {
    File { size: u32 },
    Dir { size: u32, children: Vec<Element> },
}

/// Gives the length of the longest path of the filesystem
fn get_longest(fs: &str) -> u32 {
    let root = parse_fs(fs);

    match root {
        Some(elm) => longest_sum(&elm),
        None => 0,
    }
}

/// Parses the fs to an Element, making it easier to
/// navigate, since we don't care about the strings
fn parse_fs(fs: &str) -> Option<Element> {
    unimplemented!();
}

/// Looks for the longest paths from an Element
fn longest_sum(elm: &Element) -> u32 {
    match elm {
        Element::File { size } => *size,
        Element::Dir { size, children } => {
            size + children
                .iter()
                .map(|elm| longest_sum(elm))
                .max()
                .unwrap_or(0)
        }
    }
}
