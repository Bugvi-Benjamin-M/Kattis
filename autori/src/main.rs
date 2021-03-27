use std::io;

fn input () -> String {
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {

    let line = input().trim().to_owned();

    let split = line.split("-");


    let mut output = "".to_owned();

    for w in split {

        let c = &w.chars().nth(0).unwrap();
        output.push_str(&c.to_string())
    }

    println!("{}", output);

}
