use std::io;
use std::cmp;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn get_next_line_vals() -> Vec<u64> {
    let inp = input().trim().to_owned();
    let inp_split = inp
        .split_ascii_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();
    inp_split
}

fn main() {
    
    let first_line = get_next_line_vals();
    let n: u64 = first_line[0];
    let q: u64 = first_line[1];

    let mut companies = get_next_line_vals();

    assert_eq!(n, companies.len() as u64);

    for _ in 0..q {
        let query = get_next_line_vals();

        let query_type = query[0];
        
        if query_type == 1 {
            let c = query[1];
            let x = query[2];
            companies[(c-1) as usize] = x;
        }
        else { // query_type == 2
            let a = query[1];
            let b = query[2];
            let loc_a = companies[(a-1) as usize];
            let loc_b = companies[(b-1) as usize];

            let dist = cmp::max(loc_a, loc_b) - cmp::min(loc_a, loc_b);

            println!("{}", dist);
        }
    }
}
