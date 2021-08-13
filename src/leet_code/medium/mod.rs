mod two_add;
mod repeat_str;

pub fn main() {
    let default = 2;


    match default {
        //俩数相加
        1 => two_add::main(),
        2 => repeat_str::main(),
        _ => {}
    }
}
