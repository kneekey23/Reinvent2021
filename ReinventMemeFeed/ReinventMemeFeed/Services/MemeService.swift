//
//  MemeService.swift
//  ReinventMemeFeed
//
//  Created by Stone, Nicki on 11/8/21.
//
import Foundation
import Combine

enum ServiceError: Error {
    case url(URLError)
    case urlRequest
    case decode
    case internalError(String)
}

final class MemeService {
    let urlSession = URLSession.shared
    let API_URL = "http://reinvent2021webserv-env.eba-zn2fjn6v.us-east-1.elasticbeanstalk.com"
    private var getMemesCancellable: AnyCancellable?
    init () {}
    
    func getMemes() -> AnyPublisher<[Meme], ServiceError> {
        let url = API_URL + "/memes/list"
        
        var dataTask: URLSessionDataTask?
        
        let onSubscription: (Subscription) -> Void = { _ in dataTask?.resume() }
        let onCancel: () -> Void = { dataTask?.cancel() }
        
        return Future<[Meme], ServiceError> { [self] promise in
            guard let url = URL(string: url) else {
                promise(.failure(ServiceError.urlRequest))
                return
            }
            
            var urlRequest = URLRequest(url: url)
            urlRequest.timeoutInterval = Double.infinity
            urlRequest.httpMethod = "GET"
            
            dataTask = urlSession.dataTask(with: urlRequest) { (data, _, error) in
                guard let data = data else {
                    if let error = error {
                        promise(.failure(.internalError(error.localizedDescription)))
                    }
                    return
                }
                print(String(data: data, encoding: .utf8)!)
                
                do {
                    let decoder = JSONDecoder()
                    decoder.dateDecodingStrategy = .formatted(DateFormatter.iso8601DateFormatterWithFractionalSeconds)
                    let memeResponse = try decoder.decode(MemeResponse.self, from: data)
                    promise(.success(memeResponse.memes))
                } catch {
                    promise(.failure(ServiceError.decode))
                }
            }
        }
        .receive(on: DispatchQueue.main)
        .handleEvents(receiveSubscription: onSubscription, receiveCancel: onCancel)
        .eraseToAnyPublisher()
    }
}
