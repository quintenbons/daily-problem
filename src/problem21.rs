#![allow(dead_code)]
//! Easy - Snapchat
//! Room management in a school

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lectures0 = vec![(30, 75), (0, 50), (60, 150)];
        let lectures1 =
            vec![(30, 75), (0, 50), (60, 150), (50, 60), (80, 130), (90, 130)];

        assert_eq!(min_rooms(&lectures0), 2);
        assert_eq!(min_rooms(&lectures1), 3);
    }
}

/// From a vector of lecture start and stop points
/// find the minimum amount of rooms needed
fn min_rooms(lectures: &Vec<(u32, u32)>) -> u32 {
    unimplemented!();
}
