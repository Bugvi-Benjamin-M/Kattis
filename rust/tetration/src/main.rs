mod util;

fn main() {
    
    let n:f64 = util::parse_num();

    let result = n.powf(1.0 / n);

    println!("{}", result);

}
