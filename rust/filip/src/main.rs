use std::io;

fn main() -> io::Result<()> {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut split = input.trim().split_ascii_whitespace();

    let a: i32 = split.next()
                    .unwrap()
                    .trim().
                    chars()
                    .rev()
                    .collect::<String>()
                    .parse()
                    .unwrap();

    let b: i32 = split.next()
                    .unwrap()
                    .trim().
                    chars()
                    .rev()
                    .collect::<String>()
                    .parse()
                    .unwrap();

    if a < b {
        println!("{}", b)
    }
    else {
        println!("{}", a)
    }
    
    Ok(())
}
