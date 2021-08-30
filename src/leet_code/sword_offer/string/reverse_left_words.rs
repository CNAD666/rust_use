/// 字符串的左旋转操作是把字符串前面的若干个字符转移到字符串的尾部。
/// 请定义一个函数实现字符串左旋转操作的功能。比如，输入字符串"abcdefg"和数字2，该函数将返回左旋转两位得到的结果"cdefgab"。
///
/// 输入: s = "abcdefg", k = 2
/// 输出: "cdefgab"
pub fn main() {}

struct Solution;


impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let mut head = String::new();
        let mut tail = String::new();
        for char in s.chars() {
            if tail.len() < (n as usize) {
                tail.push(char);
            } else {
                head.push(char);
            }
        }
        head.push_str(tail.as_str());

        head
    }
}