mod common;
mod v1;

use actix_web::{App, HttpServer};
use crate::v1::available_stations::available_stations;
use crate::v1::health::v1_health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(v1_health)
            .service(available_stations)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
