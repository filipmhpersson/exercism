#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let mut take = 0;
    if first_list.len() == second_list.len() {
        if first_list == second_list {
            return Comparison::Equal;
        } else {
            return Comparison::Unequal;
        }
    } else if first_list.len() > second_list.len() {
        if check_sublist(second_list, first_list) {
            return Comparison::Superlist;
        }
    } else {
        if check_sublist(first_list, second_list) {
            return Comparison::Sublist;
        }
    }
    Comparison::Unequal
}

fn check_sublist(sub: &[i32], sup: &[i32]) -> bool {
    return if sub.len() > 0 {
        sup.windows(sub.len()).any(|s| s == sub)
    } else {
        true
    };
    // let mut i = 0;
    // let Some(first) = sub.first() else {
    //     return true;
    // };
    //
    // 'big: loop {
    //     let pos = sup[i..].iter().position(|s| s == first);
    //     if let Some(pos) = pos {
    //         let pos = pos + i;
    //         i = pos + 1 + i;
    //         for j in 0..sub.len() {
    //             if sub[j] != sup[j + pos] {
    //                 continue 'big;
    //             }
    //         }
    //         return true;
    //     } else {
    //         break;
    //     }
    // }
    // false
    // 'outer: for x in sub {
    //     let mut j = 0;
    //     for y in &sup[i..] {
    //         j += 1;
    //         if x == y {
    //             i = j + i;
    //             continue 'outer;
    //         }
    //     }
    //     return false;
    // }
}
