import Foundation

public enum SnapshotValidationError: Error, Equatable {
    case emptyAccountID
    case emptyPoolID
    case emptySource
    case missingValueForAvailableReading
    case percentageOutOfRange
    case invalidWindowDuration
}

public extension QuotaSnapshot {
    func validate() throws {
        guard !accountID.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty else {
            throw SnapshotValidationError.emptyAccountID
        }
        guard !poolID.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty else {
            throw SnapshotValidationError.emptyPoolID
        }
        guard !source.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty else {
            throw SnapshotValidationError.emptySource
        }
        if quality != .unavailable, used == nil {
            throw SnapshotValidationError.missingValueForAvailableReading
        }
        if unit == .percent, let used, !(0...100).contains(used) {
            throw SnapshotValidationError.percentageOutOfRange
        }
        if let duration = window.durationSeconds, duration <= 0 {
            throw SnapshotValidationError.invalidWindowDuration
        }
    }
}

public struct SnapshotKey: Hashable, Sendable {
    public let provider: Provider
    public let accountID: String
    public let poolID: String
    public let durationSeconds: Int?

    public init(snapshot: QuotaSnapshot) {
        provider = snapshot.provider
        accountID = snapshot.accountID
        poolID = snapshot.poolID
        durationSeconds = snapshot.window.durationSeconds
    }
}

public enum SnapshotSelection {
    public static func current(from snapshots: [QuotaSnapshot]) -> [SnapshotKey: QuotaSnapshot] {
        snapshots.reduce(into: [:]) { result, snapshot in
            let key = SnapshotKey(snapshot: snapshot)
            guard let existing = result[key] else {
                result[key] = snapshot
                return
            }
            if snapshot.observedAt > existing.observedAt {
                result[key] = snapshot
            }
        }
    }
}

