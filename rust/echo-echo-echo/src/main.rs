use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let word: String = input().trim().to_owned();

    println!("{} {} {}", word, word, word)
}
