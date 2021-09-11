mod util;

fn get_indices(s: String, mut indices: Vec<u32>, index: u32) -> Vec<u32> {

    if s.is_empty() { return indices; }
    match s {
        _ if s.starts_with(":)") | s.starts_with(";)") => {
            indices.push(index);
            let new_string = s[2..].to_owned();
            return get_indices(new_string, indices, index + 2);
        }
        _ if s.starts_with(":-)") | s.starts_with(";-)") => {
            indices.push(index);
            let new_string = s[3..].to_owned();
            return get_indices(new_string, indices, index + 3);
        }
        _ => {
            let new_string = s[1..].to_owned();
            return get_indices(new_string, indices, index + 1);
        }
    }
}

fn main() {
    
    let input_line = util::input().trim().to_owned();

    let indices = get_indices(input_line, Vec::new(), 0);
    
    let mut output = "".to_owned();
    for i in indices {
        let line = i.to_string() + "\n";
        output.push_str(&line);
    }
    print!("{}", output);
}
