pub fn collatz(n: u64) -> Option<u64> {
    let mut i = 0;
    let mut res = n;
    loop {
        if res == 1 {
            return Some(i);
        } else if res == 0 {
            return None;
        } else if res % 2 == 0 {
            res = res / 2;
        } else {
            res = (res * 3) + 1
        }
        i += 1;
    }
}
