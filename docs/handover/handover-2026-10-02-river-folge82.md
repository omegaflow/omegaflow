<!--
  title: Handover — River-Folge 82 (2026-10-02)
  session: River-Folge 82
  class: handover
  date: 2026-10-02
  sha256: 1a02a5a1be8b94110093f6cb635c621506abb088654cec45cf31fda8ce8a86d7
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
  trägt-Kandidaten als `## An mountain` (Methodenfrage) falten.

### Pioneer-Anomalie — Nachrechnung (Frage des Operators)
- **Status:** offen (Operator-Frage, Rat noch nicht gerufen) | **Bindung:** eigen/Rat
- **Trigger:** Rats-Wort / Operator-Wort
- **Lage:** (gemessen 2026-10-02, Baum) Front C hat die Anomalie als unter dem
  Reduktionsboden (133/218 Hz) liegend gemessen; Pioneer 10/11 NAVIO geerntet;
  `pioneer-11-ODF-Manifest` (`56866a3d9`); die Anderson-Ephemeriden (5 NAIF-Cruise-Ports +
  6 Horizons-Arcs) und das Drei-Haus-Tor existieren. Offen: ob Turyshevs thermische
  Erklärung vollständig ist oder ein Rest bleibt, wenn man seine Daten mit
  Westfall-Young + Drei-Haus-Tor nachrechnet.
- **Blockade:** das ist eine Rat-/Architektur-Frage (neues Atom), kein Sofort-Bau
- **Braucht:** Operator-Wort, den Rat zu rufen; dann eigenes Rats-Blatt + Probe.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `tools/measure/src/bin/flyby_anderson_probe.rs`
- `docs/blatt/blatt-anderson-flyby-ephemeridenhaus.md`
- `docs/handover/handover-2026-10-02-river-folge82.md`, und `…-folge81.md` → `archiv/` (Move)

## Burn: open 0.0000 · close 0.1735 · cap 0.50 Grund: Galileo-Fix + Rats-Blatt + Ephemeriden-Auftrag + Schwarm-Dispatch (gemessen `session_burn`, River-Session `$0.1735`; Gesamt $4.4019 → $6.0773, Parallel-Linien teilen den Total)
