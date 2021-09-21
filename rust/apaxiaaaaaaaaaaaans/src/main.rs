mod util;

fn main() {
    
    let name = util::input();
    let name = name.trim();
    let name_chars: Vec<char> = name.chars().collect();

    let mut compressed = String::new();

    for i in 0..(name.len()-1) {
        
        if name_chars[i] == name_chars[i+1] { continue; }

        compressed.push(name_chars[i]);

    }

    compressed.push(name_chars[name.len()-1]);

    println!("{}", compressed);
}
