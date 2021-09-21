mod util;

fn main() {

    let nums = util::parse_line_of_nums::<f64>();

    let s:f64 = nums.iter().sum::<f64>() / 2.0;

    let a = nums[0];
    let b = nums[1];
    let c = nums[2];
    let d = nums[3];

    let result = ((s - a) * (s - b) * (s - c) * (s - d)).sqrt();

    println!("{}", result);
}
