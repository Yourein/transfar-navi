use actix_web::{get, Responder};
use crate::common::health;

#[get("/v1/health")]
pub(crate) async fn v1_health() -> impl Responder {
    health()
}