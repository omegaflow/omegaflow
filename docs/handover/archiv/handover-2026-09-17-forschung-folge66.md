<!--
  title: Handover — Forschung-Folge 66 (Stand 2026-09-17)
  session: Forschung-Folge 66
  class: handover
  date: 2026-09-17
  sha256: f96430a0fca3b0af1cd699e864d9b6b6f6074d4932b95e20f830f386eb774b1e
  status: live
-->
# Handover — Forschung-Folge 66 (2026-09-17)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits,
der Arbeitsbaum darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht. Der
Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist der
härteste undatierte); die Session arbeitet so viele ab wie möglich. Wartestellungen
(`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser und werden nie
als Handlungsschritt geführt. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (automatisch, keine Auswahl)

- **Postfach** — keine neue Zeile seit Folge 65 (NSE/Haug „in a few days" 09:29Z;
  GitHub-Support-Ticket **#4761801** offen; Sotgiu: CSES-02-Umstellung „wait a few
  weeks"). Der Zustand-Eintrag `docs/zustand/external-state.md:20` ist fällig —
  fremd-modifiziert, der besitzenden Linie überlassen (nicht angefasst).
- **CI-Status @HEAD `8d821322`** (Watchdog-Snapshot 21:21Z + `ci_manage list`):
  `pioneer-cell-census` `35264753611` in_progress (alter Probe-Stand);
  `swot-cdn`/`gedi-cdn`/`icesat2-cdn` failure (protected-bucket 403, gemessen);
  `allwise`/`ned`/`vlass-tap`/`maxi`/`dl3-skymap`/`harvest`/`paper-check` success;
  `ps1-cdn` in_progress. Der Zustand-Eintrag `docs/zustand/external-state.md:22`
  (@`bc9d6a0b`) ist bei HEAD-Wechsel fällig — fremd-modifiziert, nicht angefasst.

## §4 fsky-Census — entscheidende Messung (Probe erweitert)

- `tools/measure/src/bin/pioneer10_cell_census_probe.rs` um den **Kreuz-Rang**
  erweitert (resid-Dominant im fsky-Gitter und umgekehrt: `cross_rank`/`xrank_line`,
  Call-Sites CELL CENSUS + CROSS) und um die **r[2]-LS-Sektion**
  (`=== REFERENCE r[2] LS — st63 lt10 1992 (fsky-fine cells) ===`); `Field::Ref`
  + `--field ref`. `cargo check` mit `RUSTFLAGS=-Dwarnings` grün. Dispatch nach Push
  gemessen: `gh workflow run pioneer-cell-census.yml` → Lauf **`35266366575`**
  (queued, 2026-09-17T19:41Z; Artefakt `pioneer-cell-census`). (Schritt: Lauf einmal
  lesen — `ci_manage view 35266366575`, dann `gh run download 35266366575` und
  `pioneer-cell-census.txt` —, Kreuz-Rang + r[2]-Peak gegen die Zwei-Arm-Frage
  deuten: sub-dominant vorhanden ⇔ Um-Ranken, absent ⇔ in der NOCC-Kette erzeugt.)
- Bei mehrdeutigem Kreuz-Rang-Befund: `research-max` (Klasse „ODP-Stufen-Zuordnung",
  Sieger registriert).

## Docs-Pendings — Registrierung offen

- Ulysses/LRO/BepiColombo/GOES-16: Grind-Drafts `phi/pipeline/queue/grind_*.φ`
  geschrieben (lokal, gitignored); gemessen Ulysses-PPI absent/userspace HTTP 200,
  LRO/PSA/GOES-16 HTTP 200, BepiColombo `bc_mpo_more/data/` 404 → `pending` mit Ort.
  (Schritt: die `sources.φ`-Blöcke registrieren (`phi/sources.φ` clean) + Workflow je
  Sonde; GOES-16 = Spiegel des registrierten GOES-19.)

## MAVEN-TNF-Shards (`blockiert parser`)

- TRK-2-34-SFDU-TNF-Record-Parser + PDS4-XML-Label-Leser in `extract.rs`
  (`pds-ppi.igpp.ucla.edu/.../tnf/`). (Schritt: Parser-Gap-Auftrag, `grind-max`.)

## CDN-Concurrency Follow-up

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht
  (`grep -q "^mro_odf_"` übersieht ein halbes Set); `cancel-in-progress: false`
  bleibt bis dahin. (Schritt: auf die erwarteten Shard-Namen härten, messbar erst
  nach einem erfolgreichen `mro_odf`-Lauf.)

## WWLLN — Lizenz (`operator-gebunden`)

- Thunder-Hour (Zenodo-Spiegel `records/10725446`, CC BY-SA 4.0; Quell-Lizenz
  „research (non-commercial) use"). Die CDN-Manifestation braucht den
  Operator-Entscheid; Post-Zeile `An entscheid` steht. (Schritt: bei Operator-Wort
  den Zenodo-Spiegel in `phi/sources.φ` registrieren.)

## NSE/Haug — Rohdaten zugesagt (`wartend`)

- Thomas Keller (TRISP) sendet die NSE-I(q,t)-Rohdaten „in a few days" (Mail
  2026-09-17 09:29Z). **Trigger = Dateieingang.** (Schritt: bei Eingang
  `nse_haug_trisp`-Quelle + Compiler + `sources.φ`; 0-Kanon: kein Asset ohne Datei.)

## Legacy-Konzepte (`operator-gebunden`, hintenangestellt)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als 4.,
  Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt: bei
  Wiederaufnahme den Rat-Erster-Atom bauen — `tools/measure/src/bin/silence_map_probe.rs`,
  Null-Kalibrierung als Spiegel der FP/FN/Symmetrie-Gates von `te.rs`.)

## Paper / Präregistrierung (`termin`)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- §4-Probe-Bau (Routine): `grind-flash`, **kein Doppellauf** — die Klasse
  „Probe-Bau" hat den registrierten Sieger flash. Die §4-Deutung („ODP-Stufen-
  Zuordnung") gewann `research-max` bereits (Folge 65). Kreuz-Rang-Deutung offen →
  bei Mehrdeutigkeit `research-max`.
- Lehre (Report-Fälschung) bestätigt: beide Schreib-Aufträge dieser Session lieferten
  die geforderte Rücklese-Beweispflicht (`sread`/`glob`/`cargo check`).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `tools/measure/src/bin/pioneer10_cell_census_probe.rs`,
  `phi/blocked_sources.φ`,
  `docs/handover/handover-2026-09-17-forschung-folge66.md` (+ archiviertes
  `handover-2026-09-17-forschung-folge65.md`). Fremde uncommittete/gestagte Arbeit
  (nicht anfassen): `docs/zustand/external-state.md`, `.github/workflows/harvest.yml`,
  `tools/utils/src/bin/harvest_reg.rs`, die gestagten Renames
  `handover-2026-09-16-*` → `archiv/`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
