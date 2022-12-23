fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Input / output
    let mut scan = Scanner::new(std::io::stdin().lock());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    macro_rules! read {
        ( $t:ty ) => {
            scan.next::<$t>()
        };
    }
    macro_rules! read_vec {
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
                write!(out, "{} ", $x)?;
            )*
            writeln!(out, "")?
        };
    }

    // Solution
    let t = read!(i32);
    for _ in 0..t {
        let a = read_vec!(i32);
        out!(a.iter().min().unwrap(), a.iter().max().unwrap());
    }

    Ok(())
}

/* Library */
