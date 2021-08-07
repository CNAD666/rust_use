#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;


mod api;
mod data;
mod demo;

use api::web_api::{init_web};
use demo::{call_back};

fn main() {
    call_back::test();

    //init web service
    init_web();
}