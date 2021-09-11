mod util;

fn main() {
    
    let t: u8 = util::parse_num();

    for _ in 0..t {

        let encoded_mess = util::input().trim().to_owned();

        let square_len = (encoded_mess.len() as f64).sqrt() as usize;

        let splits = encoded_mess
            .chars()
            .collect::<Vec<char>>()
            .chunks(square_len)
            .map(|ch| ch.iter().rev().map(|c| c.to_owned()).collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut decoded_mess: Vec<char> = Vec::with_capacity(square_len * square_len);

        for i in 0..square_len {
            for j in 0..square_len {
                let sub_string = &splits[j];
                decoded_mess.push(sub_string[i]);
            }
        }

        println!("{}", decoded_mess.iter().collect::<String>());

    }

}
