<!--
  title: Handover — River-Folge 58 (2026-09-28)
  session: River-Folge 58
  class: handover
  date: 2026-09-28
  sha256: 35671477170054e3f4f451e2a5bb6686cc79e9a3038f4701bd0e676a4e5915cc
  status: live
-->
# Handover — River-Folge 58 (2026-09-28)

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
„hast du alles bis zur kante geplant?" | 2026-09-28 | Operator (Session, River 58)
„hast du gemessen?" | 2026-09-28 | Operator (Session, River 58)
„warum misst du nicht?" | 2026-09-28 | Operator (Session, River 58)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-28 | Operator (Session, River 58) — session-weiter Delegations-Consent

## Offen (aufgeschlüsselt)

### matrix-rotor `36436173707` — Rerun-Ergebnis offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der Rerun (attempt 2) des Runs `36436173707` schließt ab.
- **Lage:** (gemessen 2026-09-28 via `ci_manage view`/`jobs`, GH-API, `.github/workflows/matrix-rotor.yml`) Der rote Lauf `36436173707` lief auf `4fc1221ef` (River-eigen). **Gemessener Grund: Runner-Shutdown** — Step 7 „Den verborgenen Rotor fahren" `cancelled` (14:29:36→14:31:21Z), Log `##[error]The runner has received a shutdown signal`, Prozess-Kill `rc=137`; **kein** Timeout (350 min / 18000 s unerreicht), **kein** Code-Marker in 1108 Log-Zeilen. Transient (Runner-Abbruch), nicht River-Code. Rerun angefordert 2026-09-28.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36436173707` nach dem Rerun; grün = Gate zu. Kein Run-ID-Pin.

### ksg-k-Fix — te-gate-Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `te-gate`-Lauf `36442790162` (dispatched 2026-09-28) schließt ab (Job `ksg-k-gate`).
- **Lage:** (gemessen 2026-09-28: Code-Lesen + `cargo check`) `ksg_k.rs:166` misst `ksg_te_phase_null(&b,&a)` — die wahre a→b-Kopplung; der Sweep ist `#[ignore]`/te-gate-only (`:293`). `cargo check` 0/0. Lauf dispatcht.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36442790162` — Job `ksg-k-gate`. Kein Run-ID-Pin.

### Membran-Parität — ci-check-Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf `36442797218` (dispatched 2026-09-28) schließt ab.
- **Lage:** (gemessen 2026-09-28: Code-Lesen) Fix `618871109` (te.rs `causal_pair_at_mi_lag` + MI-lag-Horizont); `membrane_forward` lebt (`te.rs:3461-3476`); `dropped-gate` mit baseline 1060 geschlossen (`docs/zustand/dropped-baseline.md:16`). Lauf dispatcht.
- **Blockade:** keine.
- **Braucht:** `ci_manage jobs 36442797218` — der `test`-Job trägt `membrane_forward`; grün = Gate zu. Kein Run-ID-Pin.

### Flyby-Path-2 (revised) — Δ/σ_recon offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neuer CDN-`release` der post-Flyby-Solution samt 1-σ-Kovarianz.
- **Lage:** (gemessen 2026-09-28 via `flyby_ephemeris_gate`, Bin-Lauf) **δ = 0,16847323696971178 km** (DE441 vs DE442, sealed-arc-geklammertes ±21 d Perigäum-Fenster, 997 Stunden-Samples); `flyby_ephemeris_gate` gebaut (`d310d5888`); beide NAIF-Kernel registriert (`phi/sources.φ:3448`/`:3469`); `docs/paper/flyby-path-2-preregistration-revised.md:85-88` auf den Baum gezogen (δ gemessen, σ_recon `pending`).
- **Blockade:** keine.
- **Braucht:** `cargo run -p omegaflow-measure --bin flyby_ephemeris_gate -- --recon <arc> --sigma-recon <km>`; Riß gegen beide Hashes (`aeb3c82f…`/`eee376ef…`) tragen.

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin | **Bindung:** termin:2026-09-29
- **Trigger:** 29.09. 00:00 UTC Snapshot.
- **Lage:** (gemessen 2026-09-28 via `ci_manage view`) 28.09-Snapshot-Lauf `36421869230` success (`flyby-path2-fill.yml`).
- **Blockade:** 29.09.-Snapshot fehlt.
- **Braucht:** nach 29.09. 00:00 UTC `gh workflow run flyby-path2-fill.yml`; Artefakt `flyby-path2-fill`.

### bedingte TE (Umwelt-Kanäle) — Entwurf offen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner — der erste Schritt (Rat/Entwurf) ist dispatchbar.
- **Lage:** (gemessen 2026-09-28 via `sread docs/concepts/kybernetische-astrophysik.md:418-427`) „die bedingte TE (TE(A→B | Umwelt-Kanäle)) ist ein pending-Instrument, kein vorhandenes" — ohne gemessenen Kanal kein Ausschluss (0 honored).
- **Blockade:** keine.
- **Braucht:** Rat/Architektur — Entwurf der bedingten TE im Estimator (`src/mathematikerin/te.rs`), danach Bau.

### Prosa-Träger (aus `register_lookup --orphan-docs`)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner — Trägerschaft ist die Registratur, kein Bau.
- **Lage:** (gemessen 2026-09-28) Eigene Trägerschaft: `docs/concepts/pfeiler-der-architektur.md` (Orphan-Marker `:37` ist Prosa, kein Handlungspunkt), `docs/surveys/survey-fortschritt.md` (C-Items `descoped`/`ueberholt`, gemessen River 50) und `docs/surveys/survey-messpunkt-verteilung.md` (2D-Kandidat `:94` gemessen/geschlossen; Restmarker getragen).
- **Blockade:** keine.
- **Braucht:** kein Bau — die zwei Docs sind hiermit getragen.

## An fremde Feder (Absender-Zeilen — Aufenthalt = Eigentum)

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| `survey-2026-09-26-membran-ladearchitektur.md:214` ω-Loop-Verdict-Term ist gebaut (Kette `weberin_verdicts.rs:201`→`spatial.rs:488/491`→`omega.rs:1907`; Tests `tests.rs:2580-2651`); `!hidden`-Relay = Konsens-by-design | `docs/handover/handover-2026-09-28-mountain-folge197.md` (Mountain) | River 58 (gemessen) | Marker stale — schließen. |
| Orphan-Docs mountain-Natur (Träger setzen): `arxiv-api`, `exzellenz-konzept`, `daten-holdings-inventur`, `tmp-opencode-scan`, `kapitulationen-pendings-inventur`, `dead-sources-relevanz` | `docs/handover/handover-2026-09-28-mountain-folge197.md` (Mountain) | River 58 (`register_lookup --orphan-docs` + Klassifikation) | je Doc eine Träger-Zeile. |
| Orphan-Docs mycelium-Natur (Träger setzen): `tools-map`, `survey-2026-09-03-orphan-verdicts`, `survey-2026-09-20-browser-anbindung` | `docs/handover/handover-2026-09-28-mycelium-folge196.md` (Mycelium) | River 58 | je Doc eine Träger-Zeile. |
| dropped-gate-Umbau — selbst-messend/träger-bewusst statt handgepflegter Absolutwert | `docs/handover/handover-2026-09-28-mycelium-folge196.md` (Mycelium) | River-Session 57 | Der Zähler rattert (984→1060 in einem Tag, jedes Mal „Absorbiert durch Bump"). Bau: (a) `register_lookup --dropped` träger-bewusst; (b) `dropped-gate` gegen den gemessenen Vor-HEAD-Zähler. |
| `devcontainer`-CLI in einem Actions-Job (Umgebungs-Parität) | `docs/handover/handover-2026-09-28-mycelium-folge196.md` (Mycelium) | River-Session 54 (Operator-Wort) | `.devcontainer/devcontainer.json` von keinem Workflow konsumiert; Job mit `devcontainer up`/`exec`. |
| 81-Block-Fix — `format ephemeris_binary`, `ttl 86400` | `docs/handover/handover-2026-09-28-mountain-folge197.md` (Mountain) | river folge51/52/53 | Prüfintervall aus Live-Release-Abständen oder 2²⁵ s ≈ 388 d. |
| flyby-odf-cdn `36414284741` rot — `odf-persist` ohne `--file` → 0-Byte census → `gh release upload` HTTP 400 | `docs/handover/handover-2026-09-28-mycelium-folge196.md` (Mycelium) | River-Session 57 | Fix: `--file src/archivar/kernels/odf07155.dat`; dann `gh workflow run flyby-odf-cdn.yml`. Mycelium-Feder (Workflow). |

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
