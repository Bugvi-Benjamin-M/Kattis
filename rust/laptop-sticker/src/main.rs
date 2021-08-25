use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let inp = input().trim().to_owned();
    
    let mut splits = inp.split_ascii_whitespace();

    let wc: u16 = splits.next().unwrap().parse().unwrap();
    let hc: u16 = splits.next().unwrap().parse().unwrap();

    let ws: u16 = splits.next().unwrap().parse().unwrap();
    let hs: u16 = splits.next().unwrap().parse().unwrap();

    if wc >= ws + 2 && hc >= hs + 2 { println!("1"); }
    else { println!("0"); }
}
