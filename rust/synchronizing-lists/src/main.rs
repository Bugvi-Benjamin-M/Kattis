mod util;

fn gather_list(n: u16) -> Vec<i16> {
    let mut list: Vec<i16> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let num: i16 = util::parse_num();
        list.push(num);
    }
    list
}

fn main() {
    
    loop {
        let n: u16 = util::parse_num();

        if n == 0 { break; }

        let list1 = gather_list(n);

        let mut list1_sorted = list1.clone();
        list1_sorted.sort();

        let mut list2 = gather_list(n);
        list2.sort();
        
        let mut new_list2: Vec<i16> = vec![0; n as usize];
        for i in 0..n {
            let el = list1_sorted[i as usize];
            let idx = list1.iter().position(|&j| j == el).unwrap();

            new_list2[idx] = list2[i as usize];
        }

        for elem in new_list2 {
            println!("{}", elem);
        }
        println!();
        
    }

    
}
