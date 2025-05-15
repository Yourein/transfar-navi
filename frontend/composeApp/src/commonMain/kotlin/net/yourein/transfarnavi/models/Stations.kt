package net.yourein.transfarnavi.models

import kotlinx.serialization.Serializable

@Serializable
data class Stations(
    val stations: List<Station>,
)
