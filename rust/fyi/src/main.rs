mod util;

fn main() {
    
    let phone_number = &util::input().trim()[..3].to_owned(); 

    if phone_number == "555" { println!("1"); }
    else { println!("0"); }
}
