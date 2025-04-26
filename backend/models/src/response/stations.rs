use serde::Serialize;
use super::station::ResStation;

#[derive(Serialize, Debug, Clone)]
pub struct ResStations {
    pub stations: Vec<ResStation>
}