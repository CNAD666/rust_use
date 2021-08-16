mod easy;
mod hard;
mod medium;
mod sword_offer;
mod common;

pub fn main() {
    let default = 4;

    match default {
        //Easy题
        1 => invoke(|| easy::main()),
        //Medium题
        2 => invoke(|| medium::main()),
        //Hard题
        3 => invoke(|| hard::main()),
        //剑指offer专题
        4 => invoke(|| sword_offer::main()),
        _ => {}
    }
}

fn invoke(method: fn()) {
    println!();
    println!("力扣: 打印开始");
    println!("+++++++++++++++++++++++++++++++++++++++++++++++");
    method();
    println!("+++++++++++++++++++++++++++++++++++++++++++++++");
    println!("力扣: 打印结束");
    println!();
}