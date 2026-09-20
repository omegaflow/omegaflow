<!--
  title: Handover — Bau-Folge 111 (Stand 2026-09-20)
  session: Bau-Folge 111
  class: handover
  date: 2026-09-20
  sha256: 2e130fef42ffd62958f77445c539e43f3cad39a276d12c98cb9ede707026f75d
  status: live
-->
# Handover — Bau-Folge 111 (2026-09-20)

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
Der Planungs-Pass nennt die offenen Punkte als Tafel (Punkt | Status | Bindung |
Schritt); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-20, Session-Beginn)

- **HEAD** `cf6b9799` (forschung folge119) == `origin/main` → gepusht, Fast-Forward.
  Snapshot Start `refs/safety/1789926226`. Arbeitsbaum fremd: `R`
  `handover-…-entscheid-folge63.md` → `archiv/`, `M`
  `phi/pipeline/probe_hapi_proposed.φ` + `probe_wave.φ`, `??`
  `handover-…-entscheid-folge64.md` — **nicht angefasst**.
- **Postfach** — letzter Ledger-Eingang `1789922257` (Pine64 `sales@` → Verweis
  `info@`); kein neuer Eingang. Zitiert aus `external-state.md` (Trigger
  „neuer Eingang / 2⁶ min" seit Forschung-Folge 119 nicht gefeuert).
- **CI** — `ci_manage view` 2026-09-20 (Bau-Folge 111): `ci-check`
  **`35526753935` pending @`cf6b9799`** (kein Verdikt); `te-gate` `35513982359`
  @`7d0a1272` **in_progress** seit 13:36Z (kein Update); `tools-build`
  `35526340075` success; ältere `ci-check` cancelled (Concurrency-Gruppe). Kein
  sauberes Verdikt am HEAD.
- **Zustand-Ledger** — `external-state.md` CI-Zeile auf HEAD `cf6b9799`
  fortgeschrieben (Trigger HEAD-Wechsel gefeuert); Postfach-Zeile zitiert.

## Offen

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| Queue-Korpora Re-Lauf (7 `parser-gap`, `ledger.φ:70–96`) | `wartend` | `termin` (tools-build nach Fix-Push) | Operator-Wort für den lokalen Lauf erteilt (2026-09-20); lokales Binär war **veraltet** (2026-09-14) → Frische-Lücke geschlossen (`bin/omegaflow` + `tools-build` publiziert das Kern-Binär nach `tools-latest`); Re-Lauf über `bin/omegaflow` mit `--port` + `--probe`, dann die Ledger-Notes fortschreiben. Eintrag: `external-state.md`. |
| `ci-check`-Verdikt am HEAD `cf6b9799` | `wartend` | `termin` (Run-Abschluss) | Run `35526753935` einmalig aus `/tmp/opencode/ci_status.md` / `ci_manage view` lesen; bei Rot `ci_manage log <id>`. |

Kein session-abarbeitbarer undatierter Punkt — der Queue-Punkt wartet auf das
frische Release-Binär (Run `35527312242`), der CI-Punkt ist Wartestellung. Die
Session erfindet keine Arbeit aus einem Wait.

**Messung dieses Atoms (kein Punkt):** der lokale `--port`/`--probe`-Lauf auf den
10 Queue-Korpora lief mit **veraltetem** `target/release/omegaflow` (2026-09-14) —
die identischen Zähler (13k 1048/8, 14k 874/8, 15k 1048/8, 183l 19/1, 2k 177/0,
7k 425/6, staging 50/8, astro 31/2, earth 4/0, exotic 16/0; `--probe` 0 Survivor,
41 declined „fetch returned void") messen den **Vor-Fix-Code** und sind kein
Ergebnis. Post-Fix-Binär nötig (Release `v2026-09-20` ist pre-Fix).

## Benchmark

- **Bau-Folge 111**: Register-/Übergabe-Atom (kein Code, kein Taucher). Kein
  Doppellauf — die Routine-Register-Klasse ist geschlossen (flash
  $0.0008–0.0017, `docs/concepts/tools-map.md`); die Session (build/flash) führt
  den stehenden Pass direkt. Kein pro/max-Bedarf.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `.github/workflows/tools-build.yml` (Kern-Binär `-p omegaflow` +
  Manifest + Upload), neues `bin/omegaflow` (Wrapper), `phi/pipeline/ledger.φ` (7
  `parser-gap`-Notes), `docs/zustand/external-state.md` (CI-Zeile + Binär-Zeile +
  Header-sha256), `docs/handover/post.md` (Tafel-Zeile Z.18 entfernt + Header-sha256),
  neues `docs/handover/handover-2026-09-20-bau-folge111.md`, Move
  `handover-2026-09-20-bau-folge110.md` → `archiv/`.
- **Fremd (nicht anfassen):** der `folge63`-Move, `phi/pipeline/probe_hapi_proposed.φ`,
  `phi/pipeline/probe_wave.φ`, `handover-2026-09-20-entscheid-folge64.md`. Nie ein
  nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
