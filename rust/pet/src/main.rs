use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {

    let mut max_sum = 0;
    let mut contestant_idx = 0;
    for i in 1..6 {
        let line = input();
        let sum:u32 = line.trim()
                         .split_ascii_whitespace()
                         .map(|n| n.parse::<u32>().unwrap())
                         .sum();
        
        if sum > max_sum {
            max_sum = sum;
            contestant_idx = i;
        }
    }

    println!("{} {}", contestant_idx, max_sum);
    
}
