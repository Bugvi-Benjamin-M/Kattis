mod util;

fn main() {
    

    let n = util::parse_num::<u16>();

    let mut output_total = String::new();

    for _ in 0..n {
        let line1 = util::input();
        let line1 = line1.trim();

        let line2 = util::input();
        let line2 = line2.trim();

        assert_eq!(line1.len(), line2.len());
        
        let mut output_testcase = String::new();
        for i in 0..line1.len() {
            let c1: char = line1.chars().nth(i).unwrap();
            let c2: char = line2.chars().nth(i).unwrap();
            
            if c1 == c2 { output_testcase.push_str("."); }
            else { output_testcase.push_str("*"); }
        }

        output_total.push_str(line1);
        output_total.push_str("\n");
        output_total.push_str(line2);
        output_total.push_str("\n");
        output_total.push_str(&output_testcase);
        output_total.push_str("\n");
        output_total.push_str("\n");
    }

    print!("{}", output_total);
}
