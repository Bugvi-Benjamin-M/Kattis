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

    let x: u16 = numbs.next().unwrap().parse().unwrap();
    let y: u16 = numbs.next().unwrap().parse().unwrap();

    let res = x * 4 + y * 3;

    if res % 2 == 0 { println!("possible"); }
    else { println!("impossible"); }
}
