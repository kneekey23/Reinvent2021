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
import ClientRuntime

final class DynamoService {
    let credProvider: AWSCredentialsProvider
    let s3Config: S3Client.S3ClientConfiguration
    let dynamoClient: DynamoDbClient
    let tableName = "memes"
    
    init() throws {
        let accessKey = try Configuration.value(for: "AWS_ACCESS_KEY")
        let secret = try Configuration.value(for: "AWS_SECRET_KEY")
        let credentials = AWSCredentials(accessKey: accessKey,
                                         secret: secret,
                                         expirationTimeout: 60)
        self.credProvider = try AWSCredentialsProvider.fromStatic(credentials)
        self.s3Config = try S3Client.S3ClientConfiguration(credentialsProvider: credProvider,
                                                         region: "us-east-1")
        let config = try DynamoDbClient.DynamoDbClientConfiguration(credentialsProvider: credProvider,
                                                                    region: "us-east-1")
        self.dynamoClient = DynamoDbClient(config: config)
    }
    
    func getImagesToModerate() async throws -> [Meme]  {
        let scanInput = ScanInput(tableName: tableName)
        let result = try await dynamoClient.scan(input: scanInput)
        guard let items = result.items else {
            return []
        }
        
        let memes = try items.map { meme -> Meme in
            guard let s3Uri = meme["s3Uri"],
                  case let DynamoDbClientTypes.AttributeValue.s(s3Uri) = s3Uri else {
                return try Meme(dictionary: meme, url: nil)
            }
  
            let url = try getPreSignedUrl(s3Uri: s3Uri)
            let sanitizedString = url.absoluteString.replacingOccurrences(of: ":443", with: "")
            let sanitizedUrl = URL(string: sanitizedString)
            return try Meme(dictionary: meme, url: sanitizedUrl)
        }
        return memes
    }
    
    func approveImage(meme: Meme) async throws -> Bool {
        let itemKey = ["id" : DynamoDbClientTypes.AttributeValue.s(meme.id)]
        let updatedValues = ["status": DynamoDbClientTypes.AttributeValueUpdate(action: .put, value: DynamoDbClientTypes.AttributeValue.s("approved")), "approvalTimestamp": DynamoDbClientTypes.AttributeValueUpdate(action:  .put, value: DynamoDbClientTypes.AttributeValue.s(Date().iso8601FractionalSeconds()))]
        let input = UpdateItemInput(attributeUpdates: updatedValues, key: itemKey, tableName: tableName)
        _ = try await dynamoClient.updateItem(input: input)
        return true
    }
    
    func denyImage(meme: Meme) async throws -> Bool {
        let itemKey = ["id" : DynamoDbClientTypes.AttributeValue.s(meme.id)]
        let updatedValues = ["status": DynamoDbClientTypes.AttributeValueUpdate(action: .put, value: DynamoDbClientTypes.AttributeValue.s("rejected"))]
        let input = UpdateItemInput(attributeUpdates: updatedValues, key: itemKey, tableName: tableName)
        _ = try await dynamoClient.updateItem(input: input)
        return true
    }
    
    func getPreSignedUrl(s3Uri: String) throws -> URL {

        let bucketAndKey = s3Uri.substringAfter("s3://")
        let bucket = bucketAndKey.substringBefore("/")
        let key = bucketAndKey.substringAfter("/")
        let input = GetObjectInput(bucket: bucket, key: key)
        guard let url = input.presignURL(config: self.s3Config, expiration: 10000) else {
            print("exiting")
            exit(1)
        }
        print(url)
        return url
    }
}

enum Configuration {
    enum Error: Swift.Error {
        case missingKey, invalidValue
    }

    static func value(for key: String) throws -> String {
        guard let value = Bundle.main.object(forInfoDictionaryKey:key) else {
            throw Error.missingKey
        }

        return value as! String
    }
}
