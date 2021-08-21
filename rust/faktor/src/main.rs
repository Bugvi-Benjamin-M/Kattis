use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let line = input().trim().to_owned();
    let mut split = line.split_ascii_whitespace();

    let a: u32 = split.next().unwrap().parse().unwrap();
    let i: u32 = split.next().unwrap().parse().unwrap();

    for count in 1.. {
        
        let res = (count as f64 / a as f64).ceil() as u32;

        if res == i {
            println!("{}", count);
            break;
        }

    }
}
