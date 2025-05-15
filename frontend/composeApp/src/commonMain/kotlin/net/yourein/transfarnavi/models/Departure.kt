package net.yourein.transfarnavi.models

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class Departure(
    @SerialName("ride_type") val rideType: String,
    @SerialName("aka_type") val akaType: String?,
    @SerialName("type_foreground") val typeForeground: String,
    @SerialName("type_background") val typeBackground: String,
    @SerialName("type_pronounce") val typePronounce: String,
    val to: Station,
    @SerialName("career_type") val careerType: String,
    @SerialName("depart_at") val departAt: String,
    @SerialName("transfars") val transfers: List<List<Transfer>>,
)
