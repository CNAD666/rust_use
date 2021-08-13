/// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
///
/// 输入: s = "abcabcbb"
/// 输出: 3
/// 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
///
/// 输入: s = "bbbbb"
/// 输出: 1
/// 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
pub fn main() {
    let msg = String::from("pwwkew");
    let num = Solution::length_of_longest_substring(msg);
    println!("{}", num);
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut num_list: Vec<i32> = Vec::new();
        let mut num = 0;
        let char_vec: Vec<char> = s.chars().collect();
        let pre_byte: char = '.';
        for byte in char_vec {
            if byte != pre_byte {
                num += 1;
            } else {
                num_list.push(num);
                num = 0;
            }
        }

        let mut temp = 0;
        for num in num_list {
            if num > temp {
                temp = num;
            }
        }

        temp
    }
}