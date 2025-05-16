package net.yourein.transfarnavi.theme

import androidx.compose.material3.ColorScheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color

val colorTextPrimary = Color(0xFFF0F0F0)

private val colorScheme: ColorScheme
    @Composable get() =  MaterialTheme.colorScheme.copy(
        background = Color(0xFF323232),
        surface = Color(0xFF323232),
        onSurface = Color(0xFFF0F0F0),
    )

@Composable
fun NaviTheme(
    content: @Composable () -> Unit,
) {
    MaterialTheme(
        colorScheme = colorScheme,
        content = content,
    )
}