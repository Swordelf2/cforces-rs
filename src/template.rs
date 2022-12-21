use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = Scanner::new();
    let mut out = std::io::BufWriter::new(std::io::stdout());
    let t = scan.n();
    for _ in 0..t {
        writeln!(out, "YES")?;
    }
    Ok(())
}

/* Library */
