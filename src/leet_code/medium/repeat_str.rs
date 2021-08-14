use std::collections::{HashMap, HashSet};

/// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
///
/// 输入: s = "abcabcbb"
/// 输出: 3
/// 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
///
/// 输入: s = "pwwkew"
/// 输出: 3
/// 解释: 因为无重复字符的最长子串是"wke"，所以其长度为 3。
///  请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
pub fn main() {
    let msg = String::from("abcabcbb");
    let num = Solution::length_of_longest_substring(msg);
    println!("{}", num);
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let s_chars: Vec<char> = s.chars().collect();
        let mut pre = 0;
        let mut num = 0;
        for (index, value) in s_chars.iter().enumerate() {
            if let Some(value) = map.get(&value) {
                if pre <= *value {
                    num = num.max(index - pre);
                    pre = *value + 1;
                }
            }
            map.insert(value, index);
        }
        num = num.max(s.len() - pre);
        num as i32
    }

    // pub fn length_of_longest_substring(s: String) -> i32 {
    //     // 哈希集合，记录每个字符是否出现过
    //     let mut occ: HashSet<char> = HashSet::new();
    //     let cv: Vec<char> = s.chars().collect();
    //     let n = s.len();
    //     // 右指针，初始值为 0，相当于我们在字符串的左边界的左侧，还没有开始移动
    //     let mut rk = 0;
    //     let mut ans = 0;
    //     for i in 0..n {
    //         if i != 0 {
    //             // 左指针向右移动一格，移除一个字符
    //             occ.remove(&cv[i - 1]);
    //         }
    //         while rk < n && !occ.contains(&cv[rk]) {
    //             // 不断地移动右指针
    //             occ.insert(cv[rk]);
    //             rk += 1;
    //         }
    //         // 第 i 到 rk 个字符是一个极长的无重复字符子串
    //         ans = ans.max(rk - i);
    //     }
    //     ans as i32
    // }
}