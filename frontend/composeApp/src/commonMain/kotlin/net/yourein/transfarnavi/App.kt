package net.yourein.transfarnavi

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import kotlinx.datetime.LocalTime
import net.yourein.transfarnavi.models.Departure
import net.yourein.transfarnavi.models.Station
import net.yourein.transfarnavi.theme.NaviTheme
import org.jetbrains.compose.ui.tooling.preview.Preview


@Composable
fun App() {
    NaviTheme {
        Scaffold(
            topBar = {
                TransferNaviTopBar(
                    stationName = "亀田支所前",
                    onRefreshButtonClick = {},
                    onChangeStationButtonClick = {}
                )
            }
        ) { innerPadding ->
            Box(
                modifier = Modifier.padding(innerPadding)
            ) {
                DepartureContentTest()
            }
        }
    }
}

@Composable
private fun DepartureContentTest() {
    Column {
        val departures = listOf(
            Departure(
                rideType = "55G",
                akaType = null,
                typeForeground = "#FFD700",
                typeBackground = "#000000",
                typePronounce = "",
                to = Station(
                    id = "HAKODATEBUS_050019",
                    name = "赤川",
                    pronounce = "あかがわ",
                ),
                careerType = "BUS",
                departAt = "11:02",
                transfers = listOf()
            ),
            Departure(
                rideType = "JAL 584",
                akaType = null,
                typeForeground = "#F0F0F0",
                typeBackground = "#C91412",
                typePronounce = "",
                to = Station(
                    id = "",
                    name = "羽田空港",
                    pronounce = "",
                ),
                careerType = "BUS",
                departAt = "12:30",
                transfers = listOf()
            ),
            Departure(
                rideType = "JAL 2754",
                akaType = null,
                typeForeground = "#F0F0F0",
                typeBackground = "#C91412",
                typePronounce = "",
                to = Station(
                    id = "",
                    name = "札幌丘珠空港",
                    pronounce = "",
                ),
                careerType = "BUS",
                departAt = "12:47",
                transfers = listOf()
            ),
            Departure(
                rideType = "AIRDO 58",
                akaType = "ANA 4758",
                typePronounce = "",
                typeForeground = "#62B1DF",
                typeBackground = "#FFF265",
                to = Station(id = "", name = "羽田空港", pronounce = ""),
                careerType = "AIRPLANE",
                departAt = "13:40",
                transfers = listOf(),
            ),
            Departure(
                rideType = "特急北斗 1号",
                akaType = null,
                typePronounce = "",
                typeForeground = "#E01010",
                typeBackground = "#1A1A1A",
                to = Station(id = "", name = "札幌", pronounce = ""),
                careerType = "LTDEXP",
                departAt = "15:15",
                transfers = listOf(),
            )
        )
        val time = LocalTime.parse("10:00")
        DepartureList(
            departures = departures,
            currentTime = time,
            modifier = Modifier
        )
    }
}

@Preview
@Composable
private fun DepartureContentPreview() {
    MaterialTheme {
        val time = LocalTime.parse("10:00")
        Column {
            val departure = Departure(
                rideType = "55G",
                akaType = null,
                typeForeground = "#FFD700",
                typeBackground = "#000000",
                typePronounce = "",
                to = Station(
                    id = "HAKODATEBUS_050019",
                    name = "赤川",
                    pronounce = "",
                ),
                careerType = "BUS",
                departAt = "11:02",
                transfers = listOf()
            )
            DepartureContent(
                departure = departure,
                index = 0,
                currentTime = time,
            )

            val anotherDeparture = Departure(
                rideType = "JAL 584",
                akaType = null,
                typeForeground = "#F0F0F0",
                typeBackground = "#C91412",
                typePronounce = "",
                to = Station(
                    id = "",
                    name = "羽田空港",
                    pronounce = "",
                ),
                careerType = "BUS",
                departAt = "12:30",
                transfers = listOf()
            )
            DepartureContent(
                departure = anotherDeparture,
                index = 1,
                currentTime = time,
            )
        }
    }
}