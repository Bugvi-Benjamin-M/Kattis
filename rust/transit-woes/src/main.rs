use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let inp = input();
    let mut first_line = inp.trim().split_ascii_whitespace();
    
    let s: u16 = first_line.next().unwrap().parse().unwrap();
    let t: u16 = first_line.next().unwrap().parse().unwrap();
    let n: u16 = first_line.next().unwrap().parse().unwrap();

    let inp = input();
    let mut transits = inp.trim()
        .split_ascii_whitespace()
        .map(|t: &str| t.parse::<u16>().unwrap());
        
    let inp = input();
    let mut ride_times = inp.trim()
        .split_ascii_whitespace()
        .map(|t: &str| t.parse::<u16>().unwrap());
    
    let inp = input();
    let mut intervals = inp.trim()
        .split_ascii_whitespace()
        .map(|t: &str| t.parse::<u16>().unwrap());

    let mut time_spent = s;
    time_spent += transits.next().unwrap();
    for _ in 0..n {
        let interval = intervals.next().unwrap();
        time_spent += if time_spent < interval { interval - time_spent } else { time_spent % interval };
        time_spent += ride_times.next().unwrap();
        time_spent += transits.next().unwrap();
    }
    
    if time_spent >= t { println!("no") }
    else { println!("yes") }
}
