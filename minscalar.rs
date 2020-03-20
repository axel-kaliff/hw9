use std::io::{self, BufRead};

pub fn minscal() {
    let stdin = io::stdin();
    let mut case = 0;
    let mut first = None;

    for (count, line) in stdin.lock().lines().map(|l| l.unwrap()).enumerate() {
        let mut nums: Vec<i64> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        if count != 1 && nums.capacity() != 1 {
            if first.is_none() {
                first = Some(nums);
            } else {
                case += 1;
                let mut first_ready = first.clone().unwrap();
                first_ready.sort();
                nums.sort();
                let mut result = 0;
                for (index, item) in first_ready.iter().rev().enumerate() {
                    result += nums[index] * item;

                }

    println!("Case #{}: {}", case, result);
    first = None;
            }
        }
    }

}
