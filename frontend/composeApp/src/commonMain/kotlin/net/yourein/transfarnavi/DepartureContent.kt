package net.yourein.transfarnavi

import androidx.compose.foundation.background
import androidx.compose.foundation.basicMarquee
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
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
import net.yourein.transfarnavi.theme.colorTextPrimary

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
        val orderInfoText = when (index) {
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

        if (index == 0) {
            NextDepartureContent(
                departure = departure,
                currentTime = currentTime,
            )
        } else {
            NormalDepartureContent(
                departure = departure,
                currentTime = currentTime,
            )
        }
    }
}

@Composable
private fun NextDepartureContent(
    departure: Departure,
    currentTime: LocalTime,
) {
    Row(
        verticalAlignment = Alignment.CenterVertically
    ) {
        Column(
            modifier = Modifier.weight(1f)
        ) {
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
                text = "${departure.to.name} (${departure.to.pronounce})",
                color = colorTextPrimary,
                fontWeight = FontWeight.Bold,
                fontSize = 48.sp,
                maxLines = 1,
                overflow = TextOverflow.Clip,
                modifier = Modifier.basicMarquee()
            )
        }

        Spacer(modifier = Modifier.size(8.dp))

        DepartIn(
            departureTimeStr = departure.departAt,
            currentTime = currentTime,
        )
    }
}

@Composable
private fun NormalDepartureContent(
    departure: Departure,
    currentTime: LocalTime,
) {
    Row(
        verticalAlignment = Alignment.CenterVertically,
    ) {
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

        Row(
            modifier = Modifier.weight(1f)
        ) {
            Text(
                text = departure.to.name,
                color = colorTextPrimary,
                fontWeight = FontWeight.Bold,
                fontSize = 48.sp,
                maxLines = 1,
                overflow = TextOverflow.Clip,
                modifier = Modifier
                    .fillMaxWidth()
                    .basicMarquee()
            )
        }

        Spacer(modifier = Modifier.size(8.dp))

        DepartIn(
            departureTimeStr = departure.departAt,
            currentTime = currentTime,
        )
    }
}

@Composable
private fun DepartIn(
    departureTimeStr: String,
    currentTime: LocalTime,
) {
    val departureTime = LocalTime.parse(departureTimeStr)
    val timeDelta = (departureTime.toSecondOfDay() - currentTime.toSecondOfDay()) / 60
    Column(
        horizontalAlignment = Alignment.End,
    ) {
        Text(
            text = departureTimeStr,
            color = colorTextPrimary,
            fontSize = 18.sp,
        )
        Text(
            text = "${timeDelta}分後",
            color = colorTextPrimary,
            fontSize = 18.sp,
        )
    }
}