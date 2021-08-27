use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    let t: u8 = input().trim().parse().unwrap();
    
    for _ in 0..t {
        
        let n: u8 = input().trim().parse().unwrap();
        
        let mut store_locations: Vec<i16> = input()
            .trim()
            .split_ascii_whitespace()
            .map(|l| l.parse::<i16>().unwrap())
            .collect();

        store_locations.sort();

        let mut dist = store_locations[store_locations.len()-1] - store_locations[0];

        for i in 0..n-1 {
            dist += store_locations[(i + 1) as usize] - store_locations[i as usize];
        }
        println!("{}", dist);
    }

}
