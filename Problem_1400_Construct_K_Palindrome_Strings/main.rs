struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut hash:Vec<i32> = vec![0;26];
        let a:i32 = 'a' as i32;
        for c in s.chars() {
            let tmp:i32 = c as i32;
            let tmp:usize = (tmp - a) as usize;
            hash[tmp]+=1;
            // println!("{}",tmp);
        }
        let mut odd:i32 = 0;
        for val in hash{
            if val&1 == 1{
                odd += 1;
            }
        }
        let minimum:i32 = odd;
        let maximum:i32 = s.len() as i32;
        if minimum <= k && k<= maximum{
            return true;
        }
        return false;
    }
}

fn main() {
    let s = String::from("annabelle");
    let k = 2;
    let result = Solution::can_construct(s, k);
    println!("Test case: s = \"annabelle\", k = 2");
    println!("Expected: true");
    println!("Output: {}", result);
}
