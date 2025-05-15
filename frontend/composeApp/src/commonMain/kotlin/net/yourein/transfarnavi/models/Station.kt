package net.yourein.transfarnavi.models

import kotlinx.serialization.Serializable

@Serializable
data class Station(
    val id: String,
    val name: String,
    val pronounce: String,
)