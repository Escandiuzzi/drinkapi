use actix_web::{get, post, HttpResponse, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Application is running!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}