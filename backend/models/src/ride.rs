use std::{fs::File, io::BufReader};
use std::error::Error;
use serde::Deserialize;
use crate::id::{RideId, StationId, ID};

#[derive(Debug, Clone, PartialEq)]
pub struct Ride {
    pub ride_type: String,
    pub aka_type: String,
    pub type_foreground: String,
    pub type_background: String,
    pub type_pronounce: String,
    pub to: StationId,
    pub career_type: String,
    pub route: Vec<StationId>
}

#[allow(dead_code)]
impl Ride {
    pub fn from_raw(raw: RawRide) -> Self {
        Self {
            ride_type: raw.ride_type,
            aka_type: raw.aka_type,
            type_foreground: raw.type_foreground,
            type_background: raw.type_background,
            type_pronounce: raw.type_pronounce,
            to: StationId::new(raw.to.to_string()),
            career_type: raw.career_type,
            route: raw.route.iter().map(|x| StationId::new(x.clone())).collect()
        }
    }

    pub fn from_id(id: String) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let f = File::open(RideId::new(id).build_path())?;
        let reader = BufReader::new(f);
        let raw: RawRide = serde_json::from_reader(reader)?;
        Ok(Self::from_raw(raw))
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct RawRide {
    pub(crate) ride_id: String,
    #[serde(rename(deserialize = "type"))] pub ride_type: String,
    pub aka_type: String,
    pub type_foreground: String,
    pub type_background: String,
    pub type_pronounce: String,
    pub to: String,
    pub career_type: String,
    pub route: Vec<String>,
}