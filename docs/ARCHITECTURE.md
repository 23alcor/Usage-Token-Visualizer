# Initial Architecture

## Components

The first release should keep five responsibilities separate:

1. Provider adapters observe documented local data sources.
2. Normalization converts provider records into versioned domain objects.
3. Local storage retains snapshots and activity without prompt content.
4. Planning computes transparent, conservative capacity ranges.
5. Native views present allowance, freshness, and connection health.

The Tauri core owns the provider-independent model in Rust. The React client renders these normalized records and can share its interface and API types with future mobile and extension clients.

## Data rules

`QuotaSnapshot` represents a provider's state at one observation time. Snapshots from two devices must not be added together. For the same provider, account, pool, and window, the newest valid observation normally replaces the older current state while history retains both.

`UsageEvent` represents observed activity. Events may be summed only after the adapter establishes whether upstream values are deltas or cumulative counters and provides a stable identity for deduplication.

`MoneyRecord` separates billed amounts from estimates. Subscription activity must not be presented as an invoice merely because an API-equivalent price can be calculated.

`ConnectionHealth` preserves the last successful observation and an actionable failure category. A failed refresh leaves the last observation available with its age rather than replacing it with zero.

## Adapter contract

Each provider adapter must declare:

- Supported product, plans, and minimum version
- Data source and required permission
- Whether its scope is account-wide or local-only
- Update trigger and expected freshness
- Known blind spots
- Schema version

Adapters accept untrusted input and must reject invalid percentages, units, timestamps, and identifiers. Fixtures should cover absent fields, nulls, resets, late records, duplicate events, multiple accounts, clock skew, and partial writes.

## Initial integrations

Claude Code should use documented status-line quota fields for eligible subscription accounts and optional OpenTelemetry for activity. The integration must compose with an existing status-line configuration and support clean removal.

Codex should use its documented local App Server account usage and rate-limit methods. The prototype must verify actual installed-version behavior and record nullable or absent fields honestly.

Provider credentials remain owned by provider-supported components. The application should store only its own pairing secrets or explicitly supported API credentials in platform secure storage.
