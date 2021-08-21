use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let _ = input(); // ignore first line

    let line = input().trim().to_owned();
    
    let replaced = line.replace("-1", "")
                       .trim()
                       .replace("  ", " ");
        
    let length = replaced.len() as u32;
    let divisor = length - (length / 2);
    
    let sum:u32 = replaced.split_ascii_whitespace()
                      .map(|c| c.parse::<u32>().unwrap())
                      .sum();
    
    let result: f64 = sum as f64 / divisor as f64;
    println!("{}", result);

}