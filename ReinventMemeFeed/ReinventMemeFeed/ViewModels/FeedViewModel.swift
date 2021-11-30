//
//  FeedViewModel.swift
//  ReinventMemeFeed
//
//  Created by Stone, Nicki on 11/8/21.
//

import Combine
import SwiftUI

final class FeedViewModel: ObservableObject {
    @Published private(set) var memeViewModels: [MemeViewModel] = []
    @Published private(set) var hasMoreMemes: Bool = false
    var continueFrom: Date? = nil
    
    let memeService: MemeService
    var getMemesCancellable: AnyCancellable?
    
    init(memeService: MemeService = MemeService()) {
        self.memeService = memeService
    }
    
    func loadMemes() {
        getMemesCancellable = memeService.getMemes(continueFrom: continueFrom).sink { result in
            switch result {
            case .finished:
                print("call to get memes finished")
            case .failure(let err):
                print(err)
            }
        } receiveValue: { memeResponse in
            
            self.memeViewModels = memeResponse.memes.map({ meme in
                MemeViewModel(meme: meme)
            })
            self.hasMoreMemes = memeResponse.continueFrom != nil
            self.continueFrom = memeResponse.continueFrom
        }
    }
}
