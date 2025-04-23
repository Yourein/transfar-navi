use std::{error::Error, fs::File, io::BufReader};

use serde::Deserialize;
use chrono::NaiveDate;
use crate::id::{CalendarId, StationId, ID};

#[derive(Debug, PartialEq)]
pub struct TimeTable {
    pub(crate) station_id: String,
    pub(crate) versions: Vec<CalendarVersion>
}

impl TimeTable {
    #[allow(dead_code)]
    pub(crate) fn from_raw(raw: RawTimetable) -> Result<Self, chrono::ParseError> {
        let versions = raw
            .versions
            .into_iter()
            .map(CalendarVersion::from_raw)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TimeTable {
            station_id: raw.station_id,
            versions,
        })
    }

    #[allow(dead_code)]
    pub fn from_station_id(id: StationId) -> Result<Self, Box<dyn Error + Sync + Send + 'static>> {
        let path = id.to_timetable_id().build_path();
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let raw: RawTimetable = serde_json::from_reader(reader)?;

        Ok(Self::from_raw(raw)?)
    }

    #[allow(dead_code)]
    pub fn get_valid_calendar(&self, date: NaiveDate) -> Option<CalendarVersion> {
        let first_valid_version = self.versions
            .iter()
            .filter(|x| date <= x.valid_until)
            .nth(0);

        return first_valid_version.cloned()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalendarVersion {
    pub calendar_id: CalendarId,
    pub valid_until: NaiveDate,
}

impl CalendarVersion {
    fn from_raw(raw: RawVersion) -> Result<Self, chrono::ParseError> {
        let valid_until = NaiveDate::parse_from_str(&raw.valid_until, "%Y/%m/%d")?;
        Ok(CalendarVersion {
            calendar_id: CalendarId::new(raw.calendar_id),
            valid_until,
        })
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct RawTimetable {
    station_id: String,
    pub versions: Vec<RawVersion>
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct RawVersion {
    pub calendar_id: String,
    pub valid_until: String
}
