<!--
  title: Handover — Ernte-Folge 69 (Stand 2026-09-17)
  session: Ernte-Folge 69
  class: handover
  date: 2026-09-17
  sha256: f4f8e5f5c7d4fdc0710bc21397b22c2684a1ddfb278e6b9dd6367017051d2c49
  status: live
-->
# Handover — Ernte-Folge 69 (2026-09-17)

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
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Harvest-Architektur — erster absent-Asset-Block steht, Dispatch-Beweis offen (härtester undatiert)

Der rosetta_odf-Block ist geschrieben (Council-gehalten): `phi/harvest.φ` trägt
jetzt zwei Blöcke — `maven_tnf` (`asset present`) und `rosetta_odf` (`asset fehlt`,
`timeout 350`). Council-Verdikt (2026-09-17): rosetta_odf ist der einzige
registrierte 1:1-absent-Kandidat (läuft mit nur `--ci-mode`, Consumer-Arm
`src/archivar/ifms_agc.rs` verdrahtet, `format rosetta_odf` in `phi/sources.φ:6374`);
`asset` = gemessene CDN-Wahrheit, Dispatch folgt, das Gate ist der zweite Zeuge;
`timeout` gehört in den Block (35148936648 starb gemessen am 240-Cap, das alte
`planetary-odf-cdn.yml:29` maß 350).

- **Dispatch-Beweis offen** — Push → `harvest-dispatch` → `harvest.yml -f
  format=rosetta_odf -f timeout=350` → Gate `present=false` → Compile (bis 350 min).
  (Schritt: nach Push `ci_manage list`/`view <id>` einmal; bei success `note` des
  rosetta-Blocks auf `asset present` + Run-Id/Bytes/sha256 setzen.)
- **Ungelaufene Pfade, `pending`** — das Idempotenz-Gate selbst, der `force`-Lauf,
  der neue `timeout`-Input-Pfad, der zweite Zeuge mit Shard-Vollständigkeit
  (`harvest.yml`, gebaut), `--check` Feldeindeutigkeit + timeout-Validierung
  (`harvest_reg.rs`, gebaut) — alle erst im ersten rosetta-Lauf / CI-Dispatch gemessen.
- **Migration — nächste Familien:** die arm≠format-Familien
  (`ephemeris_epm`/`epm_compiler`, `openneuro_eeg`/`openneuro_compiler`,
  `catalog_gaia_sso`/`gaia_sso_compiler`) sind absent+1:1 — der Block trägt `arm`
  getrennt vom `format`, der Dispatch mappt bereits `format`→Block. (Schritt: je
  Block schreiben, `asset fehlt` — `grind-pro`.) Dann die parameterisierten
  (`gedi_l2a`/`icesat2_atl03`/`swot_l2_lr_ssh` verlangen `--day`/`--prefix`,
  `dl3_skymap` `--telescope`, `juno_ocru_odf` `--volume`) brauchen einen
  Argument-Punkt im Register; `text`/`volume` sind generische Keys mit mehreren
  Quellen unter einem Format — ein Block je Format braucht eine Entscheidung.
  `auto-dispatch` zuletzt falten (kein Löschen vor Parität).
- **Benchmark:** `grind-pro` wählte zuerst `fermi_4fgl` — falsch (kein
  `fermi-cdn.yml`, nicht in `phi/sources.φ`, kein Consumer-Arm → kein
  Migrations-Familienmitglied; verworfen). `grind-flash` scannte 62 Compiler → nur
  `rosetta_odf` als 1:1-absent; deckte zugleich die `arm ≠ format`-Klasse auf.
  Routine-Scan bleibt flash.

## CDN-Register-Schuld

- **Registriert (dieses Atom):** vlass-tap `35251315622`, dl3-skymap-hess
  `35251318800`, maxi `35251322353` — `phi/sources.φ`-Notes auf success gesetzt
  (Tag/Asset/URL/Felder waren bereits korrekt, nur die CDN-Messung fehlte).
- **`magic_dl3`** — FITS-URL gemessen (`grind-flash`): `https://door06.pic.es:8452/
  magic_dl3_pdr1/data/CrabNebula/dark/single_offset/20131005_05029789_DL3_
  CrabNebula-W0.40+215.fits` HTTP 200, 328320 B; compiler-kompatible Alternative
  `.../magic_dl3_pdr1/magic_dl3_pdr1-main.tar.gz` HTTP 200. Dispatch offen — `dl3_skymap`
  ist parameterisiert. (Schritt: `gh workflow run dl3-skymap-cdn.yml -f telescope=magic
  -f url=<fits>` oder Argument-Punkt im Register — `grind-flash`.)
- **gedi/icesat2/swot protected-Bucket-403** — `research-max` gemessen: der
  `EARTHDATA_EDL_TOKEN` (`.secrets.local:24`) wird an den Credential-Endpoints
  akzeptiert (3× 200: `data.lpdaac`, `data.nsidc`, `archive.podaac` `/s3credentials`);
  der Workflow-403 ist NICHT das Token. Direkt-Bearer-Listing auf den Bucket-Hosts
  ist kein Schema (404). Nächster Schritt: Credentials münzen, EIN SigV4-Listing
  reproduzieren und den XML-Fehlerkörper lesen — `AccessDenied` → Datenabkommen
  (SWOT) = operator-gebunden; `AuthorizationHeaderMalformed`/`PermanentRedirect` →
  Signing-Bug in `src/archivar/range.rs` (Region/Host/path-style). (`research-max`)
- **`.secrets.local` Erst-Treffer** — die Compiler lesen die erste
  `EARTHDATA_EDL_TOKEN=`-Zeile per `find_map`; in 97 Zeilen ist Zeilen-Robustheit
  kein Theorieproblem (Zeile 6 vs. 24). (Schritt: Parser auf exakten Key/letzte
  Zeile härten — `grind-flash`.)
- **`ned.json` Resumability** — `break` mitten im Slice vor `gh release upload`,
  1/40 Slices, jeder Lauf beginnt bei Cone 48 neu. (Schritt: `ned-cdn.yml`
  Shard-Loop resumierbar — `grind-pro`.)

## Stehender Pass (gemessen 2026-09-17)

- **Postfach** — `state/mail/` am Baum absent (gitignored); kein neuer externer
  Eingang; der Zustand steht allein in `docs/zustand/external-state.md:20`
  (fremd-modifiziert — besitzende Linie faltet).
- **CI-Status** — Watchdog-Snapshot `/tmp/opencode/ci_status.md` 21:21:56+02:00
  gelesen; dieser Push (neuer HEAD) macht den CI-Status-Eintrag in
  `docs/zustand/external-state.md:22` fällig — nicht angefasst (fremd-modifiziert).
- `git_safety --snapshot` → `refs/safety/1789673509`.

## Wartend (kein Auswahlpunkt)

- `planetary-odf-cdn` `35231817955` `in_progress` seit 14:10Z (rosetta+mro; updated
  18:08Z) — bei success wird `rosetta_odf.bin` present; dann Block-note auf present.
- `demeter-cdn` `35228716483`; `ned-cdn` `35258227020`; `ps1-cdn` `35257451098`;
  `allwise-cdn` `35263717637`; `health-check` `35245084696`.
- Present-Kette aus folge68: `harvest-dispatch` `35264838482` (queued),
  `harvest` `35264854799` (in_progress, `-f format=maven_tnf`).
- Failed (attempt 1): `swot-cdn` `35251359490`, `gedi-cdn` `35250788545`,
  `ci-check` `35250778775`.

## Benchmark

- `council` (pro/max) hielt das Timeout-/Register-Verdikt (richtig — Architektur).
- `research-max` maß die EDL-Route (richtig — mehrstufig).
- `grind-flash` (×3) baute Härtungen + timeout-Feld, scannte 62 Compiler, registrierte
  3 CDN-Assets, maß die magic-URL — Routine bleibt flash.
- `grind-pro` wählte einen unregistrierten Kandidaten (`fermi_4fgl`) — die Klasse
  „absent-Asset wählen" braucht die Registrierungs-Bedingung im Prompt; Flash-Scan
  war hier präziser.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/harvest.φ`, `phi/sources.φ`,
  `tools/utils/src/bin/harvest_reg.rs`,
  `.github/workflows/{harvest,harvest-dispatch}.yml`,
  `docs/handover/handover-2026-09-17-ernte-folge69.md`
  (+ archiviertes `handover-2026-09-17-ernte-folge68.md`).
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md` (modifiziert),
  `tools/utils/src/bin/archive_search/pdf.rs` (modifiziert), die gestagten
  Handover-Archiv-Renames (`handover-2026-09-16-*`), `handover-2026-09-17-bau-folge67.md`,
  `handover-2026-09-17-entscheid-folge37.md`,
  `docs/paper/twenty-second-band-ground-chain.md`,
  `tools/measure/src/bin/pioneer10_cell_census_probe.rs`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
