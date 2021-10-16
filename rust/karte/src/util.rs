use std::io;
use std::str::FromStr;
use std::fmt::Debug;

#[allow(dead_code)]
pub fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

#[allow(dead_code)]
pub fn input_trimmed () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret = ret.trim().to_owned();
    ret
}

#[allow(dead_code)]
pub fn parse_num<T>() -> T where T: FromStr, <T as FromStr>::Err: Debug {
    let inp = input().trim().parse::<T>().unwrap();
    inp
}

#[allow(dead_code)]
pub fn parse_line_of_nums<T>() -> Vec<T> where T: FromStr, <T as FromStr>::Err: Debug {
    let inp = input()
        .trim()
        .split_ascii_whitespace()
        .map(|i| i.parse::<T>().unwrap())
        .collect::<Vec<T>>();
    inp
}