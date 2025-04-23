use std::{error::Error, fs::File, io::BufReader};

use serde::Deserialize;
use chrono::{Datelike, NaiveDate, Weekday};
use crate::id::{CalendarId, DeparturePatternId, ID};

// fixme: 曜日とかexceptionのpattern_idの型をstringじゃなくてちゃんとしたwrapperにしたい。

#[derive(Debug, PartialEq)]
pub struct Calendar {
    pub calendar_id: String,
    pub monday: DeparturePatternId,
    pub tuesday: DeparturePatternId,
    pub wednesday: DeparturePatternId,
    pub thursday: DeparturePatternId,
    pub friday: DeparturePatternId,
    pub saturday: DeparturePatternId,
    pub sunday: DeparturePatternId,
    pub exception: Vec<CalendarException>
}

impl Calendar {
    #[allow(dead_code)]
    pub(crate) fn from_raw(raw: RawCalendar) -> Result<Self, chrono::ParseError> {
        let exceptions: Result<Vec<CalendarException>, chrono::ParseError> = raw.exception
                .iter()
                .map(|x| CalendarException::from_raw(x))
                .collect();
        
        match exceptions {
            Ok(exception) => {
                let res = Self {
                    calendar_id: raw.calendar_id,
                    monday: DeparturePatternId::new(raw.monday),
                    tuesday: DeparturePatternId::new(raw.tuesday),
                    wednesday: DeparturePatternId::new(raw.wednesday),
                    thursday: DeparturePatternId::new(raw.thursday),
                    friday: DeparturePatternId::new(raw.friday),
                    saturday: DeparturePatternId::new(raw.saturday),
                    sunday: DeparturePatternId::new(raw.sunday),
                    exception: exception
                };
                Ok(res)
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    #[allow(dead_code)]
    pub fn from_id(calendar_id: CalendarId) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let path = calendar_id.build_path();
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let raw: RawCalendar = serde_json::from_reader(reader)?;

        Ok(Self::from_raw(raw)?)
    }

    #[allow(dead_code)]
    pub fn get_pattern_id(&self, date: NaiveDate) -> DeparturePatternId {
        let exception = self.exception.iter().find(|x| x.date == date);
        if exception.is_some() {
            return exception.unwrap().pattern_id.clone()
        }
        
        let weekday = date.weekday();
        match weekday {
            Weekday::Mon => self.monday.clone(),
            Weekday::Tue => self.tuesday.clone(),
            Weekday::Wed => self.wednesday.clone(),
            Weekday::Thu => self.thursday.clone(),
            Weekday::Fri => self.friday.clone(),
            Weekday::Sat => self.saturday.clone(),
            Weekday::Sun => self.sunday.clone()
        }
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct RawCalendar {
    pub calendar_id: String,
    pub monday: String,
    pub tuesday: String,
    pub wednesday: String,
    pub thursday: String,
    pub friday: String,
    pub saturday: String,
    pub sunday: String,
    pub exception: Vec<RawCalendarException>
}

#[derive(Deserialize, Debug, PartialEq)]
pub(crate) struct RawCalendarException {
    pub date: String,
    pub pattern_id: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct CalendarException {
    pub date: NaiveDate,
    pub pattern_id: DeparturePatternId,
}

impl CalendarException {
    #[allow(dead_code)]
    pub fn from_raw(raw: &RawCalendarException) -> Result<Self, chrono::ParseError> {
        let date = NaiveDate::parse_from_str(&raw.date, "%Y/%m/%d")?;
        Ok(CalendarException {
            date,
            pattern_id: DeparturePatternId::new(raw.pattern_id.clone()),
        })
    }
}
