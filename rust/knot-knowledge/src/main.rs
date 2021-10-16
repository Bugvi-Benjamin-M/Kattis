mod util;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    
    // We do not need "n"
    let _ = util::parse_num::<u8>();

    let knots_2_know = util::parse_line_of_nums::<u16>();

    let knots_sonja_knows = util::parse_line_of_nums::<u16>();

    let knots_sonja_knows_set: HashSet<u16> = HashSet::from_iter(knots_sonja_knows);

    for knot in knots_2_know {
        if !knots_sonja_knows_set.contains(&knot) {
            println!("{}", knot);
            break;
        }
    }
}
