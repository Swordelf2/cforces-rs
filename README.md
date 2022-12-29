
# Overview

**cforces-rs** is a template project for writing contests and solving problems **in Rust**
on competitive programming contest websites.

The project mainly uses [just](https://github.com/casey/just) - simpler alternative to `make`.

Main features include:
* Input/output in Rust - see `src/template.rs`.
* Common algorithms and data structures - see `src/lib/`.
* Creating new source files and text input/output files - see `just init`.
* Automatic backup of old solutions - see `just save|reinit|restore`.
* Testing solutions - see `just run`.

# Supported sites

* [Codeforces](https://codeforces.com) - `cforces`
* [AtCoder](https://atcoder.jp) - `atcoder`
* [Leetcode](https://leetcode.com) - `leetcode`
* Other - recommended to use `cforces` configuration

# Basic usage

## Preparation

1. [Install Rust](https://www.rust-lang.org/tools/install).
2. Install [just](https://github.com/casey/just) with `cargo install just`.
3. (Opt.) Add `~/.cargo/bin/` to your `PATH`.
4. Clone this repo.
5. `just config <site>`. See [supported \<site\>s](#supported-sites).

## Solving problems / Competing in contests

1. `just init`.
2. Write your solution in `main.rs`.
3. (Opt) Use lib functions or data structures from `src/lib/` by copying and pasting.
4. (Opt) Paste tests in `input/*.in` and `input/*.out`, test with `just run`.
5. Upload your solution.
6. `just re <problem_id>` and go back to 2.

# More details

`just --list` or see doc comments in the [justfile](./justfile) for more details

# Future work

* Expand `src/lib/` -
check out [Rust algorithms](https://github.com/EbTech/rust-algorithms) by EbTech.
* Fully automate testing of solutions, including Leetcode
