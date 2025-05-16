package net.yourein.transfarnavi.utils

sealed class LoadState<out T> {
    abstract val value: T?

    data class Loading<T>(override val value: T?) : LoadState<T>()
    data class Success<T>(override val value: T) : LoadState<T>()
    data class Error<T>(override val value: T?, val throwable: Throwable) : LoadState<T>()
}