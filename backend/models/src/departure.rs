use serde::Deserialize;
use chrono::NaiveTime;

use crate::id::{DeparturePatternId, RideId, ID};

#[derive(Debug, PartialEq, Clone)]
pub struct DeparturePattern {
    pub pattern_id: DeparturePatternId,
    pub departures: Vec<Departure>
}

impl DeparturePattern {
    pub fn from_raw(raw: RawDeparturePattern) -> Result<Self, chrono::ParseError> {
        let departures = raw.departures
            .into_iter()
            .map(Departure::from_raw)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(DeparturePattern {
            pattern_id: DeparturePatternId::new(raw.pattern_id),
            departures,
        })
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Departure {
    pub ride_id: RideId,
    pub trip_id: String,
    pub time: NaiveTime,
}

impl Departure {
    pub fn from_raw(raw: RawDeparture) -> Result<Self, chrono::ParseError> {
        let time = NaiveTime::parse_from_str(&raw.time, "%H:%M")?;
        Ok(Departure {
            ride_id: RideId::new(raw.ride_id),
            trip_id: raw.trip_id,
            time: time
        })
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct RawDeparturePattern {
    pub pattern_id: String,
    pub departures: Vec<RawDeparture>
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct RawDeparture {
    pub ride_id: String,
    pub trip_id: String,
    pub time: String
}
