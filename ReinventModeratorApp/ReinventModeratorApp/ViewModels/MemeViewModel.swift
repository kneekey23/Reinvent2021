//
//  MemeViewModel.swift
//  ReinventModeratorApp
//
//  Created by Stone, Nicki on 11/9/21.
//

import Foundation
import SwiftUI

final class MemeViewModel: ObservableObject {
    @Published var createdTime: String
    @Published var url: URL
    @Published var id: UUID = UUID()
    
    let meme: Meme
    
    init(meme: Meme) {
        self.meme = meme
        self.createdTime = meme.submitTimestamp.toTimeString()
        self.url = URL(string: meme.s3Uri)!
    }
}
