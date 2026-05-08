# folgore-repro

Minimal code to attemp to reproduce [coffee-tools/folgore#100](https://github.com/coffee-tools/folgore/issues/100) — folgore's esplora backend panics with `ParseIntError { kind: InvalidDigit }` when `GET /blocks/tip/height` returns a non-numeric body with HTTP 200.

Uses the same [`esplora-api`](https://github.com/dev-crew-host/btc-utils) crate and the same call chain as folgore (pre-fix):

```rust
client
    .raw_call("/blocks/tip/height")
    .map_err(|err| format!("{err}"))
    .map(|raw| raw_to_num(&raw))   // panics here on non-numeric body
```

Non-200 responses (429, 503, etc.) are caught by `esplora-api`'s status check and never reach `raw_to_num`. The panic only triggers on a **200 OK with non-numeric content** — e.g. an HTML error page served by a CDN or reverse proxy that preserves the 200 status.

## Usage

```bash
# Default: 800 requests against blockstream.info/api
cargo run

# Custom URL and request count
cargo run -- "https://blockstream.info/api" 800
cargo run -- "https://blockstream.info/liquid/api" 800
cargo run -- "https://mempool.space/api" 800

# Hit rate limit threshold (700 req/hr on blockstream.info)
cargo run -- "https://blockstream.info/api" 750
```

If the bug is triggered, the process panics with:
```
thread 'main' panicked at 'impossible parse a string into a i64: ParseIntError { kind: InvalidDigit }'
```

If all requests succeed or return non-200 errors, it prints:
```
Done. No panic encountered.
```

## Context

- **Issue**: https://github.com/coffee-tools/folgore/issues/100
- **Fix**: https://github.com/coffee-tools/folgore/pull/102
- First seen around Sep 23, 2025 in Core Lightning nodes using folgore as a bitcoin backend
- The fix converts `raw_to_num` to return `Result` instead of panicking, so the recovery strategy can retry
- **Update**: So far are unable to reproduce this issue
