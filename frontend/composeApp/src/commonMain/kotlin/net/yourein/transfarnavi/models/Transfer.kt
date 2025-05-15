package net.yourein.transfarnavi.models

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class Transfer(
    @SerialName("ride_type") val rideType: String,
    @SerialName("type_foreground") val typeForeground: String,
    @SerialName("type_background") val typeBackground: String,
    val at: Station,
    val to: Station,
    @SerialName("career_type") val careerType: String,
    @SerialName("depart_at") val departAt: String, // hh:mm
    @SerialName("transfar_time") val transferTime: Int,
)
