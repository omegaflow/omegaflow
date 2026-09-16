<!--
  title: Handover — Forschung-Folge 40 (2026-09-16)
  session: Forschung-Folge 40
  class: handover
  date: 2026-09-16
  sha256: 17a63816b829b4121325c361e5047a1728fc7ba5ce5029e4729010376394236f
  status: live
-->
# Handover — Forschung-Folge 40 (2026-09-16)

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
Der Planungs-Pass nennt **einen schweren und fünf leichte** offene Punkte (der
schwere ist der erste offene Abschnitt, die leichten sind mechanisch
schließbar); die Session arbeitet beide ab.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Council

- **304-Å-Trigger falten vs. descopen + Blatt 2/3 fam/max-T-Bound.** (Schritt:
  Rat, vor jedem Blatt-Schreiben.)

## h0 / CMB — Reste

- **CMB clik/CosmoMC-Eigenevaluation** — die Chain-Statistik ist gewogen, der
  Likelihood-Code-Lauf selbst bleibt offen. (Schritt: Bau eines `clik`/CosmoMC-
  Atoms — oder descoped nach Messung; `docs/paper/h0-lines-register.md`
  §„Named pending points" (1).)

## Quellen-Pass

- **CHAMP/GFZ-ISDC PLPT** — PLPT-zip→Tabellen-Compiler `pending` (kein
  Harvest-Bin verifiziert). (Schritt: `grind-pro`, Harvest-Bin.)
- **Rosetta RSI** — Register an ernte (gepostet); nur EAR2 (2007). (Schritt:
  ernte faltet.)
- **NRS-Hydrophon** — native FLAC, Compiler fehlt. (Schritt: Bau-Linie,
  FLAC-Decoder.)
- **`ephemeris_juice_cog.bin` 404** — gemessen: das Asset fehlt auf dem CDN,
  upstream erreichbar (`spiftp.esac.esa.int` HTTP 200); die Manifest-Stufe in
  `.github/workflows/kernel-flatten.yml:83` (seit `99c423a1`, 2026-09-13) lief
  nie (letzter Lauf 34623430699 predatiert sie). `kernel-flatten` run
  35123314530 @ 77c8be18 dispatcht (2026-09-16). (Schritt: Run-Ergebnis lesen;
  bei Erfolg Asset vorhanden, sonst den Eintrag als `pending`/`absent` messen.)

## Bande-Split / Sonden-ODF

- **NOCC-Reduktionsvorschrift** — die Datei ist forschung-eigen (geklärt
  2026-09-16); die Integration der Reduktionsvorschrift selbst bleibt offen
  (§176: „the named machine of the NOCC reduction remains open"). (Schritt: die
  Vorschrift in `twenty-second-band-ground-chain.md` integrieren.)
- **7 planetare ODF** — `planetary-odf-cdn` run 35097513235 noch `in_progress`
  (mars_express, rosetta laufen); odyssey + mro `failure` (CDN `odyssey_odf.bin`/
  `mro_odf.bin` absent); juno/magellan/mgs/messenger `success` (Assets present).
  (Schritt: `gh run view 35097513235 --log-failed` nach Abschluss; den
  odyssey/mro-Fehlergrund messen.)

## Positionslinien / Ephemeriden

- **Horizons-Residual** — `ephemeris-horizons-check` 35117188074 success, alle
  Körper `0 uncovered`; der Chebyshev-Residual reicht bis neptune 457925 m. (Schritt:
  prüfen, ob der Residual die erwartete Chebyshev-Approximation ist oder eine
  Schwelle braucht — `ephemeris-horizons-check.yml`.)
- **Sonne-Anker-Fix verifizieren** — gemessen: drei `ephemeris_binary`-Quellen
  trugen `at sun`; die Probe dedupliziert nach Name, `ephemeris_neptune_c.bin`
  sortierte zuerst → `props=None` → „sun absent". `phi/sources.φ:2149` →
  `at neptune_c`, `:2284` → `at uranus_c` (2026-09-16). (Schritt: `s2-weberin-probe`
  nach dem Push neu dispatchen; die Sonne muss im sub-resolution-Verdikt
  erscheinen.)

## Die Weberin — Rest

- **Vlies-Konsumption** — die Manifestation ist gemessen (Lauf 34064753336,
  `vlies_density.vlde`, sha256 `7bf53447…`); offen bleibt der Archivar-Reader
  für `.vlde` (`format`/Reader fehlen). (Schritt: Reader + `format vlde`,
  eigenes Atom.)

## Paper / Präregistrierung

- **20-s-Bande-Papier — Tag/Branch** — gewandert an `entscheid` (Post
  `docs/handover/post.md`): das Namens-Wort ist operator-gebunden. (Schritt:
  entscheid faltet; danach mechanisch `git tag`/`git branch`/`git push`.)
- **Flyby Path 2** — Zellen pending, Operator-Siegel; füllen nach JUICE
  28./29.09., Clipper 03.12. (Schritt:
  `docs/paper/flyby-path-2-preregistration.md`.)
- **JWST disequilibrium** — O2/O3-, red-edge-, saisonale Kanäle `pending`, bis
  eine Detektion samt Spektrum ins Register tritt. (Schritt:
  `docs/paper/jwst-disequilibrium-survey.md` §6.)

## CI / geteilter Zustand

- **CI-Status** — der Eintrag in `docs/zustand/external-state.md` (CI-Zeile) ist
  bei jedem HEAD-Wechsel fällig; zuletzt @ 77c8be18 gemessen (run 35122482891:
  format/clippy rot, build success, test/index offen). Nach dem Push dieser
  Session erneut fällig. (Schritt: `gh run list --workflow=ci-check.yml` +
  Check-Runs des HEAD; den Eintrag fortschreiben.)

## Benchmark

- **Rat (pro/max) gegen Baum-Messung (pro/flash) — gemessen:** der Rat
  beantwortete den schweren Punkt „Parser-Bauten" aus dem Register und schlug
  MiniSEED zuerst; die parallele Baum-Messung (`grind-pro`-Recon) ergab: alle
  drei Reader (COSMIC-2, TEC-GIM, MiniSEED) sind gebaut — der Register-Punkt war
  veraltet. Lehre: der Rat braucht die Baum-Messung als Prämisse, sonst trägt
  sein Verdikt eine veraltete Annahme. Der gemessene Rest (LZW-Zweig in
  `ionex_compiler.rs`) ist gebaut (flash, `cargo check` sauber).
- **Workflow-Dispatch:** `kernel-flatten` 35123314530 @ 77c8be18
  (juice_cog-Manifest).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
