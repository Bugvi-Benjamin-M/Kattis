use std::io;
use std::collections::HashSet;

fn input () -> String {
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {

    let mut winners = HashSet::<String>::new();

    let n:u8 = input().trim().parse().unwrap();

    let mut output = "".to_owned();
    for _ in 0..n {
        let line = input().trim().to_owned();
        let mut split = line.split_ascii_whitespace();

        let uni = split.next().unwrap();
        let name = split.next().unwrap();

        if !winners.contains(uni) && winners.len() < 12 {
            winners.insert(uni.to_string());
            output.push_str(&format!("{} {}\n", uni, name));
        }
    }

    println!("{}", output);

}
