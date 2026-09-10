<!--
  title: Handover — Pending-Register aufgeräumt: 67 Pflichten im Blatt, Source-Port-Queue gemessen, Bucket-Litmus verdiktet
  class: handover
  date: 2026-09-10
  sha256: 017787033d04e9cdc94d198fbc20316eaebd71a7a801ab9e569851379940c1d8
  status: live
  see-also: docs/blatt/blatt-offene-pendings.md docs/handover/handover-2026-09-09-mechanische-reste.md
-->
# Handover — Pending-Register aufgeräumt (2026-09-10)

Diese Sitzung trug die offenen Pflichten der sechs lebenden Handover zusammen,
maß die mechanischen Source-Port-Reste und hielt die Rats-Entscheidung fest.
Die verbrauchten Handover (disjunkte-linien-folge, operator-label-korrektur)
sind archiviert; ihr offener Bestand lebt im stehenden Register
`mechanische-reste.md` und im Blatt `blatt-offene-pendings.md`.

## Gebaut/geschlossen (trägt Git)

- **Blatt der offenen Pendings** (`docs/blatt/blatt-offene-pendings.md`): 67
  offene Pflichten aus sechs Handovern, je Zeile Quelle + Zustand
  (`pending`/`läuft`/`Operator-Wort`), Doppelnennungen zählen einmal.
- **SOURCE_PORT.md §5 korrigiert:** die Keys sind GitHub-gespiegelt (Parität
  §10, `gh secret list` gemessen), nicht „nur auf der Maschine"; lokal sind
  nur die ~120-MB-Korpora. Die Discovery-Ernte bleibt lokal wegen Korpora +
  Review, nicht wegen der Keys.
- **Korpora-Heim-Frage eröffnet:** Korpora in ein privates Repo → Discovery-
  Front CI-fähig; unentschieden (Drittanbieter-Redistribution + Quota gegen
  CI-Fähigkeit). Register-Frage, kein Operator-Wort.
- **Bucket-Litmus verdiktet** (Rat, einstimmig, 2026-09-10): Drei-Fragen-Kette
  (Oszillator-Gate → Form → Manifestation) + `pending`-Überlauf; C1-Doktrin
  bindend. Die Regel liegt als note im `ledger.φ`; die Disposition der 101
  Datasets ist der benannte Folgeschritt.
- **Source-Port-Queue gemessen:** 10 Untested-Korpora → **0 Survivors**; 38
  VizieR-Bulks recheckt (12 `verifiziert`); 77 Archeology-Gaps waren
  **bereits disponiert** (voids/disponiert/blocked in den Registern bestätigt,
  34 `geparkt` im `ledger.φ`). `sources.φ` unverändert (0 neue Survivors).

## Offen (an die nächste Sitzung)

- **Bucket-Litmus-Anwendung** — die 101 NOAA-NODD-Datasets nach der verdikteten
  Regel disponieren (url-line / Compiler-Lease / Konsument / `pending`).
- **Vier Review-Fragen** — cddis finals2000A (blocked vs EOP-Duplikat),
  supermag, ESO, maia-finals2000A-Zweit-Datei.
- **Die echten Arbeits-Postes** (unverändert, kein Recheck — Code/Analyse):
  abfluss-trishuli, R2, Step-5-Byte-Vergleich, Gaia-XP-Brücke,
  Parquet/GRIB-2/OPeNDAP-Reader, docs-reference-verteilung, bande-split,
  gic-p-wert, Flut-Satellit, Membran M02–M07, OPeNDAP, Korpora-Heim.
- **Blockierte** (kein Sitzungs-Atom): papier-kleinpass (Merge), de441
  Re-Verifikation (grüner bodies-Job), matrixmachine-Urkunde (grüner CI-Lauf).

## Verbraucht/archiviert

- `handover-2026-09-09-operator-label-korrektur.md` → archiv (geklappt).
- `handover-2026-09-09-disjunkte-linien-folge.md` → archiv (geklappt).
- Lebend bleiben: `mechanische-reste.md` (stehend), die TE-Linie
  (`te-blatter-bz-laic-tscaling`), die Tiefenphasen-Linie
  (`zonen-flotte-sp-dual`), die de441-Linie (`de441-reverifikation-pending`).
