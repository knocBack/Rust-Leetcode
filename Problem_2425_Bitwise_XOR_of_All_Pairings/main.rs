struct Solution;

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n1: i32 = nums1.len() as i32;
        let n2: i32 = nums2.len() as i32;
        let mut xor1: i32 = 0;
        let mut xor2: i32 = 0;
        for i in nums1{
            xor1^=i;
        }
        for i in nums2{
            xor2^=i;
        }
        let mut ans: i32 = 0;
        if n1%2==0 && n2%2==0 {
            ans = 0;
        }
        else if n1%2==0 {
            ans = xor1;
        }
        else if n2%2==0 {
            ans = xor2;
        }
        else {
            ans = xor1^xor2;
        }
        ans
    }
}

fn main() {
    let nums1:Vec<i32> = vec![2,1,3];
    let nums2:Vec<i32> = vec![10,2,5,0];

    println!("Actual Output: {}", Solution::xor_all_nums(nums1, nums2));
    println!("Correct Output: 13");
}
