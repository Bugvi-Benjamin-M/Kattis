fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read from input");
    input
}

fn main() {
    
    let c: f64 = input().trim().parse().unwrap();

    let l: u8 = input().trim().parse().unwrap();

    let mut accum = 0.0;

    for _ in 0..l {
        let line = input().trim().to_owned();

        let mut split = line.split_ascii_whitespace();

        let width: f64 = split.next().unwrap().parse().unwrap();
        let length: f64 = split.next().unwrap().parse().unwrap();

        accum += width * length;
    }

    let result = accum * c as f64;
    println!("{}", result);
}
