package aws.sdk.kotlin.example.reinvent_2021.webserv

import kotlin.random.Random

class IdGenerator {
    fun next(): String = Random
        .nextLong(0L, Long.MAX_VALUE)
        .toString(36)
}
