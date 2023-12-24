use rocket::{Build, Rocket};

pub mod web_api;

pub fn main() {
    let default = 1;


    match default {
        //闭包写法
        1 => invoke(web_api::init_web),
        _ => {}
    }
}

fn invoke(method: fn() -> Rocket<Build>) {
    println!();
    println!("web: 打印开始");
    println!("*************************************************");
    method();
    println!("*************************************************");
    println!("web: 打印结束");
    println!();
}