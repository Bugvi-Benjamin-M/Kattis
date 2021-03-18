use std::io;

fn main() {
    
    let mut x_input = String::new();

    match io::stdin().read_line(&mut x_input) {
        Ok(_) => {},
        Err(_) => println!("Error reading input!")
    }
    
    let mut y_input = String::new();
    match io::stdin().read_line(&mut y_input) {
        Ok(_) => {},
        Err(_) => println!("Error reading input!")
    }

    let x: i64 = x_input.trim().parse().unwrap();
    let y: i64 = y_input.trim().parse().unwrap();

    if x > 0 && y > 0 {
        println!("1");
    } 
    else if x > 0 {
        println!("4");
    }
    else if x < 0 && y < 0 {
        println!("3");
    }
    else {
        println!("2")
    }
}
