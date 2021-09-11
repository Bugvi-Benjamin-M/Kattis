mod util;

fn main() {

    let params = util::parse_line_of_nums::<u16>();
    
    let n = params[0];
    let w = params[1];
    let h = params[2];

    let hyp = ((w.pow(2) + h.pow(2)) as f64).sqrt();

    for _ in 0..n {
        let mat: u16 = util::parse_num();
        if mat as f64 <= hyp { println!("DA"); }
        else { println!("NE"); }
    }
}
