//
//  ReinventMemeFeedApp.swift
//  ReinventMemeFeed
//
//  Created by Stone, Nicki on 11/8/21.
//

import SwiftUI

@main
struct ReinventMemeFeedApp: App {
    var body: some Scene {
        WindowGroup {
            FeedView(viewModel: FeedViewModel())
        }
    }
}
