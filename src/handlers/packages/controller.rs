use actix_web::Responder;

pub async fn index() -> impl Responder {
    format!("hello from get packages")
}