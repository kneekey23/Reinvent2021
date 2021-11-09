//
//  ReinventModeratorAppApp.swift
//  ReinventModeratorApp
//
//  Created by Stone, Nicki on 11/9/21.
//

import SwiftUI

@main
struct ReinventModeratorAppApp: App {
    var body: some Scene {
        WindowGroup {
            ModeratorFeedView(viewModel: ModeratorFeedViewModel())
        }
    }
}
