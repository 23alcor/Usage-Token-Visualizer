# Next Milestone

## Claude Code quota snapshot

Build the first local proof that Token Usage can observe a real provider allowance without storing or modifying provider credentials.

- [ ] Verify a compatible Claude Code installation and Pro or Max account.
- [ ] Create a `token-usage` executable target in the Swift package.
- [ ] Accept Claude Code status-line JSON on standard input.
- [ ] Extract five-hour and seven-day percentage and reset fields when present.
- [ ] Store a normalized snapshot with source, version, and observation time.
- [ ] Preserve an absent rate-limit field as `unavailable`.
- [ ] Print a terminal summary with its freshness state.
- [ ] Test a fresh snapshot, an unavailable snapshot, and a passed reset time.

## Done means

Running the collector after a Claude Code response produces a local record that clearly identifies the quota window, provider-reported usage, expected reset time, and when the record was observed.

