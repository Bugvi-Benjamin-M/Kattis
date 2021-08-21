use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    let input = input().trim().to_owned();
    let mut numbs = input.split_ascii_whitespace();

    let a: u32 = numbs.next().unwrap().parse().unwrap();
    let b: u32 = numbs.next().unwrap().parse().unwrap();

    if a <= b { println!("{} {}", a, b) }
    else { println!("{} {}", b, a) };
}
