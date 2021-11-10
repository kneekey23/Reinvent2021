//
//  DynamoService.swift
//  ReinventModeratorApp
//
//  Created by Stone, Nicki on 11/9/21.
//

import Foundation
import AWSDynamoDB
import AWSS3
import AWSClientRuntime

final class DynamoService {
    let credProvider: AWSCredentialsProvider
    
    init() throws {
        let credentials = AWSCredentials(accessKey: "AKIA3XEC53A2LM6SWXOV", secret: "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", expirationTimeout: 60)
        self.credProvider = try AWSCredentialsProvider.fromStatic(credentials)
    }
    
    func getImagesToModerate() async throws -> [Meme]  {
        let tableName = "memes"
        let config = try DynamoDbClient.DynamoDbClientConfiguration(credentialsProvider: credProvider, region: "us-east-1")
        let client = DynamoDbClient(config: config)
        let scanInput = ScanInput(tableName: tableName)
        let result = try await client.scan(input: scanInput)
        guard let items = result.items else {
            return []
        }
        
        for meme in items {
            print(meme)
        }
        
       let memes = try items.map { meme in
           try Meme(dictionary: meme)
        }
        
        for var meme in memes {
            meme.url = try await getPreSignedUrl(s3Uri: meme.s3Uri)
        }
        
        return memes
    }
    
    func getPreSignedUrl(s3Uri: String) async throws -> URL {
        let config = try S3Client.S3ClientConfiguration(credentialsProvider: credProvider,
                                                        region: "us-east-1")
        let bucketAndKey = s3Uri.substringAfter("s3://")
        let bucket = bucketAndKey.substringBefore("/")
        let key = bucketAndKey.substringAfter("/")
        let input = GetObjectInput(bucket: bucket, key: key).presign(config: config, expiration: 86400)
        print(input?.endpoint.url!)
        return (input?.endpoint.url)!
    }
 }
