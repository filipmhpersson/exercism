/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut is_odd = true;

    let mut digits = 0;
    let validation = code.chars().into_iter().try_for_each(|c| {
        return if !c.is_digit(10) && !c.is_whitespace() {
            Err(())
        } else {
            if c.is_digit(10) {
                digits += 1;
            }
            Ok(())
        };
    });

    if validation.is_err() || digits <= 1 {
        return false;
    }

    let digits = code
        .chars()
        .into_iter()
        .filter(|c| c.is_digit(10))
        .rev()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .reduce(|acc, e| {
            is_odd = !is_odd;
            if is_odd {
                return acc + e;
            } else {
                let res = e * 2;
                if res > 9 {
                    return res - 9 + acc;
                } else {
                    return res + acc;
                }
            };
        })
        .unwrap();

    digits % 10 == 0
}

#[test]
fn default_input() {
    assert!(is_valid("4539 3195 0343 6467"));
}
