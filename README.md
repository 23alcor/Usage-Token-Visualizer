# Token Usage

Token Usage is an early-stage personal AI capacity planner. The project will help people understand provider-defined usage windows, see whether readings are current, and plan work before a Claude or ChatGPT allowance interrupts a session.

The initial product is developer-focused:

- A native macOS collector and menu-bar experience
- Claude Code and Codex as the first integrations
- Five-hour and weekly allowance windows kept separate
- Local history with explicit source and freshness information
- A later iPhone companion that displays synchronized snapshots

## Current status

The repository contains the shared data model, validation rules, tests, and market/product research. It does not yet connect to a live provider account.

The first engineering milestone is a local feasibility collector that reads documented Claude Code and Codex usage surfaces without modifying provider credentials. The collector must preserve missing or stale data instead of presenting it as unused capacity.

## Repository layout

```text
Sources/TokenUsageCore/        Shared domain model and validation
Tests/TokenUsageCoreTests/     Core behavior tests
docs/                          Architecture and delivery roadmap
research/                      Competitive and platform research notes
AI Usage Tracker Strategy Report.docx
```

## Build and test

```bash
swift test
```

## Product principles

1. Keep subscription allowance, observed tokens, context occupancy, API spend, credits, and throughput limits distinct.
2. Every reading carries its source, observation time, supported scope, and quality state.
3. Missing, unavailable, stale, and estimated values are real states. They never silently become zero.
4. Keep raw activity local by default and avoid collecting prompts, source code, or provider session credentials.
5. Earn provider breadth through tested adapters rather than unsupported claims.

See [docs/ROADMAP.md](docs/ROADMAP.md) for the proposed implementation sequence and [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the initial data contract.

