use std::error::Error;

use chrono::NaiveDate;
use log::debug;
use models::{
    calendar::Calendar,
    departure::{Departure, DeparturePattern},
    id::{ID, StationId},
    response::transfar::ResTransfar,
    ride::Ride,
    station::Station,
    timetable::TimeTable,
};
use repositories::{
    impls::station_repository::StationRepositoryImpl, traits::station_repository::StationRepository,
};

pub(crate) fn calc_transfars(
    ride: &Ride,
    departure: &Departure,
    start: &Station,
    station_loop_count: i32,
) -> Vec<Vec<ResTransfar>> {
    let route: Vec<StationId> =
        calc_stop_after(ride.route.clone(), &start.station_id, station_loop_count)
            .into_iter()
            .skip_while(|x| x.is_same_station(&start.station_id))
            .collect();
    let today = NaiveDate::from_ymd_opt(2025, 05, 11).expect("valid ymd");
    let station_repository = StationRepositoryImpl;
    route
        .iter()
        .map(|cur_station| {
            find_valid_transfar(cur_station, &departure, &ride, today, &station_repository)
        })
        .filter(|x| !x.is_empty())
        .collect()
}

/// ある駅よりあとの停車駅を抽出する
/// 形式的にはsplitで右側に寄せる
/// [1, 2, 3, 4, 2, 5]でstart=2のとき[[1], [2, 3, 4], [2, 5]]になる
fn calc_stop_after(
    route: Vec<StationId>,
    start: &StationId,
    station_loop_count: i32,
) -> Vec<StationId> {
    let mut chunks: Vec<Vec<StationId>> = Vec::new();
    let mut current: Vec<StationId> = Vec::new();

    for station in route.into_iter() {
        if station.is_same_station(start) {
            chunks.push(std::mem::take(&mut current));
            current.push(station);
        } else {
            current.push(station);
        }
    }
    if !current.is_empty() {
        chunks.push(current);
    }

    match chunks.get::<usize>(station_loop_count.try_into().unwrap()) {
        Some(x) => x.to_owned(),
        None => vec![],
    }
}

/// あるdeparturesよりあとのdeparturesをすべて抽出する。
/// ride_id (系統) が同じものも含めて返すことに注意。
fn calc_depart_after(departures: Vec<Departure>, transfar_from: &Departure) -> Vec<Departure> {
    departures
        .iter()
        .skip_while(|x| {
            x.trip_id != transfar_from.trip_id || x.loop_count != transfar_from.loop_count
        })
        .skip(1)
        .map(|x| x.to_owned())
        .collect()
}

fn find_valid_transfar(
    cur_station: &StationId,
    transfar_from: &Departure,
    cur_ride: &Ride,
    today: NaiveDate, // todo: naivedate zl current datetime.
    station_repository: &dyn StationRepository,
) -> Vec<ResTransfar> {
    let Ok(timetable) = TimeTable::from_station_id(cur_station.clone()) else {
        debug!(
            "Timetable for station {:?} not found.",
            cur_station.get_raw_id()
        );
        return vec![];
    };
    let Some(current_calendar) = timetable.get_valid_calendar(today) else {
        debug!(
            "No valid calendar found in timetable {:?}, date: {:?}",
            cur_station.get_raw_id(),
            today
        );
        return vec![];
    };
    let Ok(calendar_res) = Calendar::from_id(current_calendar.calendar_id) else {
        return vec![];
    };
    let pattern_id = calendar_res.get_pattern_id(today);
    let Ok(departure_pattern) = DeparturePattern::from_id(pattern_id) else {
        return vec![];
    };

    let mut departures = departure_pattern.departures;
    departures.sort_by_key(|x| x.time);

    let depart_after = calc_depart_after(departures, transfar_from);

    depart_after
        .into_iter()
        .filter_map(|x| {
            // fixme: ここどうにかしたい
            let Ok(target_ride) = Ride::from_id(x.ride_id.get_raw_id()) else {
                return None;
            };
            calc_transfar(
                cur_ride,
                &transfar_from,
                cur_station,
                &x,
                &target_ride,
                station_repository,
            )
            .ok()
        })
        .collect::<Vec<ResTransfar>>()
}

/// 2つのdepartureを対象にtransfar_fromからtarget_departureに乗り継ぐことが有効化を判定する
/// 無効な乗り換えの場合はErrが返る。
/// 0分乗り換えは許容しているので、フロント側で出さないようにするか、警告文を出すなどで対応する。
fn calc_transfar(
    cur_ride: &Ride,
    transfar_from: &Departure,
    transfar_at: &StationId,
    target_departure: &Departure,
    target_ride: &Ride,
    station_repository: &dyn StationRepository,
) -> Result<ResTransfar, Box<dyn Error + Send + Sync + 'static>> {
    // 認められた乗り換え
    // - 全く行き先が違うやつ
    // - 戻らない
    // - 行き先は同じだけれども今の便が通らない駅に停まるやつ
    //   - ただし、次に合流する前の駅に向かうことにする

    let route = cur_ride.route.clone();
    let target_route = target_ride.route.clone();
    let valid_destinations: Vec<StationId> =
        calc_stop_after(target_route, transfar_at, target_departure.loop_count)
            .into_iter()
            .skip_while(|x| x.is_same_station(&transfar_at))
            .take_while(|x| route.iter().find(|y| y.is_same_station(x)) == None)
            .collect();

    if valid_destinations.is_empty() {
        return Err("There are no valid transfars.".into());
    }

    let transfar_to_id = valid_destinations.last().unwrap().to_owned();
    let transfar_to = station_repository.from_id(transfar_to_id)?;
    let transfar_time = (target_departure.time - transfar_from.time).num_minutes();
    if transfar_time < 0 {
        return Err("There are no time to change.".into());
    }
    Ok(ResTransfar {
        ride_type: target_ride.ride_type.clone(),
        type_foreground: target_ride.type_foreground.clone(),
        type_background: target_ride.type_background.clone(),
        to: transfar_to.into(),
        career_type: target_ride.career_type.clone(),
        depart_at: target_departure.time.format("%H:%M").to_string(),
        transfar_time: transfar_time,
    })
}

#[allow(non_snake_case)]
#[cfg(test)]
mod test {
    use super::{calc_depart_after, calc_stop_after, calc_transfar};
    use chrono::NaiveTime;
    use models::departure::Departure;
    use models::id::{ID, RideId, StationId};
    use models::response::transfar::ResTransfar;
    use models::ride::Ride;
    use repositories::development::station_repository::DevelopmentStationRepository;
    use repositories::traits::station_repository::StationRepository;

    fn dummy_ride(to: StationId, route: Vec<StationId>) -> Ride {
        Ride {
            ride_type: "".to_string(),
            aka_type: "".to_string(),
            type_foreground: "".to_string(),
            type_background: "".to_string(),
            type_pronounce: "".to_string(),
            to: to,
            career_type: "".to_string(),
            route: route,
        }
    }

    #[test]
    fn stop_afterを正しく計算できる_通常ケース() {
        let route: Vec<StationId> = (1..=10).map(|x| StationId::new(x.to_string())).collect();
        let start = StationId::new(4.to_string());
        let actual = calc_stop_after(route.clone(), &start, 1);
        let expected: Vec<StationId> = (4..=10).map(|x| StationId::new(x.to_string())).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn stop_afterを正しく計算できる_環状ケース() {
        let route: Vec<StationId> = vec![
            StationId::new("Hikarigaoka".to_string()),
            StationId::new("Nakano-Sakaue".to_string()),
            StationId::new("Tochomae~1".to_string()),
            StationId::new("Shinjuku".to_string()),
            StationId::new("Kokuritsu-kyogijo".to_string()),
            StationId::new("Daimon".to_string()),
            StationId::new("Ueno-Okachimachi".to_string()),
            StationId::new("Idabashi".to_string()),
            StationId::new("Tochomae~2".to_string()),
            StationId::new("Nishi-Shinjuku-Gochome".to_string()),
        ];
        let start = StationId::new("Tochomae".to_string());
        let actual = calc_stop_after(route.clone(), &start, 1);
        let expected: Vec<StationId> = vec![
            StationId::new("Tochomae~1".to_string()),
            StationId::new("Shinjuku".to_string()),
            StationId::new("Kokuritsu-kyogijo".to_string()),
            StationId::new("Daimon".to_string()),
            StationId::new("Ueno-Okachimachi".to_string()),
            StationId::new("Idabashi".to_string()),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn stop_afterを正しく計算できる_環状ケース2回目以降() {
        let route: Vec<StationId> = vec![
            StationId::new("Hikarigaoka".to_string()),
            StationId::new("Nakano-Sakaue".to_string()),
            StationId::new("Tochomae~1".to_string()),
            StationId::new("Shinjuku".to_string()),
            StationId::new("Kokuritsu-kyogijo".to_string()),
            StationId::new("Daimon".to_string()),
            StationId::new("Ueno-Okachimachi".to_string()),
            StationId::new("Idabashi".to_string()),
            StationId::new("Tochomae~2".to_string()),
            StationId::new("Nishi-Shinjuku-Gochome".to_string()),
        ];
        let start = StationId::new("Tochomae".to_string());
        let actual = calc_stop_after(route.clone(), &start, 2);
        let expected: Vec<StationId> = vec![
            StationId::new("Tochomae~2".to_string()),
            StationId::new("Nishi-Shinjuku-Gochome".to_string()),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn このあと出発するDepartureを正しく抽出できる() {
        let departures: Vec<Departure> = (0..5)
            .into_iter()
            .map(|x| Departure {
                ride_id: RideId::new("TargetRide".to_string()),
                trip_id: x.to_string(),
                time: NaiveTime::from_hms_opt(x + 7, 0, 0).expect("Valid hms"),
                loop_count: 0,
            })
            .collect();
        let cur_ride = departures[3].clone();
        let actual_depart_after = calc_depart_after(departures, &cur_ride);

        let expected_depart_after = vec![Departure {
            ride_id: RideId::new("TargetRide".to_string()),
            trip_id: "4".to_string(),
            time: NaiveTime::from_hms_opt(11, 0, 0).expect("Valid hms"),
            loop_count: 0,
        }];

        assert_eq!(expected_depart_after, actual_depart_after);
    }

    #[test]
    fn loop_countがあってもこのあと出発するDepartureを正しく抽出できる() {
        let departures = vec![
            Departure {
                ride_id: RideId::new("TargetRide".to_string()),
                trip_id: "0".to_string(),
                time: NaiveTime::from_hms_opt(7, 0, 0).expect("Valid hms"),
                loop_count: 0,
            },
            Departure {
                ride_id: RideId::new("TargetRide".to_string()),
                trip_id: "0".to_string(),
                time: NaiveTime::from_hms_opt(7, 0, 0).expect("Valid hms"),
                loop_count: 1,
            },
            Departure {
                ride_id: RideId::new("TargetRide".to_string()),
                trip_id: "1".to_string(),
                time: NaiveTime::from_hms_opt(7, 0, 0).expect("Valid hms"),
                loop_count: 0,
            },
            Departure {
                ride_id: RideId::new("TargetRide".to_string()),
                trip_id: "2".to_string(),
                time: NaiveTime::from_hms_opt(7, 0, 0).expect("Valid hms"),
                loop_count: 0,
            },
        ];

        let cur_departure = Departure {
            ride_id: RideId::new("TargetRide".to_string()),
            trip_id: "0".to_string(),
            time: NaiveTime::from_hms_opt(7, 0, 0).expect("Valid hms"),
            loop_count: 1,
        };

        let expected_depart_after: Vec<Departure> = vec![
            Departure {
                ride_id: RideId::new("TargetRide".to_string()),
                trip_id: "1".to_string(),
                time: NaiveTime::from_hms_opt(7, 0, 0).expect("Valid hms"),
                loop_count: 0,
            },
            Departure {
                ride_id: RideId::new("TargetRide".to_string()),
                trip_id: "2".to_string(),
                time: NaiveTime::from_hms_opt(7, 0, 0).expect("Valid hms"),
                loop_count: 0,
            },
        ];

        let actual_depart_after = calc_depart_after(departures, &cur_departure);
        assert_eq!(expected_depart_after, actual_depart_after);
    }

    #[test]
    fn そもそもtransfar_fromがdeparturesの中に存在しないときは空Vecを返す() {
        let departures: Vec<Departure> = (0..5)
            .into_iter()
            .map(|x| Departure {
                ride_id: RideId::new("TargetRide".to_string()),
                trip_id: x.to_string(),
                time: NaiveTime::from_hms_opt(x + 7, 0, 0).expect("Valid hms"),
                loop_count: 0,
            })
            .collect();
        let transfar_from = Departure {
            ride_id: RideId::new("TargetRide".to_string()),
            trip_id: "0".to_string(),
            time: NaiveTime::from_hms_opt(7, 0, 0).expect("Valid hms"),
            loop_count: 1,
        };
        let actual = calc_depart_after(departures, &transfar_from);
        let expected: Vec<Departure> = Vec::new();
        assert_eq!(expected, actual);
    }

    #[test]
    fn valid_transfar_ナイーブなケース() {
        let cur_station = StationId::new("03".to_string());
        let transfar_from = Departure {
            ride_id: RideId::new("A".to_string()),
            trip_id: "A".to_string(),
            time: NaiveTime::from_hms_opt(12, 0, 0).expect("valid hms"),
            loop_count: 1,
        };
        let cur_ride = dummy_ride(
            StationId::new("04".to_string()),
            vec![
                StationId::new("01".to_string()),
                StationId::new("02".to_string()),
                StationId::new("03".to_string()),
                StationId::new("04".to_string()),
            ],
        );
        let target_departure = Departure {
            ride_id: RideId::new("B".to_string()),
            trip_id: "B".to_string(),
            time: NaiveTime::from_hms_opt(12, 10, 0).expect("valid hms"),
            loop_count: 1,
        };
        let target_ride = dummy_ride(
            StationId::new("07".to_string()),
            vec![
                StationId::new("05".to_string()),
                StationId::new("03".to_string()),
                StationId::new("06".to_string()),
                StationId::new("07".to_string()),
            ],
        );

        let station_repository = DevelopmentStationRepository::new(|_| vec![]);
        let actual = calc_transfar(
            &cur_ride,
            &transfar_from,
            &cur_station,
            &target_departure,
            &target_ride,
            &station_repository,
        )
        .unwrap();
        let expected = ResTransfar {
            ride_type: "".to_string(),
            type_foreground: "".to_string(),
            type_background: "".to_string(),
            to: station_repository
                .from_id(StationId::new("07".to_string()))
                .unwrap()
                .into(),
            career_type: "".to_string(),
            depart_at: "12:10".to_string(),
            transfar_time: 10,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn calc_transfar_invalid_戻るケース() {
        let cur_station = StationId::new("02".to_string());
        let transfar_from = Departure {
            ride_id: RideId::new("A".to_string()),
            trip_id: "A".to_string(),
            time: NaiveTime::from_hms_opt(12, 0, 0).expect("valid hms"),
            loop_count: 1,
        };
        let cur_ride = dummy_ride(
            StationId::new("03".to_string()),
            vec![
                StationId::new("01".to_string()),
                StationId::new("02".to_string()),
                StationId::new("03".to_string()),
            ],
        );
        let target_departure = Departure {
            ride_id: RideId::new("B".to_string()),
            trip_id: "B".to_string(),
            time: NaiveTime::from_hms_opt(12, 10, 0).expect("valid hms"),
            loop_count: 1,
        };
        let target_ride = dummy_ride(
            StationId::new("01".to_string()),
            vec![
                StationId::new("04".to_string()),
                StationId::new("02".to_string()),
                StationId::new("01".to_string()),
            ],
        );

        let station_repository = DevelopmentStationRepository::new(|_| vec![]);
        let actual = calc_transfar(
            &cur_ride,
            &transfar_from,
            &cur_station,
            &target_departure,
            &target_ride,
            &station_repository,
        );
        assert!(actual.is_err());
    }

    #[test]
    fn calc_transfar_valid_合流するけど別ルートなケース() {
        let cur_station = StationId::new("02".to_string());
        let transfar_from = Departure {
            ride_id: RideId::new("A".to_string()),
            trip_id: "A".to_string(),
            time: NaiveTime::from_hms_opt(12, 0, 0).expect("valid hms"),
            loop_count: 1,
        };
        let cur_ride = dummy_ride(
            StationId::new("03".to_string()),
            vec![
                StationId::new("01".to_string()),
                StationId::new("02".to_string()),
                StationId::new("03".to_string()),
            ],
        );
        let target_departure = Departure {
            ride_id: RideId::new("B".to_string()),
            trip_id: "B".to_string(),
            time: NaiveTime::from_hms_opt(12, 10, 0).expect("valid hms"),
            loop_count: 1,
        };
        let target_ride = dummy_ride(
            StationId::new("01".to_string()),
            vec![
                StationId::new("04".to_string()),
                StationId::new("02".to_string()),
                StationId::new("05".to_string()),
                StationId::new("06".to_string()),
                StationId::new("01".to_string()),
            ],
        );

        let station_repository = DevelopmentStationRepository::new(|_| vec![]);
        let actual = calc_transfar(
            &cur_ride,
            &transfar_from,
            &cur_station,
            &target_departure,
            &target_ride,
            &station_repository,
        )
        .unwrap();
        let expected = ResTransfar {
            ride_type: "".to_string(),
            type_foreground: "".to_string(),
            type_background: "".to_string(),
            to: station_repository
                .from_id(StationId::new("06".to_string()))
                .unwrap()
                .into(),
            career_type: "".to_string(),
            depart_at: "12:10".to_string(),
            transfar_time: 10,
        };
        assert_eq!(expected, actual);
    }

    #[test]
    fn calc_transfar_invalid_相手側が終点なケース() {
        let cur_station = StationId::new("02".to_string());
        let transfar_from = Departure {
            ride_id: RideId::new("A".to_string()),
            trip_id: "A".to_string(),
            time: NaiveTime::from_hms_opt(12, 0, 0).expect("valid hms"),
            loop_count: 1,
        };
        let cur_ride = dummy_ride(
            StationId::new("03".to_string()),
            vec![
                StationId::new("01".to_string()),
                StationId::new("02".to_string()),
                StationId::new("03".to_string()),
            ],
        );
        let target_departure = Departure {
            ride_id: RideId::new("B".to_string()),
            trip_id: "B".to_string(),
            time: NaiveTime::from_hms_opt(12, 10, 0).expect("valid hms"),
            loop_count: 1,
        };
        let target_ride = dummy_ride(
            StationId::new("02".to_string()),
            vec![
                StationId::new("04".to_string()),
                StationId::new("02".to_string()),
            ],
        );

        let station_repository = DevelopmentStationRepository::new(|_| vec![]);
        let actual = calc_transfar(
            &cur_ride,
            &transfar_from,
            &cur_station,
            &target_departure,
            &target_ride,
            &station_repository,
        );
        assert!(actual.is_err());
    }

    #[test]
    fn calc_transfar_invalid_並走区間() {
        let cur_station = StationId::new("02".to_string());
        let transfar_from = Departure {
            ride_id: RideId::new("A".to_string()),
            trip_id: "A".to_string(),
            time: NaiveTime::from_hms_opt(12, 0, 0).expect("valid hms"),
            loop_count: 1,
        };
        let cur_ride = dummy_ride(
            StationId::new("03".to_string()),
            vec![
                StationId::new("01".to_string()),
                StationId::new("02".to_string()),
                StationId::new("03".to_string()),
                StationId::new("04".to_string()),
            ],
        );
        let target_departure = Departure {
            ride_id: RideId::new("B".to_string()),
            trip_id: "B".to_string(),
            time: NaiveTime::from_hms_opt(12, 10, 0).expect("valid hms"),
            loop_count: 1,
        };
        let target_ride = dummy_ride(
            StationId::new("02".to_string()),
            vec![
                StationId::new("05".to_string()),
                StationId::new("02".to_string()),
                StationId::new("03".to_string()),
                StationId::new("04".to_string()),
                StationId::new("06".to_string()),
            ],
        );

        let station_repository = DevelopmentStationRepository::new(|_| vec![]);
        let actual = calc_transfar(
            &cur_ride,
            &transfar_from,
            &cur_station,
            &target_departure,
            &target_ride,
            &station_repository,
        );
        assert!(actual.is_err());
    }

    #[test]
    fn calc_transfar_valid_相手側が始点なケース() {
        let cur_station = StationId::new("02".to_string());
        let transfar_from = Departure {
            ride_id: RideId::new("A".to_string()),
            trip_id: "A".to_string(),
            time: NaiveTime::from_hms_opt(12, 0, 0).expect("valid hms"),
            loop_count: 1,
        };
        let cur_ride = dummy_ride(
            StationId::new("03".to_string()),
            vec![
                StationId::new("01".to_string()),
                StationId::new("02".to_string()),
                StationId::new("03".to_string()),
            ],
        );
        let target_departure = Departure {
            ride_id: RideId::new("B".to_string()),
            trip_id: "B".to_string(),
            time: NaiveTime::from_hms_opt(12, 10, 0).expect("valid hms"),
            loop_count: 1,
        };
        let target_ride = dummy_ride(
            StationId::new("04".to_string()),
            vec![
                StationId::new("02".to_string()),
                StationId::new("04".to_string()),
            ],
        );

        let station_repository = DevelopmentStationRepository::new(|_| vec![]);
        let actual = calc_transfar(
            &cur_ride,
            &transfar_from,
            &cur_station,
            &target_departure,
            &target_ride,
            &station_repository,
        )
        .unwrap();
        let expected = ResTransfar {
            ride_type: "".to_string(),
            type_foreground: "".to_string(),
            type_background: "".to_string(),
            to: station_repository
                .from_id(StationId::new("04".to_string()))
                .unwrap()
                .into(),
            career_type: "".to_string(),
            depart_at: "12:10".to_string(),
            transfar_time: 10,
        };
        assert_eq!(actual, expected);
    }
}
