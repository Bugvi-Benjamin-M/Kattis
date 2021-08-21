use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {

    let mut n: u32 = input().trim().parse().unwrap();

    loop {
        let sum = n.to_string().chars().fold(0, |sum: u32, x| sum + x.to_digit(10).unwrap());

        if n % sum == 0 {
            println!("{}", n);
            break;
        }
        n += 1;
    }
}
