package net.yourein.transfarnavi

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import kotlinx.datetime.Clock
import kotlinx.datetime.LocalTime
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import net.yourein.transfarnavi.models.Departure
import net.yourein.transfarnavi.models.Station
import net.yourein.transfarnavi.models.Transfer
import net.yourein.transfarnavi.repositories.implementations.TransferRepositoryImpl
import net.yourein.transfarnavi.theme.NaviTheme
import net.yourein.transfarnavi.utils.LoadState
import net.yourein.transfarnavi.viewmodels.TransferViewModel
import org.jetbrains.compose.ui.tooling.preview.Preview


@Composable
fun App(
    viewModel: TransferViewModel = viewModel { TransferViewModel(TransferRepositoryImpl()) },
) {
    NaviTheme {
        LaunchedEffect(Unit) {
            viewModel.loadDepartures("HAKODATEBUS_050004")
        }
        val state = viewModel.departureState
        Scaffold(
            topBar = {
                TransferNaviTopBar(
                    stationName = "",
                    onRefreshButtonClick = {
                        viewModel.loadDepartures("HAKODATEBUS_050004")
                    },
                    onChangeStationButtonClick = {}
                )
            }
        ) { innerPadding ->
            when (state) {
                is LoadState.Loading -> {
                    Box(
                        contentAlignment = Alignment.Center,
                        modifier = Modifier
                            .padding(innerPadding)
                            .fillMaxSize()
                    ) {
                        CircularProgressIndicator()
                    }
                }

                is LoadState.Error -> {
                    Box(
                        modifier = Modifier.padding(innerPadding)
                    ) {
                        val e = state.throwable.message
                        Text(text = "${e}")
                    }
                }

                is LoadState.Success -> {
                    val currentTime = Clock.System.now()
                        .toLocalDateTime(TimeZone.currentSystemDefault())
                        .time
                    DepartureList(
                        departures = state.value.departures,
                        currentTime = currentTime,
                        modifier = Modifier.padding(innerPadding)
                    )
                }
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
                transfers = listOf(
                    listOf(
                        Transfer(
                            rideType = "55G",
                            typeForeground = "#FFD700",
                            typeBackground = "#000000",
                            at = Station(
                                id = "HAKODATEBUS_050004",
                                name = "亀田中学校前",
                                pronounce = "かめだちゅうがっこうまえ",
                            ),
                            to = Station(
                                id = "HAKODATEBUS_050019",
                                name = "赤川",
                                pronounce = "あかがわ",
                            ),
                            careerType = "BUS",
                            departAt = "11:02",
                            transferTime = 0,
                        ),
                        Transfer(
                            rideType = "7F",
                            typeForeground = "#FFD700",
                            typeBackground = "#000000",
                            at = Station(
                                id = "HAKODATEBUS_050005",
                                name = "亀田支所前",
                                pronounce = "かめだししょまえ",
                            ),
                            to = Station(
                                id = "HAKODATEBUS_030001",
                                name = "函館空港",
                                pronounce = "はこだてくうこう",
                            ),
                            careerType = "BUS",
                            departAt = "11:46",
                            transferTime = 46,
                        ),
                        Transfer(
                            rideType = "JAL 2748",
                            typeForeground = "#F0F0F0",
                            typeBackground = "#C91412",
                            at = Station(
                                id = "HAKODATEBUS_030001",
                                name = "函館空港",
                                pronounce = "はこだてくうこう",
                            ),
                            to = Station(
                                id = "AP_RJCO",
                                name = "丘珠空港",
                                pronounce = "おかだまくうこう"
                            ),
                            careerType = "AIRPLANE",
                            departAt = "14:00",
                            transferTime = 133,
                        ),
                    )
                )
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