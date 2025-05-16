package net.yourein.transfarnavi

import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material3.HorizontalDivider
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.datetime.LocalTime
import net.yourein.transfarnavi.models.Departure
import net.yourein.transfarnavi.theme.colorDivider

@Composable
internal fun DepartureList(
    departures: List<Departure>,
    currentTime: LocalTime,
    modifier: Modifier = Modifier,
) {
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

            if (index != departures.lastIndex) {
                HorizontalDivider(
                    thickness = 4.dp,
                    color = colorDivider,
                )
            }
        }
    }
}