struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut cnt:Vec<i32> = vec![0;26];
        let a: usize = 'a' as usize;
        for c in s.chars(){
            cnt[(c as usize)-a]+=1;
        }
        let mut ans:i32 = 0;
        for i in cnt{
            if i==0 {continue;}
            if i%2==1 {ans+=1;}
            else {ans+=2;}
        }
        ans
    }
}

fn main() {
    let s: String = "abaacbcbb".to_string();
    println!("Correct Output: 5");
    println!("Actual Outpu: {}",Solution::minimum_length(s));
}
