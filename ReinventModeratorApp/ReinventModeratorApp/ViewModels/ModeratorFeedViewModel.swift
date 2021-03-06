//
//  ModeratorFeedViewModel.swift
//  ReinventModeratorApp
//
//  Created by Stone, Nicki on 11/9/21.
//

import Foundation
import SwiftUI

final class ModeratorFeedViewModel: ObservableObject {
    @Published private(set) var memeViewModels: [MemeViewModel] = []
    
    let dynamoService: DynamoService
    
    init() {
        
        self.dynamoService = try! DynamoService()

    }
    
    func loadMemesToModerate() async throws {
       let result = try await dynamoService.getImagesToModerate()
        self.memeViewModels = result.map { meme in
            MemeViewModel(meme: meme, service: dynamoService)
        }
    }
    
}
