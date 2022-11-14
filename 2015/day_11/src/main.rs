fn main() {
    let input = "vzbxkghb";
    let mut password = input.to_string();
    loop {
        increment_password(&mut password);
        if has_straight(&password) && has_double_doubles(&password) && no_iol(&password) {
            println!("{:?}", password);
        }
    }
}

fn increment_password(curr: &mut str) {
    unsafe {
        for b in curr.as_bytes_mut().iter_mut().rev() {
            if b != &b'z' {
                *b += 1;
                break;
            }
            *b = b'a';
        }
    }
}

fn has_straight(password: &str) -> bool {
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    let mut i = 0;
    while let Some(letters) = password.get(i..i + 3) {
        if alpha.find(letters).is_some() {
            return true;
        }
        i += 1;
    }
    false
}

fn no_iol(password: &str) -> bool {
    for c in password.chars() {
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }
    }
    true
}

fn has_double_doubles(password: &str) -> bool {
    let mut i = 0;
    let mut first = None;
    while let Some(letters) = password.get(i..i + 2) {
        let mut x = letters.chars();
        if x.next() == x.next() {
            if first.is_some() && Some(letters) != first {
                return true;
            } else {
                first = Some(letters)
            }
        }
        i += 1;
    }

    false
}
