use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {

    let n: u8 = input().trim().parse().unwrap();

    let mut output = "".to_owned();

    for _ in 0..n {
        let num: i8 = input().trim().parse().unwrap();
        if num % 2 == 0 {
            let line = format!("{} is even\n", num);
            output.push_str(&line[..]);
        }
        else {
            let line = format!("{} is odd\n", num);
            output.push_str(&line[..]);
        }
    }
    print!("{}", output);
}
