package net.yourein.transfarnavi.models

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class Departures(
    val departures: List<Departure>,
)
