use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let n:u16 = input().trim().parse().unwrap();

    let mut output = "".to_owned();

    for _ in 0..n {
        let line = input().trim().to_owned();

        if line == "P=NP" {
            output.push_str("skipped\n");
        }
        else {
            let mut split = line.split('+');
            let arg1:u16 = split.next().unwrap().parse().unwrap();
            let arg2:u16 = split.next().unwrap().parse().unwrap();

            output.push_str(&format!("{}\n", arg1 + arg2));
        }
    }
    
    print!("{}", output);
}
