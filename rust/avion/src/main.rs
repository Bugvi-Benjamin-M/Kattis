mod util;

fn main() {

    let mut res: Vec<u8> = Vec::with_capacity(5);

    for i in 1..6 {
        let line = util::input();
        let line = line.trim();

        if line.contains("FBI") { res.push(i); }
    }

    if res.len() == 0 { println!("HE GOT AWAY!"); }
    else {
        for i in res { print!("{} ", i); }
        println!();
    }
}
