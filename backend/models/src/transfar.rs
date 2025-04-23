use chrono::Duration;

use crate::{departure::Departure, station::Station};

#[derive(Debug, Clone, PartialEq)]
pub struct Transfar {
    pub transfar_at: Station,
    pub transfar_time: Duration,
    pub connect_to: Departure
}