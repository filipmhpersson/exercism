pub fn is_armstrong_number(num: u32) -> bool {
    let pow = num.to_string().len() as u32;
    return num
        .to_string()
        .chars()
        .map(|n| n.to_digit(10).unwrap().pow(pow))
        .reduce(|acc, n| acc + n)
        .unwrap()
        == num;
}
