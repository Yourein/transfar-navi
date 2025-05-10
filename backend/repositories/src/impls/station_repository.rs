use std::{error::Error, fs::File, io::BufReader, path::Path};
use models::{id::{StationId, ID}, station::{RawStation, Station}};
use crate::traits;

pub struct StationRepository;

impl traits::station_repository::StationRepository for StationRepository {
    fn from_id(&self, id: models::id::StationId) -> Result<Station, Box<dyn Error + Send + Sync + 'static>> {
        let path = id.build_path();
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let raw: RawStation = serde_json::from_reader(reader)?;

        Ok(self.from_raw(raw))
    }

    fn from_raw(&self, raw: models::station::RawStation) -> Station {
        Station {
            station_id: StationId::new(raw.station_id.clone()),
            name: raw.name,
            pronounce: raw.pronounce,
            join: raw.join.iter().map(|x| StationId::new(x.to_owned())).collect()
        }
    }

    fn check_is_valid(&self, station: Station) -> bool {
        let timetable_path = station.station_id.to_timetable_id().build_path();
        Path::new(&timetable_path).exists()
    }
}