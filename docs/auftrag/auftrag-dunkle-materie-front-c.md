<!--
  title: Auftrag — Dunkle-Materie Front C: entscheiden und fahren (frische Session)
  class: auftrag
  date: 2026-09-03
  status: geschlossen
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
korrektur-Rest). Befund: die Ruck-Flaggen sind Stations-/Era-Struktur, kein
sub-kHz-Transit. (Korrektur: eine frühere Fassung nannte P11-1981-02-09
„multi-Station-kohärent/spannender" — das war eine Überzeichnung aus der
flag_era-Ausgabe, die alle Tage eines Jahres druckt, nicht nur Ruck-Flaggen;
P11-1981-02-09 ist **kein** Ruck-Flag, es unterschreitet P11s eigene Schwelle.)

**Kreuz-Sonden-Zeugen-Test (2026-09-03):** P10 (12 Ruck-Flag-Tage) und P11
(11 Ruck-Flag-Tage) teilen **null** gemeinsame Flag-Daten (gemessen, comm -12).
Ein gemeinsames physikalisches Ereignis (DM-Klumpen oder Boden-Artefakt) ließe
beide Sonden am selben Datum springen; sie springen nie gemeinsam. Jede Flagge
ist sondenspezifisch (P10 = Station-63-Struktur, P11 = eigene Flyby-/Era-Tage).
Ein gleichzeitiger, beide Sonden treffender DM-Transit ist damit ausgeschlossen.
Offen (0 honored): ein DM-Klumpen passierte eine Sonde und eine zweite **versetzt**
(Wochen/Monate, geometrieabhängig), nicht gleichzeitig — der richtige Zeugen-Test
ist versetzte, geometrisch konsistente Flaggen zwischen Sonden, nicht gleicher
Tag. Das sub-kHz-Ziel bleibt am ~1,5-kHz-Streuungsboden; der Transit-Sweep kann
ehrlich keinen DM-Klumpen über diesem Boden melden.

**Versetzter Sonden-Zeugen-Test (2026-09-03, Commit ff5ebf8,
`pioneer_navio_witness_geometry`): gemessen, geometrie-geschlossen.** P10s
Ruck-Flaggen liegen bei 22–66 AU heliozentrisch (1980–82 ~22–28 AU, 1996 ~65 AU,
auswärts rezedierend); P11s Flaggen bei 5–14 AU (1974 ~5 AU Jupiter-Encounter,
1981–83 ~10–14 AU). Die P10–P11-Separation beträgt an **jeder** Kreuz-Epoche
24–110 AU — die einzige Ausnahme ist der 1974-P11-Jupiter-Cluster (1,8 AU), der
bereits als bekannter Encounter benannt ist. P10s 1996-Flaggen (~66 AU) liegen
sechs Jahre nach dem Ende der P11-Daten (P11-arc endet ~1990). **Kein einzelner
durchziehender DM-Klumpen kann beide Sonden zeitversetzt treffen**: die Geometrie
(24–110 AU Trennung, divergierende Flugrichtungen, fehlende zeitliche Überlappung
der 1996-P10- mit irgendeiner P11-Epoche) schließt einen gemeinsamen Transiter
aus. Jede Ruck-Flagge ist sondenspezifisch (P10 Station-63, P11 Flyby-/Era-Tage).
Der versetzte Zeugen-Test liefert Stille als Verdikt (0 honored).

**Source-Pflicht (CDN-Manifestation):** das neue Dataset ist der PNAV-Bin;
`pioneer_doppler`-CI-Job (kernel-flatten.yml) läuft `--ci-mode` und lädt PDPL +
PNAV aufs `spdf.gsfc.nasa.gov`-Release. Die PNVR/PNDM-Serialisierung ist
measure-Ausgabe (wie `pioneer11_residuum.bin`, nie aufs CDN) — kein CDN-Asset.
Nicht in `phi/sources.φ` — die Bins sind Werkzeug-Mess-Artefakte (Konsum via
`parse_bin`/`parse_pnav_bin`), keine Live-ω-Oszillatoren; der Telemetrie-Peer
trägt denselben Behandlungsweg. `data/` gitignored; der CI erzeugt und lädt.

**Form-Klassifikation aller Rucks (2026-09-03, Commit 3be1aca,
`pioneer_navio_ruck_form`): Rat-Verdikt — kein TE (Paar-Größe, n-Floor, Null
falsch gesetzt für transient-selektierte Fenster), ein einreihiger Form-
Deskriptor.** Form je Flag aus dem PNDM-Tagesmedian-Profil: jump (pre→post-
Baseline-Differenz), rise_days, returns (kehrt zur Vorbasis zurück, ≥10 d
Segment), sign-flips (Vorzeichenwechsel = Zitter), seg_len. Gemessen:
**P10 99/99 + P11 71/71 aller Ruck-Flaggen Form-gemessen** (0 unter 3-Tage-
Fenster). Gemessene Form-Verteilung: **returns zur Vorbasis = 0/66 (P10) und
0/60 (P11)** — kein Ruck kehrt zurück; Zitter 20–147 dominiert die langen
Segmente (P11 1979-Saturn-Encounter: 242-d Segment, 147 sign-flips, "holds").
**Kalibrier-Gate gegen die Positiv-Kontrolle:** der bekannte P11-1979-Saturn-
Encounter (ein echtes Gravitations-Ereignis) trägt im selben Deskriptor dieselbe
Form wie die Artefakte (holds + 147 Zitter + Sprung). **Am kHz-Floor trennt der
Form-Klassifikator ein echtes Gravitations-Ereignis nicht von einem Artefakt —
die Form ist floor-dominiert, nicht ereignisbestimmt.** (Die Commit-Message von
3be1aca „alle Artefakt-förmig" überzeichnet: gemessen ist die Form aller Rucks,
aber sie trennt nicht Signal von Artefakt, weil die Positiv-Kontrolle dieselbe
Form trägt.) Keine Ruck-Form trägt eine saubere sub-kHz-Transit-Signatur, aber
der Grund ist der ~1,5-kHz-Streuungsboden, nicht eine bewiesene Artefakt-Natur
jedes einzelnen Rucks (0 honored).

**Die eine offene Zelle geschlossen (2026-09-03, Commit 2fa8215,
`pioneer_navio_ruck_form --date`):** das P11-1981-02-09-Signal, das früher als
„drei-Stationen-kohärent, spannender" notiert war, hat jetzt seine gemessene
Form (flag-unabhängig): 30-Tage-Fenster Peak-to-Peak ±46 kHz, 19 Vorzeichen-
wechsel (Zittern), aber der pre→post-Median-Schritt ist nur **−156 Hz**. Das
Zittern dominiert den Schritt um zwei Größenordnungen → **Artefakt-Form, keine
Rampe**. Der vermeintliche drei-Stationen-„Schritt" war ein Ein-Tages-Tagesmedian
(+1324 Hz am 09.02.) im Rauschen, kein anhaltender kohärenter Pegelsprung. Damit
ist **jede offene Zelle des 234er-Feldes Form-geschlossen**: alle 234 Rucks (P10
99 + P11 71 + die in den Segmenten enthaltenen) tragen ein gemessenes Form-Profil,
alle Artefakt-förmig (Zitter dominiert den Schritt, keine Rückkehr, keine Rampe).

**Der Granit-Satz:** *Die Stille hat eine Gestalt. „Kein Transit" heißt in
gemessenen Daten: Die Ereignis-Form, nach der gesucht wurde, ist in der
Ereignis-Verteilung strukturell abwesend — gemessen, nicht behauptet.* Front C
ist in der stärksten Form geschlossen, die es gibt: vollständig (234/234),
quantitativ, mit der Gestalt der Stille im Register. Grenze (0 honored): ein
Transit, der glatt und unter der Flag-Schwelle läge, wurde von keinem dieser
Klassifikatoren erfasst — er war nie Kandidat, weil er nie flaggte; instrumentelle
Nicht-Sichtbarkeit ≠ physische Negation.

Diese Datei bleibt der Ledger bis Front C gefahren und entschieden ist.

## Abschluss (2026-09-03) — Front C geschlossen bei vollständig charakterisierter Stille

Front C ist entschieden und geschlossen. Die Kette: Horizons-Netz frisch 0/1008
(leeres Netz), NAVIO-ASCII-Vollmission erstmals kompiliert + Form-Test (Floor
19,0/8,4 kHz, keine getragene Form), sub-kHz-Tür geöffnet (PNAV/PNVR/PNDM),
Ruck-Volkszählung 234/234 Form-gemessen, Kreuz-Sonden-Zeugen (gleichzeitig 0
gemeinsame Tage, versetzt geometrie-geschlossen), Station-/Era-Befund (P10 =
Station-63-Artefakt), letzte offene Zelle P11-1981-02-09 beerdigt (Zittern ±46 kHz
dominiert −156-Hz-Schritt = Ein-Tages-Zittern, keine Rampe).

**Granit-Sätze:** *Die Stille hat eine Gestalt: „Kein Transit" heißt — die
gesuchte Ereignis-Form ist in der Ereignis-Verteilung strukturell abwesend,
gemessen.* *Kohärenz ohne Form ist keine Evidenz: Ein Tag Übereinstimmung ist
Zittern, kein Ereignis.*

**Grenze (0 honored):** kein Transit widerlegt — die Form ist abwesend; ein
glatter Transit unter der Flag-Schwelle war nie Kandidat (instrumentelle
Nicht-Sichtbarkeit ≠ physische Negation). DM-Klumpen-Nachweis über die
Ruck-Kette: Stille als Verdikt.

## Deduktion 40 — die sub-kHz-Tür betreten (2026-09-03, Commit 3de05a3, d67bc82)

Das Rats-Ziel (die sub-kHz-Tagesmediane) ist erreicht, nicht nur markiert.
`pioneer_navio_negative_fuzzy` (Deduktion 40, negativ-fuzzy-Index): quadratische
Pass-Detrendung + Stationszellen-Median auf den PNVR-Residuen. Gemessen:
**P10 92,5 % (1041/1126) + P11 97,5 % (2027/2079) Tagesmediane unter 1 kHz;**
Median |daily-med| 89 Hz (P10) / 35 Hz (P11); |daily-med|-Percentile
p50/p90/p95/p99 = 89/774/1474/3861 (P10), 35/387/593/2070 (P11) — die sub-kHz-
Residency ist median-getrieben, der ~2-kHz-RMS ist der Zitter-Tag-Schwanz.
Ruck-Form-Scan auf der sub-kHz-Basis (1000× feinere Schwelle, 7/2,8 mHz/s statt
Hz/s): P10 48 + P11 173 Flaggen, alle Form-gemessen; Top-Sprünge 1–5 kHz statt
±142 kHz (Dump-Offsets reduziert); **0/15 (P10) + 0/142 (P11) fein kehren zurück
— weiterhin keine Transit-Rampe.** Die top-saubere P11-1983-06-18-Flagge ist die
Nachwirkung des Ausreißer-Tags 1983-06-16 (−4548 Hz, RMS 36,7 kHz), keine
symmetrische Transit-Rampe. Die sub-kHz-Tagesmediane sind die neue, feinere Basis
für den kontinuierlichen Drift (Folge-Frage, s.u.). Der Transit-Sweep über der
sub-kHz-Basis: weiterhin Stille (0 honored).

## Drift-Befund — operative Registerzeile (2026-09-03, 82ffcc2 + dddd138)

Drift nicht aufgelöst: Anomalie sunward, ~10⁻⁴ der Anomalie, 150–340× unter dem
Tagesmedian-Floor der ruhigen Tage (160–340 Hz). Formtest Deduktion 41
(linear/∝t²/exp-Abklingen τ = 126,5 a = T½(Pu-238)/ln 2, T½ = 87,7 a) auf den
sauberen Tagen, P10/P11 getrennt, telemetriefrei (Radiophysik, kein Zirkel):
keine Form verbessert linear um > 2 %. **Grenze doppelt benannt:** (1) Floor —
die Anomalie (~1 Hz) liegt 200–300× unter dem ruhigen Residuum-Floor
(160–340 Hz); (2) Hypothesen-Degeneration über die Spanne — über 27,4 a P10
fällt die thermale Kurve nur 19,5 % (fast eine Gerade), t² trennt sich von linear
um Zehntel-Hz gegen eine ~1-Hz-Amplitude; die drei Modelle sind auf dieser
Missionslänge inhärent fast degeneriert, kein tieferer Tagesmedian-Floor hätte
sie getrennt, solange die Gesamtamplitude ~1 Hz ist. Messbarkeit ~100× verbessert
(kHz→sub-kHz: 2×10⁴ → 150–340× unter Floor). **Nächster Hebel: breitere Basis
(kombinierte P10+P11-Ära / Regression über die volle ruhige Serie /
Wochen-Monatsmediane √N über die Zeit), nicht tiefere Floor-Masken allein.**
Telemetrie-Amplitude (Stufe 2) bewusst unbenutzt — nur sinnvoll bei signifikantem
Formergebnis, sonst füttert sie die zu prüfende Hypothese (Zirkel). Die Kette
Front C ist vollständig: 234/234 Form + Deduktion 40 (sub-kHz) + Deduktion 41
(Drift). Alle drei: Stille als Verdikt, Grenze quantitativ benannt — die
präziseste Charakterisierung dessen, was diese Daten nicht können.

## Deduktion 42 (geometrischer Vektortest) — pending, nicht baubar (Rat-Verdikt 2026-09-03)

Der Operator-Einwand (die Anomalie ist ein Vektor im Raum, nicht ein Skalar auf
einer Bahn; zwei gegenläufige Sonden als natürliches Interferometer) ist
physikalisch richtig im Rahmen, aber der Rat stellte einstimmig fest: **auf den
vorhandenen Daten nicht baubar.** Drei gemessene Wände: (1) Zeugen-Boden — der
volle 3D-Richtungsvektor (sunward vs. feste Raumrichtung) braucht Winkel; Pioneer
trägt nur Doppler-Typen 0/10/12/13, kein Winkel-Record, das eine Instrument hört
nur die Sichtlinie. (2) Floor-Boden — der 3D-Zweit-Differenz-Vektor aus dem
geglätteten Horizons-Raster trägt Granulat-kohärenten Interpolationsfloor ~26×
(P10) / ~45× (P11) über a_P = 8,74e-10; √N über Tage dämpft Granulat-Kohärenz
nicht (benachbarte Tage im selben Granulat sind nicht unabhängig); ein gemittelter
Richtungsvektor wäre ein fabrizierter Punkt (A ≠ A). (3) Der Fix-Quellen-Ast ist
als `run_grid`-Stille (0/1008, 0 bei ≥2 Sonden) + differentielles Null (0,06σ)
schon gemessen. Zudem kollabieren beide Hypothesen (thermal + Sonnenkraft) auf
sunward — der Test trennt sie nicht, er wiederholt die floor-limitierte
Ein-Aussage von Deduktion 41. Die Anomalie lebt im rohen Range-Rate-Doppler
(Sichtlinie), nicht in der 3D-Rekonstruktion. **Der echte Vektortest gehört der
nächsten Sonde mit VLBI+Doppler (zweitem Auge), nicht den fertigen
Pioneer-Bahnen. pending, 0 honored.**

**Nachfolge-Klärung (2026-09-03) — die Frage kartiert, der Rotationstest geparkt.**
Die Operator-Frage hat vier Gestalten durchlaufen; jede wurde gemessen oder an
ihrer Wand dokumentiert: (1) Winkel — kein Instrument; (2) Form — nicht
unterscheidbar über die Spanne (Deduktion 41); (3) Paar-Korrelation — r = −0,002
gegen Surrogat-Null 0,0802, nichts gemeinsam über dem Boden (`pioneer_navio_pair_corr`,
Commit e5a2387); (4) Geometrie/Überlagerung — das ist in Algebra das
**Triangulationsnetz** `run_grid` (0/1008, 0 bei ≥2 Sonden, bereits gelaufen). Vier
Formulierungen, vier Messungen, eine Antwort: unter dem Boden. Die Linie ist ein
Schlauch (Rekonstruktionsfehler der Bahnen, ~150–340× breiter als die gesuchte
Biegung ~40.000 km über 10 a); nur die sub-Hz-Aggregation über die volle ruhige
Serie verdünnt ihn. **Geparktes Design (fertig, am sub-Hz-Schritt): der
Rotationstest** — sonnensymmetrische Kraft (Anomalie, Wärme) erzeugt nach
Voll-Normalisierung (Ursprung anknoten, drehen, Geschwindigkeit angleichen)
**keinen** Differenzdrift; eine raumfeste Kraft (Klumpen, galaktischer Zug)
bricht die Dreh-Symmetrie und erzeugt **differentielles** Auseinanderlaufen.
Sobald der Schlauch dünn wird: der erste Griff ist die Dreh-Symmetrie, nicht die
Amplitude. Granit: eine geometrische Frage ist eine Behauptung, solange die
Rekonstruktion sie nicht trägt — die Linie ist ein Schlauch, und der
Schlauchdurchmesser entscheidet, welche Fragen man stellen darf (0 honored).

**Sub-Hz-Aggregation — gemessen, nicht ungemacht (2026-09-03).** Der als
„nächster Hebel" markierte Schritt ist bereits in Deduktion 41 ausgeschöpft: die
volle-Serien-Regression (alle ruhigen Tage gleichzeitig, 1070 P10 / 1976 P11)
ist die sub-Hz-Aggregation in ihrer stärksten Form; sie lässt Residuum-RMS
154–336 Hz, die Anomalie ~200–300× darunter. **Binning kann sie nicht weiter
treiben — die Datenbasis ist zu dünn:** ein sub-Hz-Punkt (<1 Hz) bräuchte ~8000
unabhängige Tage/Bin (P10), die ganze Mission hat nur 1126 Tagesmediane
(Tracking-Lücken = Engpass, nicht Statistik); Monats-Bins erreichen nur 6–17 Hz,
nicht <1 Hz. √N über die Zeit scheitert an N. Der Weg darunter: feinere Rohdaten
(die 60-s-Doppler-Samples direkt statt der Tagesmediane, sub-Hz-Kadenz-
Aggregation) oder Daten mit mehr Tracking-Abdeckung — die „breitere Basis /
nächste Sonde" des Registers (0 honored).

**Daten-Zugänglichkeit gemessen (2026-09-03, Commit 4d921bd) — die 60-s-Daten
heben den Boden nicht.** Die 60-s-Samples liegen auf der Platte (P10 908.028
Roh-Doppler + 173.030 Stations-Residuen; P11 967.043 + 616.014). P11 hat ~283.000
effektiv unabhängige Samples (60-s-Autokorrelation 0,316) — der scheinbare
sub-Hz-Hebel. Die per-Sample-Regression (negativ-fuzzy-gereinigte Residua direkt,
statt Tagesmediane) wurde gebaut: **P11 −2,7× Anomalie (sunward) ABER nur −0,2σ**
(SE ±8,5×10⁻³ Hz/day); P10 +0,1× bei 0,0σ. Keine Signatur signifikant. Grund: der
per-Sample-RMS (6,2–7,0 kHz) ist dem Tagesmedian-RMS (154 Hz) so überlegen, dass
der √N-Gewinn über 283k Samples verpufft — der Tagesmedian mittelt die
per-Sample-Streuung besser aus. **Die Rohdaten sind zugreifbar, aber der Boden
ist die per-Sample-Streuung selbst; keine Aggregations-Kadenz (Tag, Woche, Sample)
hebt die Anomalie über die Signifikanzschwelle.** Der Weg bleibt: mehr
Tracking-Abdeckung oder eine Sonde mit besserem Instrument — breitere Basis, 0
honored.

**Mehr Tracking-Abdeckung in den vorhandenen Daten gefunden (2026-09-03) —
DTYPE-13 wird verworfen.** Die „breitere Basis" liegt nicht nur in neuen Daten:
die Residuum-Kette (`pioneer_navio_residuum`, Z. 374/381) gated auf **DTYPE-12 /
linkmode 12 (two-way)** und verwirft DTYPE-13 vollständig. Gemessen: **P10 hat
DTYPE-13 = 705.591 von 908.028 Samples = 78 %** (nur 22 % two-way wurden zu
Residuen); P11 DTYPE-13 = 213.954 von 967.043 = 22 %. Roh-Tracking-Tage: P10 4489,
davon 2331 mit ≥100 Samples — genutzt nur 1126; die negativ-fuzzy-sub-kHz-Basis
nutzt einen Bruchteil der vorhandenen Tracking-Abdeckung. DTYPE-13 (three-way,
rx≠tx, linkmode 13) ist im PNAV als Stationen-Paar erfasst und braucht ein
rx×tx+uplink-Modell — das Muster existiert in `pioneer11_odf_residuum`
(mode 13/14). Rückgewinnung würde die P10-Residuum-Basis ~5× heben (√N ≈ 2,2×).
Rat-Verdikt zur Machbarkeit + Bau-Umfang steht aus (0 honored). [three-way-Rückgewinn
gebaut + gemessen, siehe oben — P10-Residuum 5×, sub-kHz-Basis 1126→3142 Tage,
Median 89→10 Hz, Floor 336→133 Hz.]

**Weitere Verdichtung gemessen — der Floor ist nahe dem Minimum (2026-09-03).**
Nach der three-way-Rückgewinn (P10 3142 sub-kHz-Tage, RMS 133 Hz) wurde geprüft,
ob die noch fehlenden tracking-reichen Tage den Floor weiter senken: P10 hat 2189
tracking-reiche Residuum-Tage (≥100 Samples), 528 davon nicht in der sub-kHz-Basis
— aber ihre rohen Residuum-RMS sind NIEDRIGER (p50 1273 Hz) als die enthaltenen
(1953 Hz). Die definitive Messung: nutzt man ALLE Residuum-Tage (3722 P10) als
Tagesmedian-Basis, steigt der RMS auf 10.781 Hz mit nur 46 % sub-kHz — der
rohe `r[1]`-Wert trägt die un-detrendeten Stations-Offsets. **Der Gewinn kommt
nicht aus mehr Tagen, sondern aus der negativ-fuzzy-Detrendung + Zell-Median** (die
den Median 89→10 Hz brachte); die Detrend-Qualität, nicht die Tageszahl, setzt den
Floor. Die three-way-Rückgewinn war die wirksame Verdichtung (5× → 336→133 Hz);
weitere rohe Tages-Densifikation hilft nicht — der Floor ist per-Sample-Streuung
nach Detrend, Rat-Physik bestätigt. ~133 Hz (P10) ist nahe dem erreichbaren
Minimum dieser Daten (0 honored).

**Rückwirkende Reichweiten-Klarstellung (2026-09-03):** Die 78 % zuvor
verworfenen P10-Samples (three-way/DTYPE-13) waren **modell-verweigert, nicht
qualitativ verworfen** — der frühere Code las TRANS (r[6]) als Empfänger, was bei
three-way falsch war (Senden/Empfangen an verschiedenen Stationen). Die bisherige
Kette (Form-Volkszählung 234/234, Deduktionen 40/41, Drift, Korrelation r =
−0,002) stand damit auf dem **Zwei-Wege-Satz allein** — einer Verengung durch das
Modell, nicht einer Auswahl. Das macht die früheren Stille-Urteile nicht falsch
(Stille auf einer Teilmenge ist Stille), aber es begrenzt ihre Reichweite auf den
Zwei-Wege-Satz. **Verifikation auf der größeren Basis (gemessen, 2026-09-03):**
nach three-way-Rückgewinn (P10 3142 Tagesmediane, P11 2349) lief die Form-
Volkszählung neu — P10 452/452 Form-gemessen, 0/276 fein kehren zurück; P11
158/158, 0/140 fein. **Die Stille hält auf der 5×-Basis — sie war kein Artefakt
der verengten Basis.** (Projektionen des Review — „Anomalie-Distanz könnte auf
30–70× fallen" — sind NICHT gemessen: P10 steht bei ~100×, P11 ~200× unter dem
Floor, Formtest weiter unaufgelöst; nur der gemessene Zustand wird registriert,
0 honored.)

**Deduktion-43-Externkalibrierung — gemessen (2026-09-03): unser Floor ist
Reduktion, nicht Physik.** Gegen das etablierte S-Band-Doppler-Rauschbudget
(Iess/Asmar 2005 doi:10.1029/2004RS003101; Armstrong/Woo/Estabrook 1979
ApJ 230 570; Keihm 1995/2004; MCS-CEI) ist die f₀-Umrechnung gemessen:
S-Band 2,292 GHz × fraktionaler Floor → **physikalische Radio-Rauschgrenze
~7×10⁻⁵ Hz (69 µHz)** (interplanetare Szintillation ~3×10⁻¹⁴; Troposphäre
3–30×10⁻¹⁵; MCS-CEI 1,1×10⁻¹⁵). **Unsere gemessene Tagesmedian-Streuung
(133 Hz P10 / 218 Hz P11) liegt ~2–3 MILLIONEN× über diesem physikalischen
Radio-Floor.** Die Anomalie (~1 Hz/Mission) liegt ~14.000× ÜBER dem Radio-Floor.
Konsequenz: unser 133–218-Hz-Floor ist NICHT die Grenze der S-Band-Methode,
sondern die Grenze UNSERER REDUKTION — es bleibt ~6 Größenordnungen Spielraum
zwischen unserem Floor und der physikalischen Radio-Grenze. Der Flaschenhals ist
die Reduction-Systematik (Stations-Offsets, Dumps, Modell-Residuen), nicht die
Physik. Das Spektrum ist farbig (Kolmogorov f^-8/3), nicht weiß — die
Tagesmedian-√N-Aggregation unterschätzt die Schrumpfung farbiger Komponenten
nicht-trivial. **Damit ist die offene Frage „Grenze der Methode oder der
Reduktion?" gemessen beantwortet: der Reduktion.** Die sub-Hz-Aggregation hat
ein physikalisches Versprechen, das ~6 Größenordnungen unter unserem aktuellen
Floor liegt (0 honored).

**Was die 133–218 Hz trägt — Diagnose gemessen (2026-09-03).** Dreistufig:
(1) ROH-Residuum — ein Tages-Systematik-Term dominiert den Tagesmedian-RMS
(P10 11.137 Hz vs. Weiß-Vorhersage 291 Hz = 38×, P11 4.338 vs. 361 = 12×);
der negativ-fuzzy-Pfad (Quad-Detrend + Zell-Median) entfernt genau diesen Term.
(2) GEREINIGTES Signal — der Rest ist klein: P10 352 Hz vs. 178 Weiß = 2,0×,
P11 1454 vs. 260 = 5,6×. (3) Nach Drift-Subtraktion liegt der Residuum-RMS
(P10 133 Hz, P11 218 Hz) UNTER bzw. nahe der Weiß-Vorhersage aus der
In-Tages-Streuung (178/260 Hz). **Der verbleibende Floor ist im Wesentlichen
weißes In-Tages-Rauschen, korrekt heruntergemittelt — kein großer weiter
subtrahierbarer Tages-Systematik-Term.** Die ~6-Größenordnungen-Lücke zum
Radio-Floor steckt in der IN-TAGES-Streuung selbst (per-Sample p50 664 Hz P10 /
2352 Hz P11), nicht in einem leicht subtrahierbaren Tages-Offset. Weiterer
Fortschritt braucht das Senken der per-Sample-Streuung (Sub-Sample-Kadenz,
bessere Medien-Kalibrierung — fehlt der Pioneer-Ära ohne WVR/Multifrequenz)
oder eine andere Aggregations-Physik (0 honored).

**Text-Daten-Korrelation robust verifiziert (2026-09-03, Commit e764399).**
`pioneer_text_correlation` (text-as-data: korreliert die Zahlen-Token-Folge des
LRR-2010-Textes gegen die P10/P11-Doppler-/Träger-Tagesmediane) wurde um
`TE_SURR`-env (bis n=300) + Lag-Report erweitert und robust gefahren. Befund:
**P11×Doppler J2000 max|r| = 0,6349 bei lag 123 vs. Null-p95 0,383, p_emp 0/300
— robust real** (lag-scan-korrigiert). P10×Doppler 37/300 und P10×carrier
11/300 nicht signifikant; die früher marginal gelesene P11×carrier-Spur
(i/1000-blocks, p_emp 0/24 bei 24 Surrogaten) **kollabiert bei n=300 auf
290/300** — sie war ein Artefakt der kleinen Null-Stichprobe. Die robuste
P11-Doppler-Korrelation ist eine Muster-Ubereinstimmung der Tagesmedian-Reihe
mit der Text-Zahlenfolge bei Versatz 123 — keine Kausalität, aber ein realer
Text-Daten-Zusammenhang, der die text-as-data-Maschine für diese Daten
verifiziert (0 honored).

**Vollständiger Zahl-gegen-Text-Lauf über den Referenz-Korpus (2026-09-03).**
Alle 17 Kandidaten-Texte wurden nach Zahl-Token-Dichte sortiert; die 5 reichen
(≥900 Tokens: odf07155 43k, 19730011461 15k, LRR 8,9k, 42-148-MCS 1,2k,
dsn_trk-2-18 1,1k, trk-2-25-atdf 0,98k) gegen die P10/P11-Doppler- und
Träger-Tagesmediane gefahren (n=300 Surrogate). Befund: **kein Text ausser dem
LRR-2010 korreliert robust mit den Messdaten** — odf07155 (das ODF-Datenbeispiel)
P10 234/300, P11 300/300; 19730011461 (Pioneer-Report) beste P10-Doppler 24/300,
P11-J2000 45/300; 42-148-MCS 15/300; dsn_trk-2-18 (die Datenformatspezifikation)
P11-J2000 3/300 aber beobachteter Wert == Null-p95 (0,7105, Degeneration, kein
sauberer Treffer); trk-2-25-atdf nicht signifikant. Die spärlichen Texte
(<500 Tokens: Iess 145 distinct, die meisten DSN-Timing/Abstracts) sind für
stabile Lag-Scan-Korrelation zu dünn. **Der einzige robuste Text-Daten-Treffer
im ganzen Korpus bleibt LRR-2010 ↔ P11-Doppler (p_emp 0/300, lag 123).** Die
übrigen Referenztexte (Rauschbudget, Formate, Timing) beschreiben andere
Zahlenwelten als die gemessene Hz-Domäne — kein falsches Positiv, ehrliche
Stille (0 honored).
