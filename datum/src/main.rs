use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let inp = input();

    let mut split = inp.trim().split_ascii_whitespace();

    let day: u16 = split.next().unwrap().parse().unwrap();
    let month: u16 = split.next().unwrap().parse().unwrap();

    let days_in_months = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let slice = &days_in_months[..(month-1  ) as usize];
    let sum: u16 = slice.iter().sum();
    
    let total = sum + day - 1;
    let weekday = total % 7;

    if weekday == 0 { println!("Thursday"); }
    else if weekday == 1 { println!("Friday") }
    else if weekday == 2 { println!("Saturday") }
    else if weekday == 3 { println!("Sunday") }
    else if weekday == 4 { println!("Monday") }
    else if weekday == 5 { println!("Tuesday") }
    else if weekday == 6 { println!("Wednesday") }
}
