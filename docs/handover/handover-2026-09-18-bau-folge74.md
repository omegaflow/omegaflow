<!--
  title: Handover — Bau-Folge 74 (Stand 2026-09-18)
  session: Bau-Folge 74
  class: handover
  date: 2026-09-18
  sha256: b65cfec18c57b4e2edd904bf9df2eaf0f64199c4ab2444f495806cf298aeafba
  status: live
-->
# Handover — Bau-Folge 74 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, HEAD e0c27635)

- **Postfach** — letzter `state/mail/mail_ledger.φ`-Eingang `1789689115`
  (2026-09-18, Mandrill-Thread, **kein Agenten-Eingang**); keine `An bau`-Zeile
  in `docs/handover/post.md` (die dortigen Zeilen sind an forschung/ernte/entscheid).
- **CI am HEAD** — `ci-check` `35307448019` @`69c3ae96` in_progress;
  `release-build` `35302966400` **failure** (attempt 1); `pii-exposure`
  `35287195140` failure (exit 2 = Exposition bleibt, erwartet); `xp-pilot-cdn`
  `35305640254` failure; sonst fremde Linien (CDN/harvest).
- **HEAD** `e0c27635` == `origin/main` (Register-Prosa-Gate-Atom der Folge 73).
- **Sicherheitsnetz** — `refs/safety/1789708371` (Session-Start).

## TE-Gate — konditionales Arx-FP/FN-Gate rot (härtester undatiert, entblockt)

- Rote Zelle **gemessen** 2026-09-18 via `ci_manage log 35139361939` (der
  Subbefehl läuft jetzt; s. Werkzeug unten):
  `conditional FP/FN gate: FPR rise 6.00pp over a at rho=0.5 exceeds 2pp` bei
  `src/mathematikerin/te.rs:3726`. Gitter a∈{0,0.5,0.9} × rho∈{0,0.5,0.9}:
  2/100 · 3/100 · 2/100 · 1/100 · 4/100 · 7/100 · 6/100 · 5/100 · 2/100.
  (Schritt: den FPR-Anstieg über a bei rho=0.5 im konditionalen Arx-Gate
  `gate_conditional_arx_fpr_fn_n1000` beheben — Null/Schwelle in
  `src/mathematikerin/te.rs` — `cargo check --tests`, `gh workflow run te-gate.yml`,
  `docs/zustand/external-state.md` TE-Gate-Zeile fortschreiben.) · `pending`
  (hartes Atom: `grind-max`, TE-/Null-Urteil + Schreiben in einem Kontext).

## Werkzeug-Gap `ci_manage log` — geschlossen (dieser Atom)

- `~/.local/bin/ci_manage` war eine veraltete **Datei-Kopie** (kein Symlink) ohne
  `log`-Subbefehl; `bin/ci_manage` (Wrapper) baut `target/release/ci_manage` bei
  neuerer Quelle. **Behoben:** Symlink `~/.local/bin/ci_manage ->
  target/release/ci_manage` (Muster wie `archive_search`); `ci_manage log
  35139361939` läuft. `release-build` ist **nicht** die Wurzel (baut
  `default-members=["."]`, nur den Kern). Bei stale Binär: `bin/ci_manage` aufrufen.
  (Kein offener Punkt; Infrastruktur der Operator-Maschine, hier registriert.)

## Register-Prosa — Atom 2–5 (Masse abgetragen, Rest in Arbeit)

- **`declined_sources.φ`** — 78 notes ≤256 gekürzt, 5 `#` entfernt. · erledigt.
- **`dead_sources.φ`** — 9 notes ≤256, 12 `#` entfernt (1 Falt-note). · erledigt.
- **`blocked_sources.φ`** — 17 notes ≤256, 12 `#` entfernt (1 Falt-note). · erledigt.
- **`witnesses.φ`** — 17 notes ≤256, 9 `#` entfernt (1 Falt-note). · erledigt.
- **`footprints.φ`** — 5 notes ≤256, 49 `#` entfernt (12 Falt-notes). · erledigt.
- **`nrs_stations.φ`** — 54 `#` entfernt (Kopf+Kreuz in 4 notes ≤256), 13
  Datenzeilen byte-identisch. · erledigt.
- **`supermag_stations.φ`** — 9 `#` entfernt (1 note), 599 Datenzeilen
  unverändert. · erledigt.
- **`reports/scan_coverage.φ`** (1 `#`) + **`reports/probe_sweep_survivors.φ`**
  (7 `#` → notes). · erledigt.
- **`sources.φ` / `harvest.φ`** — 26/6 notes ≤256 gekürzt; mit dem
  forschung-Commit `af64a132` (Ulysses-ATDF) mitgegangen, jetzt 0 überlange
  notes. · erledigt.
- **Bindings-Prosa (neuer Befund):** `phi/bindings/dust-maske.φ` und
  `bathymetrie-gebco.φ` sind **Prosa-Preregistrierungen** mit `#`/`##`-Markdown;
  die `#`-freie Register-Regel kann sie nicht ohne Restrukturierung decken (der
  Compiler liest sie als Text, `--mask-run`). (Schritt: entscheiden — Bindings
  bleiben Prosa (exempt) oder die Prosa wandert in ein Konzept-Dokument, das
  Register trägt nur Direktiven; Architektur-Akt mit Operator-/Council-Wort.) ·
  `pending`
- **Vertagt (pending):** strukturierte Feld-Grammatik (Kanon-Akt, Operator-Wort);
  Pipeline-φ-Prosa (eigene Linie).

## Kanon-Gate — Verifikation offen

- Kanon deklariert (`phi/canon.φ`, 108 φ + Deklaration) und Gate gebaut
  (`canon_diff`/`declared_canon`, Aufruf in `tools/gate/src/bin/commit_check.rs`).
  (Schritt: nach dem Push `gh workflow run ci-check.yml`; Test + Gate-Block im
  CI-Lauf grün.) · `pending` (CI-Verifikation)
- Das Gate greift lokal erst, wenn das `commit_check`-Binär neu gebaut ist — bis
  dahin blockt die stille Neuanlage erst in CI.
- **`supermag_stations.φ`** (599 Stationen, statische Compiler-Eingabe) hat
  **keinen `.rs`-Leser** — Verbrauch `pending`. (Schritt: SuperMAG-Arm in
  `tools/harvest` bauen oder als `descoped` messen.) · `pending`

## Offen (unverändert, kein Handlungsschritt)

- **`opencode.json`** — fremder uncommitteter Hunk. Nicht angefasst. ·
  `operator-gebunden`
- **Scanned-/bild-only-PDFs → `vision`-OCR** und **`--pdf-text` Type0/Identity-H
  ohne ToUnicode** — beide `pending`, kein Bau nötig. · `pending`

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit-Pfad:** `phi/declined_sources.φ`, `phi/dead_sources.φ`,
  `phi/blocked_sources.φ`, `phi/witnesses.φ`, `phi/footprints.φ`,
  `phi/nrs_stations.φ`, `phi/supermag_stations.φ`,
  `phi/reports/scan_coverage.φ`, `phi/reports/probe_sweep_survivors.φ`,
  `docs/handover/handover-2026-09-18-bau-folge74.md` (+ archivierte folge73).
- **Fremd (nicht anfassen):** die Handover-Renames/Deletions der Linien
  (forschung `7dc620a7`, entscheid `b02f5cda`), `docs/handover/post.md`,
  `opencode.json`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
