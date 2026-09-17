<!--
  title: Handover — Forschung-Folge 60 (Stand 2026-09-17)
  session: Forschung-Folge 60
  class: handover
  date: 2026-09-17
  sha256: c51177cc940e325a0395c0b78d40d92f8061e4cfd92da03e3e47df49aa25c4fb
  status: live
-->
# Handover — Forschung-Folge 60 (2026-09-17)

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

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (automatisch, keine Auswahl)

Die fälligen Einträge aus `docs/zustand/external-state.md` werden zu
Session-Beginn gemessen, bevor die Auswahl steht — ihr Ausgang verändert die
Auswahl, sie sind kein Auswahlpunkt. Ergebnis direkt in dieses Handover + den
Zustand-Ledger. Karte: `docs/concepts/tools-map.md` — bei Widerspruch gilt `--help`.

- **Postfach** — `state/mail/mail_ledger.φ` (224 Zeilen) gemessen: **neuer Eingang** —
  Thomas Keller (TRISP/MPI-FKF, von Lohstroh weitergeleitet) übernimmt die
  NSE-Anfrage und **sendet die I(q,t)-Rohdaten „in a few days"** (17.09. 09:29Z);
  der Operator hat geantwortet. CSES/Sotgiu: „wait a few weeks" (CSES-02-Umstellung).
  Rubin = nur Forum-Summary (keine Antwort). Die fünf Sonden-Anfragen (NSSDC/JPL-NAV)
  bleiben ohne Antwort. `docs/zustand/external-state.md` trägt fremde uncommittete
  Änderungen → zitiert, nicht überschrieben.
- **CI-Status am HEAD `503006dc`** — der Watchdog-Snapshot (12:57Z) war vor dem
  `5dcdda0d`-Push; `ci_manage` am HEAD: **paper-check `35230433910` @`5dcdda0d`
  `queued`** (seit 13:57Z, Runner-Sättigung), **ci-check `35231346974` @`503006dc`
  `pending`** (aktueller HEAD, bau-Push); ci-check @`e197fbec` `35230744407`
  `cancelled` (vom bau-Push verdrängt), ci-check @`5dcdda0d` `35230434046`
  `cancelled`. CDN-Runs (demeter/ned/ps1/physionet/gaia/allwise/maven/planetary-odf)
  aktiv/queued. `external-state.md` fremd → zitiert.

## Paper-Gate — Reference-Gate erstmals vor-gemessen (härtester undatiert)

- **Ein-Blatt-Fix verifiziert:** `docs/blatt/blatt-der-grat.md` Kopf-`sha256` =
  `d7b28783…` == Body-Hash (`tail -n +9 … | sha256sum`); deckt sich mit
  `strip_and_parse_header` in `tools/science/src/bin/export_latex.rs:23` (Body =
  alles nach `-->` + Newline). Der zweite Gate-Schritt sollte grün sein.
- **Reference-Gate vor-gemessen (flash-Dispatch):** 33 Papiere, 66 distinkte
  Identifier (30 arXiv, 36 DOI) — **alle lösen direkt auf** (arXiv 200 +
  `citation_title`, doi.org 302) → der dritte Schritt sollte grün sein, kein
  `absent`/`pending`. **`archive_search --verdict` ist für DOIs nicht gate-treu**
  (folgt dem Redirect → 403 statt 302); der Taucher hat die Gate-eigenen
  curl-Aufrufe nachgestellt. **Coverage-Lücke:** alte arXiv-IDs
  (`arXiv:astro-ph/0012376` in `h0-lines-register.md:86`, `arXiv gr-qc/0208046` in
  `twenty-second-band-ground-chain.md:239`) werden vom Extraktor still
  übersprungen (nur `NNNN.NNNNN`) — kein Rot-Risiko, aber unverifizierte Referenzen.
- **Offen:** der queued `paper-check` `35230433910` @`5dcdda0d` liefert die
  Gate-Messung (Schritt 2 + 3); Ergebnis eintragen. (Schritt: `ci_manage view
  35230433910`.)
- **Offen (nächstes Atom):** `extract_arxiv_ids` in
  `tools/register/src/bin/reference_verify.rs:27` um alte `archive/NNNNNNN`-IDs
  härten + Gate-Test. (Schritt: Extraktor erweitern, `cargo check`, CI-Verifikation.)

## Format-Gate — ci-check am aktuellen HEAD pending

- `ci-check` `35231346974` @`503006dc` (bau) pending; der `format`-Job
  (`cargo fmt --check`) liefert den Nachweis. Der forschung-Push `5dcdda0d` hatte
  ci-check `35230434046` — cancelled durch den nächsten Push. Bei Rot die fremden
  Pfade als Post an ernte/bau. (Schritt: `ci_manage view 35231346974`.)

## NSE/Haug — Route offen, Rohdaten zugesagt

- Thomas Keller (TRISP) sendet die NSE-I(q,t)-Rohdaten „in a few days". Auf Eingang:
  `nse_haug_trisp`-Quelle + Compiler + `sources.φ` (0-Kanon: kein Asset ohne Datei).
  (Schritt: bei Mail-Eingang die Dateien lesen, dann Compiler-Draft.)

## Docs-Pendings — Sichtung 2026-09-17

- Machbare Einzelne (je Survey-Zeile belegt): Mariner `PSPA-00316` CDN-Dispatch
  (`survey-2026-09-17-sonden-request-only.md:66–67`); Juno post-EFB OCRU ↔
  `juno_odf.bin` (`:38`); Pioneer ATDF dtype-12/13 (`src/archivar/atdf.rs:508`
  gated nur 1|2); MAVEN-TNF-Shards; GOES-16 ABI `sources.φ`+Workflow
  (`survey-2026-09-14-kapitulationen-pendings-inventur.md:51`); WWLLN
  (`survey-2026-09-13-weberin-quellen.md:188`); Ulysses/BepiColombo/LRO
  `sources.φ` (`survey-2026-09-16-sonden-flotte.md:57–59`). (Schritt: je Eintrag
  der genannten Survey-Zeile.)

## CDN-Concurrency Follow-up

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht
  (`grep -q "^mro_odf_"` übersieht ein halbes Set); `cancel-in-progress: false`
  bleibt bis dahin. (Schritt: auf die erwarteten Shard-Namen härten, messbar
  erst nach einem erfolgreichen `mro_odf`-Lauf.)

## Legacy-Konzepte (hintenangestellt — Operator-Wort)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als
  4.; Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt:
  bei Wiederaufnahme den Rat-Erster-Atom bauen — `tools/measure/src/bin/
  silence_map_probe.rs`, Null-Kalibrierung als Spiegel der FP/FN/Symmetrie-Gates
  von `te.rs`.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- Reference-Gate-Vormessung als flash-Dispatch (`general`): 66 Identifier
  extrahiert + gemessen, fand die Coverage-Lücke und die `--verdict`-Untreue; der
  CI-Lauf `35230433910` ist der Gegenlauf. Kein pro/max.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/handover/handover-2026-09-17-forschung-folge60.md`
  (+ archiviertes `handover-2026-09-17-forschung-folge59.md`). Fremde uncommittete
  Arbeit (nicht anfassen): `docs/zustand/external-state.md`, `src/archivar/cdn.rs`,
  `src/archivar/fetch.rs`, `docs/concepts/kybernaut-native-methodology.md`,
  `docs/concepts/remove-bias.md`, `docs/specs/kernel-curation-ci-automation-plan.md`,
  die gestagten Renames `handover-2026-09-16-{entscheid-folge24,forschung-folge44,
  forschung-folge51}` → `archiv/`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
