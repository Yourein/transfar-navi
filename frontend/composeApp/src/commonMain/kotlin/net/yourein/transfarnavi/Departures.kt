package net.yourein.transfarnavi

import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.datetime.LocalTime
import net.yourein.transfarnavi.models.Departure
import net.yourein.transfarnavi.theme.colorDivider
import net.yourein.transfarnavi.theme.colorTextPrimary

@Composable
internal fun DepartureList(
    departures: List<Departure>,
    currentTime: LocalTime,
    modifier: Modifier = Modifier,
) {
    if (departures.isEmpty()) {
        Text(
            text = "出発情報がありません",
            color = colorTextPrimary,
            fontSize = 36.sp,
            textAlign = TextAlign.Center,
            modifier = modifier
                .fillMaxWidth()
                .padding(horizontal = 16.dp, vertical = 24.dp)
        )
    } else {
        LazyColumn(
            modifier = modifier
        ) {
            itemsIndexed(
                items = departures,
            ) { index, departure ->
                DepartureContent(
                    departure = departure,
                    index = index,
                    currentTime = currentTime,
                )

                if (departure.transfers.isNotEmpty()) {
                    Spacer(modifier = Modifier.size(8.dp))
                    Transfers(
                        chains = departure.transfers,
                        modifier = Modifier.padding(start = 80.dp)
                    )
                    Spacer(modifier = Modifier.size(8.dp))
                }

                if (index != departures.lastIndex) {
                    HorizontalDivider(
                        thickness = 4.dp,
                        color = colorDivider,
                    )
                }
            }
        }
    }
}