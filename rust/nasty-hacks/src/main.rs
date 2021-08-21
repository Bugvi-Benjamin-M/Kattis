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
        let line = input();
        let mut numbers = line.trim()
                              .split_ascii_whitespace()
                              .map(|n| n.parse::<i32>().unwrap());

        let r = numbers.next().unwrap();
        let e = numbers.next().unwrap();
        let c = numbers.next().unwrap();

        let advertise_rev = e - c;

        if advertise_rev > r {
            output.push_str("advertise\n");
        }
        else if advertise_rev == r {
            output.push_str("does not matter\n");
        }
        else {
            output.push_str("do not advertise\n");
        }
    }
    print!("{}", output);
}
