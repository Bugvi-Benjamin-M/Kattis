use std::io;
fn main() -> io::Result<()> {
    
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)?;

    let n: u32 = input.trim().parse().unwrap();

    if n % 2 == 0 {
        println!("Bob")
    }
    else {
        println!("Alice")
    }

    Ok(())
}
