use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {

    let n: u8 = input().trim().parse().unwrap();

    let mut edge_points = 2;
    for _ in 0..n {
        edge_points += edge_points - 1;
    }
    
    let result = edge_points * edge_points;
    println!("{}", result);
}
