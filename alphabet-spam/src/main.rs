use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let line = input().trim().to_owned();

    let length = line.len() as f64;

    let mut white_spaces = 0;
    let mut lower = 0;
    let mut upper = 0;
    let mut special = 0;

    
    for c in line.chars() {
        if c == '_' {
            white_spaces += 1;
        }     
        else if c.is_ascii_punctuation() || c.is_ascii_digit() {
            special += 1;
        }
        else if c.is_ascii_lowercase() {
            lower += 1;
        }
        else if c.is_ascii_uppercase() {
            upper += 1;
        }
    }

    let ws_ratio = white_spaces as f64 / length;
    let lower_ratio = lower as f64 / length;
    let upper_ratio = upper as f64 / length;
    let special_ratio = special as f64 / length;

    print!("{}\n{}\n{}\n{}\n", ws_ratio, lower_ratio, upper_ratio, special_ratio);

}