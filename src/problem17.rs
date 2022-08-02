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

    #[test]
    #[should_panic]
    fn it_is_safe() {
        let wrong_fs = "dir\n\t\tfile.txt";

        get_longest(wrong_fs);
    }
}

use std::cmp::Ordering;
use std::iter::Peekable;

enum Element {
    File { size: usize },
    Dir { size: usize, children: Vec<Element> },
}

/// Gives the length of the longest path of the filesystem
fn get_longest(fs: &str) -> usize {
    let root = parse_fs(fs);

    match root {
        Some(elm) => longest_sum(&elm),
        None => 0,
    }
}

/// Parses the fs to an Element, making it easier to
/// navigate, since we don't care about the strings
fn parse_fs(fs: &str) -> Option<Element> {
    fn to_element(path: &str) -> Element {
        let real_name = path.trim_start_matches('\t');
        if real_name.contains('.') {
            Element::File {
                size: real_name.len(),
            }
        } else {
            Element::Dir {
                size: real_name.len(),
                children: Vec::new(),
            }
        }
    }

    fn aux<'a, I>(
        iter: &mut Peekable<I>,
        current_depth: usize,
        siblings: &mut Vec<Element>,
    ) where
        I: Iterator<Item = &'a str>,
    {
        loop {
            // stop at the end of filesystem
            let next = match iter.peek() {
                Some(n) => n,
                None => return,
            };

            let depth = next.chars().take_while(|c| *c == '\t').count();

            match depth.cmp(&current_depth) {
                Ordering::Greater => panic!("Invalid filesystem"),

                // this dir is done
                Ordering::Less => return,

                // another sibling
                Ordering::Equal => {
                    let mut element = to_element(next);
                    iter.next();

                    if let Element::Dir { children, .. } = &mut element {
                        aux(iter, current_depth + 1, children);
                    }

                    siblings.push(element);
                }
            }
        }
    }

    if fs.is_empty() {
        return None;
    }

    let mut iter = fs.split('\n').peekable();
    let mut res = to_element(iter.next().unwrap());

    match &mut res {
        Element::File { .. } => Some(res),
        Element::Dir { children, .. } => {
            aux(&mut iter, 1, children);
            Some(res)
        }
    }
}

/// Looks for the longest paths from an Element
fn longest_sum(elm: &Element) -> usize {
    match elm {
        Element::File { size } => *size,
        Element::Dir { size, children } => {
            size + 1 + children
                .iter()
                .map(|elm| longest_sum(elm))
                .max()
                .unwrap_or(0)
        }
    }
}
