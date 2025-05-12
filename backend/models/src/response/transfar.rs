use serde::Serialize;
use super::station::ResStation;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ResTransfar {
    pub ride_type: String,
    pub type_foreground: String,
    pub type_background: String,
    pub at: ResStation,
    pub to: ResStation,
    pub career_type: String,
    pub depart_at: String, // hh:mm
    pub transfar_time: i64,
}