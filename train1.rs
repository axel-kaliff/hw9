use std::io::{self, BufRead};
fn main() {
    let mut s: Vec<i32> = Vec::new();
    //    let s = vec![3, 1, 2, 3];
    let mut train_collection: Vec<Vec<_>> = Vec::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        if nums.len() != 0 {

            if train_collection.is_empty() {
                let new_train = vec![nums[0]];
                train_collection.push(new_train);
            } else {
                let mut new_trains: Vec<Vec<_>> = Vec::new();
                for train in &train_collection {
                    let mut put_first = train.clone();
                    put_first.insert(0, nums[0]);
                    let mut put_last = train.clone();
                        put_last.push(nums[0]);
                    new_trains.push(put_last);
                    new_trains.push(put_first);
                }
                let longest_lis_vec = train_collection.iter().max_by_key(|train| lis_binary_search(train.as_slice()).len());
                let mut longest_lis = 0;
                if longest_lis_vec.is_some() {
        longest_lis = lis_binary_search(longest_lis_vec.unwrap().as_slice()).len();
   }

                let longest_new_trains: Vec<Vec<_>>= new_trains.iter().filter(|train| lis_binary_search(train.as_slice()).len() == longest_lis).collect();
                train_collection.clear();
                for train in new_trains {
                    if lis_binary_search(train.as_slice()).len() >= longest_lis {

                    train_collection.push(train.clone());
                    }
                }
            }
        }
    }
    let result = train_collection.iter().max_by_key(|train| lis_binary_search(train.as_slice()).len());
    if result.is_some() {
        println!("{}", lis_binary_search(result.unwrap().as_slice()).len());
    }
}

fn lis_binary_search(seq: &[i32]) -> Vec<i32> {
    let mut prefixes: Vec<usize> = Vec::new();
    let mut predecessors = vec![None; seq.len()];

    for next in 0..seq.len() {
        let prefix_len = match prefixes.binary_search_by(|last| seq[*last].cmp(&seq[next])) {
            Ok(len) | Err(len) => len,
        };

        if prefix_len > 0 {
            predecessors[next] = Some(prefixes[prefix_len - 1]);
        }
        if prefix_len >= prefixes.len() {
            prefixes.push(next)
        }
        prefixes[prefix_len] = next;
    }

    let mut lis = vec![0; prefixes.len()];
    let mut next = prefixes.last();
    for i in (0..prefixes.len()).rev() {
        let j = *next.unwrap();
        lis[i] = seq[j];
        next = predecessors[j].as_ref();
    }

    lis
}
