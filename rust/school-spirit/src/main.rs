mod util;

fn calc_group_score(scores: Vec<f64>, without_pos: Option<u8>) -> f64 {

    let mut filtered_scores = scores.clone();
    
    if without_pos.is_some() {
        let _ = filtered_scores.remove(without_pos.unwrap() as usize);
    }
    
    return (1.0_f64/5.0_f64) * filtered_scores.iter()
                                              .enumerate()
                                              .map(|(i,s)| { s * (4.0_f64/5.0_f64).powi(i as i32) })
                                              .sum::<f64>();
}

fn main() {
    
    let n = util::parse_num::<u8>();

    let mut scores = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let score = util::parse_num::<f64>();

        scores.push(score);
    }

    let group_score = calc_group_score(scores.clone(), None);
    
    let average = (0..n)
        .map(|i| calc_group_score(scores.clone(), Some(i)))
        .sum::<f64>() / n as f64;
    
    println!("{}", group_score);
    println!("{}", average);
    
}
