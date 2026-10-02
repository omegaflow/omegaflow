<!--
  title: Handover — River-Folge 82 (2026-10-02)
  session: River-Folge 82
  class: handover
  date: 2026-10-02
  sha256: 0a0015bfe39db7ef01d32f4cfd07605abecbfd9505a75bc06aa2179879997b63
  status: live
-->
# Handover — River-Folge 82 (2026-10-02)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„kannst du bitte einen benschmark mit allen zur verfügung stehenden sinnvollen stimmen machen …" | 2026-10-01 | Operator (Session, River 76)
„warum nutzt du nur die schlechten stimmen wir brauchen wirklich fähige senior reviewer …" | 2026-10-01 | Operator (Session, River 76)
„… bitte z.ai + arena noch fahren und prüfe welcher z.ai ui chat besser funktioniert" | 2026-10-01 | Operator (Session, River 76)
„future ist schon dabei eine voices agenten lösung zu bauen bitte spreche dich mit ihr ab und bitte a und b" | 2026-10-01 | Operator (Session, River 76)
„braucht es pro?" | 2026-10-01 | Operator (Session, River 77)
„braucht es pro?" (Wiederholung; gemessen: nein) | 2026-10-02 | Operator (Session, River 79)
„braucht es pro und max?" (gemessen: nein) | 2026-10-02 | Operator (Session, River 79)
„glm 5.3 ist auch stark" | 2026-10-02 | Operator (Session, River 79)
„nutze den rat aber auch die ui chats (z.ai, kimi, claude, tryopenly, togetherai)" | 2026-10-02 | Operator (Session, River 79)
„Starte die River-Linie in einem Pass …" | 2026-10-02 | Operator (Session, River 81)
„1 bitte ja" (Galileo-I-tdot bauen) | 2026-10-02 | Operator (Session, River 82)
„2 bitte formales rats blatt" | 2026-10-02 | Operator (Session, River 82)
„3. bitte gebe das bei mountain und mycellium in auftrag ich möchte auch die anderen ephemeriden" | 2026-10-02 | Operator (Session, River 82)
„4 ja ticket ist angelegt" (PII/History-Redaktion) | 2026-10-02 | Operator (Session, River 82)
„5 ja als anonyme frage dispatchen" (TE über externen Steuerparameter) | 2026-10-02 | Operator (Session, River 82)
„2. wer schliesst es?" (Träger des Rats-Blatt-Risses) | 2026-10-02 | Operator (Session, River 82)
„3 ist übergeben" | 2026-10-02 | Operator (Session, River 82)
„5 bitte an den schwarm stellen und pioneer bitte an rat und schwarm mit archive search" | 2026-10-02 | Operator (Session, River 82)

## An mountain

Origin: river folge82 (Auftrag des Operators, 2026-10-02).

- **Viertes Ephemeriden-Haus — PETREL19 (China, Wei Tian) als Verdikt/Register-Punkt.**
  (gemessen 2026-10-02 via `archive_search --verdict`/`--sniff` + IAU Commission X2
  Triennial Report 2021-2024, HTTP 200) Der IAU-Bericht nennt als Hauptbeiträger
  JPL-NASA, IAA-RAS, IMCCE **und China (PETREL19)**. Zugang:
  `https://github.com/TIAN-we/petrel19` (stage-1 206); `fmt_spice/PETREL19_translation.bsp`
  (46 976 000 B), `PETREL19.mk`, `PETREL19_time.bsp`, `PETREL19_rotation.bpc`, dazu
  DE-Format-ASCII (`PETREL19_ASCII.HEADER/PART1/PART2`). Coverage ET 1799-10-13 → 2106-05-05.
  **Riss:** kein `LICENSE` im Repo (HTTP 503) — Lizenz ungemessen; die Aufnahme braucht
  das Verdikt „Lizenz geklärt" oder einen benannten `blocked`/`declined`-Grund. Schritt:
  den `ephemeris_*`-Compiler-Arm (SPICE→`ephemeris_binary`) prüfen, `phi/sources.φ`-Zeile
  (Mountain-Verdikt) setzen, dann `ephemeris_house_gate`/`flyby_anderson_probe` auf das
  vierte Haus erweitern (heute fest `de`/`inpop`/`epm`, `ephemeris_house_gate.rs:297-299`).
  PMOE (Purple Mountain) hat keine gemessene Download-URL (nur Seiten 200); ESA führt kein
  eigenes planetares Haus (ESA-Kerne sind Mission-SPICE, ESOC nutzt JPL).
- **Veraltete Addendum-Zeile.** `docs/paper/flyby-path-2-falsification-metric-addendum.md:125`
  trägt „Galileo I `pending` (tdot_max `pending`)"; das Artefakt trägt seit 2026-10-02
  1,2494 mm/s `rift-excluded` (siehe Blatt). Träger Mountain — die Prosa auf den
  Artefakt-Stand heben (Header-sha nachziehen).
- **Probe-Artefakt vs Haus-Gate — Spalten-Riss.** An der Epoche 2005-08-02 trägt der
  `flyby_anderson_probe` de_inpop 22,485 km / inpop_epm 33,113 km, das Haus-Gate 0,1588 km /
  18,064 km; nur de_epm stimmt überein (17,914 vs 17,916). Der Probe nutzt den 00:00-UTC-Epoch,
  das Haus-Gate den Perigäum-TDB. Offener Punkt: Spalten-Semantik/Fenster im Probe benennen
  (Blatt „Offene Punkte" 1–2). Trägt das Verdikt nicht (tdot_max ist eine Rate), steht aber
  offen.
- **Rats-Blatt-Riss — Trägerschaft (Operator-Frage „wer schließt es?").** Blatt-Riss und
  offene Punkte 1–4 liegen in Mountains Feder (Rätsel Ⅱ Flyby, `survey-raetsel-bestand.md:32`;
  die Ephemeriden-Artefakte und die Prosa). Mountain schließt: Spalten-Semantik/Fenster im
  Probe benennen, die Addendum-Zeile `:125` heben, die Ein-Tages-Konvention ins Artefakt
  tragen.
- **Pioneer-Floor-Falsifikations-Blatt.** Der Rat (2026-10-02) entschied: **nicht** als
  „Anomalie-Neuanalyse" öffnen — das Drei-Haus-Tor ist für Pioneer der falsche Null
  (Haus-Spread ~3,65 µm/s leckt, dominiert nicht; die Pioneer-Systematik sind DSN-Station/
  Uhren + Sonnengravitation). Der öffenbare Rest ist ein Voranmelde-Blatt: Familie
  Station×Ära×Form, WY max-T nach GIC §3.2, Block-Bootstrap 2⁴ d/500 Surrogate/fixierter Seed;
  dreiteiliges Kriterium (Familie geräumt ∧ beide Sonden zeichengleich heliozentrisch ∧
  Drei-Form-Test linear vs ∝t² vs RTG-exp > 2× Null-p95). Owner Mountain; der Lauf ist
  Operator-Wort-gebunden, die Blatt-Vorbereitung ist der autonome Schritt. Ehrlicher Prior:
  Thermik-Konsens hält, wahrscheinlichster Ausgang „keine Entscheidung am Floor".
  **Zahlen-Korrektur:** die zitierte Vorlage „133/218 Hz" trägt der Baum nicht — er trägt
  160–340 Hz Quiet-Day-Streuung und 57,7/105 Hz post-Mask-RMS (`probe-front-dark-matter.md:516,541`).
- **ESA/ESOC ist die zweite, unabhängige Zeugenlinie (gemessen 2026-10-02).** Hoffmann &
  Budnik 2026 („Rosetta Earth Flyby Anomaly Revisited", Proc. 30th ISSFD, arXiv:2609.23482,
  `--verdict` HTTP 206) rechnen Rosettas 2005-Vorbeiflug mit **ESOCs eigenem
  Orbit-Determination-System** auf den originären radiometrischen Tracking-Daten
  (2-Weg-Doppler + Range; ESTRACK NNO1 + DSN Goldstone/Madrid). Ihre Table 1 listet weitere
  Nicht-NASA-Flybys: BepiColombo 2020-04-10 `0`, Solar Orbiter 2021-11-27 `0`, JUICE I
  2024-08-20 `0,10 ± 0,6` mm/s — eine agency-unabhängige Rekonstruktionslinie, nicht NASA-DSN.
  **Riss:** die Rohdaten sind **nicht öffentlich** (kein Data-Availability-Statement; PSA
  Rosetta RSI nur EAR2/2007, EAR1/2005 fehlt; BepiColombo MORE `release 2099`; Solar Orbiter
  kein Radio-Science-Produkt; Hayabusa2/Chang'e 5-T1 kein Tracking). Eigenständige Nachrechnung
  braucht eine ESOC-Datenanfrage (per-Akt-Wort, Operator-Hand) — neben dem bestehenden
  JPL-Entwurf `state/mail/dsn-jpl-odf-request.md`.
- **`rosetta_odf` trägt keine ODF (Name ≠ Implementation).** Das registrierte Rosetta-Bin
  trägt Felder `carrier_level_dbm`/`polar_angle_cycles` — RSI **IFMS Closed-Loop AGC**, keine
  Bodenstations-Doppler/Range; der Formatname `rosetta_odf` ist irreführend. Mountain-Verdikt:
  umbenennen oder als IFMS führen.

## An mycelium

Origin: river folge82 (Auftrag des Operators, 2026-10-02).

- **PETREL19-Manifestation, nachdem Mountain das Verdikt/Register gesetzt hat:** die
  `url`/`origin`/`compiler`-Tag-Zeile in `phi/sources.φ` schreibt Mycelium; danach den
  Kernel-Flatten-Compiler / CDN-Release für die neuen Bins fahren (`ephemeris_bin`-Route).
  Vorher nichts — ohne das Mountain-Verdikt keine Manifestation.
- **Bei der Gelegenheit:** die Release-Namespace-Drift (`ssd.jpl.nasa.gov`-Legacy-Tag vs.
  Produzenten-Tag) ist als systemischer Punkt im Umlauf (`survey-raetsel-bestand.md:62-71`);
  die PETREL19-Zeile von Anfang an unter dem Produzenten-Tag führen.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-anderson-flyby-ephemeridenhaus.md` (`class: sheet`) — das formale
  Rats-Blatt zur Anderson-Flyby-Klasse; Header-sha `8aa321e4…`.

## Offen (aufgeschlüsselt)

### Galileo-I-tdot — gebaut und gemessen
- **Status:** erledigt (git trägt); kein offener Punkt. Fix:
  `tools/measure/src/bin/flyby_anderson_probe.rs` (`FIRST_ROW_REF_S = 86_400.0`), die erste
  Zeile nimmt ihre Haus-Drift-Referenz aus dem eigenen Ein-Tages-Fenster statt `prev=None`;
  Lauf 2026-10-02: tdot_max 1,2494 mm/s → `rift-excluded`. Artefakt
  `data/flyby2/anderson-probe-2026-09-28.json` neu geschrieben.

### Anonyme Methodenfrage (TE über externen Steuerparameter) — Schwarm läuft
- **Status:** wartend | **Bindung:** eigen (Schwarm read-only)
- **Trigger:** Done-Marker `state/stimmen/2026-10-02_te-nichtzeitlich.done`
- **Lage:** (gemessen 2026-10-02) Dispatch nach Futures Muster: Prompt-Datei
  `state/stimmen/2026-10-02_te-nichtzeitlich-frage.txt` (anonymisiert, kein
  state-Datenbyte), Script `state/stimmen/2026-10-02_autolauf-te-nichtzeitlich.sh`,
  detached PID 343096, `opencode run --pure -m <modell> --agent voice … --file <prompt>`,
  10 Modelle sequenziell, Ausgabe `…-<mshort>.md`, Log + `.done`. Die drei
  `stimme.sh`-API-Stimmen (zai/gemini/mistral) antworteten 429/503 (Quota).
- **Blockade:** keine
- **Braucht:** nach dem Done-Marker die `.md` ernten, jede Kernaussage am Baum gegenprüfen,
  trägt-Kandidaten als `## An mountain` (Methodenfrage) falten. Der Pioneer-Schwarm
  (`state/stimmen/2026-10-02_autolauf-pioneer.sh`, PID 352966) wartet bounded auf diesen
  Done-Marker, dann 10 Modelle mit `archive_search`.

### Pioneer-Anomalie — Rat entschieden, Schwarm läuft, Lauf operator-gebunden
- **Status:** operator-gebunden | **Bindung:** eigen/Mountain
- **Trigger:** Operator-Wort (über Futures Operator-Queue)
- **Lage:** (gemessen 2026-10-02) Der Rat hat die Frage gestellt und entschieden: **nicht
  öffnen wie vorgelegt** — das Drei-Haus-Tor ist für Pioneer der falsche Null; der öffenbare
  Rest ist ein Voranmelde-Blatt (Familie Station×Ära×Form, WY max-T, dreiteiliges Kriterium).
  Front C hält: die Anomalie liegt unter dem Reduktions-Floor (Baum-Zahlen 160–340 Hz
  Quiet-Day / 57,7/105 Hz post-Mask-RMS — die Vorlage-Zahl „133/218" ist nicht Baum).
  Schwarm läuft: `state/stimmen/2026-10-02_autolauf-pioneer.sh` (PID 352966), Prompt
  `…_pioneer-frage.txt`, 10 Modelle mit `archive_search`.
- **Blockade:** der Lauf braucht das Operator-Wort; die Blatt-Vorbereitung (Mountain) ist
  autonom
- **Braucht:** Operator-Wort für das Pioneer-Floor-Falsifikations-Atom; bis dahin erntet
  der nächste Pass die Schwarm-`.md` und prüft die Quellen am Baum.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `tools/measure/src/bin/flyby_anderson_probe.rs`
- `docs/blatt/blatt-anderson-flyby-ephemeridenhaus.md`
- `docs/handover/handover-2026-10-02-river-folge82.md`, und `…-folge81.md` → `archiv/` (Move)

## Burn: open 0.0000 · close 0.1735 · cap 0.50 Grund: Galileo-Fix + Rats-Blatt + Ephemeriden-Auftrag + Schwarm-Dispatch (gemessen `session_burn`, River-Session `$0.1735`; Gesamt $4.4019 → $6.0773, Parallel-Linien teilen den Total)
