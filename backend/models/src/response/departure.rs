use serde::Serialize;

use super::{station::ResStation, transfar::ResTransfar};

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ResDeparture {
    pub ride_type: String,
    pub aka_type: Option<String>,
    pub type_foreground: String,
    pub type_background: String,
    pub type_pronounce: String,
    pub to: ResStation,
    pub career_type: String,
    pub depart_at: String,
    pub transfars: Vec<Vec<ResTransfar>>
}