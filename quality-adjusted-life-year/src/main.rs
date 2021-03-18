use std::io;

fn main() {

    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => println!("Error reading input!")
    }

    let n: i32 = input.trim().parse().unwrap();
    
    let mut accum = 0.0;
    for _ in 0..n {
        
        input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_) => println!("Error reading input!")
        }
        let mut iter = input.trim().split_ascii_whitespace();
        let q_of_life: f64 = iter.next().unwrap().parse().unwrap();
        let duration: f64 = iter.next().unwrap().parse().unwrap();
        accum += q_of_life * duration;
    }

    println!("{}", accum);

}
