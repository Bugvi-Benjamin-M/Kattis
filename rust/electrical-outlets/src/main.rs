use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let n:u8 = input().trim().parse().unwrap();

    let mut output = "".to_owned();

    for _ in 0..n {
        let line = input().trim().to_owned();

        let mut split = line.split_ascii_whitespace();

        let k: u8 = split.next().unwrap().parse().unwrap();

        let mut num_of_appliances = 0;
        for _ in 0..(k-1) {
            let strip: u8 = split.next().unwrap().parse().unwrap();
            num_of_appliances += strip - 1;
        }
        let strip: u8 = split.next().unwrap().parse().unwrap();
        num_of_appliances += strip;
        
        output.push_str(&format!("{}\n", num_of_appliances));
    }
    
    print!("{}", output);
}
