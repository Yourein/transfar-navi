use serde::Serialize;

use super::departure::ResDeparture;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ResDepartures {
    pub departures: Vec<ResDeparture>
}