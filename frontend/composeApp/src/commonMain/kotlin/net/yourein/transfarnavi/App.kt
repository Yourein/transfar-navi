package net.yourein.transfarnavi

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.material3.TextFieldDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.window.Dialog
import androidx.lifecycle.viewmodel.compose.viewModel
import kotlinx.datetime.Clock
import kotlinx.datetime.LocalTime
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import net.yourein.transfarnavi.models.Departure
import net.yourein.transfarnavi.models.Station
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
            viewModel.loadDepartures()
        }
        val state = viewModel.departureState
        var showChangeStationDialog by remember { mutableStateOf(false) }
        Scaffold(
            topBar = {
                TransferNaviTopBar(
                    stationName = "",
                    onRefreshButtonClick = {
                        viewModel.loadDepartures()
                    },
                    onChangeStationButtonClick = {
                        showChangeStationDialog = true
                    }
                )
            }
        ) { innerPadding ->
            if (showChangeStationDialog) {
                Dialog(
                    onDismissRequest = { showChangeStationDialog = false },
                ) {
                    var inputValue by remember { mutableStateOf("") }
                    Column(
                        horizontalAlignment = Alignment.CenterHorizontally,
                        modifier = Modifier
                            .clip(RoundedCornerShape(4.dp))
                            .background(Color(0xFFD0D0D0))
                            .padding(16.dp)
                    ) {
                        Text(
                            text = "StationIDを入力",
                            color = Color(0xFF101010),
                            fontSize = 20.sp,
                        )
                        Spacer(modifier = Modifier.padding(8.dp))
                        TextField(
                            value = inputValue,
                            onValueChange = { inputValue = it },
                            singleLine = true,
                            colors = TextFieldDefaults.colors(
                                focusedTextColor = Color(0xFF101010),
                                unfocusedTextColor = Color(0xFF101010),
                                cursorColor = Color(0xFF101010),
                            )
                        )
                        Spacer(modifier = Modifier.padding(8.dp))
                        Row {
                            Button(
                                onClick = {
                                    showChangeStationDialog = false
                                }
                            ) {
                                Text(
                                    text = "中止",
                                    color = Color(0xFFF0566E),
                                    fontSize = 18.sp,
                                )
                            }
                            Spacer(modifier = Modifier.size(8.dp))
                            Button(
                                onClick = {
                                    viewModel.setNewStationId(inputValue)
                                    showChangeStationDialog = false
                                },
                            ) {
                                Text(
                                    text = "変更",
                                    color = Color(0xFFF0F0F0),
                                    fontSize = 18.sp,
                                )
                            }
                        }
                    }
                }
            }

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