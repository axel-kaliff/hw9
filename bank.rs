use std::io;

fn main() {
    let stdin = io::stdin();
    let mut first_input = String::new();
    let mut n = 0;
    let mut t = 0;

    let mut bank = Vec::new();
    stdin.read_line(&mut first_input).expect("failed");
    let parsed: Vec<u64> = first_input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    n = parsed[0];
    t = parsed[1];

    for _ in 0..t {
        bank.push(Vec::new());
    }
    for _ in 0..n {
        let mut input = String::new();
         stdin.read_line(&mut input).expect("failed");
        let parsed1: Vec<u64> = input
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let money = parsed1[0];
        let time_left = parsed1[1];

        bank[time_left as usize].push(money);
    }
    let mut sum = 0;
    let mut alternatives = Vec::new();

    for x in (0..t).rev() {
        alternatives.append(&mut bank[x as usize]);
        alternatives.sort();
        if !alternatives.is_empty() {
            sum += alternatives.pop().unwrap();
            
        }
    }
    println!("{}", sum);
}
