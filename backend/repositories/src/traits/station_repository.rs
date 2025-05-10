use std::error::Error;

use models::id::StationId;
use models::station::{RawStation, Station};

pub trait StationRepository {
    fn from_id(&self, id: StationId) -> Result<Station, Box<dyn Error + Send + Sync + 'static>>;
    fn from_raw(&self, raw: RawStation) -> Station;
    fn check_is_valid(&self, station: Station) -> bool;
}