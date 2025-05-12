use actix_web::{error, get, web::{self, Json}};
use calc::{departures::get_departures, transfar::build_departure_path};
use chrono::{DateTime, Local};
use models::{id::{StationId, ID}, response::departures::ResDepartures};
use models::departure::Departure;
use serde::Serialize;

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

#[derive(Serialize)]
struct DebDep {
    pub ride_id: String,
    pub trip_id: String,
    pub time: String,
    pub loop_count: i32,
    pub transfar_at: String,
    pub transfar_time: i64,
}

#[derive(Serialize)]
struct ResDeb {
    departure_path: Vec<Vec<DebDep>>
}

// #[get("/deb")]
// pub(crate) async fn deb() -> Result<Json<ResDeb>, actix_web::error::Error> {
//     let datetime = DateTime::parse_from_rfc3339("2025-04-25T08:00:00+00:00").expect("valid datetime");
//     let res = build_departure_path(
//         &StationId::new("HAKODATEBUS_050004".to_string()),
//         datetime,
//     );
//     let mut r: Vec<Vec<DebDep>> = Vec::new();
//     for i in res {
//         let x = i.into_iter().map(|x| {
//             DebDep {
//                 ride_id: x.departure.ride_id.get_raw_id(),
//                 trip_id: x.departure.trip_id,
//                 time: x.departure.time.format("%H:%M").to_string(),
//                 loop_count: x.departure.loop_count,
//                 transfar_at: x.ride_at.get_raw_id(),
//                 transfar_time: x.transfar_time
//             }
//         })
//         .collect::<Vec<_>>();
//         r.push(x);
//     }

//     Ok(Json( ResDeb { departure_path: r } ))
// }