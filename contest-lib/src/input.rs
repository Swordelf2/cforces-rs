use std::io::BufRead;

pub struct Scanner {
    buffer: Vec<String>,
    stdin_lock: std::io::StdinLock<'static>,
}

impl Scanner {
    /// Locks `stdin` and returns a new Scanner
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        return Scanner {
            buffer: Vec::new(),
            stdin_lock: std::io::stdin().lock(),
        };
    }

    /// Parses next input from stdin up to whitespace into the given type
    pub fn n<T: std::str::FromStr>(&mut self) -> T {
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
