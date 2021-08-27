use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    let input = input().trim().to_owned();
    let mut numbs = input.split_ascii_whitespace();

    let g: u8 = numbs.next().unwrap().parse::<u8>().unwrap() * 3;
    let s: u8 = numbs.next().unwrap().parse::<u8>().unwrap() * 2;
    let b: u8 = numbs.next().unwrap().parse::<u8>().unwrap() * 1;

    let buying_power = g + s + b;

    if buying_power <= 1 
        { println!("Copper"); }
    else if buying_power == 2 
        { println!("Estate or Copper"); }
    else if buying_power >= 3 && buying_power <= 4 
        { println!("Estate or Silver"); }
    else if buying_power == 5 
        { println!("Duchy or Silver"); }
    else if buying_power >= 6 && buying_power <= 7 
        { println!("Duchy or Gold"); }
    else /* buying_power >= 8 */ 
        { println!("Province or Gold"); }
}
