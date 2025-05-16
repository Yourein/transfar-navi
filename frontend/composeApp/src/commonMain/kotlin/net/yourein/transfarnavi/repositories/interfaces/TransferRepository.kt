package net.yourein.transfarnavi.repositories.interfaces

import net.yourein.transfarnavi.models.Departures

interface TransferRepository {
    suspend fun getDepartures(stationId: String): Departures
}