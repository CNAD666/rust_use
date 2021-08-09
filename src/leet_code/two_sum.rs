use std::collections::HashMap;

/// 给定一个整数数组 nums和一个整数目标值 target，请你在该数组中找出和为目标值 target 的那两个整数，并返回它们的数组下标。
/// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
///
/// 你可以按任意顺序返回答案。
/// 输入：nums = [2,7,11,15], target = 9
/// 输出：[0,1]
/// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
pub fn main() {
    let num = Solution::two_sum(vec![1, 2, 3, 4, 5], 3);
    println!("{:?}", num);
}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (index, &value) in nums.iter().enumerate() {
            let key = target - value;
            if map.contains_key(&key) {
                return vec![*map.get(&key).unwrap() as i32, index as i32];
            }
            map.insert(value, index);
        }

        panic!()
    }
}