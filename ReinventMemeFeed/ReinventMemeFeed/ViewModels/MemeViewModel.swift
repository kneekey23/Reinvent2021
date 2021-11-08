//
//  MemeViewModel.swift
//  ReinventMemeFeed
//
//  Created by Stone, Nicki on 11/8/21.
//

import Combine
import SwiftUI

final class MemeViewModel: ObservableObject {
    @Published var createdTime: String
    @Published var url: URL
    @Published var id: UUID = UUID()
    
    let meme: Meme
    
    init(meme: Meme) {
        self.meme = meme
        self.createdTime = meme.timestamp.toTimeString()
        self.url = URL(string: meme.url)!
    }
}
