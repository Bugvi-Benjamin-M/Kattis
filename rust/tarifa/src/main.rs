use std::io;

fn main() -> io::Result<()> {
    
    let mut l = String::new();
    io::stdin().read_line(&mut l)?;
    let x: u32 = l.trim().parse().unwrap();

    let mut d = String::new();
    io::stdin().read_line(&mut d)?;
    let n: u32 = d.trim().parse().unwrap();

    let mut leftover = (n+1) * x;
    for _ in 0..n {
        let mut month = String::new();
        io::stdin().read_line(&mut month)?;
        let month_use: u32 = month.trim().parse().unwrap();
        
        leftover -= month_use;
    }

    println!("{}", leftover);

    Ok(())
}
