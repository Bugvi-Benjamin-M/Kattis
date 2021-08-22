use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    let inp = input().trim().to_owned();
    
    let mut per = "".to_owned();
    for _ in 0..inp.len() / 3 {
        per.push_str("PER");
    }

    let inp_chars: Vec<char> = inp.chars().collect();
    let per_chars: Vec<char> = per.chars().collect();
    assert_eq!(inp_chars.len(), per_chars.len());
    
    let mut num_days = 0;
    for i in 0..inp_chars.len() {
        if inp_chars[i] != per_chars[i] { num_days += 1 }
    }

    println!("{}", num_days);
}
