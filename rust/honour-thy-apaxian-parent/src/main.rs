mod util;

fn main() {
    let inp = util::input()
        .trim()
        .to_owned();

    let mut names = inp.split_ascii_whitespace();

    let name = names.next().unwrap();
    let parent = names.next().unwrap();

    let vowels = vec!['a', 'i', 'o', 'u'];

    let is_vowel = |c: char| vowels.contains(&c);

    if name.ends_with("e") { println!("{}{}{}", name, "x", parent); }
    else if name.ends_with(is_vowel) { println!("{}{}{}", &name[..name.len()-1], "ex", parent); }
    else if name.ends_with("ex") { println!("{}{}", name, parent); }
    else { println!("{}{}{}", name, "ex", parent); }
    
}
