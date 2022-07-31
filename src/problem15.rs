#![allow(dead_code)]
/// # Medium - Facebook
/// random selection in a stream

#[cfg(test)]
mod tests {
    use super::*;

    fn sd(f: fn(&mut dyn Iterator<Item = f32>) -> Option<f32>) {
        const NUMBER_TRIES: usize = 30000;
        let real_mean = 5.;
        let mut mean = 0.;
        let mut sd = 0.;

        for _ in 0..NUMBER_TRIES {
            let vec: Vec<f32> = vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];

            let x = f(&mut vec.into_iter()).unwrap();
            sd += (x - real_mean).powi(2);
            mean += x;
        }

        mean /= NUMBER_TRIES as f32;
        sd /= NUMBER_TRIES as f32;

        println!("real mean: {}", real_mean);
        println!("mean: {}", mean);
        println!("sd: {}", sd);
    }

    #[test]
    fn first_test() {
        sd(first_idea)
    }
}

use rand::Rng;

/// first solution is mathematical, and only valid
/// on a finite stream
fn first_idea<T>(iter: &mut dyn Iterator<Item = T>) -> Option<T> {
    let mut selected: Option<T> = None;
    let mut rng = rand::thread_rng();

    for (i, e) in iter.enumerate() {
        let should_select: bool = rng.gen_range(0usize..(i + 1)) == 0;

        selected = if should_select { Some(e) } else { selected };
    }

    selected
}

// I can't find a solution for an infinite stream. I think there is no
// solution since every element has a nul probability of being picked
