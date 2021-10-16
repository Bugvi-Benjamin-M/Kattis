mod util;

fn split(s: String) -> (String, String) {
    let length = s.len();

    let first = s[..length/2].to_owned();
    let second = s[length/2..].to_owned();
    return (first, second);
}

fn rotation_val(s: String) -> u32 {
    return s.chars().map(|c| (c as u32) - 65).sum::<u32>();
}

fn rotate_char(c: char, rot_val: u32) -> char {
    let char_val = (((c as u32 - 65) + rot_val) % 26) + 65;
    return char_val as u8 as char;
}

fn rotate_string(s: String, rot_val: u32) -> String {
    return s.chars()
            .map(|c| rotate_char(c, rot_val))
            .collect::<String>();
}

fn rotate_by(s: String, by: String) -> String {
    return s.char_indices()
            .map(|(i,c)| rotate_char(c, by.chars().nth(i).unwrap() as u32 - 65))
            .collect::<String>();
}

fn main() {
    
    let string = util::input_trimmed();

    let (first, second) = split(string);

    let rot_val_first = rotation_val(first.to_owned());
    let rot_val_second = rotation_val(second.to_owned());

    let first_rotated = rotate_string(first, rot_val_first);
    let second_rotated = rotate_string(second, rot_val_second);

    let drm = rotate_by(first_rotated, second_rotated);

    println!("{}", drm);
}
