use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

trait ToRad {
    fn to_rad(&self) -> f32;
}
impl ToRad for f32 {
    fn to_rad(&self) -> f32 {
        (self * 3.141592) / 180.0
    }
}

fn main() {
    
   let n: u8 = input().trim().parse().unwrap();

   for _ in 0..n {

        let inp = input();

        let mut splits = inp.trim().split_ascii_whitespace();

        let v0: f32 = splits.next().unwrap().parse().unwrap();

        let theta: f32 = splits.next().unwrap().parse().unwrap();

        let x1: f32 = splits.next().unwrap().parse().unwrap();

        let h1: f32 = splits.next().unwrap().parse().unwrap();

        let h2: f32 = splits.next().unwrap().parse().unwrap();
        
        let g: f32 = 9.81;

        let t: f32 = x1 / (v0 * theta.to_rad().cos());

        let y: f32 = v0 * t * theta.to_rad().sin() - 0.5 * g * t.powi(2);

        if y >= (h1 + 1.0) && y <= (h2 - 1.0) { println!("Safe") }
        else { println!("Not Safe ") }
   }
}
