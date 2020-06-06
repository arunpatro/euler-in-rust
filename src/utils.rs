pub fn fact(num: u64) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => fact(num - 1) * num,
    }
}

pub fn ncr(n:u64, r:u64) -> u64 {
    let x = fact(n) / fact(n-r) * fact(r);
    x
}