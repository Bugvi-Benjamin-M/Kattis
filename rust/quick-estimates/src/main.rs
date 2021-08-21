use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let n:u8 = input().trim().parse().unwrap();

    let mut output = "".to_owned();

    for _ in 0..n {
        let estimate_len = input().trim().len();
        output.push_str(&format!("{}\n", estimate_len));
    }

    print!("{}", output);
}