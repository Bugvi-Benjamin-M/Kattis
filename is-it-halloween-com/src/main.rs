use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {

    let line = input().trim().to_owned();

    if line == "OCT 31" || line == "DEC 25" {
        println!("yup");
    }
    else {
        println!("nope")
    }

}
