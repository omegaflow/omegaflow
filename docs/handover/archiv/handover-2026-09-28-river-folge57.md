<!--
  title: Handover — River-Folge 57 (2026-09-28)
  session: River-Folge 57
  class: handover
  date: 2026-09-28
  sha256: 6d2f0e500649035ebc829cb4a88e5e6387a509a73c63e7b8e892950e818a7bc1
  status: live
-->
# Handover — River-Folge 57 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`. Nur eigene Arbeit: pfad-begrenzter Commit,
fremde uncommittete Arbeit unangetastet.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md`" | 2026-09-27 | Operator (Session, River 49)
„du sollst keine operator worte tragen das ist sache von future du bist bis zur kante" | 2026-09-27 | Operator (Session, River 48)
„die ttl muss die Aktualisierung der Quelle sein" | 2026-09-27 | Operator (Session, River 47)
„#body erzeugt das bias … komplett rückgängig" — kein Körper privilegiert | 2026-09-27 | Operator (Session, River 47)
„frag den rat" / „folge dem rat" | 2026-09-27 | Operator (Session, River 47)
GIC-Paper-Einreichung: höchste Priorität | 2026-09-27 | Operator-Wort
Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort
Harte-Läufe-LOCK aufgehoben | 2026-09-26 | Operator-Wort
HTTPS ja | 2026-09-26 | Operator-Wort folge36
Entscheidungen nie als Liste vorlegen — jede braucht eine Erklärung | 2026-09-27 | Operator (Future-Session)
„die Kante bin ich" — Wert, Wort, Dritt-Akt und Send bleiben seine Hand | 2026-09-27 | Operator (Future-Session)
ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben | 2026-09-27 | Operator (Future-Session)
`/consent` — session-weiter Delegations-Consent, nicht das Commit-Wort | 2026-09-27 | session-weiter Consent (`/consent`)
Commit-Wort (`/commit`) — pfad-begrenzter Commit + Push, das Doppel-Ask | 2026-09-27 | Operator
„Erste Handlung: `sread docs/concepts/tool-forms.md`" | 2026-09-28 | Operator (Session, River 57)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) … Commit und Push trägt `/commit`." | 2026-09-28 | Operator (Session, River 57) — session-weiter Delegations-Consent
„hast du alles eigene bis zur kante gemessen und geplant?" | 2026-09-28 | Operator (Session, River 57)
„warum ist das binary nicht aktuell?" | 2026-09-28 | Operator (Session, River 57)
„bitte fixen" | 2026-09-28 | Operator (Session, River 57) — Werkzeug-Frische

## Offen (aufgeschlüsselt)

### Membran-Parität — CI-Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `test`-Job eines `ci-check`-Laufs auf dem Fix-HEAD schließt ab.
- **Lage:** (gemessen 2026-09-28 via `ci_manage status`/`jobs`/`log 36423924944` + `bin/ci_triage`) Der Fix `618871109` (te.rs `causal_pair_at_mi_lag` + Test-Rewiring) steht. Die **cancelled**-Läufe `36423691576`/`36423611573`/`36423483765` sind **Pending-Duplikate** (GitHub-Concurrency `cancel-in-progress: false` cancelt wartende Doppel, nicht den laufenden; der Watchdog hat `ci-check` nie gecancelt — `/tmp/opencode/ci_watchdog.log: no median basis`). Der lebende Lauf `36423924944` (HEAD `4ce2f1823`): `build`/`format` success, `test` in_progress, `dropped-gate` **failure** (operator-gebundene Baseline, `delta 6`), `clippy` **failure** — zwei `collapsible_if` in **River-Dateien** (`matrix.rs:335`, `te.rs:2895`), in diesem Atom geheilt.
- **Blockade:** keine.
- **Braucht:** `gh workflow run ci-check.yml` (der Push triggert ihn ohnehin); `ci_manage jobs <id>` — der `test`-Job trägt `membrane_forward`; grün = Gate zu. `dropped-gate` ist mit `dropped-baseline 1060` geschlossen (Operator-Wort 2026-09-28, dieser Atom); der Gate-Umbau auf selbst-messend/träger-bewusst steht als Absender-Zeile an Mycelium. Kein Run-ID-Pin (jeder Push überholt ihn).

### ksg-k-Fix — te-gate-Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein `te-gate`-Lauf auf dem Fix-HEAD schließt ab (Job `ksg-k-gate`).
- **Lage:** (gemessen 2026-09-28 via Code-Lesen) `ksg_k.rs:166` misst jetzt `ksg_te_phase_null(&b,&a)` — die wahre Kopplung a→b; zuvor maß der FN-Loop die Nullrichtung (`target→driver`), daher `sweep=void`/FNR 100 %. `cargo check` 0/0.
- **Blockade:** keine.
- **Braucht:** `gh workflow run te-gate.yml`; `ci_manage log <id>` — Job `ksg-k-gate`. Kein Run-ID-Pin.

### 2D-Voronoi-Probe (Träger `survey-messpunkt-verteilung.md`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** keiner — dispatchbar; Beleg `tools/measure/src/bin/a_posteriori_placement_probe.rs:143`.
- **Lage:** (gemessen 2026-09-28 via `sgrep -i voronoi src/`) 0 Treffer; `tools/measure/src/bin/a_posteriori_placement_probe.rs:143` nennt „2D Voronoi/quadtree topology … the named next step"; River 47 maß die 1D-radiale Variante (N_adaptiv/N_analytisch 0,75–1,50 — kein Ordnungsgewinn).
- **Blockade:** keine.
- **Braucht:** `a_posteriori_placement_probe.rs:143` auf 2D-Voronoi-Topologie erweitern, dann `docs/surveys/survey-messpunkt-verteilung.md:94` als gemessen schließen.

### Flyby-Path-2 (revised) — δ gemessen, Benotung offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neuer CDN-`release` `ephemeris_juice.bin` samt 1-σ-Kovarianz.
- **Lage:** (gemessen 2026-09-27) δ = 0,1684732 km (DE441 vs DE442; `phi/sources.φ:3448`/`:3469`). Δ/σ_recon `pending`.
- **Blockade:** keine.
- **Braucht:** nach dem Flyby `cargo run -p omegaflow-measure --bin flyby_ephemeris_gate -- --recon <arc> --sigma-recon <km>`; Riß gegen beide Hashes tragen.

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin | **Bindung:** termin:2026-09-29
- **Trigger:** 29.09. 00:00 UTC Snapshot.
- **Lage:** (gemessen 2026-09-28 via `ci_manage view`) 28.09-Snapshot-Lauf `36421869230` success (`flyby-path2-fill.yml`).
- **Blockade:** 29.09.-Snapshot fehlt.
- **Braucht:** nach 29.09. 00:00 UTC `gh workflow run flyby-path2-fill.yml`; Artefakt `flyby-path2-fill`.

## An fremde Feder (Absender-Zeile — Aufenthalt = Eigentum)

Ziel ist die **lebende** Übergabe der fremden Linie; der Eigentümer faltet die Zeile
in seinem Pass.

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| dropped-gate-Umbau — selbst-messend/träger-bewusst statt handgepflegter Absolutwert | `docs/handover/handover-2026-09-28-mycelium-folge196.md` (Mycelium) | River-Session 57 (gemessen `register-dropped 36423401422` + `dropped-baseline.md`) | Der Zähler rattert (984→1060 in einem Tag, jedes Mal „Absorbiert durch Bump"). Bau: (a) `register_lookup --dropped` **träger-bewusst** — nur zählt, was weder Commit noch Register-Aufenthalt hat (die `git: none`-Klasse zählt nachweislich Getragenes, z. B. `Europa-Clipper` → `state/zustand/wartend.φ:15`); (b) `dropped-gate` gegen den gemessenen **Vor-HEAD-Zähler** statt gegen den handgepflegten Absolutwert. Dann braucht das Gate nie wieder einen Bump. |
| `devcontainer`-CLI in einem Actions-Job (Umgebungs-Parität) | `docs/handover/handover-2026-09-28-mycelium-folge196.md` (Mycelium) | River-Session 54 (Operator-Wort) | `.devcontainer/devcontainer.json` von keinem Workflow konsumiert (gemessen 2026-09-28 via `sgrep -i devcontainer .`); Job mit `devcontainer up`/`exec` soll die Toolchain reproduzierbar in CI stellen. |
| 81-Block-Fix — `format ephemeris_binary`, `ttl 86400` | `docs/handover/handover-2026-09-28-mountain-folge196.md` (Mountain) | river folge51/52/53 | Prüfintervall aus Live-Release-Abständen oder 2²⁵ s ≈ 388 d; AGENTS.md-Präzisierung im selben Atom. |
| Blatt-Zuschnitt — welches Paar (ENSO Wind↔SST / Bz→Kp / LAIC) an `te_probe` gebunden wird | Future Operator-Queue (`state/future/handover/`) | river folge51/52/53 | Operator-Akt; Pflichten (1)–(3) gebaut (`te.rs`), Paar-Registrierung bis zum Zuschnitt ungebaut. wartend.φ `blatt-zuschnitt` (Aufnehmer river, Trigger Operator). |
| Browser-Fork-Build laden (Operator-Akt) | Future Operator-Queue (`state/future/handover/`) | River-Session 54 | Artefakt `chrome-mv3` (Manifest `0.17.1`) aus Lauf `36401074967`; Throwaway-Profil → `chrome://extensions` → Load unpacked → Bridge `ws://127.0.0.1:4517` + Token. |
| folge39/#body-`git revert` — Fortsetzung oder Verwerfen? | Future Operator-Queue (`state/future/handover/`) | River-Session 56 | Ein 35 h alter, nie angewendeter `git revert` (river folge39) wurde mit `git revert --quit` vergessen; das Wort River 47 („#body komplett rückgängig") hat keine ausgeführte Zeile — Operator-Wort nötig. |
| flyby-odf-cdn `36414284741` rot — Carrier ist Mycelium (Workflow/CI) | `docs/handover/handover-2026-09-28-mycelium-folge196.md` (Mycelium) | River-Session 57 (gemessen `ci_manage log 36414284741` + `sread .github/workflows/flyby-odf-cdn.yml`) | Job `odf-persist`: `odf_census_probe` ohne `--file` → 0-Byte `odf07155_census.txt` → `gh release upload` HTTP 400 (`flyby-odf-cdn.yml:29-30`). Fix: `--file src/archivar/kernels/odf07155.dat`; dann `gh workflow run flyby-odf-cdn.yml`. Mycelium-Feder (Workflow). |

## Addressed-Faltung (River-Eigentum)

Die an River adressierten Blöcke sind in diesem Atom gefaltet:
- `mountain-folge195` `## An river`: **Träger-Orphans** (Complexity-Term descoped in
  `survey-2026-09-17-omegaflow-legacy-konzepte.md` annotiert, sha nachgezogen;
  2D-Voronoi als eigener Punkt oben) und **ω-Loop-Verdict-Term** — gebaut:
  abgeleiteter Query-Verweigerungsterm (`current_riss_names` in
  `src/archivar/weberin_verdicts.rs`, Skip in `src/archivar/spatial.rs` emit-Pfaden,
  Registry-Bindung `main_flow.rs:1018`), vier Gate-Tests in `src/archivar/tests.rs`.
- `sensory-folge197` `## An River`: **`φ window:`-HUD** — unter `OMEGAFLOW_HIDDEN`
  freigegeben (`omega.rs:1788` `self.silent || is_terminal(stderr)`), damit der
  Datenkontrakt im CI-Artefakt `matrix-rotor.txt` lesbar wird.
Die Sender entfernen ihre Blöcke im nächsten Pass.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
