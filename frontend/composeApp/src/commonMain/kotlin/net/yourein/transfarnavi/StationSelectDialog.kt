package net.yourein.transfarnavi

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.material3.TextFieldDefaults
import androidx.compose.runtime.Composable
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

@Composable
internal fun StationSelectDialog(
    onDismissRequest: () -> Unit,
    onOKButtonClicked: (String) -> Unit,
) {
    Dialog(
        onDismissRequest = onDismissRequest,
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
                    onClick = onDismissRequest,
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
                        onOKButtonClicked(inputValue)
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