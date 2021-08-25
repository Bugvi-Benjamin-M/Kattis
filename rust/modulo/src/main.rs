use std::io;
use std::collections::HashSet;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let mut set: HashSet<u16> = HashSet::with_capacity(10);
    
    for _ in 0..10 {
        let num: u16 = input().trim().parse().unwrap();

        set.insert(num % 42);
    }

    println!("{}", set.len());

}
