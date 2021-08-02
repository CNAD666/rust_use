mod api;
mod data;

use api::web_api::{init_web};

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);

    //init web service
    init_web().unwrap_or_else(|err| println!("{:?}", err));
}
