//
//  FeedView.swift
//  ReinventMemeFeed
//
//  Created by Stone, Nicki on 11/8/21.
//

import SwiftUI

struct FeedView: View {
    @StateObject var viewModel: FeedViewModel
    var body: some View {
        NavigationView {
            List {
                ForEach(viewModel.memeViewModels, id: \.id) { memeVm in
                    MemeRow(viewModel: memeVm)
                }
                if viewModel.hasMoreMemes == true {
                    ProgressView()
                    .onAppear {
                        viewModel.loadMemes()
                    }
                }
            }.navigationBarTitleDisplayMode(.inline)
            .toolbar() {
                ToolbarItem(placement: .principal, content: {
                    Text("Generated Memes").foregroundColor(.black)
                })
            }
        }.onAppear() {
            viewModel.loadMemes()
            
        }
    }
}

struct FeedView_Previews: PreviewProvider {
    static var previews: some View {
        FeedView(viewModel: FeedViewModel())
    }
}
