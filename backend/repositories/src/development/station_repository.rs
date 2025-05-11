use models::{
    id::{ID, StationId},
    station::Station,
};

use crate::traits::station_repository::StationRepository;

pub struct DevelopmentStationRepository<F>
where
    F: Fn(StationId) -> Vec<StationId>,
{
    calc_join: F,
}

impl<F> DevelopmentStationRepository<F> where F: Fn(StationId) -> Vec<StationId>, {
    pub fn new(calc_join: F) -> Self {
        Self {
            calc_join,
        }
    }
}

impl<F> StationRepository for DevelopmentStationRepository<F>
where
    F: Fn(StationId) -> Vec<StationId>,
{
    fn from_id(
        &self,
        id: models::id::StationId,
    ) -> Result<models::station::Station, Box<dyn std::error::Error + Send + Sync + 'static>> {
        Ok(Station {
            station_id: id.clone(),
            name: id.get_raw_id().clone(),
            pronounce: "".to_string(),
            join: (self.calc_join)(id.clone()),
        })
    }

    fn from_raw(&self, raw: models::station::RawStation) -> models::station::Station {
        Station {
            station_id: StationId::new(raw.station_id),
            name: raw.name.clone(),
            pronounce: raw.pronounce.clone(),
            join: raw.join.into_iter().map(|x| StationId::new(x)).collect(),
        }
    }

    fn check_is_valid(&self, _: models::station::Station) -> bool {
        true
    }
}
