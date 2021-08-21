use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn main() {
    
    let line = input().trim().to_owned();

    let mut split = line.split_ascii_whitespace();

    let n: u8 = split.next().unwrap().parse().unwrap();
    let m: u8 = split.next().unwrap().parse().unwrap();

    let size = (n+m+2) as usize;

    let mut sums: Vec<f32> = vec![0.0; size];

    let n_prob: f32 = 1.0 / n as f32;
    let m_prob: f32 = 1.0 / m as f32;

    let prod: f32 = n_prob * m_prob;
    let mut max: f32 = 0.0;
    let mut max_indices: Vec<usize> = Vec::with_capacity(size);
    
    for i in 1..(n+1) {
        for j in 1..(m+1) {
            let idx = (i + j) as usize;
            sums[idx] += prod;
            if sums[idx] > max {
                max = sums[idx];
                max_indices.clear();
                max_indices.push(idx);
            }
            else if sums[idx] == max {
                max_indices.push(idx);
            }
        }
    }

    for idx in max_indices {
        println!("{}", idx);
    }
    
}
