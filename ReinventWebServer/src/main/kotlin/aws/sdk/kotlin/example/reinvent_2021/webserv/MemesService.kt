package aws.sdk.kotlin.example.reinvent_2021.webserv

import aws.sdk.kotlin.services.dynamodb.DynamoDbClient
import aws.sdk.kotlin.services.dynamodb.model.AttributeValue
import aws.sdk.kotlin.services.s3.S3PresignConfig
import aws.sdk.kotlin.services.s3.model.GetObjectRequest
import aws.sdk.kotlin.services.s3.presign
import kotlinx.serialization.Serializable
import java.time.ZoneOffset
import java.time.ZonedDateTime
import java.time.format.DateTimeFormatter

private const val MAX_RESULTS_PER_PAGE = 25
private const val MEMES_TABLE = "memes"
private const val PRESIGN_TTL_SECONDS = 86400L // 1 day
private const val QUERY_INDEX = "status-approvalTimestamp-index"

class MemesService(private val ddb: DynamoDbClient, private val ids: IdGenerator) {
    private val presignConfig = S3PresignConfig { region = ddb.config.region }
    private val tz = ZoneOffset.UTC
    private val tsFormat = DateTimeFormatter.ISO_INSTANT

    private fun nowTs(): String = ZonedDateTime.now(tz).format(tsFormat)

    suspend fun list(continueFrom: String?): ListMemeOutput {
        val results = ddb.query {
            tableName = MEMES_TABLE
            indexName = QUERY_INDEX
            scanIndexForward = false
            keyConditionExpression = "#statusAttr = :statusVal AND #approvalTimestampAttr <= :approvalTsVal"
            expressionAttributeNames = mapOf(
                "#statusAttr" to "status",
                "#approvalTimestampAttr" to "approvalTimestamp",
            )
            expressionAttributeValues = mapOf(
                ":statusVal" to AttributeValue.S("approved"),
                ":approvalTsVal" to AttributeValue.S(continueFrom ?: nowTs()),
            )
            limit = MAX_RESULTS_PER_PAGE
        }
        val memes = results.items!!.map { rowToMeme(it) }

        return ListMemeOutput(
            memes = memes,
            continueFrom = if (results.lastEvaluatedKey == null) null else memes.last().timestamp,
        )
    }

    private suspend fun presignS3Uri(s3Uri: String): String {
        val parsedUri = S3Uri(s3Uri)
        val req = GetObjectRequest {
            bucket = parsedUri.bucket
            key = parsedUri.key
        }
        val httpReq = req.presign(presignConfig, PRESIGN_TTL_SECONDS)
        return httpReq.url.toString()
    }

    private suspend fun rowToMeme(item: Map<String, AttributeValue>): Meme {
        val uri = item["s3Uri"]!!.toS()
        val timestamp = item["approvalTimestamp"]!!.toS()
        return Meme(presignS3Uri(uri), timestamp)
    }

    suspend fun submit(input: SubmitMemeInput): SubmitMemeOutput {
        ddb.putItem {
            tableName = MEMES_TABLE
            item = mapOf(
                "id" to AttributeValue.S(ids.next()),
                "submitTimestamp" to AttributeValue.S(nowTs()),
                "s3Uri" to AttributeValue.S(input.s3Uri),
                "status" to AttributeValue.S("unapproved"),
            )
        }
        return SubmitMemeOutput
    }
}

private fun AttributeValue.toS() = (this as AttributeValue.S).value

@Serializable
data class ListMemeOutput(val memes: List<Meme>, val continueFrom: String?)

@Serializable
data class Meme(val url: String, val timestamp: String)

@Serializable
data class SubmitMemeInput(val s3Uri: String)

@Serializable
object SubmitMemeOutput
