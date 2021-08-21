use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let n: u32 = input().trim().parse().expect("Failed to parse int");

    let mut x = 0;

    for _ in 0..n {
        let line = input();
        let line = line.trim();
        let pwr:u32 = line.chars().last().unwrap().to_digit(10).unwrap();
        let i:u32 = line[0..line.len() - 1].parse().unwrap();
        
        x += i.pow(pwr);
    }

    println!("{}", x);
    
}
