use std::io;

fn main() -> io::Result<()> {
    
    let mut l = String::new();
    io::stdin().read_line(&mut l)?;
    let l: u32 = l.trim().parse().unwrap();

    let mut d = String::new();
    io::stdin().read_line(&mut d)?;
    let d: u32 = d.trim().parse().unwrap();

    let mut x = String::new();
    io::stdin().read_line(&mut x)?;
    let x: u32 = x.trim().parse().unwrap();

    let mut min = std::u32::MAX;

    let mut max = std::u32::MIN;

    for i in l..(d+1) {
        let sum = i.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
        if sum == x && i < min {
            min = i;
        }
        if sum == x && i > max {
            max = i;
        }
    }

    println!("{}", min);
    println!("{}", max);

    Ok(())
}
