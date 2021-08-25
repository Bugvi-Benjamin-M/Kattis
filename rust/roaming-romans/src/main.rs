use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    let em: f32 = input().trim().parse().unwrap();

    let res: u32 = ((1000.0 * (5280.0 / 4854.0)) * em).round() as u32;

    println!("{}", res);
}
