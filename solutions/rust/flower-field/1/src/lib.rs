use std::cmp::max;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    if garden.len() == 0 {
        return out;
    }
    for i in 0..garden.len() {
        let current_row = garden[i].as_bytes();
        let previous_row = if i > 0 { Some(garden[i - 1]) } else { None };
        let next_row = if i < garden.len() - 1 {
            Some(garden[i + 1])
        } else {
            None
        };
        let mut out_str = String::with_capacity(current_row.len());
        for j in 0..current_row.len() {
            if current_row[j] == b'*' {
                out_str.insert(j, '*');
                continue;
            }
            let mut flower = 0;

            let start = if j > 0 { j - 1 } else { j };
            let end = if j < current_row.len() - 1 { j + 1 } else { j };
            if let Some(prev) = previous_row {
                flower += prev.as_bytes()[start..end + 1]
                    .iter()
                    .filter(|c| **c == b'*')
                    .count();
            }

            if start != j {
                if current_row[start] == b'*' {
                    flower += 1;
                }
            }

            if end != j {
                if current_row[end] == b'*' {
                    flower += 1;
                }
            }

            if let Some(next) = next_row {
                flower += next.as_bytes()[start..end + 1]
                    .iter()
                    .filter(|c| **c == b'*')
                    .count();
            }

            if flower > 0 {
                out_str.insert(j, char::from_digit(flower.try_into().unwrap(), 10).unwrap());
            } else {
                out_str.insert(j, ' ');
            }
        }
        out.push(out_str);
    }

    out
}
