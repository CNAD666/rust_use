pub mod web_api;

pub async fn main() {
    println!();
    println!("web: 打印开始");
    println!("*************************************************");
    web_api::init_web().await;
    println!("*************************************************");
    println!("web: 打印结束");
    println!();
}