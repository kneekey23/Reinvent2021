//
//  DynamoService.swift
//  ReinventModeratorApp
//
//  Created by Stone, Nicki on 11/9/21.
//

import Foundation
import AWSDynamoDB
import AWSClientRuntime

final class DynamoService {
    
    func getImagesToModerate() async throws -> [Meme]  {
        let tableName = "memes"
        let credentials = AWSCredentials(accessKey: "AKIA3XEC53A2LM6SWXOV", secret: "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", expirationTimeout: 60)
        let credProvider = try AWSCredentialsProvider.fromStatic(credentials)
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
        
        return try items.map { meme in
           try Meme(dictionary: meme)
        }
    }
}
