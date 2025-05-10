pub const ROOT_PATH: &str = "/home/yourein/Codes/transfar-navi/backend/data";

#[allow(dead_code)]
pub trait ID {
    fn new(raw_id: String) -> Self;
    fn get_id_path_list(&self) -> Vec<String>;
    fn get_root_path(&self) -> String;
    fn get_data_type_path(&self) -> &'static str;
    fn get_raw_id(&self) -> String;
    
    fn build_path(&self) -> String {
        self.get_root_path() + "/" + self.get_data_type_path() + "/" + &self.get_id_path_list().join("/") + ".json"
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeparturePatternId {
    id: String,
    data_root_path: String,
}

impl ID for DeparturePatternId {
    fn new(raw_id: String) -> Self {
        DeparturePatternId { id: raw_id, data_root_path: ROOT_PATH.to_string() }
    }

    fn get_id_path_list(&self) -> Vec<String> {
        self.id.split("_").map(|x| x.to_string()).collect()
    }

    fn get_root_path(&self) -> String {
        self.data_root_path.clone()
    }

    fn get_raw_id(&self) -> String {
        self.id.clone()
    }

    fn get_data_type_path(&self) -> &'static str {
        "departure-pattern"
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RideId {
    id: String,
    data_root_path: String
}

impl ID for RideId {
    fn new(id: String) -> Self {
        RideId { id, data_root_path: ROOT_PATH.to_string() }
    }

    fn get_id_path_list(&self) -> Vec<String> {
        self.id.split("_").map(|x| x.to_string()).collect()
    }

    fn get_root_path(&self) -> String {
        self.data_root_path.clone()
    }

    fn get_raw_id(&self) -> String {
        self.id.clone()
    }
    
    fn get_data_type_path(&self) -> &'static str {
        "ride"
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalendarId {
    id: String,
    data_root_path: String
}

impl ID for CalendarId {
    fn new(id: String) -> Self {
        CalendarId { id, data_root_path: ROOT_PATH.to_string() }
    }

    fn get_id_path_list(&self) -> Vec<String> {
        self.id.split("_").map(|x| x.to_string()).collect()
    }

    fn get_root_path(&self) -> String {
        self.data_root_path.clone()
    }

    fn get_raw_id(&self) -> String {
        self.id.clone()
    }
    
    fn get_data_type_path(&self) -> &'static str {
        "calendar"
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StationId {
    pub(crate) id: String,
    pub(crate) data_root_path: String
}

impl StationId {
    pub fn to_timetable_id(&self) -> TimeTableId {
        TimeTableId::new(self.id.clone())
    }

    fn split_additional_info(&self) -> Vec<String> {
        self.id.clone().split("~").map(|x| x.to_string()).collect()
    }

    pub fn get_loop_count(&self) -> i32 {
        let id_data = self.split_additional_info();
        if id_data.len() >= 2 {
            id_data[1].parse::<i32>().unwrap_or_else(|_| 1)
        }
        else {
            1
        }
    }
}

impl ID for StationId {
    fn new(id: String) -> Self {
        StationId { id, data_root_path: ROOT_PATH.to_string() }
    }

    fn get_id_path_list(&self) -> Vec<String> {
        self.get_raw_id().split("_").map(|x| x.to_string()).collect()
    }

    fn get_root_path(&self) -> String {
        self.data_root_path.clone()
    }

    fn get_raw_id(&self) -> String {
        self.split_additional_info()[0].clone()
    }
    
    fn get_data_type_path(&self) -> &'static str {
        "station"
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeTableId {
    id: String,
    data_root_path: String
}

impl ID for TimeTableId {
    fn new(id: String) -> Self {
        TimeTableId { id, data_root_path: ROOT_PATH.to_string() }
    }

    fn get_id_path_list(&self) -> Vec<String> {
        self.id.split("_").map(|x| x.to_string()).collect()
    }

    fn get_root_path(&self) -> String {
        self.data_root_path.clone()
    }

    fn get_raw_id(&self) -> String {
        self.id.clone()
    }
    
    fn get_data_type_path(&self) -> &'static str {
        "timetable"
    }
}