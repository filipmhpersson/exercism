pub fn nth(n: u32) -> u32 {
    let mut primes = 0;
    let mut prime_cur = 2;
    'primeloop: while primes < n {
        prime_cur += 1;
        for i in 2..prime_cur {
            if prime_cur % i == 0 {
                continue 'primeloop;
            }
        }
        primes += 1;
    }

    prime_cur
}
