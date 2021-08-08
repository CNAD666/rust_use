use super::super::data::base_info::Result;

pub fn init_web() {
    rocket::ignite().mount("/", routes![index]).launch();
}

#[get("/")]
fn index() -> String {
    let info = Result { code: String::from("11111111"), data: String::from("2222222"), success: true };
    let msg = serde_json::to_string(&info).unwrap();
    msg
}