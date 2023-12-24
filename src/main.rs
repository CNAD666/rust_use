mod web;
mod data;
mod demo;
mod leet_code;

#[rocket::main]
async fn main() {
    let default = 3;

    match default {
        //力扣
        1 => leet_code::main(),
        //测试demo
        2 => demo::main(),
        //init web service
        3 => web::main().await,
        _ => {}
    }
}