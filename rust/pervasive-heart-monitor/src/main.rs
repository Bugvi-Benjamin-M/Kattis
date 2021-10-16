mod util;

fn is_float(s: String) -> bool {
    match s.parse::<f64>() {
        Ok(_) => { true }
        _ => { false }
    }
}

fn main() {
    
    let mut out = "".to_owned();
    
    let customers = util::read_all_lines();
    
    for customer in customers {
        
        let splits = customer.split_ascii_whitespace().map(|e| e.to_string());
    
        let measurements = splits
            .clone()
            .filter(|e| is_float((**e).to_string()))
            .map(|f| f.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
    
        let name = splits
            .clone()
            .filter(|e| !is_float(e.to_owned()))
            .collect::<Vec<String>>()
            .join(" ");

        let average = measurements.iter().sum::<f64>() / measurements.len() as f64;

        let formatted_output = format!("{} {}\n", average, name);
        out.push_str(&formatted_output);
    }    
    print!("{}", out);
}
