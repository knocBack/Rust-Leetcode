struct Solution;

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut n:i32 = 0;
        let num1_bits:i32 = num1.count_ones() as i32;
        let num2_bits:i32 = num2.count_ones() as i32;

        if num1_bits == num2_bits {
            return num1;
        }

        let mut diff_bits:i32 = num2_bits;
        for i in (0..32).rev(){
            if diff_bits<=0{
                break;
            }
            if num1 & (1<<i) != 0{
                n|=(1<<i);
                diff_bits-=1;
            }
        }
        for i in 0..32{
            if diff_bits<=0{
                break;
            }
            if n & (1<<i) == 0{
                n|=(1<<i);
                diff_bits-=1;
            }
        }

        return n;

    }
}

fn main() {
    let num1:i32 = 1;
    let num2:i32 = 12;

    println!("Correct output: 3");
    println!("Actual output: {}", Solution::minimize_xor(num1, num2));
}
