use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    // Ignore n
    let _ = input();
    let total:i32 = input()
        .trim()
        .split_ascii_whitespace()
        .map(|e| e.parse::<i32>().unwrap())
        .filter(|e| *e < 0)
        .map(|e| e.abs())
        .sum();
        
    println!("{}", total);
}
