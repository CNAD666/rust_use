use std::collections::HashMap;

pub fn main() {
    let result = Solution::find_repeat_number(vec![1, 2, 3, 4, 55, 3, 2, 1]);
    println!("{}", result);
}

pub struct Solution;

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {

        let mut map: HashMap<i32, &str> = HashMap::new();
        for num in nums {
            if map.contains_key(&num) {
                return num;
            }
            map.insert(num, "");
        }

        panic!()
    }
}