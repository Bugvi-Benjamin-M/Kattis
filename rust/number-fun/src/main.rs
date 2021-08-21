use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {

    let n: u16 = input().trim().parse().unwrap();

    let mut output = "".to_owned();

    for _ in 0..n {
        let line = input();
        let mut numbers = line.trim()
                              .split_ascii_whitespace()
                              .map(|n| n.parse::<f32>().unwrap());

        let a = numbers.next().unwrap();
        let b = numbers.next().unwrap();
        let c = numbers.next().unwrap();

        let sub_possible = a - b == c || b - a == c;
        let div_possible = a / b == c || b / a == c;

        if a + b == c || sub_possible || a * b == c || div_possible {
            output.push_str("Possible\n");
        }
        else {
            output.push_str("Impossible\n")
        }
    }
    print!("{}", output);
}
