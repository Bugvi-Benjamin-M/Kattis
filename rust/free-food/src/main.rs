mod util;

fn main() {
    
    let n: u8 = util::parse_num();
    
    let mut intervals: Vec<(u16, u16)> = Vec::with_capacity(n as usize);

    for _ in 0..n {

        let line: Vec<u16> = util::parse_line_of_nums();
        
        let start = line[0];
        let end = line[1];

        intervals.push((start, end));
    }

    let max = intervals
        .iter()
        .map(|&(_, end)| end)
        .max().unwrap();

    let mut days = vec![0; max as usize];

    for entry in intervals {

        for i in entry.0..(entry.1+1) {
            let i_usize = i as usize;
            days[i_usize-1] += (days[i_usize-1] + 1_u16).rem_euclid(2);
        }

    }

    let free_food_days:u16 = days.iter().sum();

    println!("{}", free_food_days);
}
