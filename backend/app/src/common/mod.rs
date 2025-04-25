use actix_web::Responder;

pub(crate) fn health() -> impl Responder {
    format!("maybe healthy")
}