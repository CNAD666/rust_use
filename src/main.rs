#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;


mod api;
mod data;
mod demo;

use api::web_api::{init_web};
use demo::{test};

fn main() {
    test::test();

    //init web service
    init_web();
}