use chrono::Duration;

use crate::{departure::Departure, station::Station};

#[derive(Debug, Clone, PartialEq)]
pub struct Transfar {
    transfar_at: Station,
    transfar_time: Duration,
    connect_to: Departure
}