use std::io;

fn input () -> String
{
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret
}

fn get_answer(answers: &mut Vec::<char>) {
    let line = input().trim().to_owned();
    let answer:char = line.parse().unwrap();

    answers.push(answer);
}

fn main() {
    
    let n:u16 = input().trim().parse().unwrap();

    let mut answers: Vec<char> = Vec::with_capacity((n-2) as usize);
    
    get_answer(&mut answers); // n >= 1

    let mut correct_answers = 0;
    for i in 1..n {
        
        get_answer(&mut answers);

        let last = answers[(i-1) as usize];
        let current = answers[i as usize];
        if last == current {
            correct_answers += 1;
        }
    }
    
    println!("{}", correct_answers);
    
}
