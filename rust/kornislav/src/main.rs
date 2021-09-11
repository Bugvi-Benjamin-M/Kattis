use std::cmp;
mod util;

fn main() {
    
    let mut sides = util::parse_line_of_nums::<u16>();

    sides.sort();

    let width = cmp::min(sides[0], sides[1]);

    let height = cmp::min(sides[2], sides[3]);

    let result_area = width * height;

    println!("{}", result_area);
}
