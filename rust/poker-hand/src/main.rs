use std::io;
use std::collections::HashMap;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    let input = input().trim().to_owned();
    let cards = input.split_ascii_whitespace();
    let mut map: HashMap<char, u8> = HashMap::new();

    cards.for_each(|card| {
        let key: char = card.chars().collect::<Vec<char>>()[0];
        *map.entry(key).or_insert(0) += 1;
    });

    println!("{}", map.values().max().unwrap());
}
