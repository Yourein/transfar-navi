use serde::Deserialize;
use chrono::NaiveDate;
use crate::id::{CalendarId, ID};

#[derive(Debug, PartialEq)]
pub struct TimeTable {
    pub(crate) station_id: String,
    pub(crate) versions: Vec<CalendarVersion>
}

impl TimeTable {
    pub fn from_raw(raw: RawTimetable) -> Result<Self, chrono::ParseError> {
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
pub struct RawTimetable {
    station_id: String,
    pub versions: Vec<RawVersion>
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RawVersion {
    pub calendar_id: String,
    pub valid_until: String
}
