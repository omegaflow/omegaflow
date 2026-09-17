<!--
  title: Handover — Ernte-Folge 66 (Stand 2026-09-17)
  session: Ernte-Folge 66
  class: handover
  date: 2026-09-17
  sha256: f748e1fad44ba1246af35afc27cc96a24e8fc603292fc9124ce8746f1b2347a6
  status: live
-->
# Handover — Ernte-Folge 66 (2026-09-17)

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

## Harvest-Architektur — Sofort-Entlastung gebaut, Struktur offen (härtester undatiert)

- **Gemessen:** 248 Workflow-Dateien, jede eine eigene Harvest→Compile→Upload-Kette
  in dasselbe Release `omegaflow/sources`; `auto-dispatch.yml` triggerte auf
  `.github/workflows/**` und dispatchte **jeden** geänderten Workflow → jede
  Editor-Änderung wurde zur Re-Harvest-Welle (mein 87-Datei-Sweep hätte 87 Läufe
  gefeuert); 242/248 `cancel-in-progress: false` stapelten die Duplikate; die
  2-GiB-Release-Grenze schlug zu (maven 422, Zustand-Ledger).
- **Sofort-Entlastung gebaut (dieses Atom):** `auto-dispatch.yml` von
  `.github/workflows/**` entkoppelt — Trigger jetzt `tools/harvest/src/bin/**`
  (echter Schema-/Compiler-Wechsel), Dispatch mappt den geänderten Compiler-Stem
  auf die Workflows, die `--bin <stem>` aufrufen; `cancel-in-progress: true` in den
  54 idempotenten Harvest-Workflows (53 geändert, 1 war schon `true`).
  `demeter-cdn`/`ps1-cdn` sind **nicht** idempotent und bleiben unangetastet.
- **Offen — Struktur (eigenes Atom, Council):** ein Master-Harvest-Register statt
  248 Einzel-Workflows — eine Dispatch-Stelle, ein Zustand `Asset fehlt/present`,
  und die 2-GiB-Grenze als Teil des Registers (Shard-/Split-Politik statt 422).
  (Schritt: `council`-Dispatch mit den gemessenen Zahlen; danach Atom.)
- **Offen — `phi/sources.φ`-Trigger:** eine neue/geänderte Quellen-Registrierung
  dispatte nicht mehr automatisch (kein generisches Mapping Format→Workflow).
  (Schritt: im Master-Register mitlösen, oder Sessions dispatchen explizit.)

## DEMETER — Runner läuft, Lauf in_progress

- `demeter-cdn` `35228716483` (`671f2bd9`) in_progress seit 13:41Z auf
  `runs-on: demeter-residential` — der Runner ist installiert, der Lauf läuft
  (kein Poll). (Schritt: beim nächsten Pass `ci_manage view 35228716483`; bei
  success die 77 `demeter_isl`-`url`-Zeilen + `sha256` ans Ende von `phi/sources.φ`.)
- Registrierung nach erfolgreichem Aggregat: 77 Zeilen `format demeter_isl`,
  `at earth`, `ttl 604800`, Felder
  `demeter_isl_{orbit_count,ne_cm3,ni_cm3,te_k,vf_v,vi0_ms}`.

## False-Green `|| gh_issue_once` — Sweep + Gate gebaut, Live-Abnahme offen

- 128 `||`-Swallow-Stellen in 87 Workflows auf
  `|| { bash .github/workflows/scripts/gh_issue_once.sh …; exit 1; }` umgestellt
  (`gh_issue_once.sh` blieb byte-identisch, die Dedup erhalten); 18 gesunde
  Standalone-Formen (`if: failure()` in `ci-check.yml`/`health-check.yml`/`te-gate.yml`/
  `kernel-flatten.yml`/`paper-check.yml`, report+`exit 1` in `tnbfits-cdn.yml`,
  `gll-ck-cdn.yml`/`jwst-cdn-watch.yml`/`radio-cdn-watch.yml`/`de441-cdn-watch.yml`)
  unangetastet. Gate-Marker `|| bash .github/workflows/scripts/gh_issue_once.sh`
  in `src/gate/commit_gate_vocab.json` + Tests
  `fp_tool_bare_or_swallow_gh_issue_blocked` / `fn_tool_wrapped_and_standalone_gh_issue_pass`
  in `src/gate/commit_gate.rs`. Gemessen: der Pre-commit-Hook scannt nur gestagte
  `.rs`; `.yml`/`.json` laufen über den Live-Tool-Gate, der Marker ist substring-scoped
  (kein Selbst-Trip).
- **Offen: die Live-Abnahme.** (Schritt: nach dem Push `gh workflow run swot-cdn.yml`
  — der void-Lauf muss jetzt ROT enden mit dedupliziertem Issue; Ergebnis beim
  nächsten Pass, kein Poll.)

## CDN-Register-Schuld — 5 registriert, 10 weiterhin absent (gemessen)

- Registriert (sha256 gesetzt): `cosmic_ro_temp`, `noe4_deimos`, `noe4_phobos`,
  `isc_bulletin`, `uscrn_hourly`.
- Weiterhin absent; der „success"-Lauf war falsch-grün (Void vom `||`-Swallow
  verschluckt), gemessene Ursache je Quelle:
  `swot_l2_lr_ssh` (S3-Listing `podaac-swot-ops-cumulus-public` void),
  `gedi_l2a` (S3-Listing `lp-prod-protected/GEDI02_A.002/2026.09.15/` void),
  `icesat2_atl03` (S3-Listing `nsidc-cumulus-prod-public/ATLAS/ATL03/2026.09.15/` void),
  `maxi_J0006+202` (Kernel `J0006+202_g_lc_1day_all.dat` fehlt),
  `vlass_tap_component`/`vlass_tap_source` (Compiler void),
  `hess_dl3`/`magic_dl3` (Compiler void),
  `ned.json` (nur erster Shard `ned_part_00000.json` gelandet),
  `rosetta_odf` (Lauf `35231817955` pending).
  (Schritt: nach dem Push jeden Workflow einzeln neu dispatchen — jetzt rot statt
  falsch-grün — und die Void-Ursache je Quelle mit Compiler/Probe messen;
  `grind-pro`/`research-max`.)
- `juno_ocru_odf` weder registriert noch am CDN (`atmos.nmsu.edu`). (Schritt:
  `juno_ocru_odf.bin`-Block + CDN-Route messen.)

## CI-Queue / Idempotenz-Gates — Beobachtung

- `max-parallel: 8` (`gaia-xp-full-cdn.yml`), `max-parallel: 4` (`physionet-cdn.yml`)
  gesetzt; keine Neu-Dispatches von gaia/physionet, bis Slots frei. 47 `*-cdn.yml`
  auf das aia-Idempotenz-Muster; Nachweis über die nächsten regulären Läufe.
  Die Queue-Pile-ups sind mit der Sofort-Entlastung oben adressiert.
- Watchdog 17:05Z: `demeter-cdn`/`ps1-cdn`/`physionet-cdn`/`planetary-odf-cdn`
  in_progress, `gaia-xp-full-cdn` queued; `pii-exposure`/`paper-check` failure
  (fremde Linien).

## Stehender Pass (gemessen 2026-09-17)

- Postfach: TRISP/MLZ — T. Keller (MPI-FKF, TRISP) sendet die NSE I(q,t)-Daten
  (Haug et al. 2010) in wenigen Tagen (`mail_ledger`, 09:29Z); GitHub PAT classic
  `omegaflow-ci-write` angelegt; Rubin-Forum-Zusammenfassung. `docs/zustand/external-state.md`
  ist fremd-modifiziert — Postfach/CI-Status-Eintrag dort nicht angefasst; die
  besitzende Linie faltet sie beim nächsten Pass.
- CI-Status am HEAD `84489abb`: demeter/allwise in_progress.

## Benchmark

- Fünf parallele `grind-flash`-Sweeps (je ~20 Workflow-Dateien, mechanisches
  `||`-Wrap): alle korrekt und vollständig, keine Korrektur nötig. Der `council`-Dispatch
  trug das Urteil (Sweep + Gate, ein Atom); `grind-max` baute das Gate
  (Marker + Fixtures + Tests, `cargo check`/`--tests` 0/0). Ein weiterer
  `grind-flash` setzte `cancel-in-progress: true` in 53 idempotenten Workflows
  (1 war schon `true`), korrekt. Routine-Klassen bleiben flash-geschlossen.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/sources.φ`, `.github/workflows/*.yml` (130 Dateien:
  Sweep ∪ `cancel-in-progress` ∪ `auto-dispatch.yml`),
  `src/gate/commit_gate.rs`, `src/gate/commit_gate_vocab.json`,
  `docs/handover/handover-2026-09-17-ernte-folge66.md`
  (+ archiviertes `handover-2026-09-17-ernte-folge65.md`). Fremde uncommittete
  Arbeit (nicht anfassen): die gestagten Renames
  `handover-2026-09-16-{entscheid-folge24,forschung-folge44,forschung-folge51}` → `archiv/`,
  `docs/zustand/external-state.md`, `docs/concepts/tools-map.md`,
  `tools/utils/src/bin/archive_search.rs`, `tools/utils/src/bin/archive_search/pdf.rs`.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
