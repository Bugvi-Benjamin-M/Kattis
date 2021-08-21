use std::io;

fn main() -> io::Result<()> {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let num_of_es = (input.trim().len() - 2) * 2;
    
    let es = (0..num_of_es).map(|_| "e").collect::<String>();
    
    println!("h{}y", es);
    
    Ok(())
}
