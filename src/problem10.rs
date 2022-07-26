#![allow(dead_code)]

/// # Medium - Apple
/// Job scheduling there are lots of ways to implement this
/// This is a great opportunity to implement heaps.
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
        thread_manager();
    }
}

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
fn thread_manager() {
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
