mod util;
use std::collections::HashSet;

fn main() {
    
    let line = util::input_trimmed();

    let cards = line
            .chars()
            .collect::<Vec<char>>()
            .chunks(3)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();
    
    let mut card_set: HashSet<String> = HashSet::new();
    
    let mut p_suit = 13;
    let mut k_suit = 13;
    let mut h_suit = 13;
    let mut t_suit = 13;

    
    for card in cards {
        if card_set.contains(&card) { 
            println!("GRESKA");
            return; 
        }
        let suit = card.chars().nth(0).unwrap();
        if suit == 'P' { p_suit -= 1; }
        else if suit == 'K' { k_suit -= 1; }
        else if suit == 'H' { h_suit -= 1; }
        else if suit == 'T' { t_suit -= 1; }
        card_set.insert(card);
    }

    println!("{} {} {} {}", p_suit, k_suit, h_suit, t_suit);
}
