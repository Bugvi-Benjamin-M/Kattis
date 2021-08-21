use std::io;
use std::collections::HashSet;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let t: u8 = input().trim().parse().unwrap();

    for _ in 0..t {

        let n: u8 = input().trim().parse().unwrap();

        let mut set: HashSet<String> = HashSet::new();
        for _ in 0..n {
            let inp = input();
            let city = inp.trim().to_owned();
            
            set.insert(city);
        }

        println!("{}", set.len());
    }
    
}
