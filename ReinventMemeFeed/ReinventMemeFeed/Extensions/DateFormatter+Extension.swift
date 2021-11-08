//
//  DateFormatter+Extension.swift
//  ReinventMemeFeed
//
//  Created by Stone, Nicki on 11/8/21.
//

import Foundation

extension DateFormatter {
    
    /*
    Configures ISO 8601 Date Formatter With Fractional Seconds
    https://xml2rfc.tools.ietf.org/public/rfc/html/rfc3339.html#anchor14
    */
    static let iso8601DateFormatterWithFractionalSeconds =
        getDateFormatter(dateFormat: "yyyy-MM-dd'T'HH:mm:ss.SSSZ")

    static let longDateFormatter: DateFormatter = getDateFormatter(dateFormat: "MMMM d, yyyy h:mm a")

    static var time: DateFormatter = getDateFormatter(dateFormat: "h:mm a")
    
    static var day: DateFormatter = getDateFormatter(dateFormat: "MMMM d")
    
    private static func getDateFormatter(dateFormat: String) -> DateFormatter {
        let formatter = DateFormatter()
        formatter.dateFormat = dateFormat
        formatter.locale = Locale(identifier: "en_US_POSIX")
        formatter.timeZone = TimeZone.autoupdatingCurrent
        return formatter
    }
}

extension Date {
    func iso8601FractionalSeconds() -> String {
        return DateFormatter.iso8601DateFormatterWithFractionalSeconds.string(from: self)
    }
    
    func toDayString() -> String {
        return DateFormatter.day.string(from: self)
    }
    
    func toTimeString() -> String {
        return DateFormatter.time.string(from: self)
    }
    
    func toDateAndTimeString() -> String {
        return DateFormatter.longDateFormatter.string(from: self)
    }
}
