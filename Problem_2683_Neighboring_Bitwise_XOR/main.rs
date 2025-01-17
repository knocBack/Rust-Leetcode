struct Solution;

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut _xor: i32 = 0;
        for i in derived{
            _xor^=i;
        }
        _xor==0
    }
}

fn main() {
    let derived:Vec<i32> = vec![1,1,0];
    println!("Actual Output: {}", Solution::does_valid_array_exist(derived));
    println!("Correct Output: {}", true);
}
