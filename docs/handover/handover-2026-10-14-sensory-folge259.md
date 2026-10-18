<!--
  title: Handover — Sensory 259 (2026-10-18)
  class: handover
  date: 2026-10-14
  sha256: c7a8c0bb9f6a28b28d8e1db17aea6a2c7b0c35e183b484b13af79f55693e7652
  status: live
-->
# Sensory-259

## Burn: open 0.0 · close 0.04 · cap 0.15 · closed 0 · Grund: .13 SearXNG.

## Offen — eigen
- **ox64-m2c** · wartend · eigen — **Trigger:** Zustellung. **Lage:** (gemessen 2026-10-18, DHL) `LZ473049629CN`, Zoll durch; DHL-Suche **404**, kein Status. **Blockade:** Zustellung. **Braucht:** Zustell-Beleg; `dhl.de …?piececode=LZ473049629CN` (live, JS).
- **europa-clipper** · termin · eigen — **Trigger:** Flyby 2026-12-03. **Lage:** (gemessen 2026-10-18, CDN) `ephemeris_europa_clipper.bin` 200/103120 B, sha256 `3b03ea69…`. **Blockade:** Termin. **Braucht:** danach `kernel-flatten.yml` + `sniff`.

## Verweis — offene Arbeit aus abgeschlossenen Surveys
Diese Punkte stehen nicht als `## Offen` (das Gate: „do not collect"); sie sind an Adressen gebunden:
- **de2-pairing** (eigen) — WATS/IDM-Parser gebaut (`de2_raw.rs`, 4 Tests), TE auf 16-s-HAPI gemessen; **Braucht:** die 2-s/250-ms-Paarung fahren.
- **blatt-211-193** (eigen) — Blatt steht `pending`; Agent fährt die konditionale Prüfung; **Braucht:** konditionales TE-Ergebnis.
- **nssdc-antworten** (extern) — fünf NSSDC-Anfragen 2026-09-16, keine Antwort; **Braucht:** Antwort/Follow-up.
- **dead-sources-4-pending** (eigen, Riss) — „4 pending" ohne Items; **Braucht:** die vier Kandidaten itemisieren.
- **galileo-odf** (Datengap) — `--all`: kein offener Host; **Braucht:** Host/Anfrage.
- **tianwen-nssdc** (Datengap) — `--all`: keine Daten (nur MoonDB); **Braucht:** Daten/Anfrage.
- **teilchenexperiment** (operator) — Kuprat vs. Detektor/Dark-Matter; **Braucht:** Operator-Wort.
