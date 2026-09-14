import XCTest
@testable import TokenUsageCore

final class QuotaSnapshotTests: XCTestCase {
    func testUnavailableReadingPreservesMissingValue() throws {
        let snapshot = QuotaSnapshot(
            provider: .anthropic,
            accountID: "local-account",
            poolID: "five-hour",
            used: nil,
            unit: .percent,
            window: QuotaWindow(durationSeconds: 18_000, resetsAt: nil),
            observedAt: Date(timeIntervalSince1970: 100),
            source: "claude-code-statusline",
            quality: .unavailable
        )

        XCTAssertNoThrow(try snapshot.validate())
        XCTAssertNil(snapshot.remainingPercent)
    }

    func testPercentageCannotExceedProviderRange() {
        let snapshot = QuotaSnapshot(
            provider: .openAI,
            accountID: "local-account",
            poolID: "primary",
            used: 101,
            unit: .percent,
            window: QuotaWindow(durationSeconds: 18_000, resetsAt: nil),
            observedAt: Date(),
            source: "codex-app-server",
            quality: .official
        )

        XCTAssertThrowsError(try snapshot.validate()) { error in
            XCTAssertEqual(error as? SnapshotValidationError, .percentageOutOfRange)
        }
    }

    func testNewestSnapshotReplacesCurrentStateWithoutSumming() {
        let older = makeSnapshot(used: 60, observedAt: 100)
        let newer = makeSnapshot(used: 20, observedAt: 200)

        let current = SnapshotSelection.current(from: [older, newer])

        XCTAssertEqual(current.count, 1)
        XCTAssertEqual(current.values.first?.used, 20)
    }

    private func makeSnapshot(used: Double, observedAt: TimeInterval) -> QuotaSnapshot {
        QuotaSnapshot(
            provider: .anthropic,
            accountID: "local-account",
            poolID: "five-hour",
            used: used,
            unit: .percent,
            window: QuotaWindow(durationSeconds: 18_000, resetsAt: nil),
            observedAt: Date(timeIntervalSince1970: observedAt),
            source: "test-fixture",
            quality: .official
        )
    }
}

