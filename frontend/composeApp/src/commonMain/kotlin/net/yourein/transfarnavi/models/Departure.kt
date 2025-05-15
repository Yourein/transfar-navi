package net.yourein.transfarnavi.models

// todo: JSONの命名規則が違うのでrenameを忘れないこと
data class Departure(
    val rideType: String,
    val akaType: String?,
    val typeForeground: String,
    val typeBackground: String,
    val typePronounce: String,
    val to: Station,
    val careerType: String,
    val departAt: String,
    val transfers: List<List<Transfer>>,
)
