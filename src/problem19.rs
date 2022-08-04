#![allow(dead_code)]
/// Medium - Facebook
/// Paint N houses with K colors, no neighbours with
/// the same color, minimal cost

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // 4 colors, 3 houses. costs as u32
        let a = vec![
            vec![1, 3, 4, 5],
            vec![1, 5, 2, 3],
            vec![5, 5, 1, 3],
        ];

        assert_eq!(color_houses(&a), vec![0, 3, 4]);
    }
}

/// colors houses and returns an array with indices of
/// the colors to use (in 0..4)
fn color_houses(_vector: &Vec<Vec<u32>>) -> Vec<usize> {
    unimplemented!();
}
