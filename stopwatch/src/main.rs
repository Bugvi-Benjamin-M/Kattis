use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let n:u16 = input().trim().parse().unwrap();

    if n % 2 == 1 {

        println!("still running");

    }
    else {

        let mut accum = 0;
        for _ in (0..n).step_by(2) {
            
            let start: u32 = input().trim().parse().unwrap();
            let stop: u32 = input().trim().parse().unwrap();
            
            accum += stop - start;
    
        }

        println!("{}", accum);
    }
}
