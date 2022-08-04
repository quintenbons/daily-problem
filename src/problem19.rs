#![allow(dead_code)]
//! Medium - Facebook
//! Paint N houses with K colors, no neighbours with
//! the same color, minimal cost

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // 4 colors, 3 houses. costs as u32
        let matrix0 = vec![
            vec![1, 5, 4, 5],
            vec![1, 5, 2, 3],
            vec![5, 5, 1, 3]
        ];

        let matrix1 = vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1]
        ];

        assert_eq!(color_houses(&matrix0), 5); // [0, 3, 2]
        assert_eq!(color_houses(&matrix1), 3); // [0, 1, 2]
    }
}

/// Colors houses and returns an array with indices of
/// the colors to use (in 0..4). Sometimes the most
/// obvious solution is the most efficient it seems
///
/// # Precondition
/// vector is at least 1x2
fn color_houses(vector: &Vec<Vec<u32>>) -> u32 {
    // Invariant: lowest and second represent the lowest
    // costs at a given point of painting houses. That
    // way we can simply choose

    // Initialize costs for the first line
    let (mut lowest, mut second, mut low_index) = if vector[0][0] < vector[0][1]
    {
        (vector[0][0], vector[0][1], 0)
    } else {
        (vector[0][1], vector[0][0], 1)
    };

    for (i, cost) in vector[0].iter().skip(2).cloned().enumerate() {
        if cost < lowest {
            second = lowest;
            lowest = cost;
            low_index = i;
        } else if cost < second {
            second = cost;
        }
    }

    // loop over all houses to make the right decision
    for house in vector.iter().skip(1) {
        // we have to store the best option for the next loop
        let (mut low_yet, mut sec_yet, mut idx_yet) = (None, None, None);

        for (i, house_cost) in house.iter().cloned().enumerate() {
            let cost = if i == low_index {
                second + house_cost
            } else {
                lowest + house_cost
            };

            if low_yet.is_none() || cost < low_yet.unwrap() {
                sec_yet = low_yet;
                low_yet = Some(cost);
                idx_yet = Some(i);
            } else if sec_yet.is_none() || cost < sec_yet.unwrap() {
                sec_yet = Some(cost);
            }
        }

        lowest = low_yet.unwrap();
        second = sec_yet.unwrap();
        low_index = idx_yet.unwrap();
    }

    lowest
}
