use core::fmt;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut out = String::new();
    for i in 0..take_down {
        let bottles = start_bottles - i;
        let txt = get_string_val(bottles);

        let bottle = if bottles != 1 { "bottles" } else { "bottle" };
        out.push_str(&format!(
            "{} green {} hanging on the wall,\n",
            txt.to_string(),
            bottle.to_string()
        ));

        out.push_str(&format!(
            "{} green {} hanging on the wall,\n",
            txt.to_string(),
            bottle.to_string()
        ));

        out.push_str("And if one green bottle should accidentally fall,\n");
        let nxt = get_string_val(bottles - 1);

        let bottle = if bottles - 1 != 1 {
            "bottles"
        } else {
            "bottle"
        };

        out.push_str(&format!(
            "There'll be {} green {} hanging on the wall.\n",
            nxt.to_string().to_lowercase(),
            bottle
        ));
        if i != take_down - 1 {
            out.push('\n');
        }
        // There'll be seven green bottles hanging on the wall.
    }
    out
}

fn get_string_val<'a>(num: u32) -> &'a str {
    return match num {
        0 => "No",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => panic!("TOO HIGH"),
    };
}

// Ten green bottles hanging on the wall,
// Ten green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be nine green bottles hanging on the wall.
//
// Nine green bottles hanging on the wall,
// Nine green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be eight green bottles hanging on the wall.
//
// Eight green bottles hanging on the wall,
// Eight green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be seven green bottles hanging on the wall.
//
// Seven green bottles hanging on the wall,
// Seven green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be six green bottles hanging on the wall.
//
// Six green bottles hanging on the wall,
// Six green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be five green bottles hanging on the wall.
//
// Five green bottles hanging on the wall,
// Five green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be four green bottles hanging on the wall.
//
// Four green bottles hanging on the wall,
// Four green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be three green bottles hanging on the wall.
//
// Three green bottles hanging on the wall,
// Three green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be two green bottles hanging on the wall.
//
// Two green bottles hanging on the wall,
// Two green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be one green bottle hanging on the wall.
//
// One green bottle hanging on the wall,
// One green bottle hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be no green bottles hanging on the wall.
//
//
//
// Ten green bottles hanging on the wall,
// Ten green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be nine green bottles hanging on the wall.
//
// Nine green bottles hanging on the wall,
// Nine green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be eight green bottles hanging on the wall.
//
// Eight green bottles hanging on the wall,
// Eight green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be seven green bottles hanging on the wall.
//
// Seven green bottles hanging on the wall,
// Seven green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be six green bottles hanging on the wall.
//
// Six green bottles hanging on the wall,
// Six green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be five green bottles hanging on the wall.
//
// Five green bottles hanging on the wall,
// Five green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be four green bottles hanging on the wall.
//
// Four green bottles hanging on the wall,
// Four green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be three green bottles hanging on the wall.
//
// Three green bottles hanging on the wall,
// Three green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be two green bottles hanging on the wall.
//
// Two green bottles hanging on the wall,
// Two green bottles hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be one green bottle hanging on the wall.
//
// One green bottle hanging on the wall,
// One green bottle hanging on the wall,
// And if one green bottle should accidentally fall,
// There'll be no green bottles hanging on the wall.
//
