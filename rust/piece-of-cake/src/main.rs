use std::io;
use std::cmp;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    let input = input().trim().to_owned();
    let mut splits = input.split_ascii_whitespace();
    
    let n: u32 = splits.next().unwrap().parse().unwrap();
    let h: u32 = splits.next().unwrap().parse().unwrap();
    let v: u32 = splits.next().unwrap().parse().unwrap();

    let largest_h = cmp::max(n - h, h);
    let largest_v = cmp::max(n - v, v);
    println!("{}", largest_h * largest_v * 4);
}
