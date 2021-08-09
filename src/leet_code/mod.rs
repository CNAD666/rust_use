mod two_sum;
mod two_add;


pub fn main() {
    let default = 2;


    match default {
        //闭包写法
        1 => invoke(|| two_sum::main()),
        2 => invoke(|| two_add::main()),
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