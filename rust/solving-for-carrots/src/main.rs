use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let l = input();

    let mut split = l.trim().split_ascii_whitespace();

    let n: i32 = split.next()
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap();

    let p = split.next()
                 .unwrap()
                 .trim();

    for _ in 0..n {
        input();
    }
    
    println!("{}", p);
    
}
