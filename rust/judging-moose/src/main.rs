use std::io;
use std::cmp;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    let tines = input()
        .trim()
        .split_ascii_whitespace()
        .map(|t| t.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    
    let l = tines[0];
    let r = tines[1];

    if l == 0 && r == 0 { println!("Not a moose"); }
    else if l == r { println!("Even {}", l + r); }
    else { // Odd
        let max = cmp::max(l, r);
        println!("Odd {}", max * 2);
    }
}
