use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    let n: u8 = input().trim().parse().unwrap();
    
    for _ in 0..n {

        let mut inp: Vec<i32> = input()
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        inp.remove(inp.len()-1); // remove 0 at end

        let mut diff = 0;
        for i in 1..inp.len() {
            if inp[i-1] == inp[i] { continue; }
            if inp[i-1] * 2 < inp[i] {
                diff += inp[i] - inp[i-1] * 2;
            }
        }    

        println!("{}", diff);
    }
    
}
