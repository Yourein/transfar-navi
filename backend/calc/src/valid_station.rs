use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use models::id::StationId;
use models::id::ID;
use models::station::Station;
use repositories::impls::station_repository::StationRepositoryImpl;
use repositories::traits::station_repository::StationRepository;

#[allow(dead_code)]
pub fn get_valid_stations(path: &PathBuf) -> Result<Vec<Station>, Box<dyn Error + Send + Sync + 'static>> {
    let mut valid_stations = Vec::new();

    if path.is_dir() {
        let station_repository = StationRepositoryImpl;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // 再帰的に探索
                let sub_paths = get_valid_stations(&path)?;
                valid_stations.extend(sub_paths);
            } else if path.is_file() {
                // 有効な駅なら追加
                if let Some(current_path) = path.to_str() {
                    let timetable_path_string = current_path.replace("/station/", "/timetable/");
                    let timetable_path = Path::new(&timetable_path_string);
                    if timetable_path.exists() && timetable_path.is_file() {
                        let station_id = current_path.split("station/")
                            .nth(1)
                            .map(|x| x.strip_suffix(".json").unwrap_or_default())
                            .map(|x| StationId::new(x.replace("/", "_")));
                        if station_id.is_some() {
                            valid_stations.push(station_repository.from_id(station_id.unwrap())?);
                        }
                    }
                }
            }
        }
    }

    valid_stations.sort_by(|x, y| x.station_id.build_path().cmp(&y.station_id.build_path()));
    Ok(valid_stations)
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use crate::valid_station::get_valid_stations;

    #[test]
    #[ignore = "comment out this ignore attribute if you want to debug get_valid_stations()"]
    fn debug_get_valid_stations() {
        let search_root = Path::new("../data/station").to_path_buf();
        let res = get_valid_stations(&search_root).unwrap();
        println!{"{:?}", res};
        assert_eq!(1, 1);
    }
}