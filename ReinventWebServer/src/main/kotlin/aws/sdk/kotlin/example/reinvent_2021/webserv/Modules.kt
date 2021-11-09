package aws.sdk.kotlin.example.reinvent_2021.webserv

import aws.sdk.kotlin.services.dynamodb.DynamoDbClient
import kotlinx.coroutines.runBlocking
import org.koin.dsl.module

val mainModule = module {
    single { IdGenerator() }
    single { runBlocking { DynamoDbClient.fromEnvironment { } } }
    single { MemesService(get(), get()) }
    single(createdAtStart = true) { WebService(get()) }
}
