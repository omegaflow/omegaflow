<!--
  title: Handover — Mountain-Folge 212 (Stand 2026-09-30)
  session: Mountain-Folge 212
  class: handover
  date: 2026-09-30
  sha256: b5a86af5f6ad9bbaf00bdf46e2e93aa787b0233d9d805f30bf014778b986dfb0
  status: live
-->
# Handover — Mountain-Folge 212 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Dieses Atom ist die
**Ausführung der folge211-Tafel bis zur Kante**: DAS2-Extract-Arm gebaut (`src/archivar/extract.rs`
`series_parse_bin` + `hapi_csv` + Test, `cargo check` 0/0), vier stale Verdiktzeilen in
`phi/blocked_sources.φ` auf den echten Baumstand geheilt, die adressierten Blöcke
(mycelium-folge211, river-folge71) gefaltet, die bereits erledigten Punkte **gelöscht**.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„jeder Punkt trägt eine Empfehlung; wartende Linien erhalten eine bevorzugte Abarbeitungsbitte" | 2026-09-29 | Operator (Mountain 204)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„braucht es wirklich pro?" — pro nur mit benanntem Hart-Atom oder gemessener flash-Fehllage | 2026-09-30 | Operator (Session, Mountain 211)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„arbeite deine Liste bis zur Kante ab" — jeder eigene Punkt bis zur Kante, nichts Machbares liegen lassen | 2026-09-30 | Operator (Session, Mountain 212)

## Haus — Mountain (Stand 2026-09-30)

Diese Übergabe **ist** das Haus: jeder offene Punkt, jedes Verdikt, jeder Parser steht hier
mit Zustand, auch um 3 Uhr nachts.

- **Die vier Orte:** `omegaflow` = `~/projects/omegaflow` + privates Schwester-Repo `state/`
  (`omegaflow/personal`); `omegaflow-legacy` = `archive-root/omegaflow-legacy`; `temp` =
  `/tmp/opencode`; `archive` = `archive-root`.
- **Mountain-Fundstellen:** `phi/`, `src/archivar`, `src/mathematikerin`, `src/gate`,
  `tools/harvest`, `tools/measure`, `tools/register`, `docs/specs`, `docs/surveys`,
  `state/zustand`, `state/mountain`.
- **Linien-Preset (privat):** `state/mountain/archive-search-preset.txt`; `state/` immer mit
  `archive_search --root state`.

## Offen (aufgeschlüsselt)

### Prosa-Träger (eigene) — verbleibende Marker
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-30) in diesem Atom geheilt/geklärt: `livefeed-gate.md` Pfad-Riss
  (Work-Verzeichnis → Gate-Verzeichnis, sha256 nachgetragen); `survey-raetsel-bestand.md` Ⅹ-Riss
  (descoped mit Befund, `kybernetische-astrophysik.md:299-300`) + Kuprat-Zeugenklasse (kein
  `witness kuprat`, die vier Kanäle sind `witness substance`, `witnesses.φ:120-142`);
  `fremde-parser-sammlungen.md:87` ist selbst-erklärt „nicht meßpflichtig" (descoped);
  `arxiv-api.md` + `dead-sources-relevanz.md` trugen keinen offenen Punkt mehr.
- **Blockade:** keine.
- **Braucht:** die zwei verbleibenden echten Marker:
  `survey-raetsel-bestand.md` Offen Ⅳ (`laic_champ.bin` Paper-Größe), Ⅴ (`negativ_fuzzy_probe`
  Dip/FAP) und der ENSO-Doc-Angleich an `sources.φ`;
  `survey-2026-09-03-daten-holdings-inventur.md:56` (drei Ephemeriden-Placeholder, außerhalb
  des getrackten Baums) / `:139` (Byte-Messung je Holding).

## An mycelium

Origin: mountain folge212.

- **Ernte + Manifestation (Mountain-Arm/Compiler stehen, Asset fehlt):**
  Kaguya LRS (`pds3_binary_compiler`), Chandrayaan Mini-RF (`pds3_img_compiler`),
  Chang'e-1/-2 MRM `data/ce{1,2}_mrm.fits` (`pds4_fits_compiler`), Akatsuki VCO-rs
  (`pds4_binary_compiler`), DAS2 Iowa (`das2_iowa_compiler`). Mountain hat die Verdikt-Feder
  (`ttl`/`frame`) gesetzt; `url`/`format`/`compiler`/Tag = Mycelium.
  **Gemessene Einzeldatei-Endpunkte (2026-09-30, verifiziert):**
  - Kaguya LRS (Rohdateien `.tbl`, **nicht** `.dat`):
    `https://data.darts.isas.jaxa.jp/pub/pds3/sln-l-lrs-2-sndr-waveform-high-v1.0/20080202/data/LRS_SW_WF_50N_286300E.tbl`
    (+`.lbl`) — HTTP 200, 8 131 335 B.
  - Chandrayaan-1 Mini-RF: `https://pds-geosciences.wustl.edu/lunar/ch1-orb-l-mrffr-1-pdr-v1/ch1mrf_0xxx/data/sar/00700_00799/level1/fsb_00720_1cd_xhu_84n209_v1.img`
    (+`.lbl`) — HTTP 206; PDR-Roh `.dat` unter `…/raw/`.
- **USGS-comcat:** Compiler committet; Workflow von dir angelegt — Lauf + `url`/`format`.
- **kernel-flatten `spk_split`:** Fix committet (`spk_split.rs`); Lauf dispatchten und lesen.
- **tao-wnd / pds4-binary:** Fenster/Idempotenz geheilt — Re-Dispatch + Lauf lesen.
- **CDSE-CCM STAC-Auth:** `stac_asset_fetch --asset <href>` mit `CDSE_TOKEN` (200 = nutzbar);
  danach Mountain `ttl`/`frame`.

## An future (Operator-Queue, private)

Origin: mountain folge212.

**Bitte bevorzugt vorlegen, sobald der Operator spricht:**
- **ODF-Flyby-Fenster fehlt** — DSN/JPL-Rohdaten-Anfrage stellen?
- **Sonden-Download-Session** — Operator-Browser-Session für die fünf `released`-Konten?
- **opencode-Config Secrets** (aus folge210) — Env-Export + Rotation.

## Burn: open 0.0033 · close 0.0816 — session_burn (Mountain-Session; zwei Taucher general $0.0446 separat)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
