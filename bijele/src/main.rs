use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let correct_nums: Vec<i32> = vec![1, 1, 2, 2, 2, 8];

    let line = input().trim().to_owned();
    let real_nums: Vec<i32> = line.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut out = "".to_owned();

    for (c, r) in correct_nums.iter().zip(real_nums.iter()) {
        out.push_str(&format!("{} ", c - r));
    }

    println!("{}", out.trim_end());
}