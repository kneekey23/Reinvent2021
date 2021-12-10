package aws.sdk.kotlin.example.reinvent_2021.webserv

import java.net.URI

data class S3Uri(val bucket: String, val key: String) {
    companion object {
        operator fun invoke(uriString: String): S3Uri {
            val uri = URI(uriString)
            require(uri.scheme == "s3") { """Unknown URI scheme "${uri.scheme}"""" }
            return S3Uri(uri.host, uri.path.substring(1))
        }
    }
}
