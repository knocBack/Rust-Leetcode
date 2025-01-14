struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n:usize = a.len();

        let mut common: Vec<i32> = vec![0;n];
        let mut freq: Vec<i32> = vec![0;n+1];
        
        let mut count:i32 = 0;

        for i in 0..n{
            freq[a[i] as usize]+=1;
            if freq[a[i] as usize]==2 {
                count+=1;
            }
            freq[b[i] as usize]+=1;
            if freq[b[i] as usize]==2 {
                count+=1;
            }
            common[i] = count;
        }
        common
    }
}

fn main() {
    let a:Vec<i32> = vec![1,3,2,4];
    let b:Vec<i32> = vec![3,1,2,4];

    println!("Actual Output: {:?}", Solution::find_the_prefix_common_array(a,b));
    println!("Correct Output: [0, 2, 3, 4]");
}
