<!--
  title: Handover — autonom: was die Kybernautin selbstständig ausführt (Stand 2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: 9ff277cedf5abbf7830680e217a9cf18461093eab747df9e985d0ba4c0a38a90
  status: live
  see-also: docs/handover/handover-2026-09-10-nicht-autonom.md
-->
# Handover — autonom (2026-09-10)

Was die Kybernautin selbstständig ausführt — kein Operator-Wort, kein CI-Warten,
kein Kalender, kein Rat. Jede Zeile ist eine Sitzungs-Arbeit; erledigt = trägt
Git. Die nicht-autonomen Pflichten stehen im Gegen-Handover.

## Register & Disposition

- **Bucket-Litmus-Anwendung** — erledigt 2026-09-10: 99 Dispositionen in
  `phi/pipeline/catalog/noaa_nodd_disposition.φ` (19 Compiler-Lease, 1 pending,
  79 descoped). Maschinen-Zwilling `bucket_litmus` + `decline_lens.φ` gebaut
  (Kalibrier-Gate FP 3 / FN 12); Karte `MANIFEST.φ` geschrieben. Siehe
  `handover-2026-09-10-bucket-litmus.md` + `handover-2026-09-10-phi-struktur.md`.
- **Voller Lauf über den phi-Bestand** — erledigt 2026-09-10: Linse →
  `probe_sweep` → Review auf der kartierten Struktur (`MANIFEST.φ`); 30
  Kandidaten → 19 live → 3 Survivor + 4 declined, alle 7 bereits in `sources.φ`
  mit korrekter Einheit registriert (0 neue Quellen); die Probe-Einheit-
  Autoableitung bleibt offen. Siehe `handover-2026-09-10-voller-phi-lauf.md`.
- **bucket_litmus auf weitere Inventare** — Copernicus-Inventar u.a. liegen im
  Katalog; der Vorentscheid läuft mit `bucket_litmus
  phi/pipeline/decline_lens.φ <inventar.φ> [--calibrate <disposition.φ>]` (aus
  `phi-struktur` übernommen).
- **Vier Review-Fragen** — gemessen 2026-09-10 (Grind-Pro), aufgelöst 2026-09-10:
  cddis finals2000A → `decline duplicate-eop` (EOP-Duplikat der maia finals.all);
  supermag → gebaut (`supermag_compiler.rs` + `supermag_1m`-Block);
  maia-Zweit-Datei hält (genau eine maia-Datei, keine zweite). Siehe
  `handover-2026-09-10-supermag.md`.
- **ESO-TAP** — offen (kein Eintrag) — Ernte + Duty (eigenes Atom).
- **Step-5-Folge** — gemessen 2026-09-10 (Grind-Flash): 23 Assets gleich
  (Schnitt-Liste steht), 4 nicht schneiden (3 verschoben/Stub + Bowserinator
  einzige Kopie); der destruktive Schnitt bleibt ein verifizierter Folgeschritt.
- **R2** — die Archiv-Zählung (archive-root + das lokale Backup-Archiv)
  als Grundwahrheit in `number_audit.rs` verdrahten.
- **docs-reference-verteilung** — die vermessene Bewegung ausführen + Referenz-
  Rewiring; Seeds → Survey-Heimat (Benennung dabei).

## Code (Bauen)

- **Struktur-Reader** — Parquet, GRIB-2, OPeNDAP (FITS + netCDF-4 + CDF-1/2
  gebaut).
- **OPeNDAP-Integration** — OPeNDAP als Fetch-Format.
- **Gaia-XP-Brücke** — Ring↔Nest source_id↔FP01-ipix; `xp_pilot_p6144.bin`
  zum CDN-Asset.
- **Membran M02–M07** — die benannten Membran-Reste.
- **TE-Baupunkte** — `cycle_phase_shift_surrogate`-Nutzung; bedingte
  Multi-Force-TE (Phasenraum).
- **SuperMAG erledigt (2026-09-10)** — `supermag_compiler.rs` + `supermag_1m`-
  GeoRec-Format + `sources.φ`-Block + `supermag-cdn.yml` gebaut (Pilot TRO
  2025-03, 267486 GeoRecs). Pending: die CDN-Manifestation (workflow_dispatch
  `supermag-cdn.yml`) + die Vollernte-Körnung der 194 Stationen. Siehe
  `handover-2026-09-10-supermag.md`.
- **Broker-Compiler + Survey-Audit** — die **9 Rubin-Broker** (7 full-stream:
  ALeRCE/AMPEL/ANTARES/Babamul/Fink/Lasair/Pitt-Google; 2 down-stream: SNAPS/
  POI) sind gemessen (Broker-Tabelle, Commit f975403); Compiler für
  die anonymen (ALeRCE/ANTARES/Fink/Babamul) fehlen. **DECaPS** (Katalog,
  g/r/i/z/Y) als Quelle registrieren; **VTSS/Mellinger** sind Bild-Surveys (kein
  Compiler). Die **10 Katalog-Kandidaten** (eROSITA/Fermi/XMM/GALEX/DSS2/
  Finkbeiner/SDSS9/PanSTARRS/GLIMPSE/SPITZER) sind bekannt, aber nicht disponiert.
  Die RSP-Bilder brauchen Datenrechte (Antrag `docs/auftrag/auftrag-rubin-data-rights-antrag.md`);
  die Alerts sind offen.
- **Befund-Migration abgeschlossen (2026-09-10)** — die 111 Befunde sind
  gegen Code/git/Register gelesen und gelöscht (108: Finding lebt im Code —
  Sonde/Compiler mit Commit — oder überholt; kein neuer Befund; die Klasse ist
  aus `docs-naming.md` gestrichen, `docs/befund/` ist leer). Getragen als
  Messergebnis: Galileo-RSS-Bestand (kein PDS4-Bündel `galileo.rss`; PDS3
  `GO-*-RSS-V1.0`: TRK-2-25 6,3 GB / TRK-2-18 0,16 GB / TRK-2-34 absent —
  kein Volumen in den Beständen);
  Dispersions-Ortungstest (2013-05-14: XRSA/94A/335A tragen den Sonnenursprung
  nicht, Kreis-Schnitt leer); Pioneer 1978–82 laut ohne Sonnentreiber
  (f107/omni2 widerlegt); Voyager-Roh-Doppler 1998–2002 ohne offene Quelle.

## Analyse

- **abfluss-trishuli** — das archivierte CSV des 08-27-Zugs lesen (genauer
  Pfad im archivierten Handover
  `docs/handover/archiv/handover-2026-09-09-disjunkte-linien-folge.md`),
  Abfluss-Pfeil messen.
- **bande-split** — Split-Ergebnis + offene Registerzeilen (f*, 1-s-Zählung,
  Amplitude); Restbestand 238 Dateien/77 Tage.
- **gic-p-wert** — p-Wert nachlegen, dann Wing/Viljanen.
- **Flut-Satellit** — robuste Flut-/Narbenfläche aus S1 (Delineation nicht
  erzeugt; eigene Ableitung).
- **Katalog-Lücken-Ernte** — RAVE DR6, APOGEE/GALAH, HyperLEDA, TGSS ADR,
  VLASS, AMS-02, GLADE+.

## Forschung — Nadeln

- **Ⅲ** TIAW vs Nanoflares (613-Ereignis-Satz, `aia_ladder_probe`).
- **Ⅳ** LAIC: CSES, TEC retro pre-2024, Instrument A, KDE-h.
- **Ⅴ** LSST-Live-Scan — läuft, Check-back.
- **Ⅷ** Dunkler Fluss — Haufen-Kanäle benennen.
- **Ⅸ/Ⅹ** FRB / Kugelblitz — Kanal-Lage.
- **Ⅺ** Placebo — Paar-EEG, fam-Schwelle, Nullkontrolle, bedingte TE.
- **Ⅻ** Urknall — Reihen-Paarung Winkelserie×z-Reihe.
- **ⅩⅢ** Voller 48er-Zensus (18 Non-Detections); Photochemie-Re-Erklärung.

## Forschung — Galileo-Floor

- **Stand:** der Floor ist per-Pass-Empfangs-Zustands-Lautheit (AGC-Klemme),
  kein Station-/Tag-Bild; keine Empfänger-Stufe (H1 widerlegt), kein Sende-
  Stations-Feld (TRK-2-25/ODR-Köpfe leer), kein Sonnen-/Wind-Treiber; genau
  ein M1-Sprung 1995-11-30/12-01 auf der resid-Achse (echt-vs-Modell
  unbestimmt). Die Messreihe: die ~60 Sonden `tools/measure/src/bin/galileo_*`
  + `docs/paper/galileo-rotor-spin-era-floor.md`.
- **Geleistet (2026-09-10):** `galileo_odr_compiler` + `galileo-odr-cdn.yml`
  gebaut — die 10 ODR-Dateien zu `galileo_odr.bin` verpackt (312 272 126 B,
  Provenance-Gate sha256 hält für alle 10, 1250 sps/Kanal, origin-verbatim).
  Messergebnis: `70580900.ODR` trägt 13 864 Records + 880 Folgebytes (kein
  ganzes Record-Multiple); die Folgebytes sind im Asset erhalten. Dispatch
  gelaufen: der erste Lauf manifestierte nur 7 Dateien (die JS-Annex-Namen
  tragen kein `JS_`-Präfix → 404); repariert (Bare-Namen-Fetch + harter
  Gate-Abbruch), der Re-Dispatch manifestierte das volle 10-Datei-Asset
  (312 273 094 B, Roundtrip hält) — registriert in `sources.φ`
  (`format galileo_odr`, em count, AD1..AD4). Siehe
  `handover-2026-09-10-galileo-odr-repair.md`.
- **Nächste Atome:** `galileo_receiver.bin` per Pass
  (`galileo_atdf_receiver_compiler.rs` steht); Jovian-Mond-Ephemeriden
  NAIF 501–504; All-Spin-Bus-CK Frame −77000 (`ck_daf_probe.rs`-Vorlage);
  empirische Rausch-Kurve aus TRK-2-25/2-18 (~6,5 GB Download);
  negativ-fuzzy: pscomppars `st_met`-Bio-Zeugen lesen
  (`disequilibrium_register_probe.rs` steht).

## Forschung — Weberin (zweite Linien)

- Planeten/Monde zweite Abstammung (INPOP `.dat` gegen `testpo`, oder SPK-Weg).
- Raumsonden-Doppler echte zweite Linie (VLBI-Winkel + Range, oder zweite
  Ephemeriden-Abstammung).
- Neptun-Planetenzentrum-Tabelle (Astrometrie-Kopplung) — Source-Port.

## Forschung — TE

- **n=1000-Riß** — Block-Länge n^(1/3)=10 bei n=1000 zu kurz / KSG-Dimension;
  die Null bleibt Block (`src/mathematikerin/te.rs` `gate_fpr_cells`).

## Forschung — Tiefenphasen

- **sP-Δ-Faltung** — ungemessen (Register-Duty, nicht 0.0).
- **sP-Beine** der pP-übersprungenen Stationen.
- **Externe Tiefen-Referenz** (TauP/KEB95).
- **Head-Wave-Lücke 410/660** — P-Triplikations-Gate gegen TauP.
- **Quell-Strahlungsterm (CMT)**.
- **W-Phase-CMT** als M9-Nachfolger-Atom.
- **Stromboli** als Vulkan-Lehrer.
- **Eikonal-Löser** über das volle Gitter (Dijkstra über ETOPO1; Gitter steht).
- **Die Erde als Sender** — Kreuzbereichs-Kalibrierung (Tonga 2022).
