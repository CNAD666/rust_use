#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod web;
mod data;
mod demo;
mod leet_code;

fn main() {
    //力扣
    leet_code::main();

    //测试demo
    demo::main();

    //init web service
    web::main();
}

