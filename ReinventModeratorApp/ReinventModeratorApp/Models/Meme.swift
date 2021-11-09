//
//  Meme.swift
//  ReinventModeratorApp
//
//  Created by Stone, Nicki on 11/9/21.
//

import Foundation

struct Meme: Codable {
    let id: String
    let approvalTimestamp: Date
    let s3Uri: String
    let status: MemeStatus
    let submitTimestamp: Date
}

enum MemeStatus: Codable {
    case approved
    case rejected
    case unapproved
}
