use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let p:u16 = input().trim().parse().unwrap();

    let mut output = "".to_owned();

    for _ in 0..p {
        let line = input().trim().to_owned();
        let mut split = line.split_ascii_whitespace();

        let k: u16 = split.next().unwrap().parse().unwrap();
        let n: u32 = split.next().unwrap().parse().unwrap();

        let range: Vec<u32> = (1..(n+1)).collect();
        let num_of_candles:u32 = range.iter().sum::<u32>() + (range.len() as u32);

        output.push_str(&format!("{} {}\n", k, num_of_candles));
    }

    print!("{}", output);
}