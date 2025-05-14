use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime};
use log::debug;
use models::{
    calendar::Calendar,
    departure::{Departure, DeparturePattern},
    id::{RideId, StationId, ID},
    response::{station::ResStation, transfar::ResTransfar},
    ride::Ride,
    station::Station,
    timetable::TimeTable,
};
use repositories::{
    impls::station_repository::StationRepositoryImpl, traits::station_repository::StationRepository,
};
use std::collections::{HashSet, VecDeque};
use std::error::Error;

#[derive(Clone, Debug)]
pub struct TransfarChain {
    pub departure: Departure,
    pub ride_at: StationId,
    pub ride_for: StationId,
    pub transfar_time: i64,
}

pub(crate) fn calc_transfars(
    departure: &Departure,
    start: &Station,
    datetime: DateTime<FixedOffset>,
) -> Vec<Vec<ResTransfar>> {
    let station_repository = StationRepositoryImpl;
    let chains = build_departure_path(&start.station_id, &departure, datetime, &station_repository);
    let mut res: Vec<Vec<ResTransfar>> = Vec::new();
    // DFSの関係上乗り換えに時間がかかるやつから出てくるのでここで逆順にする
    for chain in chains.into_iter().rev() {
        let res_chain: Vec<ResTransfar> = chain
            .iter()
            .map(|x| {
                let ride = Ride::from_id(x.departure.ride_id.get_raw_id()).unwrap();
                let to: ResStation = station_repository
                    .from_id(x.ride_for.clone())
                    .unwrap()
                    .into();
                let at: ResStation = station_repository.from_id(x.ride_at.clone()).unwrap().into();
                ResTransfar {
                    ride_type: ride.ride_type.clone(),
                    type_foreground: ride.type_foreground.clone(),
                    type_background: ride.type_background.clone(),
                    to: to,
                    career_type: ride.career_type.clone(),
                    depart_at: x.departure.time.format("%H:%M").to_string(),
                    at: at,
                    transfar_time: x.transfar_time
                }
            })
            .collect::<_>();
        res.push(res_chain);
    }
    res
}

/// ある駅よりあとの停車駅を抽出する
/// 形式的にはsplitで右側に寄せる
/// \[1, 2, 3, 4, 2, 5\]でstart=2のとき\[\[1\], \[2, 3, 4\], \[2, 5\]\]になる
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

/// 有効かは置いておいてあるdepartures以降のdeparturesをすべてVec<Departures>から抽出する。
/// ride_id (系統) が同じものも含めて返すことに注意。
fn get_transfarable_departures(
    departures: Vec<Departure>,
    transfar_from: &Departure,
) -> Vec<Departure> {
    let mut sorted = departures.clone();
    sorted.sort_by(|x, y| x.time.cmp(&y.time));
    sorted
        .iter()
        .skip_while(|x| {
            x.trip_id != transfar_from.trip_id || x.loop_count != transfar_from.loop_count
        })
        .filter(|x| x.departure_type != "incoming".to_string())
        .map(|x| x.to_owned())
        .collect()
}

fn get_departures_from_station_id(
    id: &StationId,
    today: NaiveDate,
    station_repository: &dyn StationRepository,
) -> Result<Vec<Departure>, Box<dyn Error + Sync + Send + 'static>> {
    let mut already_joined: HashSet<String> = HashSet::new();
    already_joined.insert(id.get_raw_id());
    let timetable = TimeTable::from_station_id(id.clone())?;
    let current_calendar = timetable
        .get_valid_calendar(today)
        .ok_or("There are no valid calendar version.")?;
    let calendar_res = Calendar::from_id(current_calendar.calendar_id)?;
    let pattern_id = calendar_res.get_pattern_id(today);

    let mut root_departure_pattern = DeparturePattern::from_id(pattern_id)?.departures;
    let station = station_repository.from_id(id.clone())?;
    for other_station_id in station.join {
        already_joined.insert(other_station_id.get_raw_id());
        let other_station = station_repository.from_id(other_station_id.clone());
        if other_station.is_err() {
            continue;
        }
        if !station_repository.check_is_valid(other_station.unwrap()) {
            continue;
        }

        let other_timetable = TimeTable::from_station_id(other_station_id.clone())?;
        let other_current_calendar = other_timetable
            .get_valid_calendar(today)
            .ok_or("There are no valid calendar version.")?;
        let other_calendar_res = Calendar::from_id(other_current_calendar.calendar_id)?;
        let other_pattern_id = other_calendar_res.get_pattern_id(today);
        DeparturePattern::from_id(other_pattern_id)?
            .departures
            .into_iter()
            .for_each(|x| root_departure_pattern.push(x));
    }

    root_departure_pattern.sort_by_key(|x| x.time);
    Ok(root_departure_pattern)
}

pub fn build_departure_path(
    root_station: &StationId,
    root_departure: &Departure,
    datetime: DateTime<FixedOffset>,
    station_repository: &dyn StationRepository,
) -> Vec<Vec<TransfarChain>> {
    let today = datetime.date_naive();
    let mut stack: VecDeque<Vec<TransfarChain>> = VecDeque::new();
    let mut res: Vec<Vec<TransfarChain>> = Vec::new();
    // 初期状態 (乗り換え検索の最初の駅であるdepartureに乗車した状態) をpush
    let Ok(root_ride) = Ride::from_id(root_departure.ride_id.clone().get_raw_id()) else {
        return vec![];
    };
    let root_last_stop = root_ride.route.last().unwrap().to_owned();
    stack.push_back(
        vec![
            TransfarChain{ 
                departure: root_departure.clone(),
                ride_at: root_station.clone(),
                ride_for: root_last_stop,
                transfar_time: 0,
            }
        ]
    );
    while let Some(chain) = stack.pop_back() {
        // 計算時間の都合で最大3回までの乗り換えを検索する。
        if chain.len() > 3 {
            continue;
        }

        // 初めの1要素は最初に乗った便なので一回以上乗り換えしていたら返り値に含める
        if chain.len() > 1 {
            res.push(chain.clone());
        }

        let transfar_from = chain.last().unwrap().to_owned();
        let Ok(ride) = Ride::from_id(transfar_from.departure.ride_id.get_raw_id()) else {
            continue;
        };

        // 次駅以降を対象にに乗り換え可能駅と乗り換えを探す
        let stop_after = calc_stop_after(ride.route, &transfar_from.ride_at, transfar_from.ride_at.get_loop_count())
            .into_iter()
            .skip(1)
            .collect::<Vec<_>>();
        for station in stop_after {
            let Ok(pattern) = get_departures_from_station_id(&station, today, station_repository) else {
                continue;
            };
            
            // 今乗っている便が到着するときのdepartureを探す
            let Some(arrive) = pattern
                .iter()
                .filter(|x| x.departure_type == "incoming".to_string() || x.departure_type == "both".to_string())
                .find(|x| {
                    x.trip_id == transfar_from.departure.trip_id && x.time >= transfar_from.departure.time
                }) else {
                    // debug!("Arrivalが見つからなかった");
                    // debug!("transfar_from: {:?} pattern: {:?}", transfar_from, pattern);
                    continue;
                };
            
            // 自分の出発情報を先頭に含むので1つskipする
            let depart_after = get_transfarable_departures(pattern.clone(), &arrive)
                .into_iter()
                .skip(1)
                .filter(|x| x.departure_type != "incoming".to_string());
            let mut ride_id_memo: HashSet<String> = HashSet::new();
            for target in depart_after {
                // 最低限の枝狩り (あとから発車する同じ系統 | すでに乗り換えが見つかっているのと同じ系統) を行う
                if target.ride_id != arrive.ride_id && !ride_id_memo.contains(&target.ride_id.get_raw_id()) {
                    let explored_stations = get_explored_stations(&chain);
                    let Ok(target_ride) = Ride::from_id(target.ride_id.get_raw_id()) else { continue; };
                    debug!("target: {:?}, target_original_route: {:?}", target_ride, target_ride.route);
                    let target_route = calc_stop_after(target_ride.route, &station, target.loop_count)
                        .into_iter()
                        .collect::<Vec<_>>();
                    debug!("explored_stations: {:?}\ntarget_route: {:?}", explored_stations, target_route);
                    let Some(valid_destinations) = find_valid_transfar_route(
                        explored_stations,
                        target_route,
                        &station,
                        &target
                    ) else {
                        // 有効な乗り換え出ない場合はスキップ
                        continue;
                    };

                    let mut next = chain.clone();
                    ride_id_memo.insert(target.ride_id.get_raw_id().clone());
                    next.push(TransfarChain {
                        departure: target.clone(),
                        ride_at: station.clone(),
                        ride_for: valid_destinations.last().unwrap().to_owned(),
                        transfar_time: (target.time-arrive.time).num_minutes(),
                    });
                    stack.push_back(next);
                }
            }
        }
    }
    res
}

fn find_valid_transfar(
    cur_station: &StationId,
    transfar_from: &Departure,
    // cur_ride: &Ride,
    today: NaiveDate, // todo: naivedate zl current datetime.
    station_repository: &dyn StationRepository,
) -> Vec<Vec<ResTransfar>> {
    let departure_pattern = get_departures_from_station_id(cur_station, today.clone(), station_repository);
    if departure_pattern.is_err() {
        return vec![];
    }
    let mut departures = departure_pattern.unwrap();
    departures.sort_by_key(|x| x.time);

    let depart_after_inclusive = get_transfarable_departures(departures, transfar_from);
    let arrive = depart_after_inclusive.first().unwrap().to_owned();

    for departure in depart_after_inclusive {
        // let transfar_route
    }

    // depart_after_inclusive
    //     .into_iter()
    //     .skip(1)
    //     .filter_map(|x| {
    //         // fixme: ここどうにかしたい
    //         let Ok(target_ride) = Ride::from_id(x.ride_id.get_raw_id()) else {
    //             return None;
    //         };
    //         calc_transfar(
    //             cur_ride,
    //             &arrive,
    //             cur_station,
    //             &x,
    //             &target_ride,
    //             station_repository,
    //         )
    //         .ok()
    //     })
    //     .map(|x| vec![x])
    //     .collect::<_>()
    todo!()
}

fn get_explored_stations(
    chain: &Vec<TransfarChain>
) -> Vec<StationId> {
    chain
        .into_iter()
        .map(|x| x.departure.ride_id.clone())
        .filter_map(|x| Ride::from_id(x.get_raw_id()).ok())
        .map(|x| x.route)
        .flatten()
        .collect::<Vec<StationId>>()
}

fn find_valid_transfar_route(
    explored_stations: Vec<StationId>,
    target_route: Vec<StationId>,
    transfar_at: &StationId,
    transfar_to: &Departure,
) -> Option<Vec<StationId>> {
    let valid_destinations: Vec<StationId> = 
        calc_stop_after(target_route, transfar_at, transfar_to.loop_count)
            .into_iter()
            .skip_while(|x| x.is_same_station(&transfar_at))
            .take_while(|x| explored_stations.iter().find(|y| y.is_same_station(x)) == None)
            .collect();
    if valid_destinations.is_empty() {
        None
    }
    else {
        Some(valid_destinations)
    }
}

fn is_valid_transfar(
    trip_route: Vec<StationId>,
    target_departure: &Departure,
    target_ride: &Ride,
    transfar_at: &StationId,
) -> Result<StationId, Box<dyn Error + Send + Sync + 'static>> {
    let target_route = target_ride.route.clone();
    let valid_destinations: Vec<StationId> =
        calc_stop_after(target_route, transfar_at, target_departure.loop_count)
            .into_iter()
            .skip_while(|x| x.is_same_station(&transfar_at))
            .take_while(|x| trip_route.iter().find(|y| y.is_same_station(x)) == None)
            .collect();

    if valid_destinations.is_empty() {
        return Err("Invalid.".into());
    }
    Ok(valid_destinations.last().unwrap().to_owned())
}

/// 2つのdepartureを対象にtransfar_fromからtarget_departureに乗り継ぐことが有効化を判定する
/// 無効な乗り換えの場合はErrが返る。
/// 0分乗り換えは許容しているので、フロント側で出さないようにするか、警告文を出すなどで対応する。
fn calc_transfar(
    trip_route: Vec<StationId>,
    arrival_time: &NaiveTime,
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
    let transfar_to = is_valid_transfar(trip_route, target_departure, target_ride, transfar_at)?.to_owned();

    let transfar_station = station_repository.from_id(transfar_at.clone())?;
    // let transfar_to_id = valid_destinations.last().unwrap().to_owned();
    let transfar_to = station_repository.from_id(transfar_to)?;
    let transfar_time = (target_departure.time - arrival_time.to_owned()).num_minutes();
    if transfar_time < 0 {
        return Err("There are no time to change.".into());
    }
    Ok(ResTransfar {
        ride_type: target_ride.ride_type.clone(),
        type_foreground: target_ride.type_foreground.clone(),
        type_background: target_ride.type_background.clone(),
        at: transfar_station.into(),
        to: transfar_to.into(),
        career_type: target_ride.career_type.clone(),
        depart_at: target_departure.time.format("%H:%M").to_string(),
        transfar_time: transfar_time,
    })
}

#[allow(non_snake_case)]
#[cfg(test)]
mod test {
    use super::{calc_stop_after, calc_transfar, find_valid_transfar, find_valid_transfar_route, get_transfarable_departures};
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
    fn debug_find_valid_transfar() {
        let explored_stations = vec![
            StationId::new("01".to_string()),
            StationId::new("02".to_string()),
            StationId::new("03".to_string()),
        ];
        let target_route = vec![
            StationId::new("00".to_string()),
            StationId::new("02".to_string()),
            StationId::new("04".to_string()),
        ];
        let transfar_at = StationId::new("02".to_string());
        let transfar_to = Departure {
            ride_id: RideId::new("Hoge".to_string()),
            trip_id: "Hoge".to_string(),
            time: NaiveTime::from_hms_opt(12, 0, 0).expect("Valid hms"),
            loop_count: 1,
            departure_type: "both".to_string()
        };
        let actual = find_valid_transfar_route(explored_stations, target_route, &transfar_at, &transfar_to);

        assert!(actual.is_some());
    }
/* todo: ちゃんとテストが動くように治す
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
        let actual_depart_after = get_transfarable_departures(departures, &cur_ride);

        let expected_depart_after = vec![
            Departure {
                ride_id: RideId::new("TargetRide".to_string()),
                trip_id: "3".to_string(),
                time: NaiveTime::from_hms_opt(10, 0, 0).expect("Valid hms"),
                loop_count: 0,
            },
            Departure {
                ride_id: RideId::new("TargetRide".to_string()),
                trip_id: "4".to_string(),
                time: NaiveTime::from_hms_opt(11, 0, 0).expect("Valid hms"),
                loop_count: 0,
            },
        ];

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

        let actual_depart_after = get_transfarable_departures(departures, &cur_departure);
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
        let actual = get_transfarable_departures(departures, &transfar_from);
        let expected: Vec<Departure> = Vec::new();
        assert_eq!(expected, actual);
    }

    #[test]
    fn valid_transfar_ナイーブなケース() {
        let cur_station = StationId::new("03".to_string());
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
            cur_ride.route.clone(),
            &NaiveTime::from_hms_opt(12, 0, 0).expect("valid hms"),
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
            at: station_repository.from_id(cur_station).unwrap().into(),
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
            cur_ride.route.clone(),
            &NaiveTime::from_hms_opt(12, 0, 0).expect("valid hms"),
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
            cur_ride.route.clone(),
            &NaiveTime::from_hms_opt(12, 0, 0).expect("valid hms"),
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
            at: station_repository.from_id(cur_station).unwrap().into(),
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
            cur_ride.route.clone(),
            &NaiveTime::from_hms_opt(12, 0, 0).expect("valid hms"),
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
            cur_ride.route.clone(),
            &NaiveTime::from_hms_opt(12, 0, 0).expect("valid hms"),
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
            cur_ride.route.clone(),
            &NaiveTime::from_hms_opt(12, 0, 0).expect("valid hms"),
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
            at: station_repository.from_id(cur_station).unwrap().into(),
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
*/
}
