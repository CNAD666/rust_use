pub fn main() {
    let num = Solution::two_sum(vec![1], 2);
    println!("{:?}", num);
}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        vec![target, nums[0]]
    }
}