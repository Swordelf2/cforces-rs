#![warn(clippy::todo, clippy::unimplemented, clippy::dbg_macro)]
#![allow(clippy::mutable_key_type, clippy::needless_range_loop)]

// Main solution
impl Solution {
    #[allow(dead_code)]
    fn hello() {}
}

/* Leetcode-specific */

#[allow(unused)]
struct Solution;

#[allow(clippy::bool_assert_comparison)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tests
    Ok(())
}
