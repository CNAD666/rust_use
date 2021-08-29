/// 请实现一个函数，把字符串 s 中的每个空格替换成"%20"。
/// 输入：s = "We are happy."
/// 输出："We%20are%20happy."
pub fn main() {
    let msg = Solution::replace_space(String::from("A B C D E F"));
    println!("{}", msg);
}

struct Solution;

impl Solution {
    pub fn replace_space(s: String) -> String {
        let char_list = s.chars();
        let mut result = String::new();
        for char in char_list {
            match char {
                ' ' => result.push_str("%20"),
                _ => result.push(char),
            }
        }

        result
    }
}