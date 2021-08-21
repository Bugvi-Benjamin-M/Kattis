use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {

    let line = input();
    let mut split = line.trim().split_ascii_whitespace();

    let h: f32 = split.next().unwrap().parse().unwrap();

    let v: f32 = split.next().unwrap().parse().unwrap();
    
    let res = (h / v.to_radians().sin()).ceil();

    println!("{}", res);
}
