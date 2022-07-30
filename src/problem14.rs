#![allow(dead_code)]
/// # Medium - Google
/// Monte Carlo method

#[cfg(test)]
mod tests {
    use super::*;

    /// This test could fail if RAND_COUNT is too low
    #[test]
    fn it_works() {
        const THRESHOLD: f32 = 0.005;
        for _ in 0..10 {
            let res = monte_carlo(RAND_COUNT);
            assert!((res - 3.1415).abs() < THRESHOLD, "Found {}", res);
        }
    }

    #[test]
    fn multithread_test() {
        const THRESHOLD: f32 = 0.005;
        for _ in 0..10 {
            let res = parallel_approx(RAND_COUNT, 4);
            assert!((res - 3.1415).abs() < THRESHOLD, "Found {}", res);
        }
    }
}

use rand::Rng;
use std::sync::mpsc;
use std::thread;

const RAND_COUNT: u32 = 1000000;

/// Monte Carlo method for pi approximation
fn monte_carlo(rand_count: u32) -> f32 {
    let mut count: u32 = 0;

    let mut rng = rand::thread_rng();

    for _ in 0..rand_count {
        let x: f32 = rng.gen_range((-1.)..1.);
        let y: f32 = rng.gen_range((-1.)..1.);

        if x.powi(2) + y.powi(2) < 1. {
            count += 1;
        }
    }

    count as f32 / rand_count as f32 * 4.
}

/// Parallelised monte carlo
fn parallel_approx(rand_count: u32, thread_count: u8) -> f32 {
    let mut receivers = Vec::with_capacity(rand_count as usize);

    // spawn all threads and set up communication
    {
        let rand_count = rand_count / thread_count as u32;

        for _ in 0..thread_count {
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                tx.send(monte_carlo(rand_count))
                    .expect("Thread communication error");
            });

            receivers.push(rx);
        }
    }

    // Litsen to outputs. Whenever
    let mut sum = 0.;
    for rx in receivers {
        sum += rx.recv().expect("Thread communication error");
    }

    sum / thread_count as f32
}
