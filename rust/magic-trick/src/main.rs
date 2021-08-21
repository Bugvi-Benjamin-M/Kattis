use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;
fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let inp = input();
    let chars: Vec<char> = inp.trim().chars().collect();
    let set: HashSet<char> = HashSet::from_iter(chars.to_owned());

    if chars.len() == set.len() { println!("1") }
    else { println!("0") }
}
