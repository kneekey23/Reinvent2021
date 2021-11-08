//
//  MemeResponse.swift
//  ReinventMemeFeed
//
//  Created by Stone, Nicki on 11/8/21.
//

import Foundation

struct MemeResponse: Decodable {
    let memes: [Meme]
    let continueFrom: Date?
}
