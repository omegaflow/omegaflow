<!--
  title: Handover — River-Folge 56 (2026-09-28)
  session: River-Folge 56
  class: handover
  date: 2026-09-28
  sha256: 5b94ad771286aaed45f181d7f2c348405ae15920d173f8b02375b768270d4c02
  status: live
-->
# Handover — River-Folge 56 (2026-09-28)

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
„Starte die River-Linie im Planungsmodus … Name (leer = neueste)" | 2026-09-28 | Operator (Session, River 56) — Planungs-Pass
„hast du wirklich alles bis zur kante geplant und nur eigene punkte in deiner liste?" | 2026-09-28 | Operator (Session, River 56)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) … Commit und Push trägt `/commit`." | 2026-09-28 | Operator (Session, River 56) — session-weiter Delegations-Consent
„welcher vollidiot hat einen verbotenen git befehl ausgeführt?" | 2026-09-28 | Operator (Session, River 56) — zum aufgefundenen `git revert`
„ich habe keine ahnung ich weiss nicht wie man damit umgeht" | 2026-09-28 | Operator (Session, River 56) — Git-Zustand, Entscheidung delegiert

## Offen (aufgeschlüsselt)

### Membran-Parität — Fix gebaut, CI-Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein ci-check-Lauf auf dem Fix-HEAD (`src/mathematikerin/te.rs`) schließt ab.
- **Lage:** (gemessen 2026-09-28 via `cargo check` + `ci_manage log 36409581203`) Der Lauf auf `9e17cb331` (enthält `4e18d6bd9`) FAILED an `te.rs:3455` — `membrane_forward.te 0.02999 ≤ threshold 0.06166`. Ursache: der Skalarpfad liest Horizont 1, die Membran den MI-lag (`find_mi_lag`); die Fixture koppelte bei lag 1. Hebung `causal_pair_at_mi_lag` + Test-Rewiring in `src/mathematikerin/te.rs`; `cargo check` 0 Fehler/0 Warnungen.
- **Blockade:** keine.
- **Braucht:** Commit + `gh workflow run ci-check.yml`; `ci_manage log <id>` — grün = Gate zu; rot = Log auswerten.

### ksg-k-Sweep — FNR-Zelle misst die Nullrichtung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** kein externer — dispatchbar; Beleg `ksg_k.rs:166`.
- **Lage:** (gemessen 2026-09-28 via Code-Lesen, te.rs-Seitenfund) `ksg_k.rs:166` ruft `ksg_te_phase_null(&a,&b)` mit (cause,effect); die Funktion liefert `TE(target→driver)` (`ksg_k.rs:106`) = effect→cause. Das gemessene `sweep=void` (FNR k=4 = 100 %) ist ein Vorzeichen-Artefakt. `formula=8 reference=4 production=0` bleibt gültig.
- **Blockade:** keine.
- **Braucht:** `ksg_k.rs:166` Argument-Ordnung gegen `:106` messen und heilen; `cargo check`; CI.

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin | **Bindung:** termin:2026-09-29
- **Trigger:** 29.09. 00:00 UTC Snapshot.
- **Lage:** (gemessen 2026-09-28 via `gh workflow run`) 28.09-Snapshot-Lauf `36421869230` dispatcht (`flyby-path2-fill.yml`).
- **Blockade:** 29.09.-Snapshot fehlt.
- **Braucht:** nach 29.09. 00:00 UTC `gh workflow run flyby-path2-fill.yml`; Artefakt `flyby-path2-fill`.

### Flyby-Path-2 (revised) — δ gemessen, Benotung offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neuer `release` der Post-Flyby-Ephemeride (CDN `ephemeris_juice.bin`) samt 1-σ-Kovarianz.
- **Lage:** (gemessen 2026-09-27) δ = 0,1684732 km (168 m, DE441 vs DE442; `phi/sources.φ:3448`/`:3469`). Δ/σ_recon `pending`.
- **Blockade:** keine.
- **Braucht:** nach dem Flyby `cargo run -p omegaflow-measure --bin flyby_ephemeris_gate -- --recon <arc> --sigma-recon <km>`; Riß gegen beide Hashes tragen.

### flyby-odf-cdn rot — Träger-Riss
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Lauf `36414284741` (rot, abgeschlossen).
- **Lage:** (gemessen 2026-09-28 via `ci_manage log 36414284741`) Job `odf-persist`: `odf_census_probe` ohne `--file` → 0-Byte `odf07155_census.txt` → `gh release upload` HTTP 400 Bad Content-Length; das Bin verlangt ein positionales `--file` (`odf_census_probe.rs:16-20`). Der Stehende Pass tagt river, keine lebende Übergabe trägt ihn.
- **Blockade:** Träger ungeklärt (Pass: river vs. Inhalts-Domäne ODF/source+CDN).
- **Braucht:** Träger messen; dann `.github/workflows/flyby-odf-cdn.yml:29` `--file src/archivar/kernels/odf07155.dat`; `gh workflow run flyby-odf-cdn.yml`.

### Stale `git revert` — aufgeräumt, zugrundeliegender Akt offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort.
- **Lage:** (gemessen 2026-09-28 via `git status`/`find`/opencode.db) Ein `git revert` von 4 river-folge39-Commits lag seit 2026-09-27 01:05:12 unbearbeitet (`.git/sequencer`), nie angewendet (Dateisatz der Commits deckt sich nicht mit den Arbeitsbaum-Änderungen); mit `git revert --quit` vergessen (Arbeitsbaum/Index unberührt). Keine retained opencode-Session führte den Befehl aus (507 bash-Parts, 0 „revert"-Treffer außer eigener Untersuchung). Der zugrundeliegende Operator-Akt (Wort River 47 „#body … komplett rückgängig") ist nicht ausgeführt.
- **Blockade:** keine.
- **Braucht:** Operator-Wort, ob folge39/#body-revert wieder aufgenommen wird (Future-Queue-Zeile).

### Total-Coherence — Complexity-Term (descoped)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** kein externer — das Rat-Verdikt (2026-09-28) liegt vor.
- **Lage:** (gemessen 2026-09-28, Rat) `complexity` 0 Treffer in `src/` (`sgrep complexity src/`); der Breath-Zweig mittelt uniform `integral/9.0` (`omega.rs:1686`); die Permutation-Entropie braucht eine Serie und fällt nur im TE-Verdict (`te.rs:2693-2695`, `:2705-2706`); Breath-Vertrag `tests.rs:802`.
- **Blockade:** keine — der Term ist Spezifikationstext ohne Träger im Baum; mit dem Befund geschlossen.
- **Braucht:** nichts; der Befund ist der Eintrag. Kein Bau des Serien-Rings der 9 Medien, keine dritte PE-Verwendung.

## An fremde Feder (Absender-Zeile — Aufenthalt = Eigentum)

Ziel ist die **lebende** Übergabe der fremden Linie; der Eigentümer faltet die Zeile
in seinem Pass.

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| ci-check `36409581203` @ `9e17cb331` rot — Job `dropped-gate` `delta 6` (baseline 1054 \| current 1060) | `state/zustand/standing-pass.md` (Mycelium) | River 55 (gemessen `ci_manage log`) | 6 offene Punkte ohne auflösenden Commit gedroppt; `register_lookup --dropped` → Punkte zurücknehmen oder Baseline im annehmenden Commit anheben. |
| `devcontainer`-CLI in einem Actions-Job (Umgebungs-Parität) | `docs/handover/handover-2026-09-28-mycelium-folge195.md` (Mycelium) | River-Session 54 (Operator-Wort) | `.devcontainer/devcontainer.json` von keinem Workflow konsumiert (gemessen 2026-09-28 via `sgrep -i devcontainer .`); Job mit `devcontainer up`/`exec` soll die Toolchain reproduzierbar in CI stellen. |
| 81-Block-Fix — `format ephemeris_binary`, `ttl 86400` | `docs/handover/handover-2026-09-28-mountain-folge194.md` (Mountain) | river folge51/52/53 | Prüfintervall aus Live-Release-Abständen oder 2²⁵ s ≈ 388 d; AGENTS.md-Präzisierung im selben Atom. |
| Blatt-Zuschnitt — welches Paar (ENSO Wind↔SST / Bz→Kp / LAIC) an `te_probe` gebunden wird | Future Operator-Queue (`state/future/handover/`) | river folge51/52/53 | Operator-Akt; Pflichten (1)–(3) gebaut (`te.rs`), Paar-Registrierung bis zum Zuschnitt ungebaut. |
| Browser-Fork-Build laden (Operator-Akt) | Future Operator-Queue (`state/future/handover/`) | River-Session 54 | Artefakt `chrome-mv3` (Manifest `0.17.1`) aus Lauf `36401074967`; Throwaway-Profil → `chrome://extensions` → Load unpacked → Bridge `ws://127.0.0.1:4517` + Token. |
| folge39/#body-`git revert` — Fortsetzung oder Verwerfen? | Future Operator-Queue (`state/future/handover/`) | River-Session 56 | Ein 35 h alter, nie angewendeter `git revert` (river folge39) wurde mit `git revert --quit` vergessen; das Wort River 47 („#body komplett rückgängig") hat keine ausgeführte Zeile — Operator-Wort nötig. |

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
