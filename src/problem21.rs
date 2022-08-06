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
/// find the minimum amount of rooms needed. I could
/// not find better than O(n * log(n))...
fn min_rooms(lectures: &Vec<(u32, u32)>) -> u32 {
    let mut starts = Vec::with_capacity(lectures.len());
    let mut stops = Vec::with_capacity(lectures.len());

    for (start, stop) in lectures {
        starts.push(*start);
        stops.push(*stop);
    }

    starts.sort();
    stops.sort();

    let mut overlaping = 0;
    let mut highest = 0;

    let mut start_idx = 0;
    let mut stop_idx = 0;

    while start_idx < starts.len() && stop_idx < stops.len() {
        if starts[start_idx] < stops[stop_idx] {
            start_idx += 1;
            overlaping += 1;
            highest = overlaping.max(highest);
        } else {
            stop_idx += 1;
            overlaping -= 1;
        }
    }

    highest
}
