<!--
  title: Handover — Forschung-Folge 65 (Stand 2026-09-17)
  session: Forschung-Folge 65
  class: handover
  date: 2026-09-17
  sha256: 77a319bbcfb94c3c88e5109804bb3efc4337bc19daa330113ff6ff8bc7075fbe
  status: live
-->
# Handover — Forschung-Folge 65 (2026-09-17)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich; so viele offene Punkte wie möglich pro Session (die
alte „ein schwerer und fünf leichte"-Klausel ist gestrichen, `post.md` 2026-09-16).
Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks — committet wird nur
der eigene Teil, fremde uncommittete Arbeit wird nie überschrieben; gepusht wird,
sobald der eigene Commit steht und `origin/main` Vorfahr von HEAD ist
(Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Ein `wartend`-Punkt ist kein Auswahlpunkt — er trägt seinen Trigger und wird
nicht als Aktion verkleidet.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (automatisch, keine Auswahl)

- **Postfach** — neue Eingänge gemessen 2026-09-17: Thomas Keller (TRISP/MPI-FKF)
  sendet die NSE-I(q,t)-Daten „in a few days" (Mail 09:29Z, unsere Bestätigung
  12:20Z); GitHub-Support-Ticket **#4761801** weiter offen (GC); Sotgiu: CSES-02-
  Umstellung, „wait a few weeks" (16.09.). Der Zustand-Eintrag
  `docs/zustand/external-state.md:20` (Postfach) ist fällig (neuer Eingang) — die
  Datei trägt fremde uncommittete Änderungen, der Eintrag bleibt der besitzenden
  Linie überlassen (nicht angefasst).
- **CI-Status** — Watchdog-Snapshot 18:09:50Z + `ci_manage list` (18:2xZ):
  `pioneer-cell-census` `35244087174` success; `mariner-occlt-cdn` `35244811694`
  success; `swot-cdn` `35246189331` **failure** (1×); `paper-check` `35249016073`
  success (16:58Z — die zwei failure-Läufe `35228278279`/`35224760511` sind damit
  überholt); `pii-exposure` `35230489744` failure (1×, GitHub-GC offen); neue Welle
  `dl3-skymap-cdn`/`maxi-cdn`/`icesat2-cdn`/`gedi-cdn` + `auto-dispatch` queued
  (17:07Z). Der Zustand-Eintrag `docs/zustand/external-state.md:22` (CI-Status
  @`bc9d6a0b`) ist bei HEAD-Wechsel fällig — fremd-modifiziert, nicht angefasst.
- **HEAD `5f9cd61b`** — `origin/main` == HEAD (keine unpushed Commits,
  Fast-Forward frei). `git_safety --snapshot` = `refs/safety/1789664963`.

## §4 fsky-Census — die drei Restfragen beantwortet (Paper v11)

- **Q1 — die 50,000-mHz-Linie ist ein Abtastraster-Artefakt, kein 20-s-Signal.**
  Für einen Stempelabstand Δt mit Δt·50 mHz ganzzahlig (Δt = 20/40/60/80/100/120 s)
  ist die Gitterfrequenz 50,000 mHz das Faltbild des Near-DC-Inhalts; die feinen
  Klassen (Nyquist ≥ 50 mHz) tragen keine. Die 218/214/0/4-Zahlen und das
  Faltargument stehen in `docs/paper/twenty-second-band-ground-chain.md` §4.
- **Q2 — das Feld-Δ ist ein Um-Ranken, kein Erzeugen.** r[1] ist das
  Count-Differenz-Observable (keine ODP-Stufe operiert auf ihm); r[8] hat die
  NOCC-Kette durchlaufen (computed-Subtraktion, Endpunkt-Mittelung (Rj+Ri)/2 mit
  Transfer cos(πντ), 1-mHz-ATDF-Quantisierung). rx14's 46,58-mHz-Mitglied
  übersteht beide Pipelines; ob rx43's 44,119 und rx63's 50,920 in ihren fsky-Zellen
  sub-dominant vorhanden oder dort erzeugt sind, ist offen.
- **Q3 — der fsky-Komplex braucht keine Reduktionsstufe**: empfangs-ketten-geboren
  (First-Difference über das Sampler-Intervall + Referenz-Staircase ×104,25).
- **§8 (light-time)** bleibt der erste benannte Kandidat für den resid-Komplex,
  präzisiert von Träger zu Um-Ranker.
- **Gebaut (diese Session):** `tools/measure/src/bin/pioneer10_cell_census_probe.rs`
  um ein Floor-Verhältnis je Zelle erweitert (Median-Grid-Leistung, peak/floor,
  Scargle-FAP; 0-Kanon: `—` statt erfundener Zahl bei floor 0). `cargo check` mit
  `RUSTFLAGS=-Dwarnings` grün (grind-flash).
- **Entscheidende Messung (offen, Schritt):** die Probe um den **Kreuz-Rang** der
  beiden Dominanten je Zelle erweitern (resid-Dominant im fsky-Gitter und umgekehrt)
  + r[2]-LS in den 4 fsky-feinen st63-1992-Zellen; danach
  `gh workflow run pioneer-cell-census.yml` (nach dem Push) und das Artefakt
  `pioneer-cell-census.txt` lesen.

## Docs-Pendings — Drafts geschrieben, Registrierung offen

- **Ulysses / LRO / BepiColombo / GOES-16** — Grind-Drafts geschrieben:
  `phi/pipeline/queue/grind_ulysses.φ`, `grind_lro.φ`, `grind_bepicolombo.φ`,
  `grind_goes16.φ` (Work Surface ist per `.gitignore` `phi/pipeline/*` lokal, nicht
  getrackt). Gemessen: Ulysses-PPI direkt absent, userspace-Exit HTTP 200; LRO/PSA/
  GOES-16 HTTP 200; BepiColombo `bc_mpo_more/data/` HTTP 404 → `pending` mit Ort.
  **Schritt:** die vorgeschlagenen `sources.φ`-Blöcke registrieren (`phi/sources.φ`
  ist jetzt clean, kein Fremd-Hunk) + Workflow je Sonde; GOES-16 = Spiegel des
  registrierten GOES-19, kein neuer Compiler.
- **`blocked parser-def odf` stale** (`phi/blocked_sources.φ:81–83`): `src/archivar/
  odf.rs` dekodiert im TNF/TRK-2-34-Arm alle 18 Codes (`TNF_FORMAT_*` 0–17,
  `tnf_dt0…17`, `tnf_row`-match 0–17; 6/18 getestet); der PDS3-ODF-Wortarm hat zwei
  Layouts. Die Note „dekodiert nur `format_code 0` (DT0); 17 Format-Codes offen" ist
  für den TNF-Arm widerlegt. **Schritt:** im Dispositions-Atom austragen/ersetzen.
- **MAVEN-TNF-Shards** — offen: TRK-2-34-SFDU-TNF-Record-Parser + PDS4-XML-Label-
  Leser in `extract.rs` (`pds-ppi.igpp.ucla.edu/.../tnf/`). (Schritt: Parser-Gap-
  Auftrag.)

## WWLLN — Lizenz (`operator-gebunden`)

- Thunder-Hour (Zenodo-Spiegel `records/10725446`, CC BY-SA 4.0; Quell-Lizenz
  „research (non-commercial) use"). Die CDN-Manifestation braucht den
  Operator-Entscheid; als Post-Zeile `An entscheid` abgelegt. (Schritt: bei
  Operator-Wort den Zenodo-Spiegel registrieren, sonst bleibt der Kanal
  unmanifestiert.)

## CDN-Concurrency Follow-up

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht
  (`grep -q "^mro_odf_"` übersieht ein halbes Set); `cancel-in-progress: false`
  bleibt bis dahin. (Schritt: auf die erwarteten Shard-Namen härten, messbar erst
  nach einem erfolgreichen `mro_odf`-Lauf.)

## NSE/Haug — Rohdaten zugesagt (`wartend`)

- Thomas Keller (TRISP) sendet die NSE-I(q,t)-Rohdaten „in a few days" (Mail
  2026-09-17 09:29Z). **Trigger = Dateieingang.** (Schritt: bei Eingang
  `nse_haug_trisp`-Quelle + Compiler + `sources.φ`; 0-Kanon: kein Asset ohne Datei.)

## Legacy-Konzepte (hintenangestellt — Operator-Wort)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als 4.;
  Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt: bei
  Wiederaufnahme den Rat-Erster-Atom bauen — `tools/measure/src/bin/
  silence_map_probe.rs`, Null-Kalibrierung als Spiegel der FP/FN/Symmetrie-Gates
  von `te.rs`.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- **§4-fsky-Deutung** (hartes Atom; Klasse „ODP-Stufen-Zuordnung" ohne registrierten
  Sieger): doppelt gelaufen — `general` (flash) gegen `research-max`. Beide tragen
  Q1 (Raster/Alias) und Q3 (fsky prä-Reduktion) übereinstimmend; **`research-max`
  gewinnt** (Q2 als Um-Ranken + Endpunkt-Mittelung + Kreuz-Rang als entscheidende
  Messung + §4-Textvorschlag; flash ließ Q2 bei „nicht durch den Feld-Census
  erklärt" und nannte nur die Top-5-Messung). Der Probe-Bau lief `grind-flash`
  (Routine, flash-first).
- **Lehre (Report-Fälschung):** der erste `grind-flash`-Quellen-Port-Lauf behauptete
  Draft-Pfade, ohne Dateien zu schreiben — entdeckt, weil `phi/pipeline/*`
  gitignored ist und der pfadlose `glob` leer blieb; der zweite Lauf schrieb und
  bewies per pfad-gebundenem `glob` + `sread`-Rücklesung. **Schreib-Aufträge an
  Sub-Agenten brauchen eine Rücklese-Beweispflicht** (glob/sread-Beleg), sonst ist
  der Report nicht vertrauenswürdig.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/paper/twenty-second-band-ground-chain.md`,
  `tools/measure/src/bin/pioneer10_cell_census_probe.rs`,
  `docs/handover/post.md`,
  `docs/handover/handover-2026-09-17-forschung-folge65.md` (+ archiviertes
  `handover-2026-09-17-forschung-folge64.md`). Fremde uncommittete/gestagte Arbeit
  (nicht anfassen): `docs/zustand/external-state.md`, die gestagten Renames
  `handover-2026-09-16-*`/`handover-2026-09-17-bau-folge66`/`entscheid-folge36` →
  `archiv/`, `docs/handover/handover-2026-09-17-bau-folge67.md`,
  `docs/handover/handover-2026-09-17-entscheid-folge37.md`,
  `.github/workflows/harvest*.yml`, `tools/utils/src/bin/harvest_reg.rs`. Nie ein
  nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
