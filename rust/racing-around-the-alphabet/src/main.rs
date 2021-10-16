mod util;
use std::f64::consts::PI;
use std::cmp::{max, min};

fn get_char_value(c: char) -> u8 {
    if c == ' ' { return 26; }
    else if c == '\'' { return 27; }
    else {
        return c as u8 - 65;
    }
}

fn get_shortest_path(curr: char, next: char) -> u8 {
    
    let curr_val = get_char_value(curr) as i8;
    let next_val = get_char_value(next) as i8;
    
    let first_path = max(curr_val, next_val) - min(curr_val, next_val);
    if first_path < 14 { return first_path as u8; }
    else {
        return ((27 - max(curr_val, next_val)) + min(curr_val, next_val) + 1) as u8;
    }
    
}

fn main() {
    
    let n: u8 = util::parse_num();

    let circle_circumference = 2.0 * PI * 30.0;
    let tile_length = circle_circumference / 28.0;

    let mut out = "".to_owned();
    for _ in 0..n {
        let aphorism = util::input_trimmed();
        let aphorism_chars: Vec<char> = aphorism.chars().collect();

        // Start at 1 for picking up the first disk
        let mut total: f64 = 1.0;
        for i in 0..aphorism_chars.len()-1 {
            let curr = aphorism_chars[i];
            let next = aphorism_chars[i+1];
            
            let num_of_tiles = get_shortest_path(curr, next);
            let time = ((num_of_tiles as f64 * tile_length) / 15.0) + 1.0;
            total += time;
        }
        let total_str = format!("{}\n", total);
        out.push_str(&total_str);
    }

    print!("{}", out);
}
