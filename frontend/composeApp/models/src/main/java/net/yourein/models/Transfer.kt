package net.yourein.models

// todo: JSONと命名規則が違うのでrenameを忘れないようにする
data class Transfer(
    val rideType: String,
    val typeForeground: String,
    val typeBackground: String,
    val at: Station,
    val to: Station,
    val careerType: String,
    val departAt: String, // hh:mm
    val transferTime: Int,
)
