<!--
  title: Auftrag — Dunkle-Materie Front C: entscheiden und fahren (frische Session)
  class: auftrag
  date: 2026-09-03
  status: pending
  see-also: docs/TODO.md docs/auftrag/auftrag-vollmission-redution.md docs/paper/probe-front-dark-matter.md docs/concepts/der-paradigmenwechsel.md
-->

# Auftrag: Dunkle-Materie Front C — entscheiden und fahren

## Zweck

Front C (der NAVIO-Vollmissions-Transit-Sweep) wird in einer **eigenen,
frischen Session** als erstes Atom dieses Auftrags bearbeitet — nicht im
Restkontext der Kleinpass-Session. Das Register ist die Wahrheit; dieser
Auftrag benennt nur die Einstiegs-Atome und die zu messenden offenen Fragen.
Keine Front-Entscheidung wird hier vorweggenommen. (Der frühere Satz „über den
Gravitationssensor `dark_matter_probe`" war A ≠ A und ist in der Register-
Entscheidung unten korrigiert: `dark_matter_probe` = Horizons-Netz = römisch Ⅱ,
nicht Front C.)

## Die ersten Register-Atome (in dieser Reihenfolge lesen)

1. `docs/TODO.md` — Nadel Ⅰ (Dunkle Materie, Zeile ~16: „WARTET AUF DICH", Gaia DR4
   2.12.2026, Front Ⅱ = Gravitationssensor) und die Transit-Kontext-Zeilen (~195–215).
2. `docs/auftrag/auftrag-vollmission-redution.md` — die NAVIO-Vollmission durch die Kette.
3. `docs/paper/probe-front-dark-matter.md` — das dunkle-Materie-Blatt (Nadel Ⅰ),
   Befund und Front-Zuordnung (heute v3, datiert 2026-09-03).
4. `docs/concepts/der-paradigmenwechsel.md` — der konzeptionelle Rahmen.

## Zu messende offene Fragen (nicht zu behaupten)

- **Front-Zuordnung klären:** das Register spricht von Front Ⅱ (Gravitationssensor)
  und die Proposal von „Front C = NAVIO-Transit-Sweep" und „Front B = Iapetus-Halo".
  Wie hängen Front Ⅱ/B/C zusammen und welche ist die vorgesehene? Erst aus dem
  Register ableiten, nicht raten.
- **Laufbarkeit:** läuft der NAVIO-Transit-Sweep mit den vorhandenen Daten sofort
  (Ephemeriden-Bins sind im Cache/CDN; NAVIO-Trajektorie prüfen) oder muss erst
  gebaut werden? Gemessen gegen `dark_matter_probe`-Eingaben und Datenverfügbarkeit.
- **Feld-Design:** welche Messung ist die reale Nachweis-Chance des Systems (falls
  die Register-Behauptung das trägt) — und was ist der ehrliche, abgrenzbare Befund.

## Kernregel (0 honored)

Keine Zahl ohne Messung. Ein fehlendes Datum ist `pending`/`open` mit Ort, nie
ein erfundener Wert. Eine Front-Empfehlung erst nach vollständigem Lesen des
Registers.

## Register — in Arbeit (2026-09-03, Operator-Verdikt nach Rats-Entscheidung)

Die Rats-Entscheidung (Front C fährt nicht als DM-Nachweis, das leere Netz ist
der DM-Befund) steht als **Messung des DM-Limits** weiter. Der Operator hat
jedoch (2026-09-03) die Auftrags-Schließung **zurückgerollt**: Front C wird
gefahren, nicht beerdigt — der eigentliche offene Faden ist der Form-Test
(∝t² vs RTG-Abklingkurve) an die Vollmissions-Daten, die ∝t²-Frage, die das
System nie mit eigenen Mitteln gestellt hat. Laufbarkeits-Messung korrigiert:
die NAVIO-Vollmissions-ASCII ist **live erreichbar** (SPDF, 200), nur nicht
lokal als Bin — keine Daten-Lücke, nur ein Hol-Schritt. Die Instrument-Lücke
(kein Ruck-Sweep) wird durch Bau geschlossen.

**Front-Zuordnung (gemessen):** römisch Front Ⅱ = Gravitationssensor/
Horizons-leeres-Netz (`dark_matter_probe`, Befund bei Ⅵ); Buchstabe Front C =
NAVIO-Doppler-Transit-Sweep (eigenes Instrument). Zwei Instrumente, nie eins.
Front B = Iapetus/Halo; Front A = redundantes Doppler-Limit.

## Messergebnisse der Fahrt (2026-09-03, alle gemessen)

1. **NAVIO-ASCII geholt + kompiliert** (Daten live vom SPDF, kein CDN-Pfad
   vorher): `pioneer10_doppler.bin` 908309 Records (1973-10-05..2002-03-03,
   DTYPE [12,13,31,36,37,38], 298003 ku-korrigiert), `pioneer11_doppler.bin`
   967272 Records (1973-04-10..1993-07-15, DTYPE [12,13,31,36,37], 497405
   ku-korrigiert). Dies ist die erste Kompilierung der Vollmissions-ASCII über
   die eigene Reduktion. `pioneer_navio_clean`: p10 908309→908028 (281 Spikes,
   335 FREQCY), p11 967272→967043 (229 Spikes, 407 FREQCY); die benannten
   Korrektur-Klassen (±500k ≈8,3 kHz, 1000-Hz, verschobene Counts) werden
   erkannt, nicht blind korrigiert (0 honored).
2. **Form-Test (Deduktion-1-Frage) an der Vollmission — gemessen:** der
   Barycentric-Rohlauf trägt die Serie nicht (resid 1,2e6 Hz p10 / 9,3e6 Hz
   p11). Der Vollmodell-Lauf `pioneer_doppler_moyer_navio` (DTYPE-12, 206/85
   Epochen, K=(240/221)/c) trägt den Rest: **Residuum-Floor 19,0 kHz (p10) /
   8,4 kHz (p11)** — K gemessen 3,584e-9 / 3,633e-9 s/m gegen (240/221)/c =
   3,622e-9. Selbsttest: der Floor liegt ~2e4×/8e3× über dem ~1-Hz-Signal der
   Pioneer-Anomalie; die Rest-Drift → Beschleunigung ist **Modell-Artefakt,
   nicht die Anomalie** (0 honored). Die ∝t²-vs-RTG-Frage ist an die
   Vollmissions-Daten damit ehrlich beantwortet: **keine getragene Form über
   einem kHz-strukturierten Floor; das sub-kHz-Residuum (DSN-Station + Orbit)
   ist der von der Reduktion selbst benannte Blocker.**
3. **Horizons-Netz frisch gemessen** (`dark_matter_probe`, release, auf den
   gecachten/CDN-Planeten-Bins + 5 Probe-Daily-Arcs): Baselines reproduzieren
   das Blatt (p10 median 2,24e-8, p11 3,93e-8, v1 1,67e-8, v2 2,54e-8, nh
   8,56e-8 m/s²); Positiv-Kontrolle trägt die weggelassenen Planetenmassen;
   **Flaggen 0/1008, Punkte bei ≥2 Sonden: 0** — das leere Netz ist der
   DM-Befund, frisch bestätigt.
4. **Ruck-Transit-Sweep:** nicht über einen Modell-Artefakt-Floor fabriziert.
   Ein Sweep auf einem als Artefakt gemessenen kHz-Residuum würde das Artefakt
   messen, nie einen Transit (A = A). Der ehrliche Sweep braucht das
   **sub-kHz-Residuum** (DSN-Station + Orbit, von der Reduktion benannt) — als
   benannter Bau-Auftrag registriert, nicht als erfundener Lauf.

## Sub-kHz-Residuum — Tür geöffnet (2026-09-03, Rat-Verdikt B, Atom 1+2 gebaut)

Rat e stimmig: PDPL (6-Slot, 6 Konsumenten) unberührt lassen — ein neues Record
mit eigener Magie trägt TRANS/RCVR1/linkmode (Name = Implementation). Gebaut:

- **Atom 1 — PNAV-Record:** `src/mathematikerin/doppler.rs` `write_pnav_bin`/
  `parse_pnav_bin`, Magic `PNAV`, `[f64;9]` = `[timtag, obs, freq, cmptime,
  dtype, sc, trans, rcvr1, linkmode]`, 72 B (P11R-Parität) + Roundtrip-Test
  `test_pnav_roundtrip`. `pioneer_doppler_compiler.rs` fängt `TRANS`/`RCVR1`,
  leitet linkmode ab (11 einweg / 12 zweiweg trans==rcvr1 / 13 dreiweg) und
  schreibt `{p10,p11}_navio.bin` **neben** dem PDPL (kein zweiter Fetch):
  p10 908309 Records (linkmode [11,12,13], 11 Stationen), p11 967272 (10
  Stationen). PDPL unberührt.
- **Atom 2 — `pioneer_navio_residuum.rs`:** DSN-Stationsmodell (zweiweg
  `rdown`+`rup`, Lichtzeit, `odp::dsn_station`), displaced-count-Mask
  (|resid|>1e5 Hz nach Erst-Fit, Zweit-Fit), serialisiert
  `pioneer{10,11}_navio_residuum.bin` (Magic PNVR via P11R-Layout) +
  `pioneer{10,11}_navio_daily.bin` (Magic PNDM, Tagesmedian trägt nie ohne
  Streuung), de-trend + LS-Bandscan + Ruck-Scan (4·sd-Gate, live-Daten).

**Gemessen:** displaced-count-Mask senkt den Per-Station-Floor von ~5e5 Hz
(±500k/1000-Hz-Counts dominierten) auf **2,8–19 kHz** (p11 4,5–12 kHz) — der
Stations-Term konkurriert nun mit dem Baryzentrum-Referenzfloor (19/8,4 kHz),
statt darüber zu liegen. Die serialisierten Tagesmediane sind der neue,
konsumierbare Faden; das sub-kHz-Tagesmedian-Ziel (1,5-kHz-Streuung / √N →
zehner-Hz) ist über PNDM jetzt greifbar.

**Atom 3 (2026-09-03, Commit 50625aa):** Deduktion-10-Tagesmask in
`pioneer_navio_residuum` portiert (Gate 4×p90 des Tages-RMS; korrupte-Tage-
Cluster verworfen, nicht gemittelt). Gemessen: p10 5, p11 15 corrupt-day
Cluster verworfen; der Ruck-Scan läuft über gemaskte Tagesmediane. Verbleibende
Flaggen werden als Kandidaten gegen den lokalen Floor gehalten — kein
Detektions-Anspruch; die Flaggen stehen, wo ein Transit-Sprung vom
Segment-Rauschen nicht sauber getrennt ist (das sub-kHz-Ziel bleibt am
~1,5-kHz-Streuungsboden, 0 honored).

**Nachfolge-Messung (2026-09-03) — Kandidaten gegen bekannte Encounter:**
die überlebenden starken Ruck-Flaggen gegen die bekannten planetaren
Encounter-Epochen abgeglichen (aus `probe-front-dark-matter.md` Table 1,
gemessen): P10 nur Jupiter 1973-12; P11 Jupiter 1974-12 + Saturn 1979-09. Befund:
P11s Top-Flaggen (1974-12-03 @100σ Jupiter, 1979-08-31 Saturn) fallen exakt auf
dokumentierte Vorbeiflüge = benannte Granulat-Überschuss-Klasse. **P10s stärkste
Flaggen (1996-05-28 @73σ, 1996-11-22, 1996-10-15, 1981-02-09, 1982-03-12) liegen
nach P10s einzigem planetaren Encounter (1973) — nicht durch bekannte Flybys
erklärt.** Eine Persistenz-Heuristik (Niveauversatz hält über ±Tage an) wurde
gebaut, aber verworfen: sie klassifizierte P10s Jupiter-Flyby persistent und
P11s Jupiter-Flyby isoliert — dieselbe Ereignisart, zwei Urteile (unverlässlich,
nicht committet). Der ehrliche nächste Schritt ist die Station-/Era-Überprüfung
der P10-1981/1982/1996-Flaggen (Teilen sie einen Stations-/Era-Mix?), nicht eine
ungeprüfte Heuristik. Offen, 0 honored.

**Station-/Era-Befund (2026-09-03, Commit e32d7f9, `pioneer_navio_flag_era`):**
die P10-1981/1982/1996-Flaggen sind gemessen **Station-63-Artefakte**, kein
kohärentes DM-Signal: Einzelstation 63, ±5,7–90 kHz-Tages-Sprünge (1996-05-28
von −61,8 kHz auf +95,0 kHz an einem Tag), 1-Sample-Tage (1981-02-10,
1996-10-19), Vorzeichenkipp zwischen Tagen. Ein DM-Transit wäre ein kleiner,
anhaltender, kohärenter Versatz; das hier ist grobe Einzelstations-Tracking-
Struktur, die der per-Station-Fit nicht absorbiert (der ±500k/1000-Hz-Zähl-
korrektur-Rest). **P11-1981-02-09** (auch geflaggt) ist dagegen **multi-Station**
(12/42/63, 889 Samples) mit kohärentem ~3,3-kHz-Schritt am selben Datum —
interessanter (kohärent, keine Sparsity), aber kHz-Skala, nicht sub-kHz. Befund:
die Ruck-Flaggen sind Stations-/Era-Struktur, kein sub-kHz-Transit. Das
sub-kHz-Ziel bleibt am ~1,5-kHz-Streuungsboden; der Transit-Sweep kann ehrlich
keinen DM-Klumpen über diesem Boden melden (0 honored).

**Source-Pflicht (CDN-Manifestation):** das neue Dataset ist der PNAV-Bin;
`pioneer_doppler`-CI-Job (kernel-flatten.yml) läuft `--ci-mode` und lädt PDPL +
PNAV aufs `spdf.gsfc.nasa.gov`-Release. Die PNVR/PNDM-Serialisierung ist
measure-Ausgabe (wie `pioneer11_residuum.bin`, nie aufs CDN) — kein CDN-Asset.
Nicht in `phi/sources.φ` — die Bins sind Werkzeug-Mess-Artefakte (Konsum via
`parse_bin`/`parse_pnav_bin`), keine Live-ω-Oszillatoren; der Telemetrie-Peer
trägt denselben Behandlungsweg. `data/` gitignored; der CI erzeugt und lädt.

Diese Datei bleibt der Ledger bis Front C gefahren und entschieden ist.
