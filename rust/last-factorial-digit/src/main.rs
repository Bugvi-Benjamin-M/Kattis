use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {

    let t: u8 = input().trim().parse().unwrap();

    let mut output = "".to_owned();

    for _ in 0..t {
        let n: u8 = input().trim().parse().unwrap();

        if n == 0 {
            output.push_str("1\n");
        }
        else if n <= 2 {
            let formatted = format!("{}\n", n);
            output.push_str(&formatted);
        }
        else if n == 3 {
            output.push_str("6\n");
        }
        else if n == 4 {
            output.push_str("4\n");
        }
        else {
            output.push_str("0\n");
        }
    }
    print!("{}", output);
}
