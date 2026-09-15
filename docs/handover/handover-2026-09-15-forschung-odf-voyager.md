<!--
  title: Handover — Forschung: ODF-Ernte + Voyager Saturn (Stand 2026-09-15)
  session: Forschung (ODF-Ernte + Voyager Saturn)
  class: handover
  date: 2026-09-15
  sha256: f3dd5395712042b7e8a114a6b4e79ff42ef4801e31d214ff7a23989197526006
  status: live
  see-also: docs/auftrag/auftrag-sonden-rohdaten-anfragen.md, docs/paper/twenty-second-band-ground-chain.md
-->
# Handover — Forschung: ODF-Ernte + Voyager Saturn (2026-09-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

## Voyager Saturn (UNIVAC-1108) — gelesen, die Uhr offen

- **Der Saturn-Encounter ist dekodiert** (V1 `PSPA-00049`, V2 `PSPA-00123`):
  `src/archivar/voyager_saturn.rs` (2-B-Record-Header BE, 28 Sub-Records × 288 B,
  36-Bit-Wörter, 6 Tests grün) + `tools/harvest/src/bin/voyager_saturn_compiler.rs`
  (std-only TAR-Extraktion, V1+V2). Sample `DD059517_F1.DAT`: 1578 Records →
  44 169 Tracking-Records (Doppler 29 527 / Range 6 900 / Sync 7 742).
  **Offen: die Uhr.** w3/w8 sind zeitartig (monoton, wrap), Epoche + Einheit sind
  aus dem Sample nicht entscheidbar; **Station (DSS) nicht identifiziert**.
  (Schritt: NSSDC-Format-Doc `NSSD1260` oder die Voyager-Saturn-RSS-SIS laden und
  w3/w8 gegen einen bekannten Zeitstempel pinnen, die DSS-Bits in w2/w4/w17 suchen.)
- Register/CDN: `voyager_saturn.bin` ist noch keine `url`-Zeile in `phi/sources.φ`
  und nicht auf dem CDN. (Schritt: nach dem Uhr-Decode registrieren.)

## Sonden-ODF-Ernte — sieben Compiler gebaut, zwei offen

- Gebaut (TRK-2-34, `parse_odf` wiederverwendet, Muster `juno_odf_compiler`):
  Magellan (grün: 14 665 009 Samples, 1 055 880 656 B), MGS, MRO, Mars Odyssey,
  MESSENGER, Mars Express (MaRS), Rosetta (RSI) —
  `tools/harvest/src/bin/{magellan,mgs,mro,odyssey,messenger,mars_express,rosetta}_odf_compiler.rs`.
  **Register-Eintrag nicht committet**: `phi/blocked_sources.φ` ist fremd-staged
  (Migration) — die neun Blöcke (je `pending`, URL + SCID + data_type + gemessene
  Record-Zahl) sind aus den Compiler-Konstanten neu erzeugbar. (Schritt: bei
  ruhigem Baum die Blöcke in `phi/blocked_sources.φ` + `phi/sources.φ` schreiben.)
- **Dawn** — kein anonymer ODF-Pfad: PPI `/data/` 404, PDS-Geosciences `/dawn/`
  404, SBN `pds4/dawn/` trägt GRaND/gravity/mission ohne RSS. (Schritt:
  PDS-Katalog nach dem Dawn-RSS-ODF-Bundle durchsuchen.)
- **Venus Express (VeRa)** — PSA `VEX-V-VRA-1-2-3-*` trägt unter
  `DATA/LEVEL1A/CLOSED_LOOP/` nur `IFMS/`, kein DSN/ODF; `parse_odf` findet keine
  36-B-Orbit-Records. (Schritt: PSA-Katalog `VEX-V-VRA` auf eine DSN/ODF-Route prüfen.)

## Bande-Split — Papier gelandet, Reste offen

- Papier `docs/paper/twenty-second-band-ground-chain.md` **v5** (Council-Landing:
  Bandgrenze 44–58-mHz-Raster, struck drift, der station-lokale Atmosphären-Zweig
  benannt, Split-Serie + Zensus im §4) und der Sibling
  `docs/paper/probe-front-dark-matter.md` **v8** (Deduktion 27 auf den gemessenen
  Split reframed, struck drift).
- Offen: die **1988-Wertdivergenz** (Split-Peaks rx14 46,58 / rx43 44,12 /
  rx63 50,92 mHz vs. kanonischer Zensus 57,11 / 44,40 / 51,99 mHz) — Ursache
  offen (Datenversion oder Methode); der **160-Hz-Amplituden-Zensus** (`pending`);
  die **NOCC-Reduktionsmaschine** unbenannt. (Schritt: Amplitude-Zensus auf der
  kanonischen Serie fahren; die NOCC-Reduktionsdoku suchen.)

## Extern gebunden (kein Datum)

- NSE/Haug — Antwort von B. Keimer offen.
- Voyager Cruise / JPL-DSN — die Anfrage hält (request-only).
- Fünf Sonden-Anfragen (Voyager closed-loop, Mariner 10, Viking 1/2, Cassini
  closed-loop, Juno Earth-Flyby) — Operator reicht ein
  (`docs/auftrag/auftrag-sonden-rohdaten-anfragen.md`).
- Toth/Turyshev/Markwardt-Mails — der Bande-Split (Split + Registerzeilen +
  Papier) ist geschlossen, die Prüfliste steht; die Mails sind entblockt.
  (Schritt: senden — in der Entscheid-Linie geführt.)

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
