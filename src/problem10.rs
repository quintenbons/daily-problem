#![allow(dead_code)]
//! # Medium - Apple
//! Job scheduling there are lots of ways to implement this
//! This is a great opportunity to implement heaps.
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive() {
        thread_naive();
    }

    #[test]
    fn job_thread() {
        naive_scheduler();
    }

    #[test]
    fn heap_thread() {
        heap_scheduler();
    }
}

struct BinaryHeap(Vec<Job>);

impl BinaryHeap {
    fn new() -> BinaryHeap {
        BinaryHeap(Vec::new())
    }

    fn push(&mut self, job: Job) {
        let vec = &mut self.0;

        vec.push(job);

        // correct heap
        let mut i = vec.len()-1;

        while i != 0 {
            // get parent
            let j = (i - 1) / 2;

            // done
            if vec[i].when >= vec[j].when {
                return;
            }

            // switch
            vec.swap(i, j);
            i = j;
        }
    }

    fn peek(&self) -> Option<&Job> {
        self.0.get(0)
    }

    /// # Panics
    ///
    /// Panics if `BinaryHeap` is empty.
    fn pop(&mut self) -> Job {
        let vec = &mut self.0;

        let res = vec.swap_remove(0);

        // correct heap
        let mut i = 0;

        while i == 0 {
            // get children
            let left = i * 2 + 1;
            let right = i * 2 + 2;

            match (vec.get(left), vec.get(right)) {
                (None, None) => break,
                (Some(job_left), None) => {
                    if job_left.when < vec[i].when {
                        vec.swap(i, left);
                        i = left;
                    } else {
                        break;
                    }
                }
                (None, Some(job_right)) => {
                    if job_right.when < vec[i].when {
                        vec.swap(i, right);
                        i = right;
                    } else {
                        break;
                    }
                }
                (Some(job_left), Some(job_right)) => {
                    // check if we need a switch
                    let inst = vec[i].when;
                    if job_right.when >= inst && job_left.when >= inst {
                        break;
                    }

                    if job_left.when < job_right.when {
                        vec.swap(i, left);
                        i = left;
                    } else {
                        vec.swap(i, right);
                        i = right;
                    }
                }
            }
        }

        res
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
struct Job {
    todo: fn(),
    when: Instant,
}

impl Job {
    fn new(todo: fn(), n: u64) -> Job {
        let when = Instant::now() + Duration::from_millis(n);
        Job { todo, when }
    }
}

/// function that has to be scheduled
fn func1() {
    println!(
        "1 - {:?} The job is now used on thread {:?}",
        Instant::now(),
        thread::current().id()
    );
}

fn func2() {
    println!(
        "2 - {:?} The job is now used on thread {:?}",
        Instant::now(),
        thread::current().id()
    );
}

/// Use one thread per job
fn thread_naive() {
    fn delay_job(f: fn(), n: u64) -> JoinHandle<()> {
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(n));
            f()
        })
    }

    let a = delay_job(func1, 1000);
    let b = delay_job(func2, 500);

    a.join().unwrap();
    b.join().unwrap();
}

/// Use one thread that launches jobs. This is very performance hungry,
/// we could use system clock interuptions instead. This is somewhat
/// similar to JS's async approach, but JS is single threaded.
fn naive_scheduler() {
    let (tx, rx) = mpsc::channel::<Option<Job>>();
    let txx = tx.clone();

    fn manage_jobs(rx: mpsc::Receiver<Option<Job>>) {
        let mut jobs: Vec<Job> = Vec::new();
        let mut terminate = false;

        while !terminate || jobs.len() > 0 {
            // A delay of 30ms is set so your cpu doesn't burn to ashes
            // can be problematic for low time differences
            if let Ok(job) = rx.recv_timeout(Duration::from_millis(10)) {
                match job {
                    Some(job) => jobs.push(job),
                    None => terminate = true,
                }
            }

            let now = Instant::now();

            let mut to_remove: Vec<usize> = Vec::new();
            for (i, job) in jobs.iter().enumerate() {
                // this job is ready to be launched
                if now > job.when {
                    (job.todo)();
                    to_remove.push(i);
                }
            }

            to_remove.sort();
            to_remove.reverse();

            for i in to_remove {
                jobs.remove(i);
            }
        }
    }

    let job_manager = thread::spawn(move || {
        manage_jobs(rx);
    });

    let delay_job = move |f: fn(), n: u64| {
        txx.send(Some(Job::new(f, n)))
            .expect("Send job to side thread");
    };

    let terminate = move || {
        tx.send(None).expect("Sending a terminate signal");
    };

    delay_job(func2, 1030);
    delay_job(func1, 1000);
    delay_job(func2, 900);
    terminate();

    job_manager.join().expect("Waiting for job thread");

    println!("DONE");
}

/// The only difference with the previous solution is the use of
/// a heap, which makes this A lot more efficient
fn heap_scheduler() {
    let (tx, rx) = mpsc::channel::<Option<Job>>();
    let txx = tx.clone();

    fn manage_jobs(rx: mpsc::Receiver<Option<Job>>) {
        let mut jobs = BinaryHeap::new();
        let mut terminate = false;

        while !terminate || jobs.len() > 0 {
            let next_job = jobs.peek();

            let new_job = if let None = next_job {
                // We are not expecting a job, so we can wait
                Some(rx.recv().unwrap())
            } else {
                // We are expecting a job, we can wait only for so long
                let when = next_job.unwrap().when;
                let wait_time = when - Instant::now();

                if let Ok(new_job) = rx.recv_timeout(wait_time) {
                    Some(new_job)
                } else {
                    None
                }
            };

            // We received a message
            if let Some(new_job) = new_job {
                if let Some(new_job) = new_job {
                    // It is a job
                    jobs.push(new_job);
                } else {
                    // It is a terminate signal
                    terminate = true;
                }
                continue;
            }

            // The job must be executed now
            let job = jobs.pop();
            (job.todo)();
        }
    }

    let job_manager = thread::spawn(move || {
        manage_jobs(rx);
    });

    let delay_job = move |f: fn(), n: u64| {
        txx.send(Some(Job::new(f, n)))
            .expect("Send job to side thread");
    };

    let terminate = move || {
        tx.send(None).expect("Sending a terminate signal");
    };

    // now it works flawlesly 212121
    delay_job(func2, 1001);
    delay_job(func1, 1000);
    delay_job(func1, 1002);
    delay_job(func2, 999);
    delay_job(func2, 900);
    thread::sleep(Duration::from_millis(600));
    delay_job(func1, 301); // total will be 901
    terminate();

    job_manager.join().expect("Waiting for job thread");

    println!("DONE");
}
