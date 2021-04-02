use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let line1 = input().trim().to_owned();
    let line2 = input().trim().to_owned();
    let line3 = input().trim().to_owned();

    let mut split1 = line1.split_ascii_whitespace();
    let mut split2 = line2.split_ascii_whitespace();
    let mut split3 = line3.split_ascii_whitespace();

    let x1: u16 = split1.next().unwrap().parse().unwrap();
    let y1: u16 = split1.next().unwrap().parse().unwrap();

    let x2: u16 = split2.next().unwrap().parse().unwrap();
    let y2: u16 = split2.next().unwrap().parse().unwrap();

    let x3: u16 = split3.next().unwrap().parse().unwrap();
    let y3: u16 = split3.next().unwrap().parse().unwrap();

    let mut out = "".to_owned();

    if x1 == x2 {
        out.push_str(&format!("{}", x3));
    }
    else if x1 == x3 {
        out.push_str(&format!("{}", x2));
    }
    else {
        out.push_str(&format!("{}", x1));
    }

    out.push_str(" ");

    if y1 == y2 {
        out.push_str(&format!("{}", y3));
    }
    else if y1 == y3 {
        out.push_str(&format!("{}", y2));
    }
    else {
        out.push_str(&format!("{}", y1));
    }

    println!("{}", out);
}