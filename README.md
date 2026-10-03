# Token Usage

**A local-first capacity planner for AI subscriptions.**

Token Usage helps you answer a practical question before starting a demanding task: *how much provider-reported capacity do I have left?* It keeps short-window and weekly allowances separate, records when each value was observed, and treats missing data honestly instead of showing it as zero usage.

The project is an early macOS prototype built with **Tauri 2, React, TypeScript, and Rust**. It is public source code, not a hosted service.

## What works today

| Capability | Status |
| --- | --- |
| Native macOS development app | Available |
| Claude Code status-line JSON import | Available |
| Five-hour and seven-day quota snapshots | Available when supplied by the payload |
| Local snapshot history | Available |
| Explicit `unavailable` state for missing limits | Available |
| Automatic Claude Code status-line bridge | Available, opt-in |
| Codex / ChatGPT integration | Planned |
| Menu-bar app, sync, iOS, Android, Chrome extension | Planned |

## Quick start

### Prerequisites

- macOS 14 or later
- Node.js 20 or later
- Rust stable, installed through [Rustup](https://rustup.rs/)
- Xcode Command Line Tools

Clone the repository, install the JavaScript dependencies, and start the native development app:

```bash
git clone https://github.com/23alcor/Usage-Token-Visualizer.git
cd Usage-Token-Visualizer
npm install
npm run tauri dev
```

The first native build downloads Rust dependencies and can require several gigabytes of free disk space.

## Import your first reading

### Automatic setup

With Token Usage open, select **Enable automatic tracking**. The app starts a localhost-only receiver and, if you do not already use a Claude Code status line, creates:

```text
~/.claude/token-usage-statusline.sh
```

It then adds that script as the `statusLine.command` in `~/.claude/settings.json`, with a 60-second refresh interval. Claude Code sends the provider-reported JSON to this local bridge after responses and when quota windows reset. Send one Claude Code message after enabling the connection to record your first snapshot.

If you already have a custom Claude Code status line, Token Usage leaves it unchanged and shows the bridge path for manual integration. It never replaces an existing status-line command.

### Manual fallback

The prototype accepts a provider-reported Claude Code status-line JSON payload. Paste a payload like this into the app’s **Import a Claude Code status line** area and select **Save local reading**:

```json
{
  "rate_limits": {
    "five_hour": {
      "used_percentage": 35,
      "resets_at": "2026-10-02T03:00:00Z"
    },
    "seven_day": {
      "used_percentage": 20,
      "resets_at": "2026-10-06T03:00:00Z"
    }
  }
}
```

The dashboard shows the provider-reported percentage used, calculated remaining percentage, reset time when present, and observation time. A missing limit stays **Unavailable**; it is never interpreted as unused capacity.

On macOS, snapshots are written locally to:

```text
~/Library/Application Support/Token Usage/claude-code-snapshots.json
```

## Privacy and data boundaries

Token Usage is designed to work without collecting prompts, source code, chat transcripts, browser history, provider session cookies, or provider passwords.

The current prototype stores only normalized local quota snapshots: provider, quota window, usage percentage when present, reset time when present, source, quality, and observation time. The automatic bridge forwards status-line JSON only to `127.0.0.1` while the Token Usage app is open; it does not save the raw payload. There is no account system, telemetry pipeline, or cloud sync.

Do not paste credentials, API keys, cookies, or transcripts into the app. The input field is only for structured quota payloads.

## Development checks

```bash
# Frontend type-check and production build
npm run build

# Rust collector tests
cargo test --manifest-path src-tauri/Cargo.toml
```

The Rust tests cover a provider-reported quota/reset payload and the unavailable state for absent limits.

## Architecture

```text
Claude Code status-line JSON
            │
            ▼
Rust collector and normalizer ──► local JSON history
            │
            ▼
React dashboard in a Tauri desktop shell
```

The Rust layer owns provider parsing, timestamp normalization, and local persistence. The React layer presents quota windows and freshness. This boundary keeps future provider adapters and mobile sync separate from the interface.

## Roadmap

1. Validate automatic Claude Code readings against real accounts, resets, and existing status-line configurations.
2. Add Codex / ChatGPT local collection and connection-health reporting.
3. Add capacity planning: work blocks, reserves, and conservative forecasts.
4. Add opt-in snapshot sync and an iPhone companion.
5. Add Android, Chrome extension, API-spend, and additional provider adapters only after the core collector is reliable.

See the detailed [delivery roadmap](docs/ROADMAP.md), [architecture notes](docs/ARCHITECTURE.md), and [product research](AI%20Usage%20Tracker%20Strategy%20Report.md).

## Project structure

```text
src/                           React and TypeScript dashboard
src-tauri/                     Rust collector and native Tauri shell
docs/                          Architecture, roadmap, and next milestones
research/                      Competitive and platform research
```

## Contributing

This project is in active prototype development. Issues and pull requests are welcome, especially for tested provider adapters, fixtures for absent/reset values, and setup feedback. Please keep provider credentials and any private usage data out of issues, commits, and pull requests.
