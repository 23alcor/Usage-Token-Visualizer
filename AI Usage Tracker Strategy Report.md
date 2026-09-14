# AI Usage Tracker Market and Product Strategy

An AI usage tracker can be a strong flagship engineering project. Its commercial prospects depend on solving a narrower problem exceptionally well: helping people finish planned work without unexpected allowance exhaustion. A GitHub-inspired dashboard can make that experience attractive, but the proposed combination of usage bars, heatmaps, forecasts, alerts, and Apple-device sync already has close competitors.

The recommended entry point is developers who use both Claude Code and Codex on a Mac. Build a trustworthy local collector and a work-planning experience for that audience, then add an iPhone companion. Expand to everyday chat subscribers and Android only when each integration has a demonstrated, sustainable data path. Treat “any model” as an extensibility goal, not a launch promise.

## 1 Executive assessment

**Recommendation: proceed with a focused validation phase before building the full application suite.** The strongest product thesis is a personal AI capacity planner: it shows which allowance constrains the next work session, explains how certain that reading is, and helps the person preserve capacity for important work. This is a differentiation hypothesis, not an established unoccupied market.

Existing tools set a demanding baseline. CodexBar offers a free, open-source multi-provider desktop experience; AI Limits & Reset Tracker advertises a GitHub-style heatmap, pacing, widgets, and Apple-device sync. Their existence makes a generic “all your AI usage in one place” proposition difficult to charge for.[^1][^2]

Three opportunities deserve testing. First, measurable reliability: can the tracker consistently tell the user when its numbers are trustworthy? Second, planning: can it help someone allocate a two-hour work block and preserve weekly headroom better than a meter? Third, accountability: can it explain changes and produce a private, redacted record when consumption surprises the user?

For a portfolio project, this is attractive because it can demonstrate native apps, event ingestion, synchronization, observability, forecasting, security, and careful product design. For a business, broad demand and willingness to pay remain unproven. Public app listings, repositories, and individual complaints do not establish market size, revenue, retention, or the frequency of these problems.

**Evidence scope.** Findings were checked on September 13, 2026 using official documentation, product sites, App Store listings, public repositories, and issue/release histories. Product capabilities are published claims, not hands-on benchmarks. Historical fixes are labeled as such. Plans, interfaces, and prices can change; documentation review does not prove live account compatibility. The proposed roadmap assumes an independent developer or small team, with developers as the initial audience.

## 2 Define the product around the right numbers

“Token usage” covers several different products. Mixing them creates misleading dashboards and unreliable forecasts.

| Measure | What it answers | How it should appear |
| --- | --- | --- |
| Observed tokens | How much text or other tokenized input/output did recorded activity consume? | Input, output, and cache categories with source and coverage |
| Subscription allowance | How much of a provider-defined entitlement is used? | Each reported window, percentage, reset, and shared pool |
| Context occupancy | How full is this conversation's current context? | Conversation detail, separate from account allowance |
| API spend | What metered API use cost | Billed amounts or explicitly estimated cost |
| Credits or balance | What prepaid/extra capacity remains? | Its own unit, accounting scope, and timestamp |
| Throughput limits | How quickly requests may be made | Requests/tokens per minute or another provider-defined interval |

Claude distinguishes conversation-length limits from usage limits, and its supported surfaces can contribute to shared allowance. The number of tokens currently in a context is therefore not the remaining Pro/Max budget.[^3] OpenAI describes five-hour local-message estimates, possible weekly limits, and consumption that varies with model, context, tools, reasoning, and caching. Prompt length alone is not a reliable allowance estimate.[^4]

The core product should keep three records separate: account quota snapshots, activity events, and monetary records. A percentage snapshot is a state observation; adding the same account's percentage from two Macs is invalid. Activity events can be summed only after duplication and cumulative-counter semantics are handled. A subscription session's API-equivalent dollar estimate is not an invoice or demonstrated money saved.

Use provider-defined units. If a provider reports 60% used, show 40% remaining in that particular bucket. Do not translate it into an exact remaining token count without a documented conversion. Do not average Claude and Codex percentages into a universal capacity score: their workloads, model capabilities, and entitlement definitions differ.

Likewise, a reset countdown describes an expected event. When the clock reaches zero, show “reset expected; awaiting update” until refreshed data arrives. An absent field can mean unavailable, inactive, unsupported, or not yet fetched. It must not silently become zero usage.

## 3 Competitive landscape

The closest alternatives already cover much of the initial concept. Prices below are observed offers, not evidence of commercial success. Regional pricing and repository-versus-release differences matter.

| Product | Published scope and price | Strategic implication |
| --- | --- | --- |
| CodexBar | Free/MIT; macOS 14+ and CLI. Website advertises 69 providers, history, widgets, pacing and burndown.[^1] | Provider breadth and basic forecasting face a strong free baseline. |
| ccusage | MIT CLI; current README lists 18 sources, period/session reporting, model/project breakdowns and exports.[^5] | Evaluate reuse or import compatibility for local history. |
| Claude Code Usage Monitor | MIT terminal tool; current v4 README describes official statusline quota input, fallback estimates, provenance, a local warehouse and exports.[^6] | Even source/confidence labeling already exists. |
| TokenBar and Mobile | Free Mac/iPhone apps; published widgets, Watch, alerts and iCloud features. OpenAI integration is described as Admin API.[^7][^8] | Mobile availability does not imply full consumer ChatGPT coverage. |
| AI Limits & Reset Tracker | iPhone/iPad/Mac; heatmaps, pacing and multi-account. US listing: Pro $1.99/month or $17.99/year.[^2] | A direct match for much of the proposed concept, at a low price. |
| AI Session Meter | iPhone/iPad/Mac/Watch; prediction ranges, activity, multi-account and iCloud. German listing: 14-day trial and €5.99 one-time unlock.[^9] | Apple breadth and uncertainty-aware prediction are already offered. |
| SessionWatcher | Native Mac. Solo $6.99 once; Bundle $14.99 once; Pro $59 once or $24/year. Pro includes history, multiple accounts and Mac sync.[^10] | A paid utility reference; compete on demonstrated value. |
| Burnrate | MIT desktop project with macOS/Linux/Windows bundles described; combines subscription quotas and API/cloud spend.[^11] | Cross-platform desktop alone is also occupied. |
| VibeMeter | Repository archived May 3, 2026; explicitly deprecated in favor of CodexBar.[^12] | Treat as a predecessor, not another active competitor. |

Feature descriptions need to be tested on the actual release. A README on the main branch may be ahead of a shipping binary. App Store descriptions can lag release notes. A competitor offering direct subscription login does not establish that the integration is supported by the provider or suitable to copy.

There is also an adjacent API observability market. Helicone documents gateway cost tracking; Langfuse records or infers token/cost data from application generations; LiteLLM supports proxy keys, budgets, and spend tracking.[^13][^14][^15] These products see the workloads routed through or reported to them. That does not automatically reveal an unrelated personal subscription's allowance.

The conclusion is not that the project has no room. It is that feature counts will be a weak basis for differentiation. The useful comparison is the complete experience: installation, first correct reading, accuracy during real work, recovery after breakage, and decisions users make because of the product.

## 4 Where implementations have struggled

The following evidence identifies engineering failure modes. It does not establish that competitors are generally failing or that fixed bugs remain present.

| Evidence and status | Underlying product problem | Requirement for a better implementation |
| --- | --- | --- |
| ai-usagebar issue 148, September 4; closed, with September 11 fix documented.[^16][^17] | Writing shared Keychain credentials triggered repeated Claude Code permission prompts. | Preserve credential ownership; avoid modifying the monitored app's sessions. |
| ai-usagebar September 6 release notes describe a fixed Codex parser failure on null optional fields.[^17] | Small upstream schema changes can disable readings. | Tolerant parsing, versioned fixtures, explicit unsupported/stale states. |
| ccusage issue 988, May 12; closed with referenced fixes.[^18] | Forked Codex conversations could duplicate inherited activity in totals. | Test branches, replayed history, cumulative counters and event identity. |
| SessionWatcher August 5, 25 and 30 release notes list fixed scanning, polling, aggregation and history problems.[^19] | A passive utility can waste battery or corrupt the history people rely on. | Incremental reads, bounded retries, durable writes and defined aggregation semantics. |
| CodexBar issue 1287, June 3; closed.[^20] | A user reported expired-session states with an unhelpful recovery path. | Distinguish causes and provide a working reconnect action. |
| Usage for Claude App Store review, April 20.[^21] | One user wanted sync across different Apple IDs. | Test demand for explicit device pairing beyond iCloud. |

These examples suggest a reliability program, not just more error messages. Track the proportion of time supported integrations have fresh readings, whether displayed values agree with the provider at comparable times, and how long reconnection takes. Separate an upstream outage from a defect in the tracker, but show the user how both affect the current recommendation.

Another useful job is documenting unexpected consumption. A July 24 Claude Code issue reports extra-usage balance falling while included allowance appeared available; the cause is unverified.[^22] A tracker can record the observed balance change, the last quota snapshot, and source timestamps. It should not invent an explanation or imply wrongdoing. A redacted evidence export can help users investigate without sharing prompts or code.

The evidence also warns against outdated positioning. An older request for quota values in Claude Code's statusline is not evidence they are missing now. Several tools have already added provenance and better estimates. Every advertised advantage should be checked against current releases before launch.

## 5 What automatic tracking can realistically support

“Automatic” needs a coverage contract for each integration: source, permission, supported plans and versions, observable surfaces, update trigger, and known blind spots.

| Integration | Documented data path | Scope and launch recommendation |
| --- | --- | --- |
| Claude Code subscription | Statusline export in v2.1.251+ includes five-hour/seven-day usage and resets for eligible accounts, after a first response.[^23] | Primary Mac adapter. Observe official exports; test freshness and cross-surface coverage. |
| Claude Code activity | Opt-in OpenTelemetry includes token/cost metrics and model attribution.[^24] | Optional history input. Disable prompt capture and unnecessary identity fields. |
| Codex backed by ChatGPT | App Server documents account/rateLimits/read, updates and account/usage/read; fields may be null.[^25] | Primary local adapter using documented integration behavior; test installed-version compatibility. |
| Claude consumer app beyond Code | Shared usage is documented; this review does not establish a general third-party consumer quota API.[^3] | Supported local observations can help; do not promise always-current native-mobile coverage. |
| Other ChatGPT surfaces | App Server exposes service-backed account data, but that does not prove every ChatGPT feature/limit is covered.[^25] | Label the actual bucket/surface. Validate before broadening the marketing claim. |
| OpenAI API | Organization usage and costs endpoints.[^26] | Separate API accounting feature with appropriate administrative access. |
| Anthropic API | Usage/Cost Admin API for organizations; individual accounts are excluded.[^27] | Later professional integration; not a substitute for Pro/Max allowance. |
| Gemini API | Response token metadata and project-level usage/rate-limit tooling.[^28][^29] | Later API adapter; do not infer Gemini consumer-app entitlement. |
| OpenRouter | GET /api/v1/key exposes key usage and remaining configured credit limit.[^30] | Straightforward later credit view; distinguish key cap from account balance. |
| Local or other models | Available logs or explicitly instrumented requests, depending on runtime. | Activity tracking only until a specific entitlement source is verified. |

Two newer official paths make a developer-first product more plausible. Claude's quota export is event-driven and may omit inactive or expired windows; it is not continuous account polling. Codex's documented account methods provide provider state and optional daily token activity, but require compatible service-backed authentication. Keep provider-managed authentication inside supported components rather than building a cloud warehouse of copied sessions.[^23][^25]

Anthropic's current guidance prohibits third-party developers from offering Claude.ai login or collecting, storing, or intermediating Claude.ai credentials/session tokens. It distinguishes customer-managed API credentials and use of an unmodified Claude Code binary.[^31] Build the proposed Claude integration around supported local exports; obtain provider clarification before relying on any credential-based commercial connector. This is a design dependency, not a legal judgment about named competitors.

A sensible fallback ladder is: documented quota source; supported local activity source; clearly labeled estimate; manual snapshot or unavailable state. Never quietly replace an account-wide meter with local-only estimates while leaving the appearance unchanged.

## 6 Platform strategy

**macOS should collect and analyze first.** A native menu-bar app is close to developer activity and can keep useful history locally. Prototype the exact file/IPC access under the intended distribution model. Mac App Store sandboxing and user-selected file access impose real constraints; a Developer ID-signed, notarized direct release is another distribution option.[^32][^33] Distribution choice does not change provider authorization rules.

**iOS should begin as a companion.** Apple isolates third-party apps, so an ordinary tracker cannot silently read another native app's private records.[^34] The phone can show synchronized snapshots, history, alerts, and supported account connections. A Safari extension, with site-specific permission, is a different surface from native-app tracking and should be evaluated separately.[^35]

Widget freshness is a product constraint. Apple says frequently viewed widgets typically receive around 40–70 refreshes per day, with variable scheduling. Background execution is also system-controlled.[^36][^37] Display a last-observed time and derive a countdown locally from a known reset timestamp. Do not market the widget as continuously live. A notification scheduled from an old reset should say a reset is expected, not that fresh capacity has been verified.

**Android should reuse the same companion contract.** Android's sandbox also isolates app data, and WorkManager periodic work has a minimum 15-minute interval with inexact execution.[^38][^39] App-usage permissions do not turn screen time into token accounting. Build Android after testing that users actually need the cross-platform connection and that the collection source remains useful when the desktop sleeps.

| Decision | Recommended starting choice | Revisit when |
| --- | --- | --- |
| Mac interface | SwiftUI with a small collector and local database | Verified requirements justify another framework |
| iPhone interface | SwiftUI companion plus WidgetKit | Phone-first users have a supported independent source |
| Initial sync | Opt-in snapshots, initially Apple-oriented if it reduces scope | Different Apple IDs or Android demand is validated |
| Future interoperability | Versioned, platform-neutral data schema | A second client is being implemented |
| Web dashboard | Defer | Sharing or cross-platform access becomes a paid job |

A phone that last heard from a sleeping Mac six hours ago should show that explicitly. Cloud sync transports an observation; it cannot create a fresh provider reading. Work/personal accounts, data permissions, and device preferences must remain independent.

## 7 A product people might choose over free meters

The primary job should be: **help me choose and pace the AI work I can complete before my next important deadline.** Launch for people who already use at least two services several days a week. They have a coordination problem a provider's own meter cannot fully solve.

**Work-session planning.** Let someone enter a planned work block, a provider/model preference, and a desired reserve. Show the limiting window, recent consumption range, and whether evidence is sufficient. A reservation is an internal plan, not a provider-enforced quota lock. Never claim the application can guarantee that a task will finish.

An illustrative interaction: a person plans a 90-minute refactor at 2 p.m. Their selected service has 35% short-window capacity remaining but only 8% weekly capacity. The app flags the weekly constraint, offers a smaller work scope or an eligible alternative they already use, and states when its last readings were obtained. These numbers are an example, not research data or a validated prediction model.

**Reserve-aware advice.** A five-hour reset should not trigger “use it all now” advice if the weekly budget remains tight. Allow workdays, quiet hours, planned sessions, and user-defined reserves. Suggestions should respect capability and data-handling requirements: available quota alone does not make a model suitable for the work.

**Explain changes.** A daily review can show what changed in recorded usage, where attribution is known, and what remains unobserved. Useful explanations include an increase in long sessions or a changed model mix. When data cannot establish a cause, say so. Let users add optional outcome tags such as “shipped fix” without collecting conversation text.

**Prove trust at the point of decision.** Every provider panel should show source, observation age, covered scope, and a repair action. Offer a redacted diagnostic export. These features are not individually novel; the advantage must be lower failure rates and a clearer experience in independent use.

**Preserve the GitHub-inspired design.** Use restrained color, a compact sidebar, tabs, monospace numerical details, and an activity calendar. Keep remaining allowance and its limiting window above the heatmap. Let the grid switch between recorded activity, active days, and user-tagged outcomes. Unknown days should look different from zero-usage days, and higher token consumption should not automatically look like success.

| Main screen area | Content |
| --- | --- |
| Overview | Provider windows, reset times, freshness and limiting constraint |
| Plan | Next work block, reserve and conservative capacity estimate |
| Activity | Heatmap, model/project breakdown and coverage notes |
| Changes | Unusual movements, credit changes and annotations |
| Connections | Supported source, health, version and reconnect action |

The menu bar answers “what is usable now?” The full Mac view answers “how should I plan today?” The phone answers “what was last observed, and when should I return?” Accessibility requires keyboard navigation, scalable text, descriptive labels, and symbols/text alongside color.

## 8 Architecture and forecasting

Keep collection, normalization, storage, prediction, and presentation separate. A provider change should affect one adapter, not force a rewrite of every client. Begin with two adapters and a small documented schema rather than a general plugin marketplace.

**QuotaSnapshot** should identify provider, account/workspace, entitlement pool, window, reported usage, unit, reset time, observation time, source version, and quality state. **UsageEvent** should carry a stable identity, timestamp, source, token categories, model, optional project label, and whether values are deltas or cumulative totals. **MoneyRecord** should identify currency, billing period, and whether an amount is billed or estimated. **ConnectionHealth** records last success and actionable failure category.

Treat all incoming records as untrusted data. Apply an allowlist, strip unneeded identity, validate units and plausible ranges, and retain a schema version. Keep app-owned pairing secrets in platform secure storage.[^40] Avoid syncing provider credentials, prompt text, repository paths, or raw logs by default. Local-only operation should remain useful; cloud sync should have an explicit delete/revoke flow.

For multiple devices, upsert quota snapshots by account, pool and window, resolving by source validity and observation time. Do not sum them. For activity, deduplicate stable upstream IDs, detect inherited session prefixes, and record uncertainty where identity is insufficient. Keep model cache categories consistent with each source rather than assuming every token field is disjoint.

Use incremental file offsets or event subscriptions, bounded retry/backoff, atomic database transactions, and a single polling owner per local account. Respect provider retry hints. A retry loop that consumes battery or interferes with the coding tool defeats the purpose of a passive utility.

**Start forecasting with a transparent baseline.** For a window with remaining percentage R and recent comparable burn b percentage points per active minute, R/b is a rough depletion estimate. If b is zero, data is stale, or the workload changes, the result is not meaningful. Model short and weekly windows separately; apply the constraint relevant to the proposed workload. Never use a new short-window allowance to imply the weekly limit disappeared.

Build ranges from observed comparable sessions and evaluate interval coverage on held-out sessions. Reset events, missing observations, shared usage elsewhere, and policy changes should lower confidence. At cold start, show recent pace or insufficient history. A deterministic explanation is enough initially; adding an LLM to narrate a bar introduces cost and another failure path without proving value.

Maintain a versioned adapter test corpus covering nulls, absent windows, unknown models, resets, late/out-of-order snapshots, duplicate/forked logs, offline devices, clock skew, multiple accounts, and partial writes. A kill switch should disable a broken adapter and explain the limitation while leaving other providers functional.

## 9 Roadmap and acceptance gates

The following 12-week sequence is a planning estimate for one experienced full-time developer, with limited design help. It is not a delivery commitment; authentication, store review, and synchronization can extend it. Gate later platforms on evidence rather than dates.

| Phase | Deliverable | Exit condition |
| --- | --- | --- |
| Weeks 1–2 | Interviews, seven-day diaries, live feasibility spikes for Claude Code/Codex | Supported data path works across active use and a real reset; a repeated planning problem is observed |
| Weeks 3–5 | Mac private alpha: two integrations, windows, timestamps, local history and connection health | Fresh readings agree with provider displays within their precision; no secrets required in support logs |
| Weeks 6–8 | Conservative forecast, reserves, work-block planner and redacted diagnostics | Users change useful decisions; forecast beats or usefully complements a simple pacing baseline |
| Weeks 9–10 | iPhone companion, opt-in sync and widgets | Sleeping/offline Mac, delayed sync and account changes produce honest states |
| Weeks 11–12 | Paid pilot, onboarding polish, compatibility documentation and recovery testing | Retention and paid demand justify ongoing adapter support |
| After validation | Android; selective API providers or team features | Repeated demand and adequate maintenance capacity |

Limit the first public release to a Mac collector, two genuinely supported integrations, current windows, local history, clear source states, useful notifications, and one planning workflow. The iPhone companion can follow during the pilot. Defer automatic request routing, token purchasing, credential pooling, a chat client, broad organization billing, and dozens of provider adapters.

Suggested alpha targets are engineering goals, not industry benchmarks: at least 8 of 10 target users reach a correct first reading within three minutes without live help; at least 95% of eligible active-use observations arrive inside the adapter's declared freshness interval; and at least 95% of timestamp-aligned readings agree within the provider's display precision. Report sample size, exclusions, and connector version. Inactive event-driven sources should be labeled stale rather than counted as successful current observations.

Every release should pass reset, null-data, reconnect, duplicate-history, and account-isolation scenarios. Measure idle CPU/wakeups, memory and battery impact against a baseline. Verify that pairing revocation works and that diagnostics contain no secrets. Forecast evaluation must include false alarms and missed exhaustion events, not just average prediction error.

For notifications, prioritize a small number of actionable changes: insufficient likely capacity for a planned block, a verified reset when observed, an expected reset clearly labeled, a material credit change, or a broken connection requiring action. Support quiet hours and deduplicate alerts across devices.

## 10 Business model and validation

Free competitors and low-priced native utilities limit pricing power for basic meters. Test payment for dependable planning and continuity before committing to a large backend. A free local tier plus a paid planning/sync tier is a reasonable hypothesis; it is not yet a validated business model.

One price experiment is $19–29 annually versus a $39–59 desktop purchase with clearly defined update terms. If ongoing sync and connector support are included, recurring pricing better matches recurring obligations. Avoid promising lifetime cloud service without a cost model. These are proposed test prices, not a recommendation based on measured conversion.

As a simple revenue sensitivity, 1,000 annual subscribers at $24 produce $24,000 gross annual revenue; 5,000 produce $120,000. Neither is a forecast or market-size estimate. Store/payment fees, taxes, refunds, hosting, support, security, and development reduce the amount available to operate the product. A small utility can still be worthwhile, but a larger business needs distribution and repeat value beyond dashboard curiosity.

Recruit 12–15 people already paying for two or more AI services, splitting developers from chat-only users. Ask about the last actual interruption, how often they check limits, what they do when blocked, and what workaround they already use. Record a week of real decisions. Do not ask only whether they like the idea.

Then compare a plain meter with the proposed planner during real work. Measure unexpected interruptions per active work session, manual checks, useful plan changes, false alarms, reconnection burden, and week-four retention. “Interruptions avoided” is difficult to prove; use clearly defined observed outcomes or an appropriately randomized comparison rather than inventing a counterfactual savings number.

Before launch, install the closest competing releases and run the same accounts and scenarios through them. Compare onboarding, freshness, reset handling, forecast clarity, battery use, and recovery. Published claims are enough to reject novelty claims, but insufficient to claim your implementation is better.

An open-source collector or documented export format can earn trust and contributions. A polished paid application can provide planning, synchronization and support. Review the actual licenses of any reused code and dependencies. The durable asset would be a tested integration corpus, reliable operational history, and a workflow users retain—not merely a collection of provider logos.

Distribution should start where this problem is visible: coding-agent communities, practical reset/usage explainers, a transparent supported-provider matrix, and demos using clearly synthetic data. Publish measured compatibility and known limitations. Keep activity private by default; make sharing an explicit choice and avoid public token-burn leaderboards.

## 11 Final decision criteria

**Proceed** if the supported Claude/Codex paths work in live accounts, target users repeatedly encounter coordination problems, the planner improves actual decisions, and some users pay despite free alternatives. **Narrow the scope** if only one integration is dependable or a specific audience values the product. **Reconsider** if sustainable access depends on prohibited credential handling or users prefer a free meter after several weeks.

The most important unanswered questions are live coverage across provider surfaces, latency when work happens on another device, behavior while the Mac sleeps, willingness to pay, and whether planning produces a benefit beyond existing forecasts. The first development milestone should answer those questions.

A strong flagship version would be recognizable for its judgment: it knows the difference between observed activity and entitlement, handles uncertainty honestly, preserves the user's tools and privacy, and helps them plan useful work. Build the GitHub-inspired visual experience around that behavior.

## Sources

All sources accessed September 13, 2026. Undated documentation and product pages are live snapshots. App Store prices are storefront-specific; dates in version histories should be rechecked against the shipping release. Issue reports are individual observations unless a maintainer or release note confirms a fix.

[^1]: Peter Steinberger. [CodexBar](https://codexbar.app/). Product site, undated; [repository](https://github.com/steipete/CodexBar).
[^2]: AI Limits & Reset Tracker. [US App Store listing](https://apps.apple.com/us/app/ai-limits-reset-tracker/id6758946226?platform=ipad). Version 3.0.4 listed August 13, 2026.
[^3]: Anthropic. [How usage and length limits work](https://support.claude.com/en/articles/11647753-how-do-usage-and-length-limits-work). Live help documentation.
[^4]: OpenAI. [Pricing](https://learn.chatgpt.com/docs/pricing). Live official documentation, usage-limits section.
[^5]: ccusage maintainers. [ccusage repository](https://github.com/ccusage/ccusage). Current README; undated snapshot.
[^6]: Maciek-roboblog. [Claude Code Usage Monitor](https://github.com/Maciek-roboblog/Claude-Code-Usage-Monitor). Current v4 README.
[^7]: Levani Kirkitadze. [TokenBar Mac listing](https://apps.apple.com/us/app/tokenbar/id6760401627?mt=12). Version 2.0.0 listed July 13, 2026.
[^8]: Levani Kirkitadze. [TokenBar Mobile listing](https://apps.apple.com/us/app/tokenbar-mobile/id6760548040?platform=mac). Version 2.0.1 shown in current listing.
[^9]: Yuya Otake. [AI Session Meter](https://apps.apple.com/de/app/ai-session-meter/id6784566115). German storefront; version 2.4.0 listed September 1, 2026.
[^10]: Soren Starck. [SessionWatcher](https://sessionwatcher.com/). Current product and pricing page.
[^11]: jamesbrink. [Burnrate repository](https://github.com/jamesbrink/burnrate). Current README; undated snapshot.
[^12]: Peter Steinberger. [VibeMeter repository](https://github.com/steipete/VibeMeter). Archived May 3, 2026; deprecation notice.
[^13]: Helicone. [Cost tracking](https://docs.helicone.ai/guides/cookbooks/cost-tracking). Live documentation.
[^14]: Langfuse. [Token and cost tracking](https://langfuse.com/docs/observability/features/token-and-cost-tracking). Live documentation.
[^15]: LiteLLM. [Virtual keys](https://docs.litellm.ai/docs/proxy/virtual_keys). Live documentation.
[^16]: ai-usagebar contributors. [Issue 148](https://github.com/akitaonrails/ai-usagebar/issues/148). Opened September 4, 2026; closed.
[^17]: ai-usagebar maintainers. [Changelog](https://github.com/akitaonrails/ai-usagebar/blob/main/CHANGELOG.md). September 5, 6 and 11, 2026 entries.
[^18]: ccusage contributors. [Issue 988](https://github.com/ccusage/ccusage/issues/988). Opened May 12, 2026; closed with fixes referenced.
[^19]: SessionWatcher. [Release notes](https://sessionwatcher.com/releases). August 5, 25 and 30, 2026 entries.
[^20]: CodexBar contributors. [Issue 1287](https://github.com/steipete/CodexBar/issues/1287). Opened June 3, 2026; closed.
[^21]: Usage for Claude. [US App Store listing and review](https://apps.apple.com/us/app/usage-for-claude/id6755173244?platform=ipad). April 20 review and current release history.
[^22]: Claude Code contributors. [Issue 80750](https://github.com/anthropics/claude-code/issues/80750). Opened July 24, 2026; reported extra-usage discrepancy, unverified cause.
[^23]: Anthropic. [Customize your status line](https://code.claude.com/docs/en/statusline). Live documentation; quota fields require v2.1.251+.
[^24]: Anthropic. [Monitoring usage](https://code.claude.com/docs/en/monitoring-usage). Live OpenTelemetry documentation.
[^25]: OpenAI. [Codex App Server](https://learn.chatgpt.com/docs/app-server). Live official documentation; account endpoints and nullable fields.
[^26]: OpenAI. [Organization usage](https://developers.openai.com/api/reference/python/resources/admin/subresources/organization/subresources/usage). Live API reference.
[^27]: Anthropic. [Usage and Cost API](https://platform.claude.com/docs/en/manage-claude/usage-cost-api). Live Admin API documentation.
[^28]: Google. [Understand and count tokens](https://ai.google.dev/gemini-api/docs/tokens). Live Gemini API documentation.
[^29]: Google. [Rate limits](https://ai.google.dev/gemini-api/docs/rate-limits). Updated September 2, 2026.
[^30]: OpenRouter. [Credit limits and rate limits](https://openrouter.ai/docs/api_reference/limits). Live API documentation.
[^31]: Anthropic. [Legal and compliance](https://code.claude.com/docs/en/legal-and-compliance). Live authentication and credential guidance.
[^32]: Apple. [Configuring the macOS App Sandbox](https://developer.apple.com/documentation/xcode/configuring-the-macos-app-sandbox) and [accessing files](https://developer.apple.com/documentation/security/accessing-files-from-the-macos-app-sandbox). Live developer documentation.
[^33]: Apple. [Developer ID](https://developer.apple.com/developer-id/). Signing and notarization guidance.
[^34]: Apple. [Security of runtime process](https://support.apple.com/guide/security/security-of-runtime-process-sec15bfe098e/web). Platform Security documentation.
[^35]: Apple. [Safari extensions](https://developer.apple.com/safari/extensions/) and [website access permissions](https://developer.apple.com/documentation/safariservices/adjusting-website-access-permissions). Live developer documentation.
[^36]: Apple. [Keeping a widget up to date](https://developer.apple.com/documentation/widgetkit/keeping-a-widget-up-to-date/). WidgetKit refresh guidance.
[^37]: Apple. [Choosing background strategies](https://developer.apple.com/documentation/BackgroundTasks/choosing-background-strategies-for-your-app). Live developer documentation.
[^38]: Android. [Application sandbox](https://source.android.com/docs/security/app-sandbox). Platform security documentation.
[^39]: Android. [Define work requests](https://developer.android.com/develop/background-work/background-tasks/persistent/getting-started/define-work). WorkManager scheduling guidance.
[^40]: Apple. [Keychain services](https://developer.apple.com/documentation/Security/keychain-services). Live developer documentation.
