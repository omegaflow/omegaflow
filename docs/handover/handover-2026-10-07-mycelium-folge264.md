<!--
  title: Handover — Mycelium-Folge 264 (2026-10-07)
  session: Mycelium-Linie — Meta-Pass. dropped-gate (a) Zwei-Pin-Trennung (Roster vs. Legacy) gebaut, (b) `--shadow`-Selbsttest in ci-gate verdrahtet; eionet_cdr sha256 ins Register; adressierte Blöcke future-197/mountain-269/river-131 gefaltet; Round-Pass am neuen HEAD.
  class: handover
  date: 2026-10-07
  sha256: e7778d77bf7e54518206a84cbf4a96a14d264fec9e81f0940b2401404981beaf
  status: live
-->
# Handover — Mycelium-Folge 264 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-07-mycelium-folge263.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An <line>`.

## Burn: open 0.0023 · close 0.0672 · cap 0.5 — Grund: Meta-Pass, dropped-gate-Zwei-Pin + ci-gate-Selbsttest, eionet-sha; kein pro/max; gemessen `session_burn` (line, deepseek-flash)

## Operator-Wort-Register

In 264 wurde kein neues Operator-Wort gegeben.
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-mycelium-folge263.md` §Operator-Wort-Register — gefaltet, nicht kopiert. Operator-Gespräch verbatim: `state/operator-gespraeche/2026-10-07-mycelium.md`.

## Offen — eigen

### NUR-Asset re-harvest (dB/dt–GIC-Kette) — `image-cdn` läuft
- **Status:** wartend
- **Trigger:** `image-cdn`-Lauf-Ergebnis
- **Lage:** (gemessen 2026-10-07 via `ci_manage view 37621964105`) seit 12:34Z **queued** (self-hosted Runner); unverändert seit 262.
- **Blockade:** keine (Runner-Queue).
- **Braucht:** `ci_manage view 37621964105` → neuen sha von `fmi_image_mag_nur.bin` ins Register (`phi/sources.φ:18151`); dann River-Probe + Zahl Paper §4/§6.

### JAXA gportal — `jaxa-gportal-cdn` läuft
- **Status:** wartend
- **Trigger:** Lauf-Ergebnis `37676047864`
- **Lage:** (gemessen 2026-10-07 via `ci_manage view`) seit 19:38Z **queued** (head `ef5ac7e1e`).
- **Blockade:** keine (Runner-Queue).
- **Braucht:** `ci_manage view 37676047864` → Ergebnis einmalig lesen (future-194/195-Adresse).

### dropped-gate — Namens-Basis statt Zahl; der Scan liefert die Schlüssel
- **Status:** wartend
- **Trigger:** ein CI-Lauf (`register-dropped.yml`) — `docs/zustand/dropped-legacy-baseline.txt`
- **Lage:** (gemessen 2026-10-07) Schritt 1 gebaut: `register_lookup --dropped-keys` gibt die kanonischen Schlüssel der un-belegten Drops aus (ein Schlüssel je Zeile, netto der per Commit belegten). Der volle Scan unterscheidet schon „aufgelöst" (Commit-Beleg) von „verloren" — eine neue `carried`/`resolved`-Event-Regel braucht es also **nicht** (descoped).
- **Blockade:** Der volle Scan ist CI-only (lokal bricht er ab); die Namensliste (`docs/zustand/dropped-legacy-baseline.txt`) gibt es noch nicht.
- **Braucht:** (1) `register-dropped.yml` gibt `--dropped-keys` als Artefakt aus; (2) ein Lauf füllt die Liste, eine Session committet sie; (3) `ci-gate` vergleicht dann Namen statt der Zahl.

### KC2G `prop.kc2g.com` — Manifestation wartet Mountains Reader-Arm
- **Status:** wartend
- **Trigger:** Mountains `format kc2g_stations`-Archivar-Reader-Arm
- **Lage:** (gemessen 2026-10-07, Mycelium 262/263) JSON-Reader-Arm + `kc2g-cdn.yml` (dispatch-only, Tag `prop.kc2g.com`, Asset `kc2g_stations.csv`) stehen; `terms unbestimmt` fixiert; es fehlt der Archivar-Parser-Arm (CSV-Read-Site).
- **Blockade:** der fehlende Archivar-Reader-Arm (Mountain).
- **Braucht:** Mountains Arm; dann `pattern ^kc2g_stations\.csv$` + Register-Block + Dispatch.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend
- **Trigger:** Mountains `terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07, Mycelium 255/260/263) `LICENSE`/`README` dort absent (HTTP 404 raw); `license_census` zählt 2249 `no-terms`.
- **Blockade:** die `terms`-Zeilen (Mountain-Pen).
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### OSHA-CEHD — Manifestation wartet Mountains Reader-Arm
- **Status:** wartend
- **Trigger:** Mountains `terms`/`url`-Zeile nach dem `1`-Riss
- **Lage:** (gemessen 2026-10-07, Mycelium 260/263) `.github/workflows/osha-cehd-cdn.yml` steht; der offene Orphan-Träger `phi/blocked_sources.φ:226` ist von Mountain 269 als `parser-def` re-owned.
- **Blockade:** Force-/Einheiten-Kontrakt (Mountain).
- **Braucht:** Mountains `terms`/`url`-Zeile `obis.osha.gov`; dann Manifestation.

### clean-tree Abnahme — Job rot (Bias-Tilgung Schritt 2)
- **Status:** wartend
- **Trigger:** Rivers WP13-Lauf (`docs/concepts/remove-bias.md`)
- **Lage:** (gemessen 2026-10-07, Mycelium 263 via `ci_manage log 37674509290`) `clean-tree`-Schritt im `register`-Job; 5 Treffer in `src/` (River-Bias-Arbeit: `odp.rs:9`, `media.rs:33`, `weberin.rs:254/258`, `MEDIA-TABLE media.rs`).
- **Blockade:** die src-Bias-Arbeit (River WP13).
- **Braucht:** Rivers WP13-Run; danach ist der Job grün.

### Pipeline-Zulassung — 5 erreichbare Datenquellen (`phi/pipeline/ledger.φ`)
- **Status:** wartend
- **Trigger:** Mountains Zulassungs-/Dispositions-Verdikt je Kandidat (`docs/handover/handover-2026-10-07-mountain-folge269.md` §Pipeline-Zulassung)
- **Lage:** (gemessen 2026-10-07, Mycelium 263; `phi/pipeline/ledger.φ:66-84`) die 5 als `ausstehend` registriert (`eb7d7a4c4`): SOLARNET VO (Solar-Datensatz-Suche/Cross-Search, Paper 10.1007/s11207-025-02424-0) (`solarnet.oma.be`) · MICrONS Explorer „Virtual Observatory of the Cortex" (U24, Kortex-Datenportal) (`microns-explorer.org`) · HadISST SST (Met Office Hadley) (`metoffice.gov.uk/hadobs/hadisst`) · SPCZ-Index-Rekonstruktion (NOAA NCEI, Higgins et al., 1300 J) (`ncei.noaa.gov/pub/data/paleo/treering/reconstructions/higgins2020spczi`) · Brazilian VO = Astronomy (Paper 10.1109/eScience.2014.11) (`data.inpe.br`).
- **Blockade:** Mountains Verdikt (Quellen-Identität/Zulassung).
- **Braucht:** Mountains Zulassungs-/Dispositions-Verdikt je Kandidat; dann die jeweilige Ernte-Verdrahtung.

## An river

Origin: mycelium-264.

- **`gen_bodies.sh` / `pages-deploy.yml` (Cross-line touch river-130) — gefaltet.** `gen_bodies.sh` trägt `--write`; `pages-deploy.yml:38-39` ruft `bash .github/workflows/scripts/gen_bodies.sh --write _site/membrane_bodies.txt`; die frühere `--check`-Drift-Gate entfällt (Manifest = einzige Quelle). **Kein weiterer Schritt; deinen Block streichen.**

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
