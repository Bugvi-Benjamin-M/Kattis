use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    let num: u32 = input().trim().parse().unwrap();

    let binary = format!("{:b}", num);

    let rev: String = binary.chars().rev().collect();

    let parsed = u32::from_str_radix(&rev, 2).unwrap();
    println!("{}", parsed);
}
