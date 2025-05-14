use actix_web::{error, get, web::{self, Json}};
use calc::departures::get_departures;
use chrono::{DateTime, Local};
use models::response::departures::ResDepartures;

#[get("/v1/departures/{station_id}")]
pub(crate) async fn get_departures_by_station_id(
    station_id: web::Path<String>,
) -> Result<Json<ResDepartures>, actix_web::error::Error> {
    // let local_datetime = Local::now();
    // let res = get_departures(station_id.as_str(), local_datetime.into());
    
    let datetime = DateTime::parse_from_rfc3339("2025-04-25T08:00:00+09:00").expect("valid datetime");
    let res = get_departures(station_id.as_str(), datetime);
    
    if res.is_err() {
        println!{"{:?}", res.err().unwrap()};
        Err(error::ErrorInternalServerError("failed to calculate timetable."))
    } else {
        Ok(Json(res.unwrap()))
    }
}
