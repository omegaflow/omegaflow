<!--
  title: Handover — autonom: vier Atome, eine Kollision (Stand 2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: c045e3cad6dbe0579c6171bfcfaea3f73b7a71604a41990cc73d8acf4896d462
  status: archived
  see-also: docs/handover/handover-2026-09-10-autonom.md, docs/handover/handover-2026-09-10-supermag-koernung.md
-->
# Handover — autonom: vier Atome, eine Kollision (2026-09-10)

Vier Atome autonom gearbeitet; die Abschluss-Session traf auf einen von
Parallel-Sessionen geteilten Baum. Was trägt, trägt Git; was kollidiert, ist
hier als Ownership-Karte benannt.

## Geleistet (trägt Git)

- `174e4dd` — number_audit: R2 an die gemessene Archiv-Wahrheit verdrahtet
  (explizite `--archive-root`/`--backup-archive`, Exact-Equality-Prüfung,
  injizierte Test-Fixture); `cargo check -p omegaflow-register` sauber (2,90 s,
  null Warnungen).
- `c783df2` — supermag: der gemessene 599-Station-magstid-Zensus
  (`phi/supermag_stations.φ`, 608 Zeilen) als Registrierungs-Urteil versioniert.
- Der jup365-Index-Schnitt (Q3) ist im Arbeitsbaum ausgeführt
  (`phi/sources_index.φ:143527`), trägt aber kein Git: die Datei ist über
  `.gitignore:44` (`phi/*`) ignoriert und war nie getrackt (`git ls-files`
  error, keine Historie). Die Korrektur bleibt abgeleiteter Zustand; die
  durable Registrierung reist mit der NAIF-501–504-Ernte (sources.φ-Zeile).

## Die Kollision (ownership map)

- **SuperMAG-Körnung:** zwei Räte, zwei Körnungs-Verdikte — die Parallel-Session
  (`handover-2026-09-10-supermag-koernung.md`) baute merged-window mit
  64-Byte-Wire (`--all`, `supermag_2025-03.bin`); diese Session baute per-Station
  (599 gemessen, Wire unverändert, `phi/supermag_stations.φ`). Abschluss-Rat:
  per-Station schließt das Atom; der merged-Entwurf bleibt uncommittet als
  Ableitungs-Atom (Netzwerk-Fenster-Snapshot-Bin aus den 599 Bins) — sein Rekord
  lebt in dessen Handover; dessen `supermag-cdn.yml`-Edit bleibt unangetastet.
  Gemessen: der Baum trägt `tools/harvest/src/bin/supermag_compiler.rs` im
  merged-Entwurf (`merge_parts`, `--all`, kein `emit-stations`/`out-dir`); der
  Compiler wurde deshalb nicht committet.
- **geo.rs:** drei Sessionen teilen die Datei; die ANR1-Hunks dieser Session
  (Atom 4) wurden durch parallele Arbeit aus dem Baum entfernt —
  `tools/harvest/src/bin/antares_loci_compiler.rs` (untracked, 9864 B) wartet auf
  die Wiederanlage der ANR-Unterstützung (nächstes Atom, wenn der Baum ruhig ist);
  der Compiler ist gebaut und verifiziert (Proof: 200 Loci, 598 Records,
  Roundtrip), nur sein geo-Support fehlt.
- **handover-2026-09-10-autonom.md bleibt live** — die Parallel-Sessionen
  konsumieren die Kette noch (archiv-Umzug erst, wenn die konsumierenden
  Sessionen schließen).

## Register (Verdikte wörtlich)

- **Q2: R2-Anker bestätigt (gemessen im Code):** Anker-Token der Zeile ∧ Nomen
  file/files ∧ Ziffer nicht im Anker-Token. Ohne das Nomen-Gate 4 Fehlalarme
  gemessen (Pioneer-11-Zeile). Das Nomen-Set bleibt eng (nur file/files —
  „archives" öffnet den Pioneer-Fehlalarm wieder); die Enge ist Kalibrierung,
  keine Lücke.
- **Q3: sources_index.φ:143527 korrigiert:** jup365.bsp trug die .cmt-Größe 69632
  (Kopierfehler aus Zeile 143528); korrigiert auf 1136581632 / 1615735762 (HEAD
  in der Sitzung nachgemessen, Last-Modified 2021-03-14 15:29:22). Die
  sources.φ-Zeile (url/format/at) fehlt weiterhin — sie reist mit der NAIF
  501–504-Ernte samt Asset (CDN-Duty), nicht isoliert vorab. Der Index ist
  `.gitignore`-abgeleitet und nicht getrackt; die Korrektur trägt kein Git.
- **Q4: CI-Läufe gemessen:** galileo-odr 34529232620: success. SuperMAG-TRO
  34530905521: conclusion failure — Fehlgrund gemessen:
  `upload supermag_tro_2025-03.bin: gh returned void: release not found`
  (Exit-Code 1). Die Vollernte (599 Stations-Bins) braucht einen eigenen
  Station-Schleifen-Workflow — nächstes Atom, benannt.

## Offen (benannt, nie 0)

- Vollernte-Workflow (Station-Schleife über die 599) — nächstes Atom.
- ANR-Wiederanlage in `geo.rs` + `antares_loci_compiler`-Commit — nächstes Atom,
  wenn der Baum ruhig ist.
- `pack_iaga` für 163 Ziffer-Codes (Wire-Format) — eigenes Atom.
- NAIF 501–504: Ernte + `sources.φ`-Zeile + CDN-Duty reist mit dem Asset.
- ANTARES-Fläche: `meta.count=10000`-Kappe, Magnituden ohne Passband, kein
  serverseitiger Zeitfilter — gemessen, für den Folgebau.
