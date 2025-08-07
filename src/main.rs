use std::thread::{self};

use primes::is_prime;

const GOAL: usize = 10000;
const THREADS: usize = 32;

fn main() {
    let mut hcls = [0; GOAL - 1];
    let mut threads = vec![];

    for thread_idx in 0..THREADS {
        let thread = thread::spawn(move || {
            let mut thread_hcls = [0; (GOAL - 1).div_ceil(THREADS)];

            let mut b_idx = 0;
            for b in ((thread_idx as u64 + 2)..=(GOAL as u64)).step_by(THREADS) {
                thread_hcls[b_idx] = highest_cyclength(b);
                if b % 100 == 0 {
                    eprintln!("{b}");
                }
                b_idx += 1;
            }

            eprintln!("Worker {thread_idx} finished");

            thread_hcls
        });

        threads.push(thread);
    }

    for (thread_idx, thread) in threads.into_iter().enumerate() {
        let thread_hcls = thread.join().unwrap();
        for i in 0..((GOAL - 2 - thread_idx) / THREADS + 1) {
            hcls[(i * THREADS) + thread_idx] = thread_hcls[i];
        }
    }

    print!("[");
    for b in 0..(GOAL - 1) {
        let hcl_b = hcls[b];
        if b == GOAL - 2 {
            println!("{hcl_b}]");
        } else {
            print!("{hcl_b}, ");
        }
    }
}

/// Returns the highest cycle length among all numbers in base `b`.
fn highest_cyclength(b: u64) -> u64 {
    if is_prime(b) {
        b - 1
    } else {
        let mut hcl_b = 1;
        for n in 2..b {
            let cl_n_base_b = cyclength(n, b);
            if cl_n_base_b > hcl_b {
                hcl_b = cl_n_base_b;
            }
        }
        hcl_b
    }
}

/// Returns the cycle length of `n` base `b`
/// by calculating terms in the sequence:
/// `a_n = (a_{n-1} * i) mod b`
/// `a_1 = i`
/// Starting from `a_1` a repetition is found.
fn cyclength(n: u64, b: u64) -> u64 {
    let mut terms_seen = [false; GOAL];
    let mut term_idx = 1;
    let mut term = n;
    terms_seen[term as usize] = true;
    loop {
        term = (term * n) % b;
        term_idx += 1;
        if terms_seen[term as usize] {
            break;
        }
        terms_seen[term as usize] = true;
    }
    term_idx - 1
}
