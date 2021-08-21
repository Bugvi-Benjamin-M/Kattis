use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    let inp = input().trim().to_owned();
    let mut vals = inp.split_ascii_whitespace();

    let n: i32 = vals.next().unwrap().parse().unwrap();
    let k: i32 = vals.next().unwrap().parse().unwrap();

    let mut judged_sum = 0;
    for _ in 0..k {
        judged_sum += input().trim().parse::<i32>().unwrap()
    }
    
    let judged_min:i32 = vec![-3; (n-k) as usize].iter().sum();
    let judged_max:i32 = vec![3; (n-k) as usize].iter().sum();

    let min = (judged_min + judged_sum) as f32 / n as f32;
    let max = (judged_max + judged_sum) as f32 / n as f32;
    println!("{} {}", min, max)
}
