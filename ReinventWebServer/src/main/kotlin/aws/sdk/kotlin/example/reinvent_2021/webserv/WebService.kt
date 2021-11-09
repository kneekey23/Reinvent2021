package aws.sdk.kotlin.example.reinvent_2021.webserv

import io.ktor.application.*
import io.ktor.features.*
import io.ktor.http.*
import io.ktor.request.*
import io.ktor.response.*
import io.ktor.routing.*
import io.ktor.serialization.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import kotlinx.serialization.json.Json

private const val PORT = 5000

class WebService(memes: MemesService) {
    init {
        embeddedServer(Netty, PORT) {
            install(ContentNegotiation) {
                json(Json {
                    prettyPrint = true
                })
            }

            install(StatusPages) {
                exception<Throwable> { cause ->
                    call.respondText(cause.stackTraceToString(), status = HttpStatusCode.InternalServerError)
                    throw cause
                }
            }

            routing {
                get("/") {
                    call.respondText { """
                    {
                        "message": "Service is running."
                    }
                """.trimIndent() }
                }

                get("/memes/list") {
                    val continueFrom = call.request.queryParameters["continueFrom"]
                    call.respond(memes.list(continueFrom))
                }

                post("/memes/submit") {
                    call.respond(memes.submit(call.receive()))
                }
            }
        }.start(wait = true)
    }
}
