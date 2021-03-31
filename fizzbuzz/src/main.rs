fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read from input");
    input
}

fn main() {
    
    let line = input().trim().to_owned();

    let mut split = line.split_ascii_whitespace();
    
    let x: u8 = split.next().unwrap().parse().unwrap();
    let y: u8 = split.next().unwrap().parse().unwrap();
    let n: u8 = split.next().unwrap().parse().unwrap();

    let mut output = "".to_owned();
    
    for i in 1..(n+1) {
        if i % x == 0 && i % y == 0 {
            output.push_str(&"FizzBuzz\n");
        }
        else if i % x == 0 {
            output.push_str(&"Fizz\n");
        }
        else if i % y == 0 {
            output.push_str(&"Buzz\n");
        }
        else {
            output.push_str(&format!("{}\n", i));
        }
    }

    print!("{}", output);
}
