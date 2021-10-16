mod util;

fn main() {
    
    let p = util::parse_num::<u16>();

    let mut out = "".to_owned();

    for _ in 0..p {
        
        let line = util::parse_line_of_nums::<u32>();
        let k = line[0];
        let b = line[1];
        let n = line[2];

        let mut sum = 0;
        let mut new_num = n;
        loop {
            if new_num <= 0 { break; }
            let digit = new_num % b;
            sum += digit * digit;
            new_num = (new_num - digit) / b;
        }

        let formatted = format!("{} {}\n", k, sum);
        out.push_str(&formatted);
    }

    print!("{}", out);
    
}
