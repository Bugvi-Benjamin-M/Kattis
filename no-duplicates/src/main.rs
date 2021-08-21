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
    let words: Vec<&str> = inp.trim().split_ascii_whitespace().collect();
    let set: HashSet<&str> = HashSet::from_iter(words.to_owned());

    if words.len() == set.len() { println!("yes") }
    else { println!("no") }
}
