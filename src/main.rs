use primes::is_prime;
use rayon::prelude::*;

const GOAL: usize = 10000;

fn main() {
    let hcls: Vec<u64> = (2..=GOAL as u64)
        .into_par_iter()
        .map(highest_cyclength)
        .collect();

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
/// Starting from `a_1` until a repetition is found.
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
