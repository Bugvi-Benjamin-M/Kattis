use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let w: u64 = input().trim().parse().unwrap();
    let n: u64 = input().trim().parse().unwrap();

    let mut area = 0;
    for _ in 0..n {
        let inp = input().trim().to_owned();

        let mut splits = inp.split_ascii_whitespace();

        let wi: u64 = splits.next().unwrap().parse().unwrap();
        let li: u64 = splits.next().unwrap().parse().unwrap();

        area += wi * li;
    }

    let l = area / w;

    println!("{}", l);
}
