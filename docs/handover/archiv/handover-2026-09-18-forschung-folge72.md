<!--
  title: Handover — Forschung-Folge 72 (Stand 2026-09-18)
  session: Forschung-Folge 72
  class: handover
  date: 2026-09-18
  sha256: 637ed15eff0502ecc959e4666d1992ef9e49cd6b025c07e961693373204fca2e
  status: live
-->
# Handover — Forschung-Folge 72 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18)

- **HEAD** `985bee3c` == `origin/main` (`entscheid: refresh the state ledger …
  handover 42`; die Session committete + pushte während dieses Atoms, Base
  `e0c27635`). Fremd uncommittet (nicht angefasst): `opencode.json`, die drei
  `handover-2026-09-16-*`-Renames und eine laufende Register-Prosa-Relaxation in
  `phi/{blocked,dead,declined}_sources.φ`, `footprints.φ`, `nrs_stations.φ`,
  `supermag_stations.φ`, `witnesses.φ`, `reports/*.φ` (+ fremde Hunks in
  `sources.φ`/`harvest.φ`). Safety-Net `refs/safety/1789708439`.
- **CI** (`ci_manage list`): Ulysses-Re-Dispatch `harvest` `35296127974` (S) +
  `35296130236` (X) success @`1a129cd2` (= der Paarungs-Fix-Commit, Vorfahr von
  HEAD). `xp-pilot-cdn` `35305640254` failure, `release-build` `35302966400` +
  `ci-check` `35302986980` failure — nicht Forschung.
- **Postfach** — leer; die `An forschung`-Zeile (S-Band-Befund, durch den Fix
  überholt) gefaltet + gelöscht.

## Kein abarbeitbarer undatierter Punkt

- Nach dem Schließen der Ulysses-Re-Manifestation und der X-Validierung trägt
  die Forschung-Linie nur noch `wartend`/`operator-gebunden`/`termin`-Punkte.
  Die nächste Session sagt das im Planungs-Pass und arbeitet keinen erfundenen
  Punkt.

## Gemessen — Ulysses S/X (geschlossen, keine Auswahl)

- Lauf `35296127974` (S-Compiler erzeugt beide Bänder): `ulysses_atdf.bin`
  35187160 B sha256 `5aeca72e…b4869be9a` (S, 314171 Samples, median 2.293146e9 Hz),
  `ulysses_atdf_x.bin` 40028696 B sha256 `163897d2…c407ed52` (X, 357399 Samples,
  median 8.408210e9 Hz). Beide `asset present` in `phi/harvest.φ` + `phi/sources.φ`;
  issue #38 geschlossen.
- **X-Validierung** gegen die gewachsene Reihe: X-Median 8.408210e9 Hz gegen
  `X_FSKY_BASE_HZ` 8408.209876e6 (Δ 124 Hz); per-TDF X-Median 8.408232e9 /
  8.408571e9 Hz, beide innerhalb `X_FSKY_MED_HALF_WIDTH` 2.2e6; Ref-Fenster
  `[X_BAND_REF_LO, X_BAND_REF_HI]` = [21.97e6, 22.00e6] → ref-Rejects 0 / 2. Die
  Konstanten halten; der Log zählt `ref`/`median` band-kombiniert (kein
  `fmed_x`/`ref_rejected`-Token), `no_pair` erscheint als `unpaired` (14912 /
  27120).

## LRO utF (`wartend`)

- Trigger = Abschluss des Auto-Dispatch-Laufs. (Schritt: sha256/Größe messen,
  `phi/harvest.φ` + `phi/sources.φ`-Block auf present.)

## GOES-16 ABI (`wartend`)

- Der `_M6C`→`-M6C`-Filter ist behoben (ernte-folge75,
  `goes16_abi_compiler.rs:27`); Trigger = Re-Dispatch-Abschluss. (Schritt:
  `gh workflow run harvest.yml -f format=goes16_abi`, dann `ci_manage view <id>`
  einmal, Blöcke auf present.)

## §4 fsky-Census (`wartend`)

- Trigger = neuer Census-Lauf-Abschluss. (Schritt: `ci_manage view <id>` +
  `gh run download <id>` → `pioneer-cell-census.txt`; Kreuz-Rang + r²-Peak gegen
  die Zwei-Arm-Frage deuten.)

## BepiColombo (`wartend`)

- Radio-Science-Bundle `bc_mpo_more/` lebt, `data/` 404 (Cruise). Trigger =
  `data/`-Manifestation. (Schritt: bei Manifestation PDS4-TNF/ODF-Parser nach dem
  tatsächlichen Datentyp.)

## CDN-Concurrency Follow-up (`wartend`)

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht;
  `cancel-in-progress: false` bleibt. Trigger = erfolgreicher `mro_odf`-Lauf.
  (Schritt: auf die erwarteten Shard-Namen härten, messbar nach dem Lauf.)

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

- Routine-CI-Log-Extraktion (Ulysses-Assets/Census): der registrierte Sieger
  `grind-flash` (Klasse geschlossen 2026-09-16 / ernte-folge75) wurde zitiert,
  nicht gedoppelt. Der flash-Lauf lieferte die Asset-Größen und per-TDF-Zeilen;
  die Log-Token `fmed_x`/`ref_rejected` existieren nur als formatierte Ausgabe,
  nicht als Literal — die Deutung lief in der Hauptsession.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/harvest.φ`, `phi/sources.φ`,
  `docs/handover/post.md` (nur die eine gefaltete Zeile),
  `docs/handover/handover-2026-09-18-forschung-folge72.md` (+ archiviertes
  `handover-2026-09-18-forschung-folge71.md`).
- **Fremd (nicht anfassen):** `opencode.json`, die drei gestagten
  `handover-2026-09-16-*`-Renames, die laufende Register-Prosa-Relaxation in den
  übrigen `phi/*.φ` (+ fremde Hunks in `sources.φ`/`harvest.φ`). Die eigenen
  `sources.φ`/`harvest.φ`-Hunks wurden über einen zeilen-scoped Patch
  (`git diff -U0` → `git apply --cached --unidiff-zero`) isoliert gestaged. Nie
  ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
