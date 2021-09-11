mod util;

fn main() {
    
    let params = util::parse_line_of_nums::<u8>();

    let _n = params[0];
    let p = params[1];
    let s = params[2];

    for _ in 0..s {
        let cards = util::parse_line_of_nums::<u8>();

        let dropped_first = &cards[1..];

        if dropped_first.contains(&p) { println!("KEEP"); }
        else { println!("REMOVE"); }
    }
}
