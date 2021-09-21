mod util;
use std::f64::consts::PI;

fn are_all_zero(r: f64, m: f64, c: f64) -> bool {
    r == 0.0 && m == 0.0 && c == 0.0
}

fn main() {
    
    loop {
        
        let numbers = util::parse_line_of_nums::<f64>();

        let r = numbers[0];
        let m = numbers[1];
        let c = numbers[2];

        if are_all_zero(r, m, c) { break; }
        
        let true_area = PI * r.powi(2);

        let square_side = r * 2.0; 

        let area_of_square = square_side * square_side;

        let circle_square_ratio = c / m;

        let approx_area = area_of_square * circle_square_ratio;

        println!("{} {}", true_area, approx_area);
    }

}
