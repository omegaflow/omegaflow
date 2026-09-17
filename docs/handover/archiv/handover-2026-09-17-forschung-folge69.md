<!--
  title: Handover — Forschung-Folge 69 (Stand 2026-09-17)
  session: Forschung-Folge 69
  class: handover
  date: 2026-09-17
  sha256: 7ae7dad1e029107c98ba025078fbcd88a9a984cbfc4666bf3d1e675b845ea42b
  status: live
-->
# Handover — Forschung-Folge 69 (2026-09-17)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (automatisch, keine Auswahl)

- **Postfach** — keine neue Zeile an die Forschung-Linie; `post.md` trägt nur die
  `An bau`/`An entscheid`-Zeilen fremder Linien. Kein neuer externer Eingang
  (`state/mail/mail_ledger.φ`, letzter Eingang 2026-09-15/16).
- **CI-Status @Session-Start `dbe647d1`** (`ci_manage list`): die `harvest`-Läufe
  der letzten zwei Stunden trugen nur `ulysses_atdf`/`rosetta_odf`; **kein Lauf
  trug `lro_trk`/`goes16_abi`/`ulysses_atdf_x`** — der Auto-Dispatcher für
  `dbe647d1` steht noch (`harvest-dispatch 35273075476` pending,
  `auto-dispatch 35273075509` queued). Die `ulysses_atdf`-Läufe `35270658104`/
  `35270996845` **failure** am Schritt „Second witness":
  `issue already open: harvest: ulysses_atdf shard is incomplete on the CDN after the compile`.
  Census `35266366575` cancelled. `pii-exposure`/`paper-check` unverändert.
- **Zustand-Ledger** — `docs/zustand/external-state.md` fremd-modifiziert
  (uncommittet) — nicht angefasst.

## Ulysses X-Band-Recovery — gebaut, Manifestation + Validierung offen

- X-Band-Kette gebaut (Rat-Votum **B**: X als eigene Komponente, kein
  Wire-/Stride-Change). Gemessene Konstanten: `X_BAND_RATIO = 96·880/221`,
  X-Downlink-Träger 8408.209876 MHz (Kanal 9), S-Uplink 2111.607 MHz, X-Uplink
  existierte nicht; Formel strukturgleich S (`fsky = RATIO·ref_hz − sdoppler·(drate − 1e6)`),
  `ref_hz` S-skaliert. Quellen: PDS3 `ULY_SCE_INST.CAT`, DSN 810-005 Rev. E
  Mod. 201, Moyer Tab. 13-1. `reduce_uly_skyfreq → struct UlySkyFreq {sband, xband}`,
  per-Band-Median/Ref-Fenster/Halbweite; `COMP_ULY_SKYFREQ_X = 2`,
  `ulysses_sky_frequency_x_hz`; Compiler zweite Shard-Familie `ulysses_atdf_x`;
  `extract.rs`-Dispatch + `main_flow.rs`-Serienliste; `phi/harvest.φ` +
  `phi/sources.φ` X-Block (`asset fehlt`). `cargo check` 0/0 (core, `--tests`,
  `omegaflow-harvest`). (Schritt: `ulysses_atdf_x`-Harvest dispatchen/abwarten,
  dann sha256/Größe messen + Blöcke auf present; `grind-flash`.)
- **X-Validierung `wartend`** — X-Ref-Fenster `[2197e4, 2200e4]` und
  `X_FSKY_MED_HALF_WIDTH` (= S-Halbweite × X/S-Ratio) sind aus gemessenem
  S-Wert + gemessener Ratio abgeleitet, noch nicht an echten X-Records gemessen.
  Trigger = erster `ulysses_atdf_x`-Lauf-Abschluss. (Schritt: `ci_manage view <id>`
  + Log-Census `n_xband_out`/`med_rejected`/`ref_rejected` deuten.)

## Ulysses S-Band-Manifestation — blockiert am CDN-Completeness-Check

- Der `ulysses_atdf`-S-Band-Lauf scheitert am Schritt „Second witness":
  `issue already open: harvest: ulysses_atdf shard is incomplete on the CDN after the compile`
  (Läufe `35270658104`, `35270996845`). Die Workflow-Syntax-Ursache (`fromJSON`)
  ist mit `dc9291ad` behoben; der Fehler ist jetzt inhaltlich. (Schritt:
  `ci_manage view <id>` + Job-Log lesen, welcher Shard fehlt; dann `grind-flash`.)

## LRO utF — gebaut, Manifestation `wartend`

- Gemessen (grind-flash 2026-09-17): **kein** `harvest`-Lauf hat `lro_trk`
  getragen; Auto-Dispatcher `dbe647d1` pending. Asset fehlt (Release
  `imbrium.mit.edu` not found; CDN-URL `.../lro_trk.bin` 404); `phi/harvest.φ`
  `asset fehlt`, Register und Messung stimmen überein. Trigger = Abschluss des
  Auto-Dispatch. (Schritt: bei Lauf-Abschluss sha256/Größe messen,
  `phi/harvest.φ` + `phi/sources.φ`-Block auf present.)

## GOES-16 ABI — gebaut, Manifestation `wartend`

- Wie LRO: kein `harvest`-Lauf hat `goes16_abi` getragen; Auto-Dispatcher pending;
  Asset fehlt (Release `noaa-goes16.s3.amazonaws.com` not found; CDN-URL 404).
  Trigger = Auto-Dispatch. (Schritt: bei Lauf-Abschluss sha256/Größe messen,
  Blöcke auf present.)

## §4 fsky-Census — `wartend`

- Lauf `35266366575` (Kreuz-Rang + r²-LS @`98fbaeb1`) **cancelled**, kein
  Nachfolger. Trigger = neuer Lauf-Abschluss. (Schritt: bei neuem Lauf einmal
  `ci_manage view <id>` + `gh run download <id>` → `pioneer-cell-census.txt`;
  Kreuz-Rang + r²-Peak gegen die Zwei-Arm-Frage deuten.)

## BepiColombo — `wartend`

- Radio-Science-Bundle `bc_mpo_more/` lebt, `data/` 404 (Cruise). Trigger =
  `data/`-Manifestation. (Schritt: bei Manifestation PDS4-TNF/ODF-Parser nach dem
  tatsächlichen Datentyp.)

## CDN-Concurrency Follow-up — `wartend`

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht;
  `cancel-in-progress: false` bleibt. Trigger = erfolgreicher `mro_odf`-Lauf.
  (Schritt: auf die erwarteten Shard-Namen härten, messbar nach dem Lauf.)

## WWLLN — Lizenz (`operator-gebunden`)

- Thunder-Hour (Zenodo-Spiegel `records/10725446`, CC BY-SA 4.0; Quell-Lizenz
  „research (non-commercial) use"). Post-Zeile `An entscheid` steht. (Schritt: bei
  Operator-Wort den Zenodo-Spiegel in `phi/sources.φ` registrieren.)

## NSE/Haug — Rohdaten zugesagt (`wartend`)

- Keller-Antwort gesendet (17.09.). Trigger = Dateieingang. (Schritt: bei Eingang
  `nse_haug_trisp`-Quelle + Compiler + `sources.φ`; 0-Kanon: kein Asset ohne Datei.)

## Legacy-Konzepte (`operator-gebunden`, hintenangestellt)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als 4.,
  Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt: bei
  Wiederaufnahme den Rat-Erster-Atom bauen — `tools/measure/src/bin/silence_map_probe.rs`.)

## Paper / Präregistrierung (`termin`)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- **Ulysses X-Band-Recovery** — `research-max` (Konstanten-Recherche, Primärquellen)
  + `council` (Identitäts-Votum B) + `grind-pro` (Bau, `cargo check` 0/0, Rücklese).
  Klasse „novel parser construction / X-Recovery" ist kein registrierter Sieger;
  der Lauf ist der erste Eintrag. Lehre bestätigt: die Rücklese-Beweispflicht
  (`sread`/`glob`/`cargo check`) trägt den Bau.
- **Manifestations-Messung** (grind-flash): Routine-Klasse, flash-first, kein
  Doppellauf (der gemessene Routine-Klassen-Sieger `grind-flash` ist registriert).

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Hunks dieser Session**: `src/archivar/atdf.rs` (X-Konstanten,
  `UlySkyFreq`, `parse_uly_series_x`, per-Band-Reduktion, Tests),
  `src/archivar/extract.rs` (Dispatch `ulysses_atdf_x`),
  `src/archivar/main_flow.rs` (Serienliste),
  `tools/harvest/src/bin/ulysses_atdf_compiler.rs` (zweite Shard-Familie),
  `phi/harvest.φ` (X-Block), `phi/sources.φ` (X-Zeile),
  `phi/pipeline/ledger.φ` (Ulysses-Note), dieses Handover
  (+ archiviertes `handover-2026-09-17-forschung-folge68.md`).
- **Baseline:** HEAD ist während der Session von `dbe647d1` auf **`e01d9830`**
  (ernte-Linie) gewandert — der ernte-Commit hat `parse.rs`/`port.rs`/`tests.rs`/
  `sources-v2-spec`/`ernte-folge74` gelandet und **`ulysses_atdf`/`lro_trk`/
  `goes16_abi` in `phi/sources.φ` für die CDN-Manifestation registriert**.
  `origin/main` (`5fb16b35`) ist Vorfahr von HEAD.
- **Der `band_field`-Fix** (`TKFORM` 10→11 + Test
  `uly_band_field_is_downlink_band_not_station`) ist **nicht** in HEAD — er liegt
  uncommittet im Arbeitsbaum und ist **Voraussetzung der X-Kette** (ohne
  `TKFORM`-Item 11 läse die Reduktion `STATION` statt `DOWNLINK_BAND`); er gehört
  in den eigenen `atdf.rs`-Commit.
- Fremde uncommittete Arbeit (nicht anfassen): `docs/concepts/tools-map.md`,
  `tools/utils/src/bin/archive_search*.rs` (+ untracked `arxiv_src.rs`),
  `docs/SOURCE_PORT.md`, `docs/handover/post.md`, `docs/zustand/external-state.md`,
  die Worktree-Deletionen der `archiv/`-Renames (entscheid-folge24,
  forschung-folge44, forschung-folge51). Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
