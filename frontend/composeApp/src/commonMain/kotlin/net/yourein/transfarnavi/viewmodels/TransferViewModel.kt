package net.yourein.transfarnavi.viewmodels

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.launch
import net.yourein.transfarnavi.models.Departures
import net.yourein.transfarnavi.repositories.interfaces.TransferRepository
import net.yourein.transfarnavi.utils.LoadState

class TransferViewModel(
    private val transferRepository: TransferRepository,
) : ViewModel() {
    var departureState: LoadState<Departures> by mutableStateOf(LoadState.Loading(null))
        private set

    fun loadDepartures(stationId: String) {
        viewModelScope.launch {
            departureState = LoadState.Loading(departureState.value)
            try {
                val departures = transferRepository.getDepartures(stationId)
                departureState = LoadState.Success(departures)
            } catch (e: Exception) {
                departureState = LoadState.Error(departureState.value, e)
            }
        }
    }
}