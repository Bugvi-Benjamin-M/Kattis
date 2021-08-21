use std::io;

fn read_int() -> Result<i64, io::Error> 
{
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().parse().unwrap())
}

fn main() {
    
    let n: i64 = read_int().unwrap();

    for i in 1..(n+1) 
    {
        println!("{} Abracadabra", i)
    }
}
