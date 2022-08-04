#![allow(dead_code)]
//! Hard - Google
//! Maximum value of subarray
//! suboptimal solution, because it's still O(n * k) in worst
//! case scenario. Note: even online I couldn't find an optimal
//! solution even if people claim this one to be optimal.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let my_array = vec![10, 5, 2, 7, 8, 7];
        let k = 3;

        assert_eq!(max_sub(my_array, k), vec![10, 7, 8, 8]);
    }

    #[test]
    fn naive_works() {
        let my_array = vec![10, 5, 2, 7, 8, 7];
        let k = 3;

        assert_eq!(brute_solution(my_array, k), vec![10, 7, 8, 8]);
    }
}

/// Brute force O(n * k)
fn brute_solution(vector: Vec<i32>, k: usize) -> Vec<i32> {
    let mut maxes = Vec::with_capacity(k);

    for i in 0..(vector.len() - k + 1) {
        let maxi = vector[i..(i + k)].iter().max().unwrap().clone();
        maxes.push(maxi);
    }

    maxes
}

struct Queue {
    data: Vec<Option<usize>>,
    stop: usize,
    start: usize,
}

impl Queue {
    fn new(size: usize) -> Queue {
        Queue {
            data: (0..size).map(|_| None).collect(),
            stop: 0,
            start: 0,
        }
    }

    fn push(&mut self, value: usize) {
        if self.stop == self.start && self.data[self.start].is_some() {
            self.start += 1;
            self.start %= self.data.len();
        }

        self.data[self.stop] = Some(value);

        self.stop += 1;
        self.stop %= self.data.len();
    }

    fn peek(&self) -> Option<usize> {
        self.data[self.start]
    }

    fn peek_latest(&self) -> Option<usize> {
        let index = self.stop + self.data.len() - 1;
        let index = index % self.data.len();

        self.data[index]
    }

    fn shift(&mut self) {
        // if stop > start (or that stop is one loop ahead)
        if self.start != self.stop || self.data[self.start].is_some() {
            self.data[self.start] = None;
            self.start += 1;
            self.start %= self.data.len();
        }
    }

    fn pop(&mut self) {
        // if stop > start (or that stop is one loop ahead)
        if self.start != self.stop || self.data[self.start].is_some() {
            self.stop += self.data.len() - 1;
            self.stop %= self.data.len();
            self.data[self.stop] = None;
        }
    }
}

/// Gives the array of max values of subarrays.
/// The result is of length array.len() + 1 - k
/// # Precondition
/// 0 < k <= vector.len()
fn max_sub(vector: Vec<i32>, k: usize) -> Vec<i32> {
    if k == 0 || k > vector.len() {
        panic!("Preconditions were not met");
    }

    // Queue invariant: first index is a local maximum for
    // the next k elements. Queue is not empty before or
    // after a correct
    let mut queue = Queue::new(k);
    let mut res: Vec<i32> = Vec::with_capacity(k);

    let correct = |queue: &mut Queue, i: usize| {
        let j = queue.peek().unwrap();
        if i - j >= k {
            queue.shift();
        }

        loop {
            match queue.peek_latest() {
                Some(latest) => {
                    let latest_val = vector[latest];
                    if latest_val > vector[i] { break };
                    queue.pop();
                },
                None => break,
            }
        }

        queue.push(i);
    };

    // initialize queue first k elements
    queue.push(0);

    for i in 1..k {
        correct(&mut queue, i);
    }

    // fill res
    let max_index = queue.peek().unwrap();
    let maxi = vector[max_index];
    res.push(maxi);

    for i in k..vector.len() {
        correct(&mut queue, i);
        let max_index = queue.peek().unwrap();
        let maxi = vector[max_index];
        res.push(maxi);
    }

    res
}
