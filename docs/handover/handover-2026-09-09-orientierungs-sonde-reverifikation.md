<!--
  title: Handover — Orientierungs-Sonde: Rekompilat + Re-Verifikation geschlossen (vier Linien Δ 0,0 km), de441 mars bleibt
  class: handover
  date: 2026-09-09
  sha256: 36a20409cbb8dc252917f5541363eeb68265471b286a57e6b9839de66ed22e6d
  status: live
  see-also: docs/handover/handover-thematisch-membran-sonde.md docs/befund/befund-2026-09-09-orientierung-stale-matrizen.md docs/handover/archiv/handover-2026-09-09-matrix-fix-nachtrag.md
-->

# Handover — die Orientierungs-Sonde ist gebaut, der Rekompilat re-verifiziert

Übergabe für die Folge-Session. Das Atom der Membran-Orientierungs-Linie ist
geschlossen bis auf einen Stein: die Sonde ist gebaut und gemessen, vier der
fünf stale Bins sind rekompiliert und re-verifiziert, de441 mars bleibt.

## 1. Vollzug (diese Sitzung)

- **Sonde gebaut** — `tools/measure/src/bin/orientation_probe.rs` (headless,
  Muster eclipse_shadow_probe): Sub-Solar-Punkt über zwei Pfade (Matrix-Pfad
  gegen den analytischen IAU-Zweig), DSS43-Elevation gegen die unabhängige
  Lehrbuch-Formel (Schwelle 1,0°). Zwei Tests grün, check 0/0.
- **Gemessen** — de440/de442/inpop/epm earth + de441 mars trugen Vor-Fix-
  Matrizen (~120–138°-Klasse); nur de441 earth trug den Fix. Widersprach dem
  §4 der konsumierten Handover (`handover-2026-09-09-matrix-fix-nachtrag.md`).
- **Rekompilat ausgelöst** — de44-cdn (run 34363451986) + inpop-epm-cdn
  (run 34363458584), beide grün.
- **Re-Verifikation** — orientation_probe Δ 0,0 km auf de440/de442/inpop/epm
  earth; galileo sanity: DSS43-Peak ~04:00 (Matrix-Pfad im Einklang mit dem
  Lehrbuch, vorher ~117°-Shift); galileo full reproduziert den Befund exakt
  (Haupttabellen unverändert: st43 med_diff 1,828/20/5/9,688 · st63
  18,827/9/4/35,304).

## 2. Offener Stein (Folge-Session)

- **de441 mars trägt weiter Vor-Fix-Matrizen** (Δ Anker 6 045,3 km) — der
  Rekompilat hängt am kernel-flatten-Körperjob + am ungemessenen
  18-MB-vs-183-MB-Struktur-Unterschied. Steht im thematischen Handover
  `handover-thematisch-membran-sonde.md`.

## 3. Register-Lage

Die Parallel-Sitzung hat während des Rekompilats das Register aufgelöst
(bd4d123: TODO gestorben, das Handover ist das Register). Die offene mars-Zeile
trägt das thematische Handover; der Befund
`befund-2026-09-09-orientierung-stale-matrizen.md` trägt die Nachmessung. Diese
Übergabe ersetzt die frühere (archivierte) Sitzungs-Übergabe, die nur den
Sonden-Bau deckte.
