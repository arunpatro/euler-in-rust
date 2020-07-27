fn problem1(n: i64) {
    println!("n is {}", n);
    let mut arr = Vec::new();
    let mut s: i64 = 0;

    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0{
            arr.push(i);
            s += i;
        }
    }
    println!("the sum of 3-mulitples is  {}", s);

}

fn problem2() -> i64 {
    let (mut i, mut _i) = (2, 1);
    let mut s = 0;
    while i <= 4000000 {
        if i % 2 == 0 {
            s += i;
        }
        i = i + _i;
        _i = i - _i;
    }
    s
}


// Summation of primes
fn problem10(n: i64) -> i64 {
    // let mut primes = vec![];
    let limit: usize = n as usize;
    let mut sieve = vec![true; limit];
    let mut s = 0;

    for prime in 2..limit {
        if !sieve[prime] {
            continue;
        }
        for multiple in ((prime * prime)..limit).step_by(prime) {
            sieve[multiple] = false;
        }
    }

    for i in 2..limit {
        if sieve[i] {
            s += i as i64
        }
    }

    return s
}



// Amicable Numbers
fn problem21(n: i64) -> i64 {
    let limit = n as usize;
    let mut sum_arr = vec![0; limit];
    let mut out_sum = 0;

    fn proper_divisors(n:i64) -> Vec<i64> {
        let mut d = vec![1];
        if n == 1 {
            return vec![]
        }
        if n < 3 {
            return d
        }
        for i in 2..(n/2 + 1) {
            if n % i == 0 {
                d.push(i);
            }
        }
        d
    }

    for i in 0..limit {
        let d = proper_divisors((i + 1) as i64);
        let sum: i64 = d.iter().sum();
        sum_arr[i] = sum;
    }

    for i in 0..limit {
        let y = sum_arr[i];
        if y == 0 || y == (i + 1) as i64 || y >= limit as i64{
            continue
        }
        let _y = sum_arr[(y - 1) as usize];
        if _y == (i + 1) as i64 {
            out_sum += (i + 1) as i64;
        }
    }
    out_sum
}

pub fn problem25() {
    let n = 1000;
    let i = 1


    // Solve recurrence relation 
    let phi1 = (1 + (5.0).sqrt() ) / 2

    return phi1
    c1 + c2 = 0
    c1 * x1 + c2 * x2 = 1
    c1 * x1 * x1 + c2 * x2 * x2 = 2
}