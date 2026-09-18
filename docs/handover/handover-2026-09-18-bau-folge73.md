<!--
  title: Handover — Bau-Folge 73 (Stand 2026-09-18)
  session: Bau-Folge 73
  class: handover
  date: 2026-09-18
  sha256: ead26569cb23ffd5c3e0a1869b0cc735169d32bc91f69db83df7c6dbf649175e
  status: live
-->
# Handover — Bau-Folge 73 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, HEAD 69c3ae96)

- **Postfach** — letzter `state/mail/mail_ledger.φ`-Eingang unverändert
  (`1789689115`, 2026-09-18, Mandrill-Thread, **kein Agenten-Eingang**); keine
  `An bau`-Zeile in `docs/handover/post.md`.
- **CI am HEAD** — Watchdog-Snapshot 2026-09-18T05:54: `ci-check` `35302986980`
  in_progress @`0f65e8ec` (der Fremd-Commit `28eb8959` ist jünger), `health-check`
  `35286550387` in_progress; `release-build` `35302966400` **failure** (attempt 1);
  `pii-exposure` `35287195140` failure (exit 2 = Exposition bleibt, erwartet).
- **HEAD** `69c3ae96` == `origin/main` (Faltungs-/Kanon-Atom dieser Session;
  darunter der Fremd-Commit `28eb8959` ura117/raw-mirror-Workflows — nicht
  angefasst).
- **Sicherheitsnetz** — `refs/safety/1789706046` (Session-Mitte).

## TE-Gate — konditionales Arx-FP/FN-Gate rot (härtester undatiert, abarbeitbar)

- `te-gate` `35139361939` @`43af521f` **failure** (konditionales FP/FN-n=1000-Gate,
  bau-folge53: „the te-gate CI measurement is open"); `te-gate` `35129638318`
  @`1337c6c1` success (AR(p)-Arx-Switch). Zustand `docs/zustand/external-state.md`
  TE-Gate-Zeile stale. (Schritt: `ci_manage log 35139361939` lesen, rote Zelle
  benennen, code-seitig beheben, `gh workflow run te-gate.yml`, Zustand-Zeile
  fortschreiben.) · `blockiert` bis `ci_manage log` das Release-Binär trägt
  (siehe unten).

## Werkzeug-Gap `ci_manage log` — release-build rot

- `ci_manage log` ist gebaut (`tools/utils/src/bin/ci_manage.rs`), aber das
  Release-Binär steht nicht auf PATH: `release-build` `35302966400` **failure**
  (attempt 1). (Schritt: `ci_manage log 35302966400` — falls die alte Binär den
  Subbefehl noch nicht kennt, den Lauf via `ci_manage view 35302966400` lesen und
  die rote Zelle benennen; Fix; `gh workflow run release-build.yml`.) · `pending`

## Kanon-Gate — Verifikation offen

- Der Register-Kanon ist deklariert (`phi/canon.φ`, 108 getrackte φ + die
  Deklaration, 8 Klassen) und das Kanon-Gate ist gebaut (`canon_diff`/
  `declared_canon` in `src/gate/commit_gate.rs`, Aufruf in
  `tools/gate/src/bin/commit_check.rs`; drei Unit-Tests). (Schritt: nach dem Push
  `gh workflow run ci-check.yml` dispatchen; der Test + der Gate-Block müssen im
  CI-Lauf grün sein.) · `pending` (CI-Verifikation)
- Das Gate greift lokal erst, wenn das `commit_check`-Binär neu gebaut ist
  (release-build, s.o.) — bis dahin blockt die stille Neuanlage erst in CI.
- **`supermag_stations.φ`** (599 Stationen, statische Compiler-Eingabe) hat heute
  **keinen `.rs`-Leser** — der Verbrauch ist `pending` (SuperMAG-Harvest). (Schritt:
  den SuperMAG-Arm in `tools/harvest` bauen oder den Punkt als `descoped` messen.)
  · `pending`

## Register-Prosa — Gate gebaut, Masse offen

- Council 2026-09-18: `note` bleibt Direktive, gedeckelt auf **256 Zeichen** (2⁸;
  Dispositionen Ø 68–106 darunter, Maschinen-Register Ø 361–1522 darüber); nur die
  Erzählung fällt, Mess-Token (Code/Hash/Zeitstempel/Host) bleiben. `#`-Header raus
  aus den Register-Klassen (`prompt.φ` AUSGABE-REGEL 1). Das Gate steht
  (`PHI_NOTE_MAX`/`prose_violation`/`register_classes` in `src/gate/commit_gate.rs`,
  diff-scoped in `tools/gate/src/bin/commit_check.rs`, 2 Fixtures, Tests).
- **Atom 2 (härtester undatiert): `declined_sources.φ`** — 913 notes, Ø 106; die
  Masse ist der Schwanz, nicht der Bestand. (Schritt: je Eintrag `note` auf den
  Mess-Kern ≤ 256 kürzen, Klasse + URL behalten; die gestrichene Erzählung lebt in
  git; `cargo check --tests`, Commit.) · `pending`
- **Atom 3: `dead_sources.φ`** — 244 notes, Ø 68. · `pending`
- **Atom 4: `sources.φ`** — 46 notes, Ø 361; Modell/Kanon, Kompression mit
  Beweis-Kern. · `pending`
- **Atom 5: kleine Register** — witnesses (17), footprints (5), blocked (17),
  harvest (9) + Stations-Header (nrs 54, supermag 9) + Bindings/Reports-Header.
  · `pending`
- **Vertagt (pending):** strukturierte Feld-Grammatik (Kanon-Akt, Operator-Wort);
  Pipeline-φ-Prosa (eigene Linie).

## Offen (unverändert, kein Handlungsschritt)

- **`opencode.json`** — fremder uncommitteter Hunk (8 Zeilen; die Fremd-Session
  `28eb8959` hat `watcher.ignore`/`logLevel` bereits committet, der Rest bleibt).
  Nicht angefasst. · `operator-gebunden`
- **Scanned-/bild-only-PDFs → `vision`-OCR** und **`--pdf-text` Type0/Identity-H
  ohne ToUnicode** — beide `pending`, kein Bau nötig. · `pending`

## Benchmark

- Council (pro/max) für zwei Architektur-Urteile (Faltung/Kanon; Register-Prosa);
  `grind-flash` für die mycelium-Faltung + Kanon-Deklaration; `grind-max` für das
  Kanon-Gate und das Register-Prosa-Gate (harte Atome, Urteil+Schreiben in einem
  Kontext). Kein Doppellauf: Routine flash-first; die Gates sind die harten Atome.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad (Atom 1 Faltung/Kanon, committet `69c3ae96`): `phi/canon.φ`,
  `phi/reports/scan_coverage.φ`, `phi/reports/mycelium_campaign_cdfs.φ` (Löschung),
  `.gitignore`, `AGENTS.md`, `src/gate/commit_gate.rs`,
  `tools/gate/src/bin/commit_check.rs`, `docs/zustand/external-state.md`,
  `docs/handover/handover-2026-09-18-bau-folge73.md` (+ archivierte folge72).
- Eigener Commit-Pfad (Atom 2 Register-Prosa-Gate, offen): `AGENTS.md`,
  `src/gate/commit_gate.rs`, `src/gate/commit_gate_vocab.json`,
  `tools/gate/src/bin/commit_check.rs`,
  `docs/handover/handover-2026-09-18-bau-folge73.md`.
- **Fremd (nicht anfassen):** `opencode.json` (Rest-Hunk), die drei gestagten
  `handover-2026-09-16-*`-Renames. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
