use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let n: u32 = input().trim().parse().unwrap();

    let mut stack: Vec<String> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let input = input().trim().to_owned();
        stack.push(input);
    }

    let mut out = "".to_owned();
    for _ in 0..n {
        out.push_str(&format!("{}\n", stack.pop().unwrap()))
    }
    
    print!("{}", out);
}