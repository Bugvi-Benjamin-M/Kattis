use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {

    let moves: Vec<char> = input().trim().chars().collect();
    
    let mut current = 1;

    for m in moves {
        if m == 'A' {
            if current == 1 { current = 2 }
            else if current == 2 { current = 1 }
        }
        else if m == 'B' {
            if current == 2 { current = 3 }
            else if current == 3 { current = 2 }
        }
        else { // m == 'C'
            if current == 3 { current = 1 }
            else if current == 1 { current = 3 }
        }
    }

    println!("{}", current);
    
}
