<!--
  title: Handover — Mountain-Folge 215 (Stand 2026-09-30)
  session: Mountain-Folge 215
  class: handover
  date: 2026-09-30
  sha256: 20e1c3773f496b92999d06b850f1dda7f3baf53ebb5b87f1aebb3ac95c5aefcc
  status: live
-->
# Handover — Mountain-Folge 215 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`; Stand der Runde am
`d8c0d8340`, vor dem Mountain-Commit `8fdb361b3`). Dieses Atom: die „Ganz einfach"-Ausführung
des Operators aufgenommen und gegen den Baum gemessen — die drei Ephemeriden-Häuser
(DE/INPOP/EPM) als `ephemeris_house_gate` gebaut und lokal gemessen, den 16-km-Riß als
Frame-Translation aufgeklärt; das Doppler-Zeugen-Delta der Ausführung geprüft und als
`## An mycelium`-Zeile weitergereicht; die Anderson-Nachrechnung als Auftrag registriert.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„jeder Punkt trägt eine Empfehlung; wartende Linien erhalten eine bevorzugte Abarbeitungsbitte" | 2026-09-29 | Operator (Mountain 204)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„braucht es wirklich pro?" — pro nur mit benanntem Hart-Atom oder gemessener flash-Fehllage | 2026-09-30 | Operator (Session, Mountain 211)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„arbeite deine Liste bis zur Kante ab" — jeder eigene Punkt bis zur Kante, nichts Machbares liegen lassen | 2026-09-30 | Operator (Session, Mountain 212)
„verschleppen und nicht eigenes ist verboten" — Linienliste nur `eigen`, jeder Punkt im Atom bis zur Kante | 2026-09-30 | Operator (Session, Mountain 213)
„braucht es pro?" — Routine-Source-Port trägt flash; pro nur mit benanntem Hart-Atom/gemessener flash-Fehllage | 2026-09-30 | Operator (Session, Mountain 213)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-09-30 | Operator (Session, Mountain 215)
„nimm das bitte auf — A = A" (die „Ganz einfach"-Ausführung; wortgetreu in `state/operator-gespraeche/2026-09-30-mountain.md`) | 2026-09-30 | Operator (Session, Mountain 215)
„Er soll das Commit-Wort bekommen — und dann den Riß aufklären." | 2026-09-30 | Operator (Session, Mountain 215)

## Haus — Mountain (Stand 2026-09-30)

Diese Übergabe **ist** das Haus: jeder offene Punkt, jedes Verdikt, jeder Parser steht hier
mit Zustand, auch um 3 Uhr nachts.

- **Die vier Orte:** `omegaflow` = `~/projects/omegaflow` + privates Schwester-Repo `state/`
  (`omegaflow/personal`); `omegaflow-legacy` = `archive-root/omegaflow-legacy`; `temp` =
  `/tmp/opencode`; `archive` = `archive-root`.
- **Mountain-Fundstellen:** `phi/`, `src/archivar`, `src/mathematikerin`, `src/gate`,
  `tools/harvest`, `tools/measure`, `tools/register`, `docs/specs`, `docs/surveys`,
  `state/zustand`, `state/mountain`.
- **Linien-Preset (privat):** `state/mountain/archive-search-preset.txt`; `state/` immer mit
  `archive_search --root state`.
- **Verwahrt (Operator-Wort Mountain-185):** `state/mountain-185-orphan-doc-nachzug.patch` —
  der Orphan-Doc-Nachzug-Diff bleibt nach Operator-Entscheid „C" unangewendet liegen („kein
  Träger, keine Freigabe, kein Nachzug-Commit"); nicht anwenden, nicht löschen.

## Offen (aufgeschlüsselt)

### Ephemeriden generic (new_horizons/voyager1/voyager2) — 976-B-Placeholder
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** —
- **Lage:** die drei `url`-Zeilen `phi/sources.φ:15747/15950/15957` sind auf `ssd.jpl.nasa.gov-horizons` gesetzt (Mycelium-214, `7e0cad67d`; `--verdict` 200, gemessen 2026-09-30); die `-ephemeris`-Zeilen der übrigen Körper bleiben gültig (86 Zeilen, z. B. nereid 206). Die generischen Assets bleiben aber 976-B-Placeholder; der tragende Kernel liegt in `_long` (78 928 / 255 888 / 211 088 B) und `_daily` (gemessen via GH-Releases-API).
- **Blockade:** der generische `ephemeris_compiler` liefert für NH/V1/V2 nur einen 976-B-Placeholder (das SPK fehlt im Standardsatz); den Kernel trägt der `horizons_compiler --long`-Lauf. Der Placeholder ist **nicht gemessen** (`pending`, 0 honored) — das Verdikt ist Mountain-Feder.
- **Braucht:** Mycelium setzt für die drei die zugelassene Quelle auf den `_long`-Lauf (`url`/`compiler`) oder verwirft die generische Zeile zugunsten von `ephemeris_*_long`; danach die generischen Placeholder-Zeilen `descoped` mit diesem Befund.

### Drei-Häuser-Ephemeriden — der 16-km-Riß ist eine Frame-Translation (aufgeklärt)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** `tools/measure/src/bin/ephemeris_house_gate.rs` (mit `--body` und Differenzvektoren) gebaut (`cargo build -p omegaflow-measure --bin ephemeris_house_gate` 0 Warnungen) + Workflow `ephemeris-house-gate.yml` (dispatcht `36748072166`). Am JUICE-Perigäum (±2 d, 1 h) (gemessen 2026-09-30 via `./target/debug/ephemeris_house_gate`): Δ DE441↔INPOP19a (Erde) 0.0277 km; Δ INPOP↔EPM Erde 15.96 / Sonne 16.10 / Mond 15.94 / Neptun 8123 km. Die Differenzvektoren INPOP−EPM sind für **Sonne** (1.0, 16.0, 1.4), **Erde** (1.0, 15.8, 1.3) und **Mond** (1.0, 15.9, 1.3) km **gleich** → eine konstante ~16-km-Translation (Frame-/Origin-Differenz), kein Erdfehler; nach Abzug der Sonnen-Translation bleibt für die Erde ~0.19 km. Frühere Fassung dieses Punkts (Riß-Ursache ungemessen) ist damit geschlossen.
- **Blockade:** keine.
- **Braucht:** — descoped mit diesem Befund (gemessen 2026-09-30 via `ephemeris_house_gate`): DE und INPOP teilen den SSB-Ursprung auf 28 m (JUICE-Epoche), EPM trägt einen zeitabhängigen Origin-Offset (~16–20 km, 1990 → 2026 fallend, in jeder Epoche eine Translation); die EPM-SSB-Definition bleibt eine Register-`pending`-Notiz, kein eigener Bau. Das Werkzeug ist generisch (`--body`, `--epoch-ymd`) — es trägt jeden Körper und jeden Zeitpunkt, nicht nur das JUICE-Perigäum.

### Anderson-Nachrechnung — hält die Flyby-Anomalie gegen Familien-Schwelle + Drei-Haus-Tor? (Auftrag)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Rats-Entscheid (Operator-Wort: „den Rat entscheiden lassen, dann laufen")
- **Lage:** Anderson-Ephemeriden (`dfd2a4a19`), Format-Familie (`e1828a90f`) und das Drei-Haus-Tor (`ephemeris_house_gate`) liegen in einem Baum; `flyby_anderson_probe` (`tools/measure/src/bin/flyby_anderson_probe.rs`) trägt die Residuen gegen `data/flyby2/anderson_residuals.tsv` (gemessen 2026-09-30 via `open_points_check`: lokal nicht vorhanden). Die Frage: stirbt die Anomalie unter der Familien-Schwelle (wie die Alfvén-Kaskade unter der Phasen-Null), oder hält sie gegen Werkzeuge, die es 2008 nicht gab? Beides ist ein Ergebnis. Haus-Scan an allen sechs Erdvorbeiflug-Epochen (gemessen 2026-09-30 via `ephemeris_house_gate --epoch-ymd`): Δ DE↔INPOP ≤ **0.193 km** (Maximum Galileo I 1990 0.193; Galileo II 1992 0.031, NEAR 1998 0.060, Cassini 1999 0.042, Rosetta I 2005 0.101, MESSENGER 2005 0.162, JUICE 2026 0.028 km); Δ INPOP↔EPM monoton fallend 20.09 → 15.96 km (1990 → 2026), Chord-Slope −115 m/yr, lokal −138 (1990–2005) / −100 m/yr (2005–2026). Translation in jeder Epoche (Sonne/Mond/Erde-Vektor gleich, 1990 **und** 2026 geprüft), der Translationsvektor selbst dreht/schrumpft: 1990 (8.5, 17.7, 3.5) → 2026 (1.0, 15.8, 1.3) km. Vorfrage-Hypothese (nicht gemessen, erster Prüfschritt vor dem Lauf): kürzt sich eine Frame-Translation in differentiellen Doppler-Messungen heraus, kann der Haus-Riß die mm/s-Anomalie nicht erzeugen.
- **Blockade:** das Residuen-Input fehlt lokal; der Rat hat die Frage noch nicht entschieden.
- **Braucht:** den Rat die Frage entscheiden lassen (Auftrag registriert), dann `flyby_anderson_probe` gegen die Anderson-Residuen (`anderson_residuals.tsv`) laufen lassen und die Haus-Robustheit (DE/INPOP/EPM, `ephemeris_house_gate`) daneben legen.

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` — σ-Metrik-Kette (3 Marker); Trigger JUICE In-Situ /
  Δ publiziert, `flyby_ephemeris_gate` (CI).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — Holdings-Inventur; Schritt 2
  gemessen (Holdings + Repo-`data`), die Dedup-Entscheidung in `## An future`.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` — Relevanz-Erstpass; die
  Pending-Einträge sind im `dead_sources.φ`-Register disponiert, die Prosa bleibt als
  datierte Messung.
- `docs/surveys/survey-raetsel-bestand.md` — zwölf Nadeln + Blätter + Kuprat, stehende
  Messreihe; jede Zelle mit `file:line`/`pending`.

## Register-Träger (eigene)

- `phi/harvest.φ` `format hapi_csv` (DAS2 Iowa, Asset `das2_iowa_…`) — `asset fehlt`;
  Workflow `das2-iowa-cdn.yml` gebaut (Mycelium-214), dispatcht `36741690825` — `present`
  erst nach grünem Lauf.
- `phi/harvest.φ` `format pds3_img` (Chandrayaan-1 Mini-RF, Asset `pds3_img_…`) — `asset fehlt`;
  Workflow `pds3-img-cdn.yml` rot (M3-`.HDR` HTTP 403, `pds-imaging.jpl.nasa.gov`) — Route
  messen.
- `phi/harvest.φ` `format pds3_fixed_width_darts` (Akatsuki VCO-rs) — `asset fehlt`; Workflow
  `pds3-fixed-width-darts-cdn.yml` gebaut (Mycelium-214), dispatcht `36741697524`.
- `phi/harvest.φ` `format pds4_binary` (ExoMars TGO ACS / PSA) — `asset fehlt`; Workflow läuft.

## An mycelium

Origin: mountain folge215 (Antwort auf die Doppler-Zeugen-Ausführung; die Blöcke folge214 sind gefaltet).

- **Doppler-Zeugen-Delta — gemessen 2026-09-30 (via `sgrep` in `phi/`):** Die Ausführung behauptet, ESA-DSN-Doppler (JUICE, Rosetta, Mars Express) sei „vorher nicht systematisch gesucht". Gemessen **falsch** für Rosetta/Mars Express: `rosetta_odf` trägt drei Assets (`phi/sources.φ:8948/8958/8968`, Tag `archives.esac.esa.int`), `mars_express_odf` trägt (`:8940`), `ephemeris_juice` trägt (`:15592`). Neu als Quelle offen sind die ODF/Doppler-Linien der übrigen Anderson-Sonden (NEAR trägt weder ODF noch SPK, laut folge197) und die Tracking-Formate von ISRO/CNSA/JAXA; `Tianwen-1` steht `blocked_sources.φ:389/425/440` als released (Arm steht, Sample fehlt).
- **„geerntet" ist gemessen überzeichnet:** Venera 15/16-Radiometrie ist registriert (`phi/sources.φ:16020-16037`), Chang'e-1/-2 MRM-Assets `present` (`blocked_sources.φ:420-421`), `hinet_win32_compiler` existiert (`sources.φ:9350`) — aber Chang'e/CNSA bleiben `released` mit Download-Duty und Tianwen-1 RoPeR/MoRIC haben noch kein Sample. Kein Verdikt ohne Messung — die Register-Zeilen sind der Stand.

## An river

Origin: mountain folge215 (Antwort auf river-folge73; + Operator-Ausführung „denk groß", 2026-09-30 — die Wissenschaftsseite der Vision liegt bei dir).

- **σ-Asset `dr3_stars.bin`:** liegt im Baum — `tap_compiler.rs:403` `STAR_BIN_STRIDE=56`
  + drei `sigma_slot` (`sig_plx`/`sig_pmra`/`sig_pmdec`) (gemessen 2026-09-30 via `sgrep`).
  Der `--star-bin`-Erweiterungsschritt ist erledigt; `gaia-cdn` re-manifestieren + `--sniff`
  + σ-Zensus sind deine Seite.
- **`usgs_comcat_m45.bin`:** committet (`d0c070737`); Reader `src/archivar/usgs_comcat.rs`,
  `field`/`ttl`/`frame` stehen (`sources.φ:11179`). Der Quake-Kanal kann verdrahtet werden.
- **`tao_wnd_zonal.csv`:** Riss benannt, nicht geglättet — keine Mountain-Aktion.
- **Drei-Haus-Tor ist generisch (neu, gemessen, commit `781ea20f4`):** `tools/measure/src/bin/ephemeris_house_gate.rs` nimmt jetzt `--body <körper>` und `--epoch-ymd <YYYY-MM-DD>` — nicht mehr JUICE-fest. Gemessen 2026-09-30 (`./target/debug/ephemeris_house_gate`): über die sechs Anderson-Erdvorbeiflüge (Galileo I/II 1990/1992, NEAR 1998, Cassini 1999, Rosetta I 2005, MESSENGER 2005) stimmen DE441 und INPOP19a auf **≤ 0.193 km** überein (Maximum Galileo I 1990); der EPM-Origin-Offset fällt monoton **20.09 → 15.96 km** (1990 → 2026, Chord-Slope −115 m/yr, lokal −138/−100 m/yr), ist **in jeder Epoche eine Translation** (Sonne/Mond/Erde-Vektor gleich, 1990 und 2026 geprüft) und der Translationsvektor selbst **dreht/schrumpft** (1990 (8.5, 17.7, 3.5) → 2026 (1.0, 15.8, 1.3) km) — kein skalarer „Konvergenz`-Drift. **Deine Seite:** die Finsternis-als-Uhr (der 3,05-s-Riß bei INPOP) um das dritte Haus legen — ein Lauf mit dem Finsternis-Epoche (`--epoch-ymd <datum>`); ebenso Okkultationen (Quaoar, Arche) und die wachsende Sondenflotte. Der Vektor-Output (`vec INPOP-EPM … km`) trennt Frame-Translation von physischer Differenz; wo die Häuser auseinanderlaufen, ist *das* der Fund („die Unsicherheit über 'wo war die Erde' ist größer als gedacht").
- **Vision „das gekoppelte Sonne-Erde-System als ein Blatt" (Operator-Ausführung „denk groß"):** das ENSO-Blatt (Wind, Erdbeben, Bz, SST) zu einem vollständigen Sonne-Erde-Blatt erweitern — Sonne (RTSW, GOES, AIA, EVE), Magnetfeld (154 INTERMAGNET-Stationen), Ozean (SST, Argos, Pegel), Atmosphäre (Open-Meteo, ERA5), Boden (Erdbeben USGS/ISC, Echo-Tiefe) in **einer** Rechnung mit Familien-Schwelle und konditionaler TE. Die Frage, die niemand stellen kann: „Welcher Kanal treibt das System — und bei welcher Zeitskala?" als vollständiges gekoppeltes Diagramm mit dominantem Pfad pro Zeitskala. *Erste Messung:* die Kanal-Register-Zeilen gegen `phi/sources.φ` halten und das ENSO-Blatt als Träger nehmen (`docs/blatt/`); Träger-Linie River.
- **Vision „Frühwarnsystem mit Präregistrierung" (Operator-Ausführung „denk groß"):** Bz→dB/dt bleibt der dominante Treiber (GIC-Paper); vor dem nächsten Sonnensturm eine **versiegelte** Vorhersage (Bz-Schwelle → dB/dt an Station X → Verzögerung Z) mit Fehlschlag-Kriterium, währenddessen Echtzeitmessung (RTSW + 154 Stationen), danach Benotung. *Lage:* der GIC-Anschluss steht; die Präregistrierungs-Form ist dieselbe wie beim JUICE-Siegel. *Braucht:* die Vorhersage-Zelle und den Trigger (erster Sturm nach dem Siegel) benennen — River-Feder (Wissenschaft), Future-Feder (Dienst, siehe dort).
- **Vision „die Weberin fertig weben" (Operator-Ausführung „denk groß"; Architektur → Rats-Entscheid):** jeder Oszillator (1655+), jeder Körper-Anker (424+), jede Kraft (9 Medien) an jedem Ort (ICRS) zu jeder Zeit (TDB) in **einem** Feld; die Punktwolke im Browser zeigt das gesamte gekoppelte System, nicht nur Sterne. *Lage:* `src/weberin.rs` trägt `BodyLine = {Spk, Dastcom, Mpc, Inpop, Epm}` und den N-body-Stack; der Datenkontrakt steht (`docs/concepts/archivar-mathematikerin.md`). *Blockade:* der Umfang (220 Mio Fäden, Echtzeit im Browser/Handy) ist eine Architektur-Frage, kein benannter Bau — Ressourcen/Reihenfolge gehören vor den Rat. *Braucht:* die Rats-Frage stellen („was ist der nächste webende Faden, der das System sichtbar macht, ohne die Membran zu überlasten?") — nicht Mountain-Feder; Mountain hält Datenkontrakt und Register.

## An future (Operator-Queue, private)

Origin: mountain folge215 (Operator-Ausführung „denk groß", 2026-09-30; die früheren Einträge aus folge212/214 bleiben).

- **Vision „Frühwarn-Dienst als Service" (Operator-Ausführung „denk groß"):** der Übergang von Wissenschaft zu Dienstleistung — Raumwetter-Warnung für Netzbetreiber. Der Kern bleibt offen/nicht-kommerziell; der Service liegt **darüber**. Das ist eine Geld-/Korrespondenz-Sache → Operator-Queue, einfache Sprache. *Lage:* GIC-Paper gehärtet (fünf Verschärfungen, Magnetfeld als Hauptverdächtiger in 2/3), RTSW + 154 Stationen laufen. *Frage an den Operator:* soll ein Service-Pfad (Warnung für Netzbetreiber) als Förder-/Markt-Möglichkeit aufgenommen werden — der Maschinen-Akt (Anfragen, Konten, Verträge) bleibt ausnahmslos deine Hand. *Bei Ja:* Future präpariert die Kantenzeile (Adressat, Leistung, Preismodell) bis zur Ausführungsgrenze; *bei Nein:* die Vision bleibt als Richtung im Rat, kein Service.

- **Kuprat 5. Ader — beantwortet (future-160, gefaltet):** kein offenes Operator-Wort; die 13
  NSE-Läufe sind privat gesichert + kompiliert (`data/lab_a.data/SAMPLE_NSE_[RETRACTED-SAMPLE]/`);
  NSE = `substance`-Zeuge ohne Wire-Arm, kein CDN, keine `sources.φ`; Redistribution LOCK.
- (aus folge212:) **ODF-Flyby-Fenster** (DSN/JPL-Anfrage?); **Sonden-Download-Session**
  (Operator-Browser, fünf `released`-Konten); **opencode-Config Secrets** (Env-Export + Rotation).

- **Holdings-Dedup (Frage, einfache Sprache):** *Lage* — die Byte-Messung (Schritt 2) ist
  gefahren: `archive/knowledge` 29 Gi, `archive-state` 9,7 Gi, Repo-`data` 77 Gi, `cache`
  10 Gi; in Gruppen liegen 4,34 GiB byte-identische Duplikate, reclaimable sind Repo-`data`-
  Zwillinge zwischen zwei Netloc-Ordnern (`wind_orbit.bin` = `omegaflow_series_wind_orbit.bin`,
  93,5 MB; `dr3_stars.bin` = `dr3_stars_stable.bin`, 75 MB) und ~2,2 GiB interner Scratch
  (`opencode-tmp-2026-09-01`, opencode `undo-snapshot`). *Frage* — sollen die gemessenen
  Duplikate gelöscht werden (je ein byte-identischer Zwilling bleibt als Nachbau)? *Bei Ja:*
  Mountain löscht die gemessenen Zwillinge und trägt die Unique-Bytes ins CDN-/`data/<netloc>/`-
  Ziel; *bei Nein:* die Kopien bleiben liegen, die Inventur vermerkt es.

- **Registry-first (Schritt 3, Antwort auf future-161) — je Asset gemessen:**
  - `who_flunet` — Verdikt steht: `decline health-stats` (`declined_sources.φ:5113`); keine
    `sources.φ`-Zeile, die lokale Datei (119,5 MB) gehört ins archive-root.
  - `nbp_Lmon` — Verdikt steht: `decline model-forecast` (`declined_sources.φ:49/:53`); keine
    `sources.φ`-Zeile.
  - `pioneer11_residuum` — ist ein Measure-Probe (`tools/measure/src/bin/pioneer11_odf_residuum.rs`),
    kein Asset → keine `sources.φ`-Zeile.
  - `pioneer11_odf` — Harvest-Compiler `pioneer11_odf_compiler.rs` ohne Register-Zeile;
    `pioneer10_odf` ist registriert (`sources.φ:10281`) → analog zulässig, Mycelium setzt
    `url`/`ttl` (`format pioneer11_odf`).
  - `pioneer10_telemetry` — Harvest-Compiler `pioneer_telemetry_compiler.rs` (Pioneer-Radio-
    telemetrie, em) ohne Register-Zeile → zulässig, Mycelium setzt `url`/`ttl`.
  - WMM/Kernel-Satz — SPICE-Kernel/Modell (Infrastruktur, kein Feld am Punkt) → keine
    `sources.φ`-Zeile, archive-root.

## Burn: open 0.0000 · close 0.1909 · cap 0.25 · Grund: operator-getriebenes Mehr-Nachrichten-Atom (Ausführung + Riß-Aufklärung + Vision-Relais), Commit-Wort gegeben — die Mehrkosten stammen direkt aus den drei langen Operator-Aufträgen (session_burn; Runde 6 Sessions 0.7366)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
