
# Overview

This repo is for writing contests / solving problems **in Rust**
on coding contest sites like Codeforces, AtCoder etc.


# Basic usage

## Preparation

1. [Install Rust](https://www.rust-lang.org/tools/install).
2. Install [just](https://github.com/casey/just) with `cargo install just`.
3. (Opt.) Add `~/.cargo/bin/` to your `PATH`.
4. Clone this repo.

## Solving problems / Competing in contests

1. Write your solution in `main.rs`.
2. (Opt.) Use lib functions or data structures from `contest-lib` by copying and pasting.
3. Test your solution (TODO).
4. Upload it.
5. `just re <problem_id>` and go back to 1.

# More details

`just --list` or see doc comments in the [justfile](./justfile) for more details

# Future work

* Support Leetcode.
* Add `just restore A` - restores solution `A` from old solutions.
* Expand `contest-lib` and tests -
check out [Rust algorithms](https://github.com/EbTech/rust-algorithms) by EbTech.
* Automate testing solutions.
* Automate bundling lib functions and data structures into a single `.rs` file.
