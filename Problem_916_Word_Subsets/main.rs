use std::collections::HashMap;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let mut hash2: HashMap<char,i32> = HashMap::new(); // words 2 hash
        let mut hash1: HashMap<char,i32> = HashMap::new(); // words 1 hash
        for s in words2{
            let mut tmp: HashMap<char,i32> = HashMap::new();
            for ch in s.chars(){
                *tmp.entry(ch).or_insert(0) += 1;
            }
            for (key,val) in &tmp{
                let entry = hash2.entry(*key).or_insert(0);
                *entry = max(*entry, *val);
                // *hash2.entry(ch).or_insert(val)=max(*hash2.entry(ch), val);
            }
            tmp.clear();
        }
        // println!("{:?}", hash2);
        for s in words1{
            let mut check: bool = true;
            for ch in s.chars(){
                *hash1.entry(ch).or_insert(0) += 1;
            }
            // println!("{}",s);
            for (key, value) in &hash2{
                // println!("{}: {}", key, value);
                match hash1.get(key){
                    Some(x) => {
                        if *x < *value {
                            check = false;
                            break;
                        }
                    },
                    None => {check = false; break;},
                }
            }
            if check == true{
                ans.push(s.clone());
            }
            hash1.clear();
            // println!("");
        }
        return ans;
    }
}

fn main() {
    let words1 = vec![
        "amazon".to_string(),
        "apple".to_string(),
        "facebook".to_string(),
        "google".to_string(),
        "leetcode".to_string(),
    ];
    let words2 = vec!["eo".to_string(), "o".to_string()];

    let result = Solution::word_subsets(words1, words2);
    println!("{:?}", result);
}