use std::io::{self, BufRead};

fn process(v: &Vec<i64>, costs: &Vec<i64>, order: i64) {
    let mut order = order;
    let mut ans: Vec<i64> = vec![];
    if v[order as usize] == -2 {
        println!("Impossible");
        return;
    }

    if v[order as usize] == -1 {
        println!("Ambiguous");
        return;
    }

    while order > 0 {
        ans.push(v[order as usize] + 1);
        order -= costs[v[order as usize] as usize];
    }

    if order < 0 {
        println!("Ambiguoua");
        return;
    }

    ans.sort();

    for number in ans {
        print!("{} ", number);
    }
}

pub fn main() {
    let stdin = io::stdin();

    let mut number_of_items = 0;
    let mut item_prices: Vec<i64> = Vec::new();
    let mut number_of_orders = 0;
    let mut order_prices: Vec<i64> = Vec::new();

    for (count, line) in stdin.lock().lines().map(|l| l.unwrap()).enumerate() {
        match count {
            0 => number_of_items = line.parse().unwrap(),
            1 => {
                item_prices = line
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect();
            }
            2 => number_of_orders = line.parse().unwrap(),
            3 => {
                order_prices = line
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect();
            }
            _ => {}
        }
    }

    let mut v: Vec<i64> = Vec::new();

    for _ in 0..32000 {
        v.push(-2);
    }

    v[0] = 0;

    //n = number of items

    //cost = item_prices

    for i in 0..number_of_items {
        let price = item_prices[i];
        for j in 0..30000 {
            if v[j] >= 0 {
                if v[j as usize + price as usize] == -2 {
                    v[j as usize + price as usize] = i as i64;
                } else {
                    v[j as usize + price as usize] = -1;
                }
            }
            if v[j] == -1 {
                v[j as usize + price as usize] = -1;
            }
        }
    }

    for price in order_prices {
        process(&v, &item_prices, price)
    }
}
