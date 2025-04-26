# api/v1/docs

## GET available_stations

None → Stations

```json
stations {
    "stations": [Station]
}

station {
    "id": String,
    "name": String,
    "pronounce": String,
}
```

有効な駅ってどうやって判定するの？→単にstation_idからtimetable探して、あったら有効。なかったら無効な駅として扱っていいと思う。

## GET departures/\{station_id\}

station_id: String → departures

```json
departures {
    "departures": [departure]
}

departure {
    "ride_type": String,
    "aka_type": Option<String>,
    "type_foreground": String,
    "type_background": String,
    "type_pronounce": String,
    "to": station,
    "career_type": String, // String Enum
    "transfars": [[transfar]]
}

transfar {
    "ride_type": String,
    "type_foreground": String,
    "type_background": String,
    "to": station,
    "career_type": String,
    "depart_at": String, // NaiveTime
    "wait_in_minutes": Int
}
```