use std::path::Path;
use actix_web::{error, get, web::Json};
use calc::valid_station::get_valid_stations;
use models::{id::ROOT_PATH, response::{station::ResStation, stations::ResStations}};

#[get("/v1/available_stations")]
pub(crate) async fn available_stations() -> Result<Json<ResStations>, actix_web::error::Error> {
    let valid_stations = get_valid_stations(&Path::new(ROOT_PATH).to_path_buf());
    if valid_stations.is_err() {
        return Err(error::ErrorInternalServerError("failed to calculate available station."));
    }

    let available = valid_stations.unwrap()
        .iter()
        .map(|x| ResStation::from(x.clone()))
        .collect::<Vec<ResStation>>();
    let res = ResStations {
        stations: available
    };
    Ok(Json(res))
}