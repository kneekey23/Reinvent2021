//
//  LabelView.swift
//  ReinventMemeFeed
//
//  Created by Stone, Nicki on 11/8/21.
//

import SwiftUI

struct LabelView: View {
    let text: String
    let size: CGFloat
    let color: Color
    let isBold: Bool
    @Environment(\.colorScheme) var colorScheme
    
    init(text: String, size: CGFloat = 20, color: Color = Color.black, isBold: Bool = false) {
        self.text = text
        self.size = size
        self.color = color
        self.isBold = isBold
    }
    var body: some View {
        if isBold {
            Text(text)
                .font(.system(size: size))
                .foregroundColor(color)
                .bold()
        } else {
            Text(text)
                .font(.system(size: size))
                .foregroundColor(color)
        }
    }
}

struct LabelView_Previews: PreviewProvider {
    static var previews: some View {
        LabelView(text: "Test", size: 12)
    }
}
