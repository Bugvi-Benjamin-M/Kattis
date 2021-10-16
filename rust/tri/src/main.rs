mod util;

fn main() {

    let nums:Vec<i16> = util::parse_line_of_nums();
    let var1 = nums[0];
    let var2 = nums[1];
    let var3 = nums[2];

    if var1 + var2 == var3 { println!("{}+{}={}", var1, var2, var3); }
    else if var1 - var2 == var3 { println!("{}-{}={}", var1, var2, var3); }
    else if var1 * var2 == var3 { println!("{}*{}={}", var1, var2, var3); }
    else if var1 / var2 == var3 { println!("{}/{}={}", var1, var2, var3); }
    else if var1 == var2 + var3 { println!("{}={}+{}", var1, var2, var3); }
    else if var1 == var2 - var3 { println!("{}={}-{}", var1, var2, var3); }
    else if var1 == var2 * var3 { println!("{}={}*{}", var1, var2, var3); }
    else if var1 == var2 / var3 { println!("{}={}/{}", var1, var2, var3); }
}
