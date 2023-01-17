use actix_web::{HttpResponse, Responder};

pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}