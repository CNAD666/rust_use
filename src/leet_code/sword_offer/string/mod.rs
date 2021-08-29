mod replace_space;


pub fn main() {
    let default = 1;

    match default {
        //替换空字符串1
        1 => replace_space::main(),
        _ => {}
    }
}
