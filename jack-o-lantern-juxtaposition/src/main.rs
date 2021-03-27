use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let line = input();
    let numbers = line.trim().split_ascii_whitespace()
                      .map(|i| i.parse::<u32>().unwrap());

    let mut accum = 1;
    for n in numbers {
        accum *= n;
    }

    println!("{}", accum);
    
}
