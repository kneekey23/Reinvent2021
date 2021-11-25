//
//  MemeViewModel.swift
//  ReinventModeratorApp
//
//  Created by Stone, Nicki on 11/9/21.
//

import Foundation
import SwiftUI

final class MemeViewModel: ObservableObject {
    @Published var createdTime: String = ""
    @Published var url: URL? = nil
    @Published var id: UUID = UUID()
    @Published var status: MemeStatus = .rejected
    
    let meme: Meme
    let dynamoService: DynamoService
    
    init(meme: Meme, service: DynamoService) {
        self.meme = meme
        self.dynamoService = service
        self.createdTime = meme.submitTimestamp.toTimeString()
        self.url = meme.url!
        self.status = meme.status
    }
    
    func approve() async throws {
        let success = try await dynamoService.approveImage(meme: meme)
        if success {
            status = .approved
        }
    }
    
    func deny() async throws {
        let success = try await dynamoService.denyImage(meme: meme)
        if success {
            status = .rejected
        }
    }
}
