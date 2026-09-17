<!--
  title: Handover — Ernte-Folge 68 (Stand 2026-09-17)
  session: Ernte-Folge 68
  class: handover
  date: 2026-09-17
  sha256: 921ce1d63f2d131d0eec242a485c0f822159e9e9d5282ca2179da56928e0fdee
  status: live
-->
# Handover — Ernte-Folge 68 (2026-09-17)

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

## Harvest-Architektur — erster Atom steht, Migration + Beweise offen (härtester undatiert)

Der Council-Verdikt-Atom ist gebaut und Council-gehalten: `phi/harvest.φ` (ein
Block `maven_tnf`, `asset present`), `tools/utils/src/bin/harvest_reg.rs`
(`--lookup`/`--arm`/`--check`), `.github/workflows/harvest-dispatch.yml` (der eine
Dispatch-Punkt), `.github/workflows/harvest.yml` (der eine Harvest mit
Idempotenz-Gate). `cargo check -p omegaflow-utils` grün (0/0); `phi/harvest.φ`
per `!phi/harvest.φ` getrackt. `auto-dispatch.yml` steht unangetastet (kein
Löschen vor Parität).

- **Migration offen (Verdikt-Reihenfolge):** absent-Assets als `asset fehlt`
  registrieren → Dispatch; dann die 54/47 idempotenten Familien batchen;
  `demeter`/`ps1` (`idempotent false`) zuletzt; Aggregatoren als benannte
  Ausnahmen; `auto-dispatch` zuletzt falten. (Schritt: ersten absent-Asset-Block
  in `phi/harvest.φ` schreiben → Push → dispatch-Job beobachten — `grind-pro`.)
- **Ungelaufene Pfade, `pending`** — erster echter Beweis = absent-Assets-Atom:
  `fehlt → Dispatch → Gate-skip`; das Idempotenz-Gate selbst (`gh release view
  --repo omegaflow/sources pds-ppi.igpp.ucla.edu` + pattern-grep) ist nie
  gelaufen; der force-Lauf; die Void-Wickelung. Die present-Kette läuft gerade
  (beim Close `queued`): `harvest-dispatch` `35264838482` (vom Push `7ae68c34`)
  und `harvest` `35264854799` (`-f format=maven_tnf`) — Status `wartend`
  (Auslöser: Lauf-Ende). (Schritt: absent-Asset registrieren, dann je
  `gh run view <id>` einmal.)
- **Zweiter Zeuge prüft nur ∃ eines pattern-matchenden Assets, nicht die
  Shard-Vollständigkeit** (`shard 3`). (Schritt: Zeuge vergleicht Asset-Zahl mit
  dem `shard`-Feld — Shard-Politik Wand 2³¹ B / Budget 2³⁰ B.)
- **`--check` fängt keine doppelten Felder im Block** (Rust `field()` nimmt die
  erste Zeile, bash-`sed` die letzte — Divergenz). (Schritt: `--check` verlangt
  Feldeindeutigkeit.)
- **`timeout-minutes: 240`** am ersten echten Compile-Lauf messen und justieren.
  (Schritt: erster absent-Asset-Compile-Lauf.)

## CDN-Register-Schuld — Dispatches gemessen

- **Drei Dispatches success** (Fix-Commit `5f9cd61b`): vlass-tap `35251315622`,
  dl3-skymap-hess `35251318800`, maxi `35251322353`. (Schritt: neue
  `url`/`sha256`-Zeilen in `phi/sources.φ`, soweit das Asset geändert wurde —
  CDN-Manifestationspflicht; `grind-flash`.)
- **gedi + icesat2 + swot: protected-Bucket-403** — gedi-cdn `35251353969`
  (attempt 2) failure: S3-Listing `lp-prod-protected?prefix=GEDI02_A.002/GEDI02_A_2025190`
  → `curl (22) … 403` → void; icesat2-cdn `35251356600` (attempt 2) failure:
  `nsidc-cumulus-prod-protected?prefix=ATLAS/ATL03/007/2026/05/31/` → 403 → void;
  swot-cdn `35251359490` failure. Der Granule-/Slash-Date-/protected-first-Fix
  reicht nicht — die protected NASA-S3-Endpoints brauchen EDL-Token-Autorisierung.
  (Schritt: EDL-Token für die drei protected Buckets messen — `research-max`.)
- **`rosetta_odf` `35231817955`** (planetary-odf-cdn) `in_progress` (updated 18:08Z),
  **`demeter-cdn` `35228716483`** `in_progress` (updated 13:47Z, über 5 h auf
  `demeter-residential`). Status `wartend` (Auslöser: Lauf-Ende). (Schritt bei
  demeter-success: 77 `demeter_isl`-`url`+`sha256`-Zeilen + Felder
  `demeter_isl_{orbit_count,ne_cm3,ni_cm3,te_k,vf_v,vi0_ms}`, `ttl 604800` ans
  Ende `phi/sources.φ`.)
- **`swot_l2_lr_ssh`** — der Reorder reicht nicht; credentialisierte (SigV4)
  Listings beider Buckets → 403 → Void. Offene Ursache: EDL-Token-Autorisierung
  für `podaac-swot-ops-cumulus-protected`. (Schritt: Autorisierung messen —
  `research-max`.)
- **`ned.json` Resumability** — `break` mitten im Slice vor `gh release upload`,
  1/40 Slices, jeder Lauf beginnt bei Cone 48 neu. (Schritt: `ned-cdn.yml`
  Shard-Loop resumierbar machen — `grind-pro`.)
- **`magic_dl3`** — URL offen (`opendata.magic.pic.es` / Zenodo `11108474`).
  (Schritt: FITS-URL messen, dann `gh workflow run dl3-skymap-cdn.yml -f
  telescope=magic -f url=<fits-url>` — `grind-flash`.)

## Stehender Pass (gemessen 2026-09-17)

- **Postfach** — `state/mail/` ist am Baum absent (gitignored, `.gitignore:122`).
  `smail` ist send-only; der Leser ist `mail_digest`
  (`tools/service/src/bin/mail_digest.rs`) über `state/mail/mail_ledger.φ`
  (Cloudflare-Worker → `smail_recv` → Ledger). Kein neuer externer Eingang
  gemessen; der Zustand steht allein in `docs/zustand/external-state.md:20`
  (fremd-modifiziert — besitzende Linie faltet).
- **CI-Status** — Watchdog-Snapshot `/tmp/opencode/ci_status.md` 18:09:50Z gelesen;
  der Push dieses Atoms (`5f9cd61b` → neuer HEAD) macht den CI-Status-Eintrag in
  `docs/zustand/external-state.md:22` fällig — nicht angefasst (fremd-modifiziert).
- `git_safety --snapshot` → `refs/safety/1789664986`.

## Benchmark

- `council` (pro/max) hielt das Blatt: fand die **blockierende `.gitignore`-Falle**
  (`phi/harvest.φ` ungetrackt) + drei mechanische Nachbesserungen — pro/max war
  hier richtig (Architektur-/Abschluss-Entscheidung).
- `grind-max` (pro/max) baute die vier Dateien; `grind-flash` (flash) dispatchte
  die fünf CDN-Läufe; `explore` (flash) fand den Postfach-Leser. Routine-Klassen
  bleiben flash.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `.gitignore` (eine Zeile `!phi/harvest.φ`),
  `phi/harvest.φ`, `tools/utils/src/bin/harvest_reg.rs`,
  `.github/workflows/{harvest-dispatch,harvest}.yml`,
  `docs/handover/handover-2026-09-17-ernte-folge68.md`
  (+ archiviertes `handover-2026-09-17-ernte-folge67.md`).
- **Fremd (nicht anfassen):** gestagte Handover-Archiv-Renames
  (`handover-2026-09-16-*`, `handover-2026-09-17-bau-folge66.md`,
  `handover-2026-09-17-entscheid-folge36.md`), `handover-2026-09-17-bau-folge67.md`,
  `handover-2026-09-17-entscheid-folge37.md`, `docs/zustand/external-state.md`,
  `docs/paper/twenty-second-band-ground-chain.md`,
  `tools/measure/src/bin/pioneer10_cell_census_probe.rs`. Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
