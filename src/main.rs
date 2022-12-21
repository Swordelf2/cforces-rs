use contest_lib::input::Scanner;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = Scanner::new();
    let t = scan.n();
    for _ in 0..t {
        let n: usize = scan.n();
        let mut a: Vec<i32> = Vec::with_capacity(n);
        for _ in 0..n {
            a.push(scan.n());
        }
        println!("{}", a.iter().max().unwrap() - a.iter().min().unwrap());
    }
    Ok(())
}
