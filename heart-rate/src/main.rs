fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read from input");
    input
}

fn main() {
    
    let n: u16 = input().trim().parse().unwrap();

    for _ in 0..n {
        let line = input().trim().to_owned();

        let mut split = line.split_ascii_whitespace();

        let b: f32 = split.next().unwrap().parse().unwrap();
        let p: f32 = split.next().unwrap().parse().unwrap();

        let bpm = (60.0 * b) / p;

        let min = bpm - (60.0 / p);
        let max = bpm + (60.0 / p);
        
        println!("{} {} {}", min, bpm, max);
    }
}
