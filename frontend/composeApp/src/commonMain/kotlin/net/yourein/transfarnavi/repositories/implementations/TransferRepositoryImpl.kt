package net.yourein.transfarnavi.repositories.implementations

import io.ktor.client.HttpClient
import io.ktor.client.call.body
import io.ktor.client.engine.okhttp.OkHttp
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.client.request.request
import io.ktor.client.statement.HttpResponse
import io.ktor.serialization.kotlinx.json.json
import kotlinx.serialization.json.Json
import net.yourein.transfarnavi.models.Departures
import net.yourein.transfarnavi.repositories.interfaces.TransferRepository

class TransferRepositoryImpl : TransferRepository {
    override suspend fun getDepartures(stationId: String): Departures {
        val client = HttpClient(OkHttp) {
            install(ContentNegotiation) {
                json(Json {
                    prettyPrint = true
                    isLenient = true
                })
            }
        }
        val response: HttpResponse =
            client.request("http://localhost:8080/v1/departures/$stationId")
        return response.body()
    }
}