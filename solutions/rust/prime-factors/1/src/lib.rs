use std::ops;
pub fn factors(n: u64) -> Vec<u64> {
    let mut a = n;
    let mut factor = 2;
    let mut result: Vec<u64> = Vec::new();

    while a > 1 {
        let nxt = a / factor;
        if nxt * factor == a {
            result.push(factor);
            a = nxt;
        } else {
            factor += 1;
        }
    }
    result
}
