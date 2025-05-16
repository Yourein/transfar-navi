package net.yourein.transfarnavi

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import net.yourein.transfarnavi.theme.NaviTheme
import org.jetbrains.compose.ui.tooling.preview.Preview

@Composable
internal fun TypeChip(
    rawTypeForeground: String,
    rawTypeBackground: String,
    typeName: String,
    akaType: String?,
    fontSize: TextUnit = 20.sp
) {
    val typeForegroundSanitized = "FF" + rawTypeForeground.removePrefix("#")
    val typeBackgroundSanitized = "FF" + rawTypeBackground.removePrefix("#")
    val backgroundColor = Color(typeBackgroundSanitized.toLong(16))
    val textColor = Color(typeForegroundSanitized.toLong(16))
    val text = if (akaType.isNullOrEmpty()) {
        typeName
    } else {
        "$typeName (${akaType})"
    }
    Text(
        text = text,
        color = textColor,
        fontWeight = FontWeight.Bold,
        fontSize = fontSize,
        modifier = Modifier
            .clip(RoundedCornerShape(4.dp))
            .background(backgroundColor)
            .padding(horizontal = 8.dp, vertical = 4.dp)
    )
}

@Preview
@Composable
private fun TypeChipPreview() {
    NaviTheme {
        Column {
            TypeChip(
                rawTypeForeground = "#F0F0F0",
                rawTypeBackground = "#C91412",
                typeName = "JAL 2748",
                akaType = null,
            )
            Spacer(modifier = Modifier.size(4.dp))
            TypeChip(
                rawTypeForeground = "#F0F0F0",
                rawTypeBackground = "#233F9A",
                typeName = "ANA 4842",
                akaType = null,
            )
            Spacer(modifier = Modifier.size(4.dp))
            TypeChip(
                rawTypeForeground = "#62B1DF",
                rawTypeBackground = "FFF265",
                typeName = "AIRDO 58",
                akaType = "ANA 4758",
            )
            Spacer(modifier = Modifier.size(4.dp))
            TypeChip(
                rawTypeForeground = "#F0F0F0",
                rawTypeBackground = "#53C453",
                typeName = "はやぶさ 42号",
                akaType = null,
            )
            Spacer(modifier = Modifier.size(4.dp))
            TypeChip(
                rawTypeForeground = "#F58220",
                rawTypeBackground = "#1A1A1A",
                typeName = "快速エアポート 132号",
                akaType = null,
            )
            Spacer(modifier = Modifier.size(4.dp))
            TypeChip(
                rawTypeForeground = "#F0F0F0",
                rawTypeBackground = "#F172A3",
                typeName = "はこだてライナー",
                akaType = null,
            )
            Spacer(modifier = Modifier.size(4.dp))
            TypeChip(
                rawTypeForeground = "#F0F0F0",
                rawTypeBackground = "#00B16B",
                typeName = "青梅特快",
                akaType = null,
            )
            Spacer(modifier = Modifier.size(4.dp))
            TypeChip(
                rawTypeForeground = "#F0F0F0",
                rawTypeBackground = "#0169B7",
                typeName = "新快速",
                akaType = null,
            )
        }
    }
}

@Suppress("NonAsciiCharacters")
@Preview
@Composable
private fun 日本は終了しました() {
    NaviTheme {
        TypeChip(
            rawTypeForeground = "#F58220",
            rawTypeBackground = "#1A1A1A",
            typeName = "☆☆☆ 日本は終了しました ☆☆☆",
            akaType = null,
        )
    }
}