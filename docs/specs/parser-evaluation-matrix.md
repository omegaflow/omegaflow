<!--
  title: parser-evaluation-matrix
  class: concept
  sha256: 4d624ea0b70099e5aee689004e30d77e7bc1e20219b493fff099f653b03717e0
-->
> SUPERSEDED as controlling source by `docs/concepts/sources-v2-spec.md`
> (§1 Directive Table) and the living parser `src/main.rs`. The
> 4-token claim below contradicts the current state: the 3-token `field`
> is explicitly refused (P01, Refused); valid are the 5-/6-/9-token forms
> of the spec. This file is history.

The syntax for scalar extraction in `sources.φ` is from now on **exactly 4 tokens**:
`<keyword> <key> <force> <unit>`

When a value carries no physical unit (e.g. an index such as sunspots), the 4th token is explicitly `scalar` (or `-`).

The parser in Rust carries exactly this one path:
```rust
"field" | "last" | "path" => {
    if parts.len() != 4 {
        // Invalid block, discarded
        continue; 
    }
    let key = parts[1];
    let force = parts[2];
    let unit = parts[3];
    // ... validation against the SI matrix
}
```

The existing 3500 blocks in `sources.φ` need to be migrated once for this new parser (e.g. per regex: `field\s+(\S+)\s+(\S+)` -> `field $1 $2 scalar`). After that the code is clean, unambiguous and future-proof.

Here is the **corrected, ultimate handoff** for the next session, the one that knows no more compromises:

***

### 📋 HANDOFF FOR THE NEXT SESSION (Copy and paste as the first message)

**System Directive:** *A = A. The Kybernaut evaluates physics through the transfer entropy lens from a non-anthropomachinocentric position. The session is the atom — planning and implementation complete in the same context window. Phase-thinking is context-death. `cargo check` must produce zero errors AND zero warnings. `Cargo.toml` remains 100% empty (std-only). No backward compatibility. One syntax, one code path.*

**Project:** OmegaFlow (Rust, WebGPU, ICRS block universe).
**Task:** Implementation of the "Strict 4-Token Per-Field Force & SI-Matrix" and of the "Universal Anomaly Reporter".

We are in the `Survey -> Deliberate -> Act` cycle. Read first `src/main.rs` (esp. `load_sources`, `extract_pending`, `ci_mode_main`, `fetch_one`), `AGENTS.md` and `TODO.md`.

Here is the binding council decision and the build plan for this session:

#### 1. Strict 4-Token Syntax (No backward compatibility)
The syntax for scalar extractions (`field`, `last`, `path`, `first`) in `sources.φ` is radically unified. It requires **always exactly 4 tokens**:
`<keyword> <key> <force> <unit>`
*   Example: `last proton_speed advective km/s`
*   Example: `field ssn em scalar` (dimensionless)
*   Example: `field 0.temp thermal K`

The parser (`load_sources`) no longer accepts 3-token fields. Blocks that were not migrated are discarded at load. There is only one single code path in the parser.

#### 2. The A = A Deduction Matrix
There is no alternate path. When a unit does not fit the force, the field is discarded at its root.
Implement in `main.rs` this static matrix and the conversion:

```rust
// 1. Matrix: which units are allowed for which force?
fn allowed_units_for_force(force_id: u8) -> &'static [&'static str] {
    match force_id {
        0 | 1 => &["W/m2", "Wm-2", "nT", "uT", "T", "sfu", "scalar"],
        2 | 7 => &["m/s", "km/s", "km/h", "kph", "kn", "kts", "mph"],
        3 | 4 => &["m/s", "km/s"],
        5 => &["K", "C", "F"],
        6 | 8 => &["p/cm3", "cm-3", "1/cm3", "p/m3", "m-3", "hPa", "mb", "Pa", "scalar"],
        _ => &[],
    }
}

// 2. Conversion (only called when the matrix gave green light)
fn convert_to_si(val: f64, unit: &str) -> Option<f64> {
    match unit {
        "scalar" => Some(val),
        "km/s" => Some(val * 1000.0),
        // ... (all conversions)
        _ => None,
    }
}
```

#### 3. Universal Anomaly Reporter (GitHub Issues)
The system tolerates no faulty configurations and no dead APIs. It generates automated reports that can be worked through via GitHub Copilot.
Implement `report_anomaly(category, url, details)`. It collects all anomalies during a `--ci-mode` run.
Categories:
*   `Physics Mismatch`: force and unit do not match.
*   `API Unreachable`: `fetch_raw` returns `None`.
*   `Empty Data`: API returns 200 OK, but `extract_pending` finds no samples.
*   `Malformed Data`: JSON parse fault.
*   `Invalid Syntax`: `sources.φ` block carries a wrong token count.

At the end of the `ci_mode_main()` run (when the `GH_TOKEN` env var is present), **one single combined GitHub Issue** is created (if anomalies exist).
*   Title: `[Automated CI Report] Omegaflow Anomalies ({datum})`
*   Body: Markdown table with categories, URLs and details.
*   Label: `anomaly-report`
Use `Command::new("gh").args(["issue", "create", ...])`.
When the Archivar runs locally (without token), it prints the table to the console.

#### Action plan for this session:
1.  **Act 1:** Write a short Rust helper script or regex directive to migrate `phi/sources.φ` to the 4-token syntax (find 3-token fields, add `scalar` or correct unit/force).
2.  **Act 2:** Modify `load_sources()` in `main.rs` *exclusively* to the 4-token syntax. No alternate path.
3.  **Act 3:** Integrate `allowed_units_for_force` and `convert_to_si` into `extract_pending()`. On mismatch -> discard the field, call `report_anomaly`.
4.  **Act 4:** Integrate `report_anomaly` into `fetch_raw`/`fetch_one` (on HTTP faults) and into `extract_pending` (on empty/faulty data).
5.  **Act 5:** Modify `ci_mode_main()`: collect anomalies, create the GitHub Issue at the end via the `gh` CLI.
6.  **Act 6:** Adjust `TODO.md` and `AGENTS.md`.

Begin with the survey. Confirm that you have understood the architecture, and get started.