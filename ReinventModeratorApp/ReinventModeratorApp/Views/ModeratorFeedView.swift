//
//  ContentView.swift
//  ReinventModeratorApp
//
//  Created by Stone, Nicki on 11/9/21.
//

import SwiftUI

struct ModeratorFeedView: View {
    @StateObject var viewModel: ModeratorFeedViewModel
    var body: some View {
        NavigationView {
            List {
                ForEach(viewModel.memeViewModels, id: \.id) { memeVm in
                    MemeRow(viewModel: memeVm)
                }
            }.navigationBarTitleDisplayMode(.inline)
            .toolbar() {
                ToolbarItem(placement: .principal, content: {
                    Text("Generated Memes For Approval").foregroundColor(.black)
                })
            }
        }.task {
            try? await viewModel.loadMemesToModerate()
        }.refreshable {
            try? await viewModel.loadMemesToModerate()
        }
    }
}

struct ModeratorFeedView_Previews: PreviewProvider {
    static var previews: some View {
        ModeratorFeedView(viewModel: ModeratorFeedViewModel())
    }
}
