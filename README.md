# Token Usage

Token Usage is an early-stage personal AI capacity planner. It helps people understand provider-defined usage windows, see whether readings are current, and plan work before a Claude or ChatGPT allowance interrupts a session.

The initial product is developer-focused and built with Tauri 2, React, TypeScript, and Rust:

- A native macOS collector and menu-bar experience
- Claude Code and Codex as the first integrations
- Five-hour and weekly allowance windows kept separate
- Local history with explicit source and freshness information
- A later iPhone companion that displays synchronized snapshots

## Current status

The repository contains a Tauri desktop application, a local Claude Code status-line collector, and market/product research.

The first engineering milestone is a local feasibility collector that reads documented Claude Code and Codex usage surfaces without modifying provider credentials. The collector must preserve missing or stale data instead of presenting it as unused capacity.

## Repository layout

```text
src/                           React and TypeScript dashboard
src-tauri/                     Rust collector and Tauri desktop shell
docs/                          Architecture and delivery roadmap
research/                      Competitive and platform research notes
AI Usage Tracker Strategy Report.docx
```

## Build and test

Install Node.js, Rust, and the Tauri prerequisites, then run:

```bash
npm install
npm run tauri dev
```

## First local collector

Paste a Claude Code status-line JSON payload into the desktop app. The Rust collector saves provider-reported five-hour and seven-day snapshots locally. On macOS, they live under `~/Library/Application Support/Token Usage/claude-code-snapshots.json`.

The collector never reads or stores provider credentials, prompts, or transcripts. Missing rate-limit fields remain `unavailable`.

## Product principles

1. Keep subscription allowance, observed tokens, context occupancy, API spend, credits, and throughput limits distinct.
2. Every reading carries its source, observation time, supported scope, and quality state.
3. Missing, unavailable, stale, and estimated values are real states. They never silently become zero.
4. Keep raw activity local by default and avoid collecting prompts, source code, or provider session credentials.
5. Earn provider breadth through tested adapters rather than unsupported claims.

See [docs/ROADMAP.md](docs/ROADMAP.md) for the proposed implementation sequence and [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the initial data contract.
