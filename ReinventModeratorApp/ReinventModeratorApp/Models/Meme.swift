//
//  Meme.swift
//  ReinventModeratorApp
//
//  Created by Stone, Nicki on 11/9/21.
//

import Foundation
import AWSDynamoDB
enum APIError: Error {
    case decodingError
    case requestError
    case memeNotFound
}

struct Meme: Codable {
    let id: String
    let approvalTimestamp: Date?
    let s3Uri: String
    let status: MemeStatus
    let submitTimestamp: Date
    
    public struct DynamoDBField {
        static let id = "id"
        static let approvalTimestamp = "approvalTimestamp"
        static let s3Uri = "s3Uri"
        static let status = "status"
        static let submitTimestamp = "submitTimestamp"
    }
    
    public init(id: String,
                approvalTimestamp: Date,
                s3Uri: String,
                status: MemeStatus,
                submitTimestamp: Date){
        self.id = id
        self.approvalTimestamp = approvalTimestamp
        self.s3Uri = s3Uri
        self.status = status
        self.submitTimestamp = submitTimestamp
    }
    
    
    public init(dictionary: [String: DynamoDbClientTypes.AttributeValue]) throws {
        guard let id = dictionary[DynamoDBField.id],
            let status = dictionary[DynamoDBField.status],
            let approvalTimestamp = dictionary[DynamoDBField.approvalTimestamp],
            let s3Uri = dictionary[DynamoDBField.s3Uri],
            let submitTimestamp = dictionary[DynamoDBField.submitTimestamp] else {
                throw APIError.decodingError
        }
        
        guard case let DynamoDbClientTypes.AttributeValue.s(id) = id,
              case let DynamoDbClientTypes.AttributeValue.s(status) = status,
              case let DynamoDbClientTypes.AttributeValue.s(approvalTimestamp) = approvalTimestamp,
              case let DynamoDbClientTypes.AttributeValue.s(s3Uri) = s3Uri,
              case let DynamoDbClientTypes.AttributeValue.s(submitTimestamp) = submitTimestamp, let submitTimestamp = DateFormatter.iso8601DateFormatterWithFractionalSeconds.date(from: submitTimestamp) else {
                  throw APIError.decodingError
              }
        self.id = id
        self.status = MemeStatus(rawValue: status) ?? .unapproved
        self.approvalTimestamp = DateFormatter.iso8601DateFormatterWithFractionalSeconds.date(from: approvalTimestamp)
        self.s3Uri = s3Uri
        self.submitTimestamp = submitTimestamp
    }
}

enum MemeStatus: String, Codable {
    case approved
    case rejected
    case unapproved
}
