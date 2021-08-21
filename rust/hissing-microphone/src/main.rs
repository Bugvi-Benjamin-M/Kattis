use std::io;

fn input () -> String {
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {

    if input().trim().contains("ss") {
        println!("hiss\n");
    }
    else {
        println!("no hiss\n");
    }

}
