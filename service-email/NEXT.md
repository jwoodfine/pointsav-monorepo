# NEXT.md — service-email

> Last updated: 2026-05-29 (session 7)
> Read at session start. Update before session end so the next
> session knows where to pick up.

---

## Right now

- **Enable Exchange polling.** `local-email.service` is running (port 9204,
  MCP server only). To activate the EWS daemon, create an override file:
  ```
  sudo mkdir -p /etc/systemd/system/local-email.service.d/
  sudo tee /etc/systemd/system/local-email.service.d/exchange.conf <<'EOF'
  [Service]
  Environment="AZURE_ACCESS_TOKEN=<az account get-access-token --resource https://outlook.office365.com | jq -r .accessToken>"
  Environment="EXCHANGE_TARGET_USER=<mailbox@domain.com>"
  EOF
  sudo systemctl daemon-reload && sudo systemctl restart local-email
  ```
  Token expires; update + restart to rotate. Logs: `journalctl -u local-email -f`.

## Queue

- Add `service-email` to `conventions/software-units.yaml` (workspace scope)
  so `bin/deploy-binary.sh` can manage future binary updates. Binary currently
  deployed manually; ledger entry at `data/binary-ledger/service-email.jsonl`.
  **Command Session task.**
- ~~Add `service-email` as a workspace member in the monorepo root `Cargo.toml`~~
  **Done 2026-05-29 (session 7)** — `[workspace]` table removed from
  `service-email/Cargo.toml`; `Cargo.lock` absorbed into root; `service-email`
  added to root `[members]`; workspace check passes clean.

## Blocked

- Live Exchange end-to-end test — Blocked on: needs `AZURE_ACCESS_TOKEN` +
  `EXCHANGE_TARGET_USER`. MCP path is testable now; EWS daemon path needs creds.

## Deferred

- IMAP path through `service-email-egress-imap/` — Deferred:
  operator's 2026-04-25 instruction is EWS specifically. IMAP
  remains as a sibling adapter, not consumed from this crate
  unless a customer use case surfaces it.
- Outbound message sending — Deferred: this crate is the
  Communications Ledger (ingest path). Outbound message synthesis,
  if it ever lands, is a separate `service-email-template`
  concern downstream of the ledger.

## Recently done

- 2026-05-27 (session 5): **`sovereign-splinter/` renamed to `email-splitter/`.**
  "sovereign" is a Do-Not-Use prefix per nomenclature-taxonomy.md. Directory renamed;
  `scripts/spool-daemon.sh` comments updated to remove the Do-Not-Use term.
  `CLAUDE.md` file layout updated.

- 2026-05-21 (session 4): **`maildir.rs` removed.** `MaildirVault` was replaced by
  `FsClient` (2026-05-20); dead code removed. `mod maildir;` line removed from `main.rs`.
  23/23 tests pass.

- 2026-05-21 (session 3): **`local-email.service` deployed** on `127.0.0.1:9204`.
  MCP server confirmed healthy (`/healthz`, `initialize`, `tools/list`). EWS daemon
  disabled on startup (no Exchange credentials yet). Binary sha256 recorded in
  `data/binary-ledger/service-email.jsonl`.

- 2026-05-20 (session 2): **MCP server + service-fs wiring complete.**
  `src/fs_client.rs`, `src/mcp.rs`, `src/http.rs` added. `email.ingest` MCP tool.
  `X-Foundry-Module-ID` enforcement. 23 tests total.

- 2026-04-26: **EWS auth rebase complete.**
  `src/auth.rs` — replaced inline OAuth2 `client_credentials` flow
  with `EwsCredentials::from_env()` reading `AZURE_ACCESS_TOKEN` +
  `EXCHANGE_TARGET_USER` + optional `EWS_ENDPOINT` from env.
  `src/graph_client.rs` → renamed to `src/ews_client.rs` via
  `git mv`; fully rewritten as `EwsClient` implementing three EWS
  SOAP operations (FindItem, GetItem with IncludeMimeContent, UpdateItem
  IsRead=true). `src/main.rs` — daemon loop rewritten using
  `EwsCredentials` + `EwsClient`; reads `TOTEBOX_ARCHIVE_PATH`;
  includes 50ms anti-throttle pause between per-message EWS calls.
  `Cargo.toml` — removed `serde`/`serde_json`; changed reqwest to
  `rustls-tls` (avoids openssl-sys blocker); added `base64 = "0.22"`;
  added `[workspace]` table (standalone crate isolation). Six unit
  tests cover XML parsing helpers and base64 round-trip; all pass
  clean.

- 2026-04-26: **pre-framework subdirectory inventory complete.**
  Four subdirectories + one root template assessed; decisions:
  | Item | Decision |
  |---|---|
  | `ingress-harvester/` | **Retire-pending** — Rust async; EWS SOAP harvester but with inline OAuth `client_credentials` (deprecated pattern); hardcoded folder IDs. Retire once EWS rebase lands. |
  | `master-harvester-rs/` | **Retire-pending** — Rust async; Graph API (deprecated); dynamic folder discovery + micro-batching (BATCH_SIZE=3) concepts worth porting to rebased daemon. |
  | `sovereign-splinter/` | **Keep core; rename** — Rust binary; mailparse-based `.eml` parser; routing logic (maildir → service-people/discovery-queue + service-slm/transient-queues + assets/inert-media) superseded by MCP append calls. "sovereign" prefix is Do-Not-Use → queue rename to `email-splitter`. |
  | `scripts/` | **Correctly placed** — `spool-daemon.sh` already in `scripts/` per repo-layout.md. Calls sovereign-splitter binary; update path reference when renamed. |
  | `TEMPLATE_INDEX_MSFT_ENTRA_ID.md` | **Relocated** — Moved from repo root to `docs/` (repo-layout.md compliance). Done this session. |

- 2026-04-25: project activated per `~/Foundry/CLAUDE.md` §9 —
  this CLAUDE.md, this NEXT.md, and the registry row created in
  one commit; existing `src/auth.rs` (Graph OAuth) flagged as
  drift in CLAUDE.md "Current state" with the EWS rebase queued
  as Right-now per operator decision.
