# Next Milestone

## Claude Code quota snapshot

Build the first local proof that Token Usage can observe a real provider allowance without storing or modifying provider credentials.

- [ ] Verify a compatible Claude Code installation and Pro or Max account.
- [x] Create the Tauri desktop application with a React and TypeScript dashboard.
- [x] Add the Rust collector command for Claude Code status-line JSON.
- [x] Extract five-hour and seven-day percentage and reset fields when present.
- [x] Store a normalized snapshot with source and observation time.
- [x] Preserve an absent rate-limit field as `unavailable`.
- [x] Show the current reading and freshness in the desktop dashboard.
- [x] Test a fresh snapshot and an unavailable snapshot.
- [ ] Test a real account and a passed reset time.

## Done means

Running the desktop collector after a Claude Code response produces a local record that clearly identifies the quota window, provider-reported usage, expected reset time, and when the record was observed.
