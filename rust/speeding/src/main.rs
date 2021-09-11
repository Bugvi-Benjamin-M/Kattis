mod util;

fn main() {
    
    let n = util::parse_num::<u8>();

    let mut times: Vec<f32> = Vec::with_capacity(n as usize);
    let mut distances: Vec<f32> = Vec::with_capacity(n as usize);
    
    let params = util::parse_line_of_nums::<f32>();

    let t = params[0];
    let d = params[1];
    times.push(t);
    distances.push(d);

    let mut max_speed = 0.0_f32;
    for i in 1..n {

        let params = util::parse_line_of_nums::<f32>();

        let t = params[0] as f32;
        let d = params[1] as f32;
        times.push(t);
        distances.push(d);
        let i_us = i as usize;
        let speed = (distances[i_us] - distances[i_us - 1]) 
            / (times[i_us] - times[i_us - 1]).floor();

        if speed > max_speed { max_speed = speed; }
    }

    println!("{}", max_speed as u32);
}
