use std::collections::HashSet;
mod util;

fn main() {
    
    let params = util::parse_line_of_nums::<u16>();

    let p = params[0];
    let n = params[1];

    let mut unique_parts: HashSet<String> = HashSet::with_capacity(p as usize);

    let mut day = 0;
    for i in 0..n {
        let part = util::input().trim().to_owned();

        let prev_len = unique_parts.len();
        unique_parts.insert(part);
        
        let set_grew = prev_len < unique_parts.len();
        if set_grew {
            day = i + 1;
        }
    }

    if unique_parts.len() < p as usize {
        println!("paradox avoided")
    }
    else {
        println!("{}", day);
    }
    
}
