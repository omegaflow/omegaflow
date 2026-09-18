<!--
  title: Handover — Ernte-Folge 78 (Stand 2026-09-18)
  session: Ernte-Folge 78
  class: handover
  date: 2026-09-18
  sha256: 0318b9e651d284d43a988b41f1bc6a3c66906411ca062fc37ef405f994d123c1
  status: live
-->
# Handover — Ernte-Folge 78 (2026-09-18)

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

- **HEAD** `79062072` == `origin/main` (`ernte: register the three harvest dispatch
  run ids … and ci-check @6295dd5e`); die Linie startete auf `627ccea5` (folge77).
- **Postfach** — kein neuer Agenten-Eingang; Ledger-Tail ist Rubin-/LSST-Forum-Mail
  (`state/mail/mail_ledger.φ` gelesen).
- **CI** — die drei `harvest`-Läufe `35316395857`/`35316398029`/`35316400165`
  **failure** (`EARTHDATA_EDL_TOKEN absent`, gemessen per `ci_manage log`); die
  Legacy-`gedi/icesat2/swot-cdn` `35316407674`/`35316409631`/`35316411860`
  failure (protected-bucket 403); zweite `harvest`-Welle `35316442543`/`35316444708`/
  `35316448901` failure; `rosetta_odf` `35312992622` (wartend); `allwise-cdn`
  in_progress; sonst fremde Linien. Die Zustand-Ledger-CI-Zeile ist auf `79062072`
  gezogen.

## harvest.yml — EDL-Token-Env fehlte; Legacy-cdn retired (Fix steht, Re-Dispatch offen)

- Ursache gemessen (`ci_manage log 35316395857`): `EARTHDATA_EDL_TOKEN absent —
  the environment and .secrets.local carry no token`. `harvest.yml` `env:` trug nur
  `GH_TOKEN`/`RUSTFLAGS`; der CMR→GetObject-Arm verlangt das EDL-Token. Fix:
  `EARTHDATA_EDL_TOKEN: ${{ secrets.EARTHDATA_EDL_TOKEN }}` in `harvest.yml`
  `env:` ergänzt. Der CMR-Arm selbst ist korrekt (`--cmr --day … --version/--short-name …`,
  Defaults `--limit 2`/alle Beams — die Legacy-`--beams 8`/`--limit 2` waren äquivalent).
- Council-Verdikt (einstimmig, **retire**): `.github/workflows/{gedi,icesat2,swot}-cdn.yml`
  gelöscht — sie riefen dieselben Bins auf dem alten S3-Listen-Pfad (403) und wurden
  von `auto-dispatch` (Bin-String-Match) bei jedem Compiler-Push ins Leere gefeuert,
  während `harvest-dispatch` bereits `harvest.yml` dispatchte. `phi/harvest.φ` +
  `harvest.yml` trägt die drei Formate; nur die drei wurden retired, die übrigen
  `*-cdn.yml` bleiben.
  (Schritt: nach Push `gh workflow run harvest.yml -f format=gedi_l2a`, `-f
  format=icesat2_atl03`, `-f format=swot_l2_lr_ssh`; Run-IDs registrieren; Asset via
  `ci_manage view <id>`; nach success `phi/harvest.φ`-`asset fehlt` → `present` +
  `note` mit size/sha256.)

## gedi/icesat2 — non-CMR-Arm ohne Workflow (offen)

- Nach dem Retirement ruft kein Workflow mehr `gedi_l2a_compiler`/
  `icesat2_atl03_compiler` im non-CMR-Modus (`--day --beams`, S3-Listing); der Code
  bleibt als lokaler/manueller Arm stehen. (Schritt: Disposition — lokaler Pfad
  benannt lassen oder descope mit Befund; `sgrep "PRODUCT_ROOT"` + `--help`.)

## rosetta_odf — Workflow-Fix, Re-Dispatch offen (wartend)

- `harvest.yml` Zwei-Job-Fix gepusht (`6295dd5e`); der Re-Dispatch wartet auf den
  Abschluss von `35312992622` (alter Workflow). (Schritt: Re-Dispatch
  `gh workflow run harvest.yml -f format=rosetta_odf` ohne `-f timeout`; Ergebnis via
  `ci_manage view`; Run-ID registrieren.)

## Werkzeug-Wrapper — AGENTS.md-Satz (fremd)

- `AGENTS.md` ist fremd-modifiziert; der Satz „`bin/archive_search` rebuilds only
  when stale …" gehört der Linie, die `AGENTS.md` besitzt.

## Waiting (kein Auswahlpunkt)

- `ulysses_atdf_x` X-Ref-Fenster (Auslöser = nächstes `atdf`-Atom);
  `lro_utf`/`§4 census`/`bepicolombo`/`rosetta ungelaufene Pfade`/`auto-dispatch`/
  fünf Familien-Blöcke — Auslöser unverändert.

## Benchmark

- Hard atom (CI-Root-Cause + Architektur): `council` (pro/max) — Verdikt **retire**,
  einstimmig. Routine (env-Zeile, `git rm`, `ci_manage log`): `grind-flash`. Kein
  neuer Doppel-Lauf — die Routineklasse trägt ihren registrierten Sieger.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `.github/workflows/harvest.yml`,
  `.github/workflows/{gedi,icesat2,swot}-cdn.yml` (gelöscht),
  `docs/zustand/external-state.md` (CI-Zeile),
  `docs/handover/handover-2026-09-18-ernte-folge78.md` (+ archiviertes
  `handover-2026-09-18-ernte-folge77.md`).
- **Fremd (nicht anfassen):** `AGENTS.md`, `docs/SOURCE_PORT.md`, `opencode.json`,
  `src/archivar/{extract,mod,tests}.rs`, `src/archivar/arpansa.rs`,
  `tools/register/src/bin/register_lookup.rs`, `docs/handover/post.md`, die drei
  `handover-2026-09-16-*`-Renames, `handover-2026-09-18-entscheid-folge46.md`. Nie
  ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
