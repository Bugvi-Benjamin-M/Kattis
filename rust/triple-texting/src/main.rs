mod util;
use std::collections::HashMap;

fn main() {
    
    let text = util::input();
    let text = text.trim();
    
    let third_len = text.len() / 3;

    let words = text.chars()
        .collect::<Vec<char>>()
        .chunks(third_len)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();

    let mut word_counts: HashMap<String, u8> = HashMap::new();
    
    for word in words {
        *word_counts.entry(word).or_insert(0) += 1;
    }

    let max_word = word_counts
        .into_iter()
        .max_by_key(|(_, v)| *v)
        .map(|(k, _)| k)
        .unwrap();

    println!("{}", max_word);
}
