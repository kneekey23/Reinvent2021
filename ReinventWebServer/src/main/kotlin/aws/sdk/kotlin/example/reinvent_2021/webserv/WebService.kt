package aws.sdk.kotlin.example.reinvent_2021.webserv

import io.ktor.application.call
import io.ktor.application.install
import io.ktor.features.ContentNegotiation
import io.ktor.features.StatusPages
import io.ktor.http.HttpStatusCode
import io.ktor.request.receive
import io.ktor.response.respond
import io.ktor.response.respondText
import io.ktor.routing.get
import io.ktor.routing.post
import io.ktor.routing.routing
import io.ktor.serialization.json
import io.ktor.server.engine.embeddedServer
import io.ktor.server.netty.Netty
import kotlinx.serialization.json.Json

private const val PORT = 5000

class WebService(memes: MemesService) {
    init {
        embeddedServer(Netty, PORT) {
            install(ContentNegotiation) {
                json(
                    Json {
                        prettyPrint = true
                    }
                )
            }

            install(StatusPages) {
                exception<Throwable> { cause ->
                    call.respondText(cause.stackTraceToString(), status = HttpStatusCode.InternalServerError)
                    throw cause
                }
            }

            routing {
                get("/") {
                    call.respondText {
                        """
                            {
                                "message": "Service is running."
                            }
                        """.trimIndent()
                    }
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
