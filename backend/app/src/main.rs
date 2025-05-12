mod common;
mod v1;

use actix_web::{App, HttpServer, middleware::Logger};
use v1::departures::{get_departures_by_station_id};
use crate::v1::available_stations::available_stations;
use crate::v1::health::v1_health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Safety: Run this app on appropriate environment
    unsafe {
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(v1_health)
            .service(available_stations)
            .service(get_departures_by_station_id)
            // .service(deb)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
