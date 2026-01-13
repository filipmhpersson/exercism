pub fn square(s: u32) -> u64 {
    if s == 0 {
        panic!("THEY FORCE ME TO DO THIS");
    }
    if s < 2 {
        return s.into();
    }
    (2 as u64).pow(s - 1).into()
}

pub fn total() -> u64 {
    (1..65).map(|n| square(n)).sum()
}
