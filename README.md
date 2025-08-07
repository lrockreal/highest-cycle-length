# Highest "Cycle Length" Algorithm

This is a small algorithm written by me for a friend, to assist in their
analysis of _cycle length_ patterns between numeric bases. It determines the
**highest cycle lengths** for all bases between 2 and a target value
inclusively. For an explanation on what that means, see below.

## Usage

1. Clone the repository:

```sh
$ git clone https://www.github.com/lrockreal/highest-cycle-length.git
```

2. Set the target base and thread count in `src/main.rs`:

```rust
// src/main.rs
const GOAL: usize = 10000; // Calculate HCLs from base 2 to base 10000 inclusive.
```

3. Build and run using [Cargo](https://github.com/rust-lang/cargo):

```sh
$ cargo build --release
$ ./target/release/hcl > output.txt
```

> Note: The above is suggested over `cargo run` because a release build also
> includes optimizations that are excluded from dev builds to reduce build
> times.

## Explanation

In the context of this program and my peer's research the **cycle length** of a
number $i$ in a base $b$ is the number of unique ones-place digits observed in
the sequence:

$a_n = (a_{n-1} * i) \text{ mod } b$

$a_1 = i$

before either a repetition, or $0$, is observed.

For example, let $i = 5$ and $b = 8$. Starting from $a_1$:

$a_1 = 5$

$a_2 = (5 * 5) \text{ mod } 8 = 25 \text{ mod } 8 = 1$

$a_3 = (1 * 5) \text{ mod } 8 = 5 \text{ mod } 8 = 5$

$a_3$ is a repeat of $a_1$, so the cycle length of $5_8$ is $2$.

Using this definition the **highest cycle length** of a base $b$ is the highest
cycle length between all numbers in the interval $[1, b)$, or as a function $h$:

$h(b) = max(\alpha_i)$

Where $\alpha$ is the highest cycle length for a number $i$ in base $b$.
