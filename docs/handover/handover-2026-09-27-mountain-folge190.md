<!--
  title: Handover — Mountain-Folge 190 (Stand 2026-09-27)
  session: Mountain-Folge 190
  class: handover
  date: 2026-09-27
  sha256: 14de04e576a45ca4cbe40e1aac02578d713be1b68f7c5feddaf60833c92e66ce
  status: live
-->
# Handover — Mountain-Folge 190 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert
(`state/zustand/standing-pass.md`); gemessen wird nur, was der eigene Trigger
für fällig erklärt.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja aus dem register aufstellen" — die offene Liste streng aus dem Register, nicht aus der Vorgänger-Tafel | 2026-09-27 | Operator (Session, Mountain 187)
„erst messen" — pySPEDAS und die Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Session, Mountain 187)
„Listen zur Entscheidung dienen nicht — ich brauche Erklärungen" | 2026-09-27 | Operator (Session, Mountain 187)
„das war dein Vorgänger der wohl Mist gebaut hat" | 2026-09-27 | Operator (Session, Mountain 187)
„Du kannst. Führe den Plan aus — als line-Agent" | 2026-09-27 | Operator (Session, Mountain 187)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom. Dispatch flash-first. Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-27 | Operator (Mountain-Session 190)

## Offen (aufgeschlüsselt)

### CI-check — archivar-Runde verifizieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`-Lauf auf HEAD `60dfedd` beendet (`36346668721`, bei
  Messung pending); er führt die Arm-Runde `b4106e69a` + den Reader-Test
  `esacci_sst_geo_series_roundtrip_and_component_name`.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`) der `b4106e69a`-eigene Lauf
  `36346224407` wurde gecancelt (0 Jobs, Push-Überholung); der jüngste
  einschließende Lauf ist `36346668721` (HEAD `60dfedd`, pending).
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36346668721` einmal lesen, sobald beendet; grün →
  schließen; rot → Testname + erste Fehlerzeile heilen.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte liegt in `state/zustand/wartend.φ:3`)
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-27 via
  `sread tools/harvest/src/bin/ned_byparams_compiler.rs`)
  `X-NED-Timeout-Token` nur am Form-POST (`http_post_form`, Z. 116-123);
  `poll_ticket`/`fetch_body` tragen keinen Header; `state/mail/mail_ledger.φ`
  ohne NED-Eintrag (jüngster Eintrag `1790533094` = IGETS, Z. 172).
- **Blockade:** Token fehlt.
- **Braucht:** mit dem Token den echten ByParams-Job fahren; scheitern Poll/Fetch,
  Token an `http_get`/`fetch_body` ergänzen (erst messen, dann ergänzen).

### ttl-Verdikt-Zeile (Rat 2026-09-27) + no-cadence-Sprachloch
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der fremde `AGENTS.md`-Hunk ist committet.
- **Lage:** (gemessen 2026-09-27 via `git status`/`git diff`) `AGENTS.md` trägt
  weiterhin einen fremden uncommitteten Hunk (`M AGENTS.md`) — die Rat-Regeln
  „the pen is the owner's" / „one writer per handover" (2026-09-27). Rat: `ttl` in
  `phi/sources.φ` ist eine Verdikt-Zeile → Mountain; `derive_ttl`
  (`src/archivar/port.rs:1237`) liefert für Binär-SPK `None` — die Datei-Zeile
  trägt; alle 81 `format ephemeris_binary`-Blöcke tragen `ttl 86400` (HEAD). Der
  Flush-Gate `src/archivar/parse.rs:80` (`ttl 0` = inaktiv) trägt keinen eigenen
  „no-cadence"-Zustand.
- **Blockade:** fremder uncommitteter Hunk in `AGENTS.md` (ein pfad-eigener Commit
  würde fremde Arbeit sweepen).
- **Braucht:** nach dem fremden Commit „ttl = Verdikt-Zeile" in `AGENTS.md`
  nachtragen; das no-cadence-Sprachloch als `pending` registrieren.

### orphan-odf-serie — nativer Serien-Arm (odf.rs)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner — dispatchbarer eigener Schritt.
- **Lage:** (gemessen 2026-09-17 via `docs/surveys/survey-2026-09-17-sonden-request-only.md`) `src/archivar/odf.rs:1639` dekodiert alle 18 TRK-2-34-Codes; „offen bleibt der native Serien-Arm"; `src/archivar/channels.rs:623` nennt den 4D-Serien-Arm für netcdf4-Volumes als pending.
- **Blockade:** keine.
- **Braucht:** den nativen Serien-Arm in `odf.rs`/`channels.rs` bauen oder als `pending` präzisieren. (aus `docs/surveys/survey-2026-09-17-sonden-request-only.md`)

### orphan-where-source — DSCOVR-Isolat (Query-Arm)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner — dispatchbarer eigener Schritt.
- **Lage:** (gemessen 2026-09-20 via `docs/paper/flyby-path-2-falsification-metric-addendum.md:82-88`) die keyless DSCOVR-Route ist retired (404); L1 lebt im RTSW-Feed (`source IMAP`/`ACE`); ein `where source DSCOVR`-Isolat ist `pending` Parser-Prüfung, „not required by the seal"; `where source` existiert im Reader nicht (`sgrep src` 0 Treffer).
- **Blockade:** kein Query-Arm.
- **Braucht:** `where source` im Reader bauen ODER als `descoped` (nicht vom Siegel gefordert) messen. (aus `docs/auftrag/auftrag-flyby2-kette.md`)

### orphan-driver-sources — Slab2 + 3D-Tomografie
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner — dispatchbarer eigener Schritt.
- **Lage:** (gemessen 2026-09-27 via `sread docs/concepts/positive-maske.md` §Treiber-Kandidaten) GLO-30-DEM registriert (CDN-Asset 2026-09-14, HTTP 200); USGS Slab2 und 3D-Geschwindigkeitsmodelle „nicht registriert, Ernte-Kandidat".
- **Blockade:** keine.
- **Braucht:** die Quellen messen und in `phi/sources.φ` registrieren oder als `absent` benennen. (aus `docs/concepts/positive-maske.md`)

## Träger (Orphan-Faltung, Operator-Wort „falte alle")

- `docs/concepts/recherche-galileo-kadenz-reconciliation.md` — aufgelöst (Verdikt 2026-09-25, Nachtrag 2026-09-27): die Zählzeit ist ein benutzerspezifizierter Track-Parameter; kein offener Punkt. Residual: das DSMS-D-19002-Original ist öffentlich absent (NTRS), der Wortlaut läuft über die LRR-Zitierung — kein Schritt.
- `docs/concepts/positive-maske.md` — M9.1-Picker ist verdrahtet (`depth_phase_fleet_probe.rs` `--mww`/`anchor_register`); offen nur die Treiber-Quellen (Block oben).

## An fremde Feder (Absender-Zeile — Aufenthalt = Eigentum)

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| ESACCI-Reader-Arm | `docs/handover/handover-2026-09-27-mycelium-folge188.md:131` | mountain folge190 | gebaut (`b4106e69a`, `60dfedd`) und funktional verifiziert: `tools/measure/src/bin/esacci_sst_read.rs` liest den CDN-Bin (1008128 B, sha256 `8b1502c113bc51c83f085ac8d4202233f596f4511d289364317b4e653fe0c386`, 16802 Records, `val 271.19–304.61 K`, mean 286.64, 0 NaN) — Mycelium schließt die Anforderung |
| DAS2 Iowa + Occultation-DB UTFPR residual `sources.φ`-Zeile | Mycelium-Übergabe | mountain folge189 (`## An Mycelium`) | beide Arme gebaut (`b4106e69a`); die residuale `url`/`origin`/`compiler`/Tags-Zeile ist Myceliums Feder |

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
