<!--
  title: Handover — Forschung-Folge 71 (Stand 2026-09-18)
  session: Forschung-Folge 71
  class: handover
  date: 2026-09-18
  sha256: 1ce2959cbd5703e573053ba47d447b2c59248d19aaa1afaf15e409610a42e56d
  status: live
-->
# Handover — Forschung-Folge 71 (2026-09-18)

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

- **HEAD** `3b7aa5a1` == `origin/main` (`ernte: diagnose the failed ulysses/goes
  harvest runs, register ulysses_atdf_x, fix goes16_abi granule filter; handover
  75`). Fremd uncommittet (nicht angefasst): `docs/handover/post.md` +
  `docs/zustand/external-state.md` (entscheid-Folge-41-Messung @`da2cf8f9`), die
  drei `handover-2026-09-16-*`-Renames (D + `archiv/`??).
- **CI** (`ci_manage list` + Watchdog-Snapshot): X-Lauf `35277855134` @`7bf17ada`
  success; `harvest` `35277937121` failure — Ursache war der goes16-`_M6C`-Filter
  (ernte-folge75 behoben); `harvest` `35277931000` (ulysses) S-Band leer; die
  Auto-Dispatch-Läufe teils cancelled/in_progress.
- **Postfach** — keine neue Zeile an die Forschung-Linie; `post.md` trägt die zwei
  `An alle Linien`-Broadcasts (entscheid editiert die Datei gerade — nicht
  angefasst).

## Ulysses S-/X-Band Re-Manifestation (härtester undatiert, abarbeitbar)

- Der S-Band-Paarungsfehler ist behoben (`src/archivar/atdf.rs`
  `reduce_uly_skyfreq`: jedes Record paart mit dem **nächsten gleichbandigen
  Record**, Rate über die **echte Zeitdifferenz** `(dcnt[j]−dcnt[i])/(t[j]−t[i])`;
  Vollständigkeits-Zähler `no_pair`/`no_doppler`; Tests
  `reduce_uly_skyfreq_alternating_bands_pair_within_band`,
  `reduce_uly_skyfreq_contiguous_band_anchors_sequential_pairing` + Invariante;
  `cargo check`/`--tests` 0/0). Damit schließt der `blockiert`-Punkt
  `ulysses_atdf S-Band leer` aus ernte-folge75. Re-Dispatch 2026-09-18 angestoßen:
  `harvest` `35296127974` (`-f format=ulysses_atdf`) + `35296130236`
  (`-f format=ulysses_atdf_x`). (Schritt: `ci_manage view <id>` einmal, sha256/Größe
  messen, `phi/harvest.φ` S auf `asset present` + sha, X-Note auf den neuen sha,
  `phi/sources.φ` nachziehen, issue #38 schließen.)

## X-Ref-Fenster-/Median-Halbwertsbreiten-Validierung (undatiert)

- Gilt gegen die **neue (gewachsene) X-Reihe**. Am alten Asset gemessen
  (`ulysses_atdf_x.bin` 5546584 B = 8 + 49523×112, praktisch nur Datei
  `2035036A.TDF`): X-Median 8.408231e9 Hz gegen `X_FSKY_BASE_HZ` 8408.209876e6
  (Δ ~21 kHz), Ref-Fenster band-unabhängig `[2197e4, 2200e4]` (0 ref-Rejects),
  `X_FSKY_MED_HALF_WIDTH` 2.2e6 Hz. Die Paarungsänderung vergrößert die Reihe →
  die alte Validierung ist überholt. (Schritt: `ci_manage view <neuer Lauf>` +
  Log-Census `fmed_x`/`ref_rejected`/`med_rejected` gegen dieselben Konstanten.)

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

- **S-Band-Paarung** (novel parser, hartes Atom): `grind-max` (Fix + Test) und
  `grind-flash` (`no_pair`/`no_doppler`-Zähler + Invariante + Regressionanker) —
  beide `cargo check`/`--tests` 0/0; der Rat bestätigte die Same-Band-Paarung als
  korrekte Reduktion (Sampler nie Nenner) und die Zähler-Invariante. Die
  Routine-Klasse CDN-Failure-Diagnose hat ihren registrierten Sieger `grind-flash`
  (ernte-folge75: flash == pro, flash günstiger).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `src/archivar/atdf.rs`, `phi/harvest.φ`, `phi/sources.φ`,
  `docs/handover/handover-2026-09-18-forschung-folge71.md` (+ archiviertes
  `handover-2026-09-17-forschung-folge70.md`).
- **Fremd (nicht anfassen):** `docs/handover/post.md`,
  `docs/zustand/external-state.md`, die drei gestagten
  `handover-2026-09-16-*`-Renames. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
