use crate::data::base_info::BaseResult;

#[launch]
pub fn init_web() -> _ {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> String {
    let info = BaseResult { code: String::from("11111111"), data: String::from("2222222"), success: true };
    let msg = serde_json::to_string(&info).unwrap();
    msg
}

