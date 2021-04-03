use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let line = input().trim().to_owned();

    let problems = line.split(";");

    let mut num_of_problems = 0;
    for s in problems {
        let split: Vec<&str> =  s.split("-").collect();
        if split.len() == 1 {
            num_of_problems += 1;
        }
        else {
            let start: u32 = split[0].parse().unwrap();
            let end: u32 = split[1].parse().unwrap();
            num_of_problems += end - (start - 1);
        }
    }

    println!("{}", num_of_problems);
}