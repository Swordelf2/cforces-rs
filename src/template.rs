#![warn(clippy::todo, clippy::unimplemented, clippy::dbg_macro)]
#![allow(clippy::mutable_key_type, clippy::needless_range_loop)]
#![allow(clippy::bool_comparison)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* Input / output */
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let mut scan = Scanner::new(stdin.lock());
    let mut out = std::io::BufWriter::new(stdout.lock());
    macro_rules! read {
        ( $t:ty ) => {
            scan.next::<$t>()
        };
    }
    #[allow(unused_macros)]
    macro_rules! read_vec {
        ( $t: ty; $n: expr ) => {
            (0..$n).map(|_| read!($t)).collect::<Vec<$t>>()
        };
        ( $t: ty) => {
            (0..scan.next::<usize>())
                .map(|_| read!($t))
                .collect::<Vec<$t>>()
        };
    }
    use std::io::Write;
    macro_rules! print  {
        ( $( $x:expr ),* ) => {
            $(
                write!(out, "{} ", $x).ok();
            )*
            writeln!(out, "").ok();
        };
    }
    #[allow(unused_macros)]
    macro_rules! print_vec {
        ( $( $v:expr ),* ) => {
            $(
                for elem in &$v {
                    write!(out, "{} ", elem).ok();
                }
                writeln!(out, "").ok();
            )*
        };
    }

    /* Solution */
    let t = read!(usize);
    for _ in 0..t {
        let n = read!(usize);
        let a = read_vec!(u64; n);
    }
    Ok(())
}

/* Library stuff */

pub trait OptionExt<T> {
    fn update_min(min_var: &mut Self, new_value: T);
    fn update_max(max_var: &mut Self, new_value: T);
}

impl<T: Ord> OptionExt<T> for Option<T> {
    fn update_min(min_var: &mut Self, new_value: T) {
        match min_var {
            None => *min_var = Some(new_value),
            Some(old_value) if new_value < *old_value => *min_var = Some(new_value),
            _ => (),
        }
    }

    fn update_max(max_var: &mut Self, new_value: T) {
        match max_var {
            None => *max_var = Some(new_value),
            Some(old_value) if new_value > *old_value => *max_var = Some(new_value),
            _ => (),
        }
    }
}

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
    #[must_use]
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
