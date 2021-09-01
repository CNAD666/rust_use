mod replace_space;
mod reverse_left_words;


pub fn main() {
    let default = 2;

    match default {
        //替换空字符串1
        1 => replace_space::main(),
        2 => reverse_left_words::main(),
        _ => {}
    }
}
