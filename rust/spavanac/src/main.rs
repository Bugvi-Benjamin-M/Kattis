mod util;

fn main() {
    let time = util::parse_line_of_nums::<i8>();

    let hour = time[0];
    let minute = time[1];

    let new_hour = if minute < 45 { if hour - 1 < 0 { 23 } else { hour - 1 } } else { hour };

    let new_minute = if minute < 45 { 60 - (minute - 45).abs() } else { minute - 45 };

    println!("{} {}", new_hour, new_minute);
}
