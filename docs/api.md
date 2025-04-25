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

station_id: String → ?