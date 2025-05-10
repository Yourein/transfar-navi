use std::{error::Error, fs::File, io::BufReader};

use serde::Deserialize;
use chrono::NaiveTime;

use crate::id::{DeparturePatternId, RideId, ID};

#[derive(Debug, PartialEq, Clone)]
pub struct DeparturePattern {
    pub pattern_id: DeparturePatternId,
    pub departures: Vec<Departure>
}

impl DeparturePattern {
    #[allow(dead_code)]
    pub(crate) fn from_raw(raw: RawDeparturePattern) -> Result<Self, chrono::ParseError> {
        let departures = raw.departures
            .into_iter()
            .map(Departure::from_raw)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(DeparturePattern {
            pattern_id: DeparturePatternId::new(raw.pattern_id),
            departures,
        })
    }

    #[allow(dead_code)]
    pub fn from_id(id: DeparturePatternId) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let path = id.build_path();
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let raw: RawDeparturePattern = serde_json::from_reader(reader)?;
        
        Ok(DeparturePattern::from_raw(raw)?)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Departure {
    pub ride_id: RideId,
    pub trip_id: String,
    pub time: NaiveTime,
    pub loop_count: Option<i32>,
}

impl Departure {
    pub(crate) fn from_raw(raw: RawDeparture) -> Result<Self, chrono::ParseError> {
        let time = NaiveTime::parse_from_str(&raw.time, "%H:%M")?;
        Ok(Departure {
            ride_id: RideId::new(raw.ride_id),
            trip_id: raw.trip_id,
            time: time,
            loop_count: raw.loop_count,
        })
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct RawDeparturePattern {
    pub pattern_id: String,
    pub departures: Vec<RawDeparture>
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct RawDeparture {
    pub ride_id: String,
    pub trip_id: String,
    pub time: String,
    pub loop_count: Option<i32>,
}
