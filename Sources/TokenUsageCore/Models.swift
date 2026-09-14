import Foundation

public enum Provider: String, Codable, CaseIterable, Sendable {
    case anthropic
    case openAI = "openai"
    case google
    case openRouter = "openrouter"
    case other
}

public enum ReadingQuality: String, Codable, Sendable {
    case official
    case estimated
    case stale
    case unavailable
}

public enum UsageUnit: String, Codable, Sendable {
    case percent
    case tokens
    case requests
    case credits
    case currency
}

public struct QuotaWindow: Codable, Equatable, Sendable {
    public let durationSeconds: Int?
    public let resetsAt: Date?

    public init(durationSeconds: Int?, resetsAt: Date?) {
        self.durationSeconds = durationSeconds
        self.resetsAt = resetsAt
    }
}

public struct QuotaSnapshot: Codable, Equatable, Identifiable, Sendable {
    public let id: UUID
    public let provider: Provider
    public let accountID: String
    public let poolID: String
    public let used: Double?
    public let unit: UsageUnit
    public let window: QuotaWindow
    public let observedAt: Date
    public let source: String
    public let sourceVersion: String?
    public let quality: ReadingQuality

    public init(
        id: UUID = UUID(),
        provider: Provider,
        accountID: String,
        poolID: String,
        used: Double?,
        unit: UsageUnit,
        window: QuotaWindow,
        observedAt: Date,
        source: String,
        sourceVersion: String? = nil,
        quality: ReadingQuality
    ) {
        self.id = id
        self.provider = provider
        self.accountID = accountID
        self.poolID = poolID
        self.used = used
        self.unit = unit
        self.window = window
        self.observedAt = observedAt
        self.source = source
        self.sourceVersion = sourceVersion
        self.quality = quality
    }

    public var remainingPercent: Double? {
        guard unit == .percent, let used else { return nil }
        return max(0, 100 - used)
    }
}

public enum UsageValueKind: String, Codable, Sendable {
    case delta
    case cumulative
}

public struct UsageEvent: Codable, Equatable, Identifiable, Sendable {
    public let id: String
    public let provider: Provider
    public let accountID: String
    public let occurredAt: Date
    public let model: String?
    public let inputTokens: Int?
    public let outputTokens: Int?
    public let cachedInputTokens: Int?
    public let valueKind: UsageValueKind
    public let source: String

    public init(
        id: String,
        provider: Provider,
        accountID: String,
        occurredAt: Date,
        model: String? = nil,
        inputTokens: Int? = nil,
        outputTokens: Int? = nil,
        cachedInputTokens: Int? = nil,
        valueKind: UsageValueKind,
        source: String
    ) {
        self.id = id
        self.provider = provider
        self.accountID = accountID
        self.occurredAt = occurredAt
        self.model = model
        self.inputTokens = inputTokens
        self.outputTokens = outputTokens
        self.cachedInputTokens = cachedInputTokens
        self.valueKind = valueKind
        self.source = source
    }
}

