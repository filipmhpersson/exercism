use std::{
    array,
    ops::{Div, Index, Range},
};

// pub fn find(array: &[i32], key: i32) -> Option<usize> {
//     if array.len() == 0 {
//         return None;
//     }
//
//     if array.len() == 1 {
//         if array[0] == key {
//             return Some(0);
//         } else {
//             return None;
//         }
//     }
//
//     let index = array.len().div(2);
//     let comp = array[index];
//     if comp == key {
//         return Some(index);
//     } else if comp > key {
//         match find(&array[0..index], key) {
//             None => return None,
//             Some(a) => return Some(a),
//         }
//     } else {
//         match find(&array[index + 1..], key) {
//             None => return None,
//             Some(a) => return Some(index + a + 1),
//         }
//     }
// }

pub fn find<T, U>(array: U, key: T) -> Option<usize>
where
    U: AsRef<[T]>,
    T: Sized,
    T: std::cmp::PartialEq,
    T: std::cmp::PartialOrd,
{
    let array = array.as_ref();

    if array.len() == 0 {
        return None;
    }

    if array.len() == 1 {
        if array[0] == key {
            return Some(0);
        } else {
            return None;
        }
    }

    let index = array.len().div(2);
    let comp = &array[index];
    if *comp == key {
        return Some(index);
    } else if *comp > key {
        match find(&array[0..index], key) {
            None => return None,
            Some(a) => return Some(a),
        }
    } else {
        match find(&array[index + 1..], key) {
            None => return None,
            Some(a) => return Some(index + a + 1),
        }
    }
}
