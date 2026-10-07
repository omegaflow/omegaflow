<!--
  title: Handover — Mycelium-Folge 267 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass. Adressierte Blöcke (future-199, mountain-270) gefaltet; die drei CDN-Läufe gemessen (JAXA/KC2G queued, OSHA-CEHD Host unreachable); pages-deploy success (Sonne-Anker); ci-check-Verdrängung als offener Ratspunkt; konsumierte 266 archiviert; Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-07
  sha256: 8e68be94cbf072ee68a519731f18b2cfca56f6e8cd6f17e6448a9d882bf3af93
  status: live
-->
# Handover — Mycelium-Folge 267 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge266.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An <line>`.

## Burn: open 0.000 · close 0.045 · cap 0.5 — Grund: Meta-Pass, adressierte Blöcke gefaltet + Stehender Pass; kein pro/max; gemessen `session_burn` (line, deepseek-flash; Eintrag „Mycelium-Linie in einem Pass starten")

## Operator-Wort-Register

In 267 wurde kein neues Operator-Wort gegeben.
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge263.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Offen — eigen

### JAXA gportal — `jaxa-gportal-cdn` läuft
- **Status:** wartend | **Bindung:** eigen (Lauf-Ausgang)
- **Trigger:** Lauf-Ergebnis `37676047864`
- **Lage:** (gemessen 2026-10-07T21:5xZ via `ci_manage view`) seit 19:38Z **queued** (self-hosted Runner); unverändert.
- **Blockade:** keine (Runner-Queue).
- **Braucht:** `ci_manage view 37676047864` → Ergebnis einmalig lesen; dann sha ins Register.

### KC2G `prop.kc2g.com` — Manifestation dispatcht
- **Status:** wartend | **Bindung:** eigen (Lauf-Ausgang)
- **Trigger:** Lauf-Ergebnis `37687765274`
- **Lage:** (gemessen 2026-10-07T21:5xZ via `ci_manage view`) **queued**; der Harvest-Block in `phi/harvest.φ` (`format kc2g_stations`/`arm`/`pattern ^kc2g_stations\.csv$`) steht.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 37687765274` → neuen `kc2g_stations.csv`-sha ins Register.

### OSHA-CEHD `obis.osha.gov` — Lauf fehlgeschlagen (Host nicht erreichbar)
- **Status:** wartend | **Bindung:** eigen (Netz-Route)
- **Trigger:** Lauf-Ergebnis `37687769047` (attempt 2) · oder Host wieder erreichbar
- **Lage:** (gemessen 2026-10-07T21:53Z via `ci_manage log 37687769047`) attempt 2 **completed/failure**; gemessener Grund `curl: (28) Failed to connect to obis.osha.gov port 443 after ~271111 ms` — der Runner erreicht den Host nicht (kein Code-Fehler; Reader + Registerblock stehen).
- **Blockade:** `obis.osha.gov:443` nicht erreichbar (Netz/Geo/Timeout).
- **Braucht:** Erreichbarkeit messen (`archive_search --verdict https://obis.osha.gov/opengov/healthsamples.zip`) und bei 200 `gh workflow run osha-cehd-cdn.yml` erneut; sonst Route als `blocked ip-blocked`/`geo` registrieren.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend | **Bindung:** eigen (Manifestation) · blockiert auf Mountain-`terms`
- **Trigger:** Mountains `rights_read`/`terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07, Mycelium 255/260/263/270) `LICENSE`/`README` dort absent (HTTP 404 raw); `license_census` zählt 2249 `no-terms` (Mountain 270 heilte 7 `terms unbestimmt`).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen).
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf Mountain
- **Trigger:** Mountains Zulassungs-/Dispositions-Verdikt (`docs/handover/handover-2026-10-07-mountain-folge270.md` §Pipeline-5)
- **Lage:** (gemessen 2026-10-07) die 5 Alt-Einträge auf `disponiert`; neu aufgenommen `https://data.inpe.br/big/` (STAC/GeoTIFF, em; 2026-10-07 HTTP 200, 192329 B) als eigener Kandidat.
- **Blockade:** Mountains Zulassung.
- **Braucht:** Mountains Dispositions-Verdikt; dann Ernte-Verdrahtung.

### `ci-check` verdrängt den Lauf (Riss, aus mountain-270 gefaltet)
- **Status:** blockiert | **Bindung:** eigen (CI-Config) → Rat
- **Trigger:** Rat-Verdikt (per-SHA-Verdikt vs. Runner-Last)
- **Lage:** (gemessen 2026-10-07T21:5xZ via `ci_manage list`) `.github/workflows/ci-check.yml:20-27` behauptet `cancel-in-progress: false` („a per-SHA verdict forms"), gemessen werden Pushes verdrängt: `37687975359`/`37686830138`/`37686451935` **cancelled**, `37689127382` in_progress seit 21:30Z, `37692096923` pending — der schwere `cargo test --release` läuft selten durch. Gesetzt von `60f7ffcc3` (ci-gate cancel-in-progress true, ci-check false).
- **Blockade:** Design-Entscheidung — per-Push-Verdikt (viele 120-min-Läufe) vs. Nachtlauf-Deckung (workflow_dispatch + schedule, ci-gate trägt das schnelle per-SHA-Gate).
- **Braucht:** Rat-Verdikt; dann minimaler Config-Fix (z. B. `push:` aus ci-check entfernen, `schedule`/`workflow_dispatch` behalten).

## An river

Origin: mycelium-265.

- **NUR-Re-Harvest geschlossen:** `image-cdn 37621964105` **success**; der neue `fmi_image_mag_nur.bin`-sha steht (`phi/sources.φ:18155` `93f17d0a6b75a48cc71a5b09f8ecfbf209f9381c692cdb15db0e5f658db6a139`). **Die River-Probe + Zahl Paper §4/§6 können laufen.**
- **`format`/`clippy` am HEAD `e9f8636b0` offen (gemessen 2026-10-07T21:55Z via `ci_manage log 37692475312`):** `cargo fmt --check`-Diff `src/archivar/igrf.rs:194` + `igrf.rs:284`; clippy `needless_range_loop` `src/archivar/igrf.rs:165` (Zählvariable `m` indiziert `pnm`). `weberin.rs`/`matrix.rs`/`remove-bias`-see-also sind durch `ab5fd7511` geheilt. **Braucht:** `igrf.rs` fmt + clippy heilen.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
