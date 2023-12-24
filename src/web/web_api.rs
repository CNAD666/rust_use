use rocket::{get, routes};
use crate::data::base_info::BaseResult;

pub async fn init_web() {
    let rocket = rocket::build().mount("/", routes![index]);
    rocket.launch().await.expect("TODO: panic message");
}

#[get("/")]
fn index() -> String {
    let info = BaseResult { code: String::from("11111111"), data: String::from("2222222"), success: true };
    let msg = serde_json::to_string(&info).unwrap();
    msg
}

