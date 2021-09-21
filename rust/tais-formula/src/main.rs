mod util;

fn main() {
    let n: u16 = util::parse_num();

    let mut numbs: Vec<f64> = util::parse_line_of_nums();

    let mut t_prev = numbs[0] / 1000.0;
    let mut v_prev = numbs[1];
    
    let mut total_area: f64 = 0.0;
    for _ in 1..n {

        numbs = util::parse_line_of_nums();

        let t = numbs[0] / 1000.0;
        let v = numbs[1];

        let curr_trapezoid_area = ((v_prev + v) / 2.0) * (t - t_prev);
        total_area += curr_trapezoid_area;

        t_prev = t;
        v_prev = v;
    }

    println!("{}", total_area);
}
