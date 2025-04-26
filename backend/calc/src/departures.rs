use chrono::{DateTime, FixedOffset, NaiveDate, TimeDelta};
use std::{collections::HashMap, error::Error};
use models::{
    calendar::Calendar,
    departure::DeparturePattern,
    id::{StationId, ID},
    response::{departure::ResDeparture, departures::ResDepartures, station::ResStation},
    ride::Ride,
    station::Station,
    timetable::TimeTable
};

pub fn get_departures(raw_station_id: &str, datetime: DateTime<FixedOffset>) -> Result<ResDepartures, Box<dyn Error + Send + Sync + 'static>> {
    let station_id = StationId::new(raw_station_id.to_string());
    let root_station = Station::from_id(station_id.clone())?;
    if root_station.is_valid() == false {
        return Err(format!{"Timetable for {} not found.", raw_station_id}.into())
    }

    let root_timetable = TimeTable::from_station_id(station_id)?;
    let date: NaiveDate = datetime.date_naive();
    let Some(calendar) = root_timetable.get_valid_calendar(date) else {
        return Err(format!{"No valid calendar found for {}", raw_station_id}.into())
    };

    let pattern_id = Calendar::from_id(calendar.calendar_id)?.get_pattern_id(date);
    let departure_pattern = DeparturePattern::from_id(pattern_id)?;
    
    let mut res_departures: Vec<ResDeparture> = Vec::new();
    let mut ride_cache: HashMap<String, Ride> = HashMap::new();
    for departure in departure_pattern.departures {
        if departure.time.signed_duration_since(datetime.time()) < TimeDelta::zero() {
            continue
        }

        let ride: Ride = if ride_cache.contains_key(&departure.ride_id.get_raw_id()) {
            ride_cache.get(&departure.ride_id.get_raw_id()).unwrap().to_owned()
        } else {
            let key = departure.ride_id.get_raw_id();
            let _ride = Ride::from_id(key.clone())?;
            ride_cache.insert(key, _ride.clone());
            _ride
        };
        
        let bound_for = Station::from_id(ride.to)?;
        res_departures.push( ResDeparture {
            ride_type: ride.ride_type,
            aka_type: if ride.aka_type.is_empty() { None } else { Some(ride.aka_type) },
            type_foreground: ride.type_foreground,
            type_background: ride.type_background,
            type_pronounce: ride.type_pronounce,
            to: ResStation {
                id: bound_for.station_id.get_raw_id(),
                name: bound_for.name,
                pronounce: bound_for.pronounce
            },
            career_type: ride.career_type,
            depart_at: departure.time.format("%H:%M").to_string(),
            transfars: vec![vec![]]
        });
    }
    
    Ok(ResDepartures {
        departures: res_departures
    })
}

#[cfg(test)]
mod test {
    use chrono::DateTime;

    use super::get_departures;


    #[test]
    #[ignore = "comment out this ignore when you want to debug get_departures()"]
    fn debug_get_departures() {
        let datetime = DateTime::parse_from_rfc3339("2025-04-25T08:00:00+09:00").expect("valid datetime");
        let res = get_departures("HAKODATEBUS_050004", datetime);
        println!{"{:?}", res};

        assert_eq!(1, 1)
    }
}