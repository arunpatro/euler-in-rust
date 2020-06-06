
// use combinatorics::combinatorial_count;

mod utils;

fn main() {
    println!("Hello, world!");
    // o = problem2();
    // let o = problem10(2000000);
    // let o = problem21(10000);
    // let o = utils::fact(23);
    let o = utils::ncr(23, 10);
    println!("{:?}", o)

}
// 