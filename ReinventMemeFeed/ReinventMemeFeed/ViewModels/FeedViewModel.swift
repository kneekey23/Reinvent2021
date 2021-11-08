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
    
    let memeService: MemeService
    var getMemesCancellable: AnyCancellable?
    
    init(memeService: MemeService = MemeService()) {
        self.memeService = memeService
    }
    
    func loadMemes() {
        getMemesCancellable = memeService.getMemes().sink { result in
            switch result {
            case .finished:
                print("call to get memes finished")
            case .failure(let err):
                print(err)
            }
        } receiveValue: { memes in
            for meme in memes {
                self.memeViewModels.append(MemeViewModel(meme: meme))
            }
        }
    }
}
