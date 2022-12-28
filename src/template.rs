#![warn(clippy::todo, clippy::unimplemented, clippy::dbg_macro)]
#![allow(clippy::mutable_key_type)]

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
    macro_rules! out {
        ( $( $x:expr ),* ) => {
            $(
                write!(out, "{} ", $x).ok();
            )*
            writeln!(out, "").ok();
        };
    }
    #[allow(unused_macros)]
    macro_rules! out_vec {
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
        let mut a = read_vec!(usize; n);
        a.sort();
        out!(a.iter().sum::<usize>());
        out_vec!(a);
    }

    Ok(())
}

/* Library */
