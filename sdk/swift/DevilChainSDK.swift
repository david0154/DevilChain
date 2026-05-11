// DevilChain Swift/iOS SDK
import Foundation

public class DevilChainSDK {
    private let baseURL: String
    public init(baseURL: String = "http://localhost:8545") { self.baseURL = baseURL }

    private func get(_ path: String) async throws -> [String: Any] {
        guard let url = URL(string: baseURL + path) else { throw URLError(.badURL) }
        let (data, _) = try await URLSession.shared.data(from: url)
        return try JSONSerialization.jsonObject(with: data) as? [String: Any] ?? [:]
    }
    private func post(_ path: String, body: [String: Any]) async throws -> [String: Any] {
        guard let url = URL(string: baseURL + path) else { throw URLError(.badURL) }
        var req = URLRequest(url: url)
        req.httpMethod = "POST"; req.setValue("application/json", forHTTPHeaderField: "Content-Type")
        req.httpBody = try JSONSerialization.data(withJSONObject: body)
        let (data, _) = try await URLSession.shared.data(for: req)
        return try JSONSerialization.jsonObject(with: data) as? [String: Any] ?? [:]
    }
    public func getStatus() async throws -> [String: Any] { try await get("/api/status") }
    public func getLatestBlock() async throws -> [String: Any] { try await get("/api/block/latest") }
    public func getWallet(_ addr: String) async throws -> [String: Any] { try await get("/api/wallet/\(addr)") }
    public func sendTransaction(from: String, to: String, amount: Double, gasFee: Double, signature: String) async throws -> [String: Any] {
        try await post("/api/send", body: ["from": from, "to": to, "amount": amount, "gas_fee": gasFee, "signature": signature])
    }
    public func stake(address: String, amount: Double, signature: String) async throws -> [String: Any] {
        try await post("/api/stake", body: ["address": address, "amount": amount, "signature": signature])
    }
}
