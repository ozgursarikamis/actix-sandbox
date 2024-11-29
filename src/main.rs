use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use serde_json::json;
// use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: String,
    status: String,
    code: i32,
}
#[get("/")]
async fn simple_json() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Hello",
        "status": "success",
        "code": 200
    }))
}

#[get("/structured")]
async fn structured_json() -> impl Responder {
    let response = Response {
        message: String::from("Hello"),
        status: String::from("success"),
        code: 200,
    };

    HttpResponse::Ok().json(response)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(simple_json)
            .service(structured_json)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 2643))?
    .run()
    .await
}