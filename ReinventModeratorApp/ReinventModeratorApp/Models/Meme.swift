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
    var url: URL?
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
        self.url = URL(string: "")
    }
    
    
    public init(dictionary: [String: DynamoDbClientTypes.AttributeValue], url: URL?) throws {
        guard let id = dictionary[DynamoDBField.id],
              let status = dictionary[DynamoDBField.status],
              let s3Uri = dictionary[DynamoDBField.s3Uri],
              let submitTimestamp = dictionary[DynamoDBField.submitTimestamp] else {
                  throw APIError.decodingError
              }
        
        guard case let DynamoDbClientTypes.AttributeValue.s(id) = id,
              case let DynamoDbClientTypes.AttributeValue.s(status) = status,
              case let DynamoDbClientTypes.AttributeValue.s(s3Uri) = s3Uri,
              case let DynamoDbClientTypes.AttributeValue.s(submitTimestamp) = submitTimestamp, let submitTimestamp = DateFormatter.iso8601DateFormatterWithFractionalSeconds.date(from: submitTimestamp) else {
                  throw APIError.decodingError
              }
        self.id = id
        self.status = MemeStatus(rawValue: status) ?? .unapproved
        if let approvalTimestampString = dictionary[DynamoDBField.approvalTimestamp] {
            if case let DynamoDbClientTypes.AttributeValue.s(approvalTimestampString) = approvalTimestampString {
                self.approvalTimestamp = DateFormatter.iso8601DateFormatterWithFractionalSeconds.date(from: approvalTimestampString)
            } else {
                self.approvalTimestamp = nil
            }
        } else {
            self.approvalTimestamp = nil
        }
        self.s3Uri = s3Uri
        self.submitTimestamp = submitTimestamp
        self.url = url
    }
}

enum MemeStatus: String, Codable {
    case approved
    case rejected
    case unapproved
}
