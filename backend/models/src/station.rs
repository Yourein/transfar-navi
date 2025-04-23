use serde::Deserialize;

use crate::id::{StationId, ID};

#[derive(Debug, Clone, PartialEq)]
pub struct Station {
    pub(crate) station_id: StationId,
    pub name: String,
    pub join: Vec<StationId>
}

impl Station {
    pub fn from_raw(raw: RawStation) -> Self {
        Self {
            station_id: StationId::new(raw.station_id.clone()),
            name: raw.name,
            join: raw.join.iter().map(|x| StationId::new(x.clone())).collect()
        }
    }
}


#[allow(dead_code)]
#[derive(Deserialize, Debug, PartialEq)]
pub struct RawStation {
    pub station_id: String,
    pub name: String,
    pub join: Vec<String>
}
