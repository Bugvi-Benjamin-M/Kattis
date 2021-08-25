use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    let n: u16 = input().trim().parse().unwrap();
    
    for _ in 0..n {

        let inp = input();
        
        if inp.starts_with("Simon says ") {
            let rest = &inp[10..];
            println!("{}", rest);
        }
    }
    
}
