mod station;
mod departure;
mod calendar;
mod id;
mod timetable;
mod transfar;

#[cfg(test)]
mod tests {
    use crate::id::{CalendarId, DeparturePatternId, RideId, StationId, ID, ROOT_PATH};
    use crate::station::{RawStation, Station};
    use crate::departure::{Departure, DeparturePattern, RawDeparture, RawDeparturePattern};
    use crate::calendar::{Calendar, CalendarException, RawCalendar, RawCalendarException};
    use crate::timetable::{CalendarVersion, RawTimetable, TimeTable};
    use chrono::{NaiveDate, NaiveTime};
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn parse_raw_station() {
        let f = File::open("/home/yourein/Codes/transfar-navi/backend/models/test_data/station/test/test_station.json").unwrap();
        let reader = BufReader::new(f);

        let actual: RawStation = serde_json::from_reader(reader).unwrap();
        let expected = RawStation {
            station_id: "test_station".to_string(),
            name: "TestStation".to_string(),
            join: [
                "test_station_02".to_string()
            ].to_vec()
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_station() {
        let f = File::open("/home/yourein/Codes/transfar-navi/backend/models/test_data/station/test/test_station.json").unwrap();
        let reader = BufReader::new(f);
        let raw: RawStation = serde_json::from_reader(reader).unwrap();
        let actual = Station::from_raw(raw);
        let expected = Station {
            station_id: StationId::new("test_station".to_string()),
            name: "TestStation".to_string(),
            join: [
                StationId::new("test_station_02".to_string())
            ].to_vec()
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_raw_departure_pattern() {
        let f = File::open("/home/yourein/Codes/transfar-navi/backend/models/test_data/departure-pattern/0001.json").unwrap();
        let reader = BufReader::new(f);

        let actual: RawDeparturePattern = serde_json::from_reader(reader).unwrap();
        let expected = RawDeparturePattern {
            pattern_id: "test".to_string(),
            departures: [
                RawDeparture {
                    ride_id: "ride_id".to_string(),
                    trip_id: "trip_id".to_string(),
                    time: "00:00".to_string()
                }
            ].to_vec()
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_exception_date() {
        let f = File::open("/home/yourein/Codes/transfar-navi/backend/models/test_data/exception_date.json").unwrap();
        let reader = BufReader::new(f);
        let raw: RawCalendarException = serde_json::from_reader(reader).unwrap();
        let actual = CalendarException::from_raw(&raw).unwrap();

        let expected = CalendarException {
            date: NaiveDate::from_ymd_opt(2025, 04, 22).expect("Valid Date"),
            pattern_id: DeparturePatternId::new("1".to_string())
        };

        assert_eq!(expected, actual)
    }

    #[test]
    fn parse_calendar() {
        let f = File::open("/home/yourein/Codes/transfar-navi/backend/models/test_data/test_calendar.json").unwrap();
        let reader = BufReader::new(f);
        let raw: RawCalendar = serde_json::from_reader(reader).unwrap();
        let actual = Calendar::from_raw(raw).unwrap();

        let expected = Calendar {
            calendar_id: "test_calendar".to_string(),
            monday: DeparturePatternId::new("0001".to_string()),
            tuesday: DeparturePatternId::new("0002".to_string()),
            wednesday: DeparturePatternId::new("0003".to_string()),
            thursday: DeparturePatternId::new("0004".to_string()),
            friday: DeparturePatternId::new("0005".to_string()),
            saturday: DeparturePatternId::new("0006".to_string()),
            sunday: DeparturePatternId::new("0007".to_string()),
            exception: [
                CalendarException {
                    date: NaiveDate::from_ymd_opt(2025, 04, 22).unwrap(),
                    pattern_id: DeparturePatternId::new("0008".to_string()),
                }
            ].to_vec()

        };

        println!{"{:?}", actual};

        assert_eq!(expected, actual);
    }

    #[test]
    fn can_get_right_pattern_id() {
        let f = File::open("/home/yourein/Codes/transfar-navi/backend/models/test_data/test_calendar.json").unwrap();
        let reader = BufReader::new(f);
        let raw: RawCalendar = serde_json::from_reader(reader).unwrap();
        let calendar = Calendar::from_raw(raw).unwrap();

        let date1 = NaiveDate::from_ymd_opt(2025, 04, 22).expect("Valid Date");
        let expected1 = DeparturePatternId::new("0008".to_string());
        let actual1 = calendar.get_pattern_id(date1);
        assert_eq!(expected1, actual1);

        let date2 = NaiveDate::from_ymd_opt(2025, 04, 23).expect("Valid"); // 2025/04/23 is Wednesday
        let expected2 = DeparturePatternId::new("0003".to_string());
        let actual2 = calendar.get_pattern_id(date2);
        assert_eq!(expected2, actual2);
    }

    #[test]
    fn build_departure_pattern_path() {
        let id = DeparturePatternId::new(
            "TESTAGENCY_TEST-RIDE_0001".to_string()
        );

        let actual = id.build_path();
        let expected = ROOT_PATH.to_string() + "/departure-pattern/TESTAGENCY/TEST-RIDE/0001.json";
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_departure_pattern() {
        let f = File::open("/home/yourein/Codes/transfar-navi/backend/models/test_data/departure-pattern/0001.json").unwrap();
        let reader = BufReader::new(f);
        let raw: RawDeparturePattern = serde_json::from_reader(reader).unwrap();
        let actual = DeparturePattern::from_raw(raw).unwrap();
        let expected = DeparturePattern {
            pattern_id: DeparturePatternId::new("test".to_string()),
            departures: [
                Departure {
                    ride_id: RideId::new("ride_id".to_string()),
                    trip_id: "trip_id".to_string(),
                    time: NaiveTime::from_hms_opt(00, 00, 00).expect("Valid Time")
                }
            ].to_vec()
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_time_table() {
        let f = File::open("/home/yourein/Codes/transfar-navi/backend/models/test_data/timetable/test-timetable.json").unwrap();
        let reader = BufReader::new(f);
        let raw: RawTimetable = serde_json::from_reader(reader).unwrap();
        let actual = TimeTable::from_raw(raw).unwrap();

        let expected = TimeTable {
            station_id: "test_station".to_string(),
            versions: [
                CalendarVersion {
                    calendar_id: CalendarId::new("test_calendar_1".to_string()),
                    valid_until: NaiveDate::from_ymd_opt(2025, 04, 01).expect("valid date")
                },
                CalendarVersion {
                    calendar_id: CalendarId::new("test_calendar_2".to_string()),
                    valid_until: NaiveDate::from_ymd_opt(2025, 04, 20).expect("valid date")
                },
                CalendarVersion {
                    calendar_id: CalendarId::new("test_calendar_3".to_string()),
                    valid_until: NaiveDate::from_ymd_opt(2025, 04, 25).expect("valid date")
                }
            ].to_vec()
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn can_get_first_valid_date() {
        let f = File::open("/home/yourein/Codes/transfar-navi/backend/models/test_data/timetable/test-timetable.json").unwrap();
        let reader = BufReader::new(f);
        let raw: RawTimetable = serde_json::from_reader(reader).unwrap();
        let timetable = TimeTable::from_raw(raw).unwrap();

        let actual1 = timetable.get_valid_calendar(NaiveDate::from_ymd_opt(2025, 04, 15).expect("Valid"));
        let expected1 = Some(
            CalendarVersion {
                calendar_id: CalendarId::new("test_calendar_2".to_string()),
                valid_until: NaiveDate::from_ymd_opt(2025, 04, 20).expect("valid date")
            }
        );
        let actual2 = timetable.get_valid_calendar(NaiveDate::from_ymd_opt(2025, 04, 25).expect("Valid"));
        let expected2 = Some(CalendarVersion {
            calendar_id: CalendarId::new("test_calendar_3".to_string()),
            valid_until: NaiveDate::from_ymd_opt(2025, 04, 25).expect("valid date")
        });
        let actual3 = timetable.get_valid_calendar(NaiveDate::from_ymd_opt(2025, 05, 01).expect("valid"));
        let expected3: Option<CalendarVersion> = None;

        assert_eq!([actual1, actual2, actual3], [expected1, expected2, expected3]);
    }
}
