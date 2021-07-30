mod api;
mod data;
use api::web_api::{init_web};

fn main() {
    //init web service
    init_web().unwrap_or_else(|err| println!("{:?}", err));
}
