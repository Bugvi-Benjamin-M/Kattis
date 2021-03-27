use std::io;

fn input () -> String {
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {

    // Skip first line
    let _ = input();

    let temp_readings = input().trim().to_owned();

    let mut counter = 0;
    for temp in temp_readings.split_ascii_whitespace() {
        let temp:i32 = temp.parse().unwrap();
        if temp < 0 {
            counter += 1
        }
    }

    println!("{}", counter);
}
