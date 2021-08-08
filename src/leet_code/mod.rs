mod add_two;


pub fn main() {
    let default = 1;


    match default {
        //闭包写法
        1 => invoke(add_two::main),
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