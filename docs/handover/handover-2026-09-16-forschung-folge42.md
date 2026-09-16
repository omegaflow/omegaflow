<!--
  title: Handover — Forschung-Folge 42 (2026-09-16)
  session: Forschung-Folge 42
  class: handover
  date: 2026-09-16
  sha256: a1984b5a1b5b6fadcbc44ab3d412d9be1293d726aff3ae7c4ee063018cf75212
  status: live
-->
# Handover — Forschung-Folge 42 (2026-09-16)

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
der härteste undatiert); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Bande-Split / ODF — >2-GiB-Assets (härtester undatiert)

- **odyssey (3 367 454 840 B) und mro (extrapoliert ~12.7 GiB) sprengen GitHubs
  2-GiB-Limit pro Release-Asset** (gemessen: `HTTP 422: size must be less than
  2147483648`; `odyssey_odf_compiler.rs:234-236`). Die Compiler harvesten
  korrekt; nur die Manifestation passt nicht in ein Asset. Die vier ODF-Compiler
  sind parallelisiert (8 Worker, `fetch_listing` ohne 404-Retry, Anfrage-TTL
  512 s statt ~64 h) — uncommittet. (Schritt: Shard-Schema (<2 GiB je Asset) +
  Konsument entscheiden, dann in `phi/sources.φ` / `src/archivar/extract.rs` /
  Workflow registrieren.)
- **`src/archivar/fetch.rs` braucht zwei Parameter** — `:86-89`
  (`--retry 5 --retry-all-errors` wiederholt auch 404) und `:19-21`
  (Transfer-Bound aus der Cache-TTL). Nicht gebaut (geteilte Datei). (Schritt:
  Retry-Policy- und Transfer-Bound-Parameter entwerfen, im Rat vorlegen.)

## Paper / §4.5

- **76-%-Richtungsstatistik auf dem 613er-Satz nachmessen.** Die Zahl in
  `docs/paper/corona-heating-ladder.md:306-308` trägt die pending-Marke
  „measured on the partial 281-event set; remeasurement on the full 613-event
  set is pending". (Schritt: nach Push `gh workflow run aia-ladder-probe.yml`,
  dann die Zahl ersetzen.)
- **Abstract (Zeile 18) trägt die nicht re-derivierbare Spanne `0.63–0.91 × fam`.**
  (Schritt: aus dem Lauf-Artefakt 35097506781 neu ableiten oder streichen.)
- **`corona_event_probe.rs` trägt zwei vorbestehende Fabrikationsmuster**
  (`stack_direction`: `tot.max(1)` :234, `unwrap_or(0.0)` :278; Gate-Funde
  2026-09-16) — deshalb aus dem §4.5-Commit genommen; der C1.0-Print dort ist
  noch nicht gezogen. (Schritt: `stack_direction` auf `Option<f64>` umstellen,
  die Aufrufer :294/:297 überspringen `tot == 0`; dann Print :294 auf C5.0.)

## Quellen-Pass

- **PLPT-Harvest-Bin steht** (`tools/harvest/src/bin/champ_plpt_compiler.rs`,
  `.github/workflows/champ-plpt-cdn.yml`, `phi/sources.φ`-Block) — uncommittet.
  A=A: der Bin trägt die Langmuir-Elektronendichte, kein Magnetfeld. (Schritt:
  nach Commit `gh workflow run champ-plpt-cdn.yml`; Asset + Zeile manifestieren.)
- **Vlies-Konsumption: Reader + `format vlde` stehen** (`src/archivar/vlies.rs`,
  `mod.rs`, `lib.rs`, `extract.rs`, `main_flow.rs`, `zeuge.rs`) — uncommittet.
  Die Quelle ist noch nicht registriert. (Schritt: `phi/sources.φ` `url`-Zeile auf
  das CDN-Asset `vlies_density.vlde`.)

## Positionslinien / Ephemeriden

- **JUICE-Kernel: Fix steht** (`tools/harvest/src/bin/ephemeris_compiler.rs:554-566`
  schließt den v00-Platzhalter `juice_cog_v00.bsp` aus — ID-Wort „NAIF/DAF", kein
  binäres DAF; die v01-CoG-SPKs sind die gültigen) — uncommittet. (Schritt: nach
  Commit `gh workflow run kernel-flatten.yml`; `ephemeris_juice_cog.bin` prüfen.)
- **Sonne-Anker-Fix verifizieren** — `phi/sources.φ:2149` → `at neptune_c`,
  `:2284` → `at uranus_c` (2026-09-16). (Schritt: nach Push `s2-weberin-probe`
  neu dispatchen; die Sonne muss im sub-resolution-Verdikt erscheinen.)

## Paper / Präregistrierung

- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE
  28./29.09., Clipper 03.12. (Schritt:
  `docs/paper/flyby-path-2-preregistration.md`.)
- **JWST disequilibrium** — O2/O3-, red-edge-, saisonale Kanäle `pending`; die
  Abwesenheit ist gemessen (Sweeps 2026-09-16), bis eine Detektion samt Spektrum
  ins Register tritt. (Schritt: `docs/paper/jwst-disequilibrium-survey.md` §6.)

## CI / geteilter Zustand

- **CI-Status** — der `docs/zustand/external-state.md`-Eintrag ist @ `77c8be18`
  stehengeblieben; die Datei ist von einer anderen Linie aktiv bearbeitet (der
  TE-Gate-Eintrag trägt dort inzwischen den committeten Arx-Switch `1337c6c1`).
  (Schritt: @ HEAD fortschreiben, sobald die Datei frei ist.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
