use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let inp = input();
    
    let n: u16 = inp.trim().parse().unwrap();

    let mut sum: u16 = 0;
    for _ in 0..n {
        sum += input().trim().parse::<u16>().unwrap();
    }

    let out = sum - (n - 1);

    println!("{}", out);
}
