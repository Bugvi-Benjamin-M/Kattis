use std::io;
use std::cmp;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    let inp: Vec<char> = input().trim().chars().collect();
    
    let mut t = 0;
    let mut c = 0;
    let mut g = 0;

    let mut total = 0;
    for ch in inp {
        if ch == 'T' { t += 1 }
        if ch == 'C' { c += 1 }
        if ch == 'G' { g += 1 }
    }
    total += t * t;
    total += c * c;
    total += g * g;

    let min = cmp::min(cmp::min(t, c), g);
    total += 7 * min;

    println!("{}", total);
}
