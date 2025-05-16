package net.yourein.transfarnavi

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.datetime.LocalTime
import net.yourein.transfarnavi.models.Departure
import net.yourein.transfarnavi.models.Station
import net.yourein.transfarnavi.theme.NaviTheme
import net.yourein.transfarnavi.theme.colorTextPrimary
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
        val time = LocalTime.parse("10:00")
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
            careerType = "AIRPLANE",
            departAt = "12:30",
            transfers = listOf()
        )
        DepartureContent(
            departure = anotherDeparture,
            index = 1,
            currentTime = time,
        )

        val airdo = Departure(
            rideType = "AIRDO 58",
            akaType = "ANA 4758",
            typePronounce = "",
            typeForeground = "#62B1DF",
            typeBackground = "#FFF265",
            to = Station(id = "", name = "羽田空港", pronounce = ""),
            careerType = "AIRPLANE",
            departAt = "13:40",
            transfers = listOf(),
        )
        DepartureContent(
            departure = airdo,
            index = 2,
            currentTime = time,
        )

        val hokuto1 = Departure(
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
        DepartureContent(
            departure = hokuto1,
            index = 3,
            currentTime = time,
        )
    }
}

@Composable
internal fun DepartureContent(
    departure: Departure,
    index: Int,
    currentTime: LocalTime,
) {
    Row(
        verticalAlignment = Alignment.CenterVertically,
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 8.dp)
    ) {
        val orderInfoText = when(index) {
            0 -> "次発　"
            1 -> "次々発"
            else -> "　".repeat(3)
        }
        Text(
            text = orderInfoText,
            color = colorTextPrimary,
            fontWeight = FontWeight.Bold,
            fontSize = 18.sp,
        )

        Spacer(modifier = Modifier.size(8.dp))

        val typeBackground = "FF" + departure.typeBackground.removePrefix("#")
        val backgroundColor = Color(typeBackground.lowercase().toLong(16))
        val typeForeground = "FF" + departure.typeForeground.removePrefix("#")
        val textColor = Color(typeForeground.toLong(16))
        Text(
            text = departure.rideType,
            color = textColor,
            fontWeight = FontWeight.Bold,
            fontSize = 20.sp,
            modifier = Modifier
                .clip(RoundedCornerShape(4.dp))
                .background(backgroundColor)
                .padding(horizontal = 8.dp, vertical = 4.dp)
        )

        Spacer(modifier = Modifier.size(8.dp))

        Text(
            text = departure.to.name,
            color = colorTextPrimary,
            fontWeight = FontWeight.Bold,
            fontSize = 48.sp,
            maxLines = 1,
            overflow = TextOverflow.Clip,
            modifier = Modifier.weight(1f)
        )

        Spacer(modifier = Modifier.size(8.dp))

        val departureTimeStr = departure.departAt
        val departureTime = LocalTime.parse(departureTimeStr)
        val timeDelta = (departureTime.toSecondOfDay() - currentTime.toSecondOfDay()) / 60
        Text(
            text = "$departureTimeStr (${timeDelta}分後)",
            color = colorTextPrimary,
            fontSize = 18.sp,
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