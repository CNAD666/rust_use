use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};

use super::super::data::base_info::Result;

#[actix_web::main]
pub async fn init_web() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_info)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/getInfo")]
async fn get_info() -> impl Responder {
    let info = Result { code: String::from("11111111"), data: String::from("2222222"), success: true };
    HttpResponse::Ok().body(serde_json::to_string(&info).unwrap())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


trait Animal {
    fn run() -> &str;
}

impl Animal for Dog {
    fn run() -> &str {
        todo!()
    }
}