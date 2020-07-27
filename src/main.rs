// use combinatorics::combinatorial_count;
fn problem25(){
    // let n = 20;
    // let (mut i, mut o, mut min, mut max) = (1, 1, 1, 1500);
}

fn fib_digits(n: u64) -> f64 {
    let phi1: f64 = (1. + (5 as f64).sqrt() ) / 2.;
    // let phi2: f64 = (1. - (5 as f64).sqrt() ) / 2.;
    let o = phi1.powi(n as i32) / (5 as f64).sqrt();
    let digits = -(5 as f64).sqrt().log10() + (n as f64) * phi1.log10() + 1.;
    digits
}

fn fib(n: u64) -> f64 {
    let phi1: f128 = (1. + (5 as f64).sqrt() ) / 2.;
    let phi2: f128 = (1. - (5 as f64).sqrt() ) / 2.;
    println!("{} {}", phi1.powi(n as i32), phi2.powi(n as i32));
    let o = (phi1.powi(n as i32) - phi2.powi(n as i32)) / (5 as f64).sqrt();
    o
}
// mod utils;

fn main() {
    println!("Hello, world!");
    // problem25();
    let x = fib(1500).log10() as u64 + 1;
    // let x = fib_digits(4782);
    println!("{:?}", x)

}
// 