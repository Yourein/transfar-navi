package net.yourein.transfarnavi

import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.size
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import org.jetbrains.compose.resources.painterResource
import org.jetbrains.compose.ui.tooling.preview.Preview

@Composable
internal fun TransferNaviTopBar(
    stationName: String,
    onRefreshButtonClick: () -> Unit,
    onChangeStationButtonClick: () -> Unit,
) {
    @OptIn(ExperimentalMaterial3Api::class)
    TopAppBar(
        title = {
            Text(
                text = stationName,
                fontWeight = FontWeight.Bold,
                fontSize = 36.sp
            )
        },
        actions = {
            TextButton(
                onClick = onRefreshButtonClick,
            ) {
                Text(text = "更新")
            }
            TextButton(
                onClick = onChangeStationButtonClick,
            ) {
                Text(text = "別駅")
            }
        }
    )
}

@Preview
@Composable
private fun TransferNaviTopBarPreview() {
    MaterialTheme {
        TransferNaviTopBar(
            stationName = "亀田支所前",
            onRefreshButtonClick = {},
            onChangeStationButtonClick = {},
        )
    }
}