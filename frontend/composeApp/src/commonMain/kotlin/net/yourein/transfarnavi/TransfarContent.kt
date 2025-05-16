package net.yourein.transfarnavi

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.basicMarquee
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
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
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import net.yourein.transfarnavi.models.Station
import net.yourein.transfarnavi.models.Transfer
import net.yourein.transfarnavi.theme.NaviTheme
import net.yourein.transfarnavi.theme.colorTextPrimary
import org.jetbrains.compose.ui.tooling.preview.Preview


@Composable
internal fun Transfers(
    chains: List<List<Transfer>>,
    modifier: Modifier = Modifier,
) {
    Column(
        verticalArrangement = Arrangement.spacedBy(8.dp),
        modifier = modifier
            .fillMaxWidth()
    ) {
        for (transferChain in chains) {
            Row(
                // horizontalArrangement = Arrangement.spacedBy(4.dp)
                verticalAlignment = Alignment.CenterVertically,
                modifier = Modifier.basicMarquee()
            ) {
                for ((index, transfer) in transferChain.withIndex()) {
                    if (index > 0) {
                        TransferLine(
                            transferAt = transfer.at.name,
                            transferTime = transfer.transferTime,
                        )
                    }
                    TransferChip(transfer)
                }
            }
        }
    }
}

@Composable
private fun TransferLine(
    transferAt: String,
    transferTime: Int,
) {
    Text(
        text = "${transferAt}\n${transferTime}分",
        color = colorTextPrimary,
        fontSize = 18.sp,
        lineHeight = 28.sp,
        textAlign = TextAlign.Center,
        modifier = Modifier
            .drawBehind {
                drawLine(
                    color = colorTextPrimary,
                    start = Offset(0f, size.height / 2f + 4f),
                    end = Offset(size.width, size.height / 2f + 4f),
                    strokeWidth = 2f,
                )
            }
            .padding(horizontal = 8.dp)
    )
}

@Composable
private fun TransferChip(
    transfer: Transfer,
    modifier: Modifier = Modifier,
) {
    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
        modifier = modifier
            .border(border = BorderStroke(1.dp, colorTextPrimary), RoundedCornerShape(4.dp))
            .padding(4.dp)
    ) {
        TypeChip(
            rawTypeForeground = transfer.typeForeground,
            rawTypeBackground = transfer.typeBackground,
            typeName = transfer.rideType,
            akaType = null,
            fontSize = 16.sp,
        )
        Spacer(modifier = Modifier.size(4.dp))
        Text(
            text = "${transfer.to.name} 行",
            color = colorTextPrimary,
            fontSize = 20.sp,
        )
    }
}

@Preview
@Composable
private fun TransfersPreview() {
    NaviTheme {
        val original =
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
        val transfers = listOf(original, original, original.dropLast(1))
        Transfers(transfers)
    }
}

@Preview
@Composable
private fun TransferChipPreview() {
    NaviTheme {
        val transfer = Transfer(
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
        )
        TransferChip(transfer)
    }
}