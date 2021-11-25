//
//  MemeRow.swift
//  ReinventModeratorApp
//
//  Created by Stone, Nicki on 11/9/21.
//

import SwiftUI

struct MemeRow: View {
    @ObservedObject var viewModel: MemeViewModel
    
    var body: some View {
        VStack(alignment: .center) {
            Text("Status: \(viewModel.status.rawValue)")
            AsyncImage(url: viewModel.url){ image in
                image.resizable()
                    .scaledToFill()
            } placeholder: {
                ProgressView()
            }
            HStack {
                Button(action: {
                    Task {
                        try? await viewModel.approve()
                    }
                }) {
                    Text("Approve")
                        .padding()
                        .foregroundColor(Color.white)
                        .background(Color.green)
                        
                }.buttonStyle(BorderlessButtonStyle())
                Button(action: {
                    Task {
                        try? await viewModel.deny()
                    }
                }) {
                    Text("Deny")
                        .padding()
                        .foregroundColor(Color.white)
                        .background(Color.red)
                        
                }.buttonStyle(BorderlessButtonStyle())
            }
            LabelView(text: "Generated at: \(viewModel.createdTime)", isBold: true)
        }.listRowInsets(EdgeInsets())
        .frame(maxWidth: .infinity, alignment: .center)
        .cornerRadius(20)
        .shadow(color: Color.black.opacity(0.2), radius: 20, x: 0, y: 0)
    }
}

struct MemeRow_Previews: PreviewProvider {
    static var previews: some View {
        MemeRow(viewModel: MemeViewModel(meme: Meme(id: "123", approvalTimestamp: Date(), s3Uri: "test.com", status: .unapproved, submitTimestamp: Date()), service: try! DynamoService()))
    }
}
