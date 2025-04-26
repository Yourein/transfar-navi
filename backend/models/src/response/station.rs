use serde::Serialize;
use std::convert::From;
use crate::{id::ID, station::Station};

#[derive(Serialize, Debug, Clone)]
pub struct ResStation {
    pub id: String,
    pub name: String,
    pub pronounce: String,
}

impl From<Station> for ResStation {
    fn from(station: Station) -> Self {
        Self {
            id: station.station_id.get_raw_id(),
            name: station.name,
            pronounce: station.pronounce
        }
    }
}