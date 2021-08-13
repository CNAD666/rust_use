mod easy;
mod hard;
mod medium;


pub fn main() {
    let default = 2;


    match default {
        //Easy题
        1 => invoke(|| easy::main()),
        //Medium题
        2 => invoke(|| medium::main()),
        //Hard题
        3 => invoke(|| hard::main()),
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