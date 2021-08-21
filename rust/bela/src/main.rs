use std::io;
use std::collections::HashMap;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let line = input().trim().to_owned();

    let mut split = line.split_ascii_whitespace();

    let n: u16 = split.next().unwrap().parse().unwrap();

    let b: char = split.next().unwrap().parse().unwrap();

    let mut not_dominant: HashMap<char, u32> = HashMap::with_capacity(8);
    not_dominant.insert('A', 11);
    not_dominant.insert('K', 4);
    not_dominant.insert('Q', 3);
    not_dominant.insert('J', 2);
    not_dominant.insert('T', 10);
    not_dominant.insert('9', 0);
    not_dominant.insert('8', 0);
    not_dominant.insert('7', 0);

    let mut dominant: HashMap<char, u32> = HashMap::with_capacity(8);
    dominant.insert('A', 11);
    dominant.insert('K', 4);
    dominant.insert('Q', 3);
    dominant.insert('J', 20);
    dominant.insert('T', 10);
    dominant.insert('9', 14);
    dominant.insert('8', 0);
    dominant.insert('7', 0);


    let mut total_points: u32 = 0;
    for _ in 0..(4*n) {

        let line = input().trim().to_owned();
        let mut chars = line.chars();
        let card: char = chars.next().unwrap();
        let suite: char = chars.next().unwrap();

        if suite == b {
            total_points += dominant.get(&card).unwrap();
        }
        else {
            total_points += not_dominant.get(&card).unwrap();
        }
    }

    println!("{}", total_points);
}