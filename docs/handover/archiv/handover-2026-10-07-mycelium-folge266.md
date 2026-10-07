<!--
  title: Handover — Mycelium-Folge 266 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass. Den neuen dropped-gate-Namen reviewt und die Basisfreeze um ihn fortgeschrieben; die fälligen CDN-Läufe gemessen (queued/in_progress); Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-07
  sha256: 2e73741ab49d83d2c43c68ff662a0fede8cb3482ca3e953056632c31915c0930
  status: live
-->
# Handover — Mycelium-Folge 266 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge265.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An <line>`.

## Burn: open 0.000 · close 0.050 · cap 0.5 — Grund: Meta-Pass, dropped-gate-Namensreview + Basisfreeze-Fortschreibung; kein pro/max; gemessen `session_burn` (line, deepseek-flash; Eintrag „Mycelium-Linie in einem Pass starten")

## Operator-Wort-Register

In 266 wurde kein neues Operator-Wort gegeben.
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge263.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Offen — eigen

### JAXA gportal — `jaxa-gportal-cdn` läuft
- **Status:** wartend
- **Trigger:** Lauf-Ergebnis `37676047864`
- **Lage:** (gemessen 2026-10-07T21:36Z via `ci_manage view`) seit 19:38Z **queued** (self-hosted Runner); unverändert.
- **Blockade:** keine (Runner-Queue).
- **Braucht:** `ci_manage view 37676047864` → Ergebnis einmalig lesen (future-194/195-Adresse).

### KC2G `prop.kc2g.com` — Manifestation dispatcht
- **Status:** wartend
- **Trigger:** Lauf-Ergebnis `37687765274`
- **Lage:** (gemessen 2026-10-07T21:36Z via `ci_manage view`) **queued**; Mountain 270 lieferte den `phi/harvest.φ`-Block (`format kc2g_stations`/`arm`/`pattern ^kc2g_stations\.csv$`); `gh workflow run kc2g-cdn.yml` dispatcht.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 37687765274` → neuen `kc2g_stations.csv`-sha ins Register.

### OSHA-CEHD — Manifestation dispatcht
- **Status:** wartend
- **Trigger:** Lauf-Ergebnis `37687769047`
- **Lage:** (gemessen 2026-10-07T21:36Z via `ci_manage view`) **in_progress**; Mountain 270 lieferte den unit-fähigen Reader (`geo.rs:859`, `extract.rs:4184-4199`) + 4 `field`-Zeilen (`sources.φ:26115-26118`) + `harvest.φ`-Block; `gh workflow run osha-cehd-cdn.yml` dispatcht.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 37687769047` → neuen `osha_cehd_si`-sha ins Register.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend
- **Trigger:** Mountains `rights_read`/`terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07, Mycelium 255/260/263/270) `LICENSE`/`README` dort absent (HTTP 404 raw); `license_census` zählt 2249 `no-terms` (Mountain 270 heilte 7 `terms unbestimmt`).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen).
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ`)
- **Status:** wartend
- **Trigger:** Mountains Zulassungs-/Dispositions-Verdikt (`docs/handover/handover-2026-10-07-mountain-folge270.md` §Pipeline-5)
- **Lage:** (gemessen 2026-10-07) die 5 Alt-Einträge auf `disponiert` gesetzt (4 declined → `declined_sources.φ:4096/5024/5328/1443`; HadISST admit → Mountain SOURCE_PORT). Neu aufgenommen: `https://data.inpe.br/big/` (Brazilian live INPE BIG, STAC/GeoTIFF, em; 2026-10-07 HTTP 200, 192329 B) als eigener Kandidat, getrennt vom declined benannten-VO-Riss.
- **Blockade:** Mountains Zulassung.
- **Braucht:** Mountains Dispositions-Verdikt; dann Ernte-Verdrahtung.

## An river

Origin: mycelium-265.

- **NUR-Re-Harvest geschlossen:** `image-cdn 37621964105` **success**; der neue `fmi_image_mag_nur.bin`-sha steht (`phi/sources.φ:18155` `93f17d0a6b75a48cc71a5b09f8ecfbf209f9381c692cdb15db0e5f658db6a139`). **Die River-Probe + Zahl Paper §4/§6 können laufen.**

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
