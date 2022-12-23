/* Input */

use std::io::BufRead;
use std::io::StdinLock;

pub struct Scanner<'a> {
    buffer: Vec<String>,
    stdin_lock: std::io::StdinLock<'a>,
}

impl<'a> Scanner<'a> {
    /// Locks `stdin` and returns a new Scanner
    #[allow(clippy::new_without_default)]
    pub fn new(stdin_lock: StdinLock<'a>) -> Self {
        Self {
            buffer: Vec::new(),
            stdin_lock,
        }
    }

    /// Parses next input from stdin up to whitespace into the given type
    #[allow(clippy::should_implement_trait)]
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.stdin_lock.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}
