pub mod station;
pub mod departure;
pub mod calendar;
pub mod id;
pub mod timetable;
pub mod transfar;
pub mod response;
pub mod ride;

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::id::{CalendarId, DeparturePatternId, RideId, StationId, TimeTableId, ID, ROOT_PATH};
    use crate::ride::{RawRide, Ride};
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
            pronounce: "てすと".to_string(),
            join: [
                "test_station_02".to_string()
            ].to_vec()
        };
        assert_eq!(actual, expected);
    }

    // todo: このテストはrepositoriesに移す
    #[allow(deprecated)]
    #[test]
    fn parse_station() {
        let f = File::open("/home/yourein/Codes/transfar-navi/backend/models/test_data/station/test/test_station.json").unwrap();
        let reader = BufReader::new(f);
        let raw: RawStation = serde_json::from_reader(reader).unwrap();
        let actual = Station::from_raw(raw);
        let expected = Station {
            station_id: StationId::new("test_station".to_string()),
            name: "TestStation".to_string(),
            pronounce: "てすと".to_string(),
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
                    time: "00:00".to_string(),
                    loop_count: None,
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
                    time: NaiveTime::from_hms_opt(00, 00, 00).expect("Valid Time"),
                    loop_count: 1,
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

    #[test]
    fn can_parse_ride() {
        let f = File::open("/home/yourein/Codes/transfar-navi/backend/models/test_data/55G-D.json").unwrap();
        let reader = BufReader::new(f);
        let raw: RawRide = serde_json::from_reader(reader).unwrap();
        let actual = Ride::from_raw(raw);

        let expected = Ride {
            ride_type: "55G".to_string(),
            aka_type: "".to_string(),
            type_foreground: "#FFD700".to_string(),
            type_background: "#000000".to_string(),
            type_pronounce: "ごじゅうご じーけいとう".to_string(),
            to: StationId::new("HAKODATEBUS_050019".to_string()),
            career_type: "BUS".to_string(),
            route: vec![
                StationId::new("HAKODATEBUS_050004".to_string()),
                StationId::new("HAKODATEBUS_050005".to_string()),
                StationId::new("HAKODATEBUS_050016".to_string()),
                StationId::new("HAKODATEBUS_050019".to_string())
            ]
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn ループ回数と補助情報付きのStationIdから正しくidやループ回数が取得できる() {
        let id = StationId {
            id: "TAGENCY_0001-D~2".to_string(),
            data_root_path: "/hoge/fuga".to_string(),
        };

        let actual_raw_id = id.get_raw_id();
        let expected_raw_id = "TAGENCY_0001-D".to_string();
        assert_eq!(expected_raw_id, actual_raw_id);

        let actual_loop_count = id.get_loop_count();
        let expected_loop_count = 2;
        assert_eq!(expected_loop_count, actual_loop_count);
    }

    #[test]
    fn ループ回数のみがついたStationidから正しくidやループ回数が取得できる() {
        let id = StationId {
            id: "TAGENCY_0001~2".to_string(),
            data_root_path: "/hoge/fuga".to_string(),
        };

        let actual_raw_id = id.get_raw_id();
        let expected_raw_id = "TAGENCY_0001".to_string();
        assert_eq!(expected_raw_id, actual_raw_id);

        let actual_loop_count = id.get_loop_count();
        let expected_loop_count = 2;
        assert_eq!(expected_loop_count, actual_loop_count);
    }

    #[test]
    fn 補助情報のみのStationIdから正しくidやループ回数が取得できる() {
        let id = StationId {
            id: "TAGENCY_0001-1".to_string(),
            data_root_path: "/hoge/fuga".to_string(),
        };

        let actual_raw_id = id.get_raw_id();
        let expected_raw_id = "TAGENCY_0001-1".to_string();
        assert_eq!(expected_raw_id, actual_raw_id);

        let actual_loop_count = id.get_loop_count();
        let expected_loop_count = 1;
        assert_eq!(expected_loop_count, actual_loop_count);
    }

    #[test]
    fn ループ回数も補助情報もないStationIdから正しくidやループ回数が取得できる() {
        let id = StationId {
            id: "TAGENCY_0001".to_string(),
            data_root_path: "/hoge/fuga".to_string(),
        };

        let actual_raw_id = id.get_raw_id();
        let expected_raw_id = "TAGENCY_0001".to_string();
        assert_eq!(expected_raw_id, actual_raw_id);

        let actual_loop_count = id.get_loop_count();
        let expected_loop_count = 1;
        assert_eq!(expected_loop_count, actual_loop_count);
    }

    #[test]
    fn ループ回数のあるなしに限らずStationIdからTimetableIdが作れる() {
        let id_loop = StationId {
            id: "TEST~1".to_string(),
            data_root_path: "/hoge/fuga".to_string(),
        };
        let id_no_loop = StationId {
            id: "TEST".to_string(),
            data_root_path: "/hoge/fuga".to_string()
        };
        let actual_loop = id_loop.to_timetable_id();
        let actual_no_loop = id_no_loop.to_timetable_id();
        let expected_loop = TimeTableId {
            id: "TEST".to_string(),
            data_root_path: "/home/yourein/Codes/transfar-navi/backend/data".to_string(),
        };
        let expected_no_loop = TimeTableId {
            id: "TEST".to_string(),
            data_root_path: "/home/yourein/Codes/transfar-navi/backend/data".to_string(),
        };
        assert_eq!(actual_loop, expected_loop);
        assert_eq!(actual_no_loop, expected_no_loop);
    }
}
