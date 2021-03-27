use std::io;

fn main() -> io::Result<()> {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut split = input.trim().split_ascii_whitespace();

    let r1: i32 = split.next().unwrap().parse().unwrap();

    let s: i32 = split.next().unwrap().parse().unwrap();

    let result = s * 2 - r1;

    println!("{}", result);
    Ok(())
}
