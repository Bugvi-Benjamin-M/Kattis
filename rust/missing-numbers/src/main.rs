use std::io;
use std::collections::HashSet;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let n:u16 = input().trim().parse().unwrap();

    
    let mut recitations: HashSet<u16> = HashSet::with_capacity(n as usize);
    
    let mut last: u16 = 0;
    for _ in 0..n {
        
        let recitation:u16 = input().trim().parse().unwrap();
        recitations.insert(recitation);
        
        last = recitation;
        
    }

    let correct_range: HashSet<u16> = (1..last+1).collect();

    if last == n {
        println!("good job");
    }
    else {
        let mut diff: Vec<&u16> = correct_range.difference(&recitations).collect();
        diff.sort();
        for elem in diff {
            println!("{}", elem);
        }
    }
    
}
