<!--
  title: Handover — River-Folge 85 (2026-10-03)
  session: River-Folge 85
  class: handover
  date: 2026-10-03
  sha256: 1a30a946fbae9a1fe5693bfae8c5daa8c3b32eaa9fc9ade06bf5c3c95a395efd
  status: live
-->
# Handover — River-Folge 85 (2026-10-03)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die River-Linie in einem Pass …" | 2026-10-03 | Operator (Session, River 85)
„Erste Handlung: `sread docs/concepts/tool-forms.md` …" | 2026-10-03 | Operator (Session, River 85)

## Träger (Prosa, eigene)

- `docs/paper/flyby-path-2-addendum-2026-09-29.md` (`class: paper`) — Path-2-Fill-Run
  am JUICE-Perigäum (2026-09-28, Fenster 22:41:53Z→23:01:08Z); swarm cell 25 (39625 nT)
  gefüllt (Bin-Fix `river 73`). Offen (trigger-/fristgebunden): OMNI2 26 Zellen, ACE
  3/14/16, kp `def`, Δ/σ_recon — siehe Offen.
- `docs/auftrag/auftrag-flyby2-kette.md` (`class: auftrag`) — die Path-2-Kette vor dem
  JUICE-Perigäum; dieselben Rest-Zellen wie das Addendum. Trägerzeile des Auftrags.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) —
  GIC/Bz-Vorhersagezelle; die α-Ebene wartet auf die kalibrierte Westfall–Young-max-T-Null
  (`docs/specs/broken-null-control.md`). Trägerzeile (write schließt sich mit dem Lauf).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`class: survey`) — die
  Membran-Ladearchitektur; alle Sachpunkte §7 geschlossen. Trägerzeile; der `--arxiv`/
  `--brave`-Werkzeug-Vermerk ist 2026-10-03 geheilt (`--arxiv` liefert Treffer,
  `--brave` antwortet ohne 402; Header-sha `ea724611…`).
- `docs/concepts/exzellenz-konzept.md` (`class: concept`, `version: 1`) — Prüfmaßstab vor
  Veröffentlichung. Trägerzeile; die Anwendung liegt in der Gate-Survey.
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` (`class: survey`) — Anwendung des
  Maßstabs; **Offen: keiner aus diesem Gate** (River, Sensory, Mountain alle geheilt).

## Offen (aufgeschlüsselt)

### fruehwarnsystem α-Ebene — wartet auf die wy-max-t-Null
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `wy-max-t 37114779681` (Trigger = Lauf-Ende; beim Pass
  gemessen 2026-10-03: `queued`).
- **Lage:** (gemessen 2026-10-03 via `ci_manage view 37114779681`) queued; der
  Vorgänger `wy-max-t 36867148250` war `cancelled` (2026-10-01T21:42Z), die Null war
  nicht gelandet. Das Blatt steht `unsealed`.
- **Blockade:** der Lauf ist nicht gelandet (queued).
- **Braucht:** nach dem Lauf-Ende `ci_manage log 37114779681` lesen; dann α/X/Z/Bz
  benennen (Dokument-Arbeit). Das Siegel setzt der Operator (kein Siegel ohne Wort).

### nvss-CDN — RA-chunked Lauf an der Kante
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `nvss-cdn 37116107265` (gemessen 2026-10-03: `pending`).
- **Lage:** (gemessen 2026-10-03 via `ci_manage view 37116107265` + `archive_search
  --verdict`) der Workflow ist RA-chunked geheilt (8 Slices `seq 0 45 315`, je
  `--async 9000 --limit 300000`; `.github/workflows/nvss-cdn.yml`); derzeit trägt
  `…/releases/download/ssd.jpl.nasa.gov/nvss.json` HTTP 206 (1 B — der alte Stand),
  der neue Lauf noch pending.
- **Blockade:** der Lauf läuft noch.
- **Braucht:** `ci_manage log 37116107265` nach Lauf-Ende lesen; ist der Chunk-Lauf
  success und das Asset neu, ist der Punkt (folge83) geschlossen.

### Flyby-Path-2 Rest-Zellen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** OMNI2-Lag (~6 d nach dem Perigäum, HAPI 1201), kp `def` (finale
  GFZ-Freigabe), ESA/ESOC-Publikation des post-flyby SPK + Kovarianz.
- **Lage:** (gemessen 2026-10-03, aus `handover-2026-10-03-river-folge84.md`) swarm
  cell 25 gefüllt; OMNI2 26 Zellen, ACE 3/14/16, Δ/σ_recon offen.
- **Blockade:** externe Freigabe/Lag.
- **Braucht:** sobald ein Trigger feuert, `flyby_ephemeris_gate --recon` bzw. die
  Zellen wie im Addendum füllen.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md`
- `docs/handover/archiv/handover-2026-10-03-river-folge84.md` (Move aus `docs/handover/`)
- `docs/handover/handover-2026-10-03-river-folge85.md`

## Burn: open 0.0153 · close 0.1156 · cap 0.50 — Grund: River-85 — Membran-Werkzeug-Vermerk geheilt (arxiv/brave), adressierte Blöcke gefaltet, exzellenz-Gate geschlossen (gemessen `session_burn`; Gesamt von 5 Linien-Sessions geteilt, Parallel-Linien teilen den Total)
