<!--
  title: Handover — River-Folge 59 (2026-09-28)
  session: River-Folge 59
  class: handover
  date: 2026-09-28
  sha256: 75991cd82191b7f2db6359a18b872ebd46046c0bdd55ac12701a42a0e11d83e0
  status: live
-->
# Handover — River-Folge 59 (2026-09-28)

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
„warum ist das alles noch offen?" | 2026-09-28 | Operator (Session, River 58)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-28 | Operator (Session, River 58) — session-weiter Delegations-Consent
„hast du alles gemessen und geplant?" | 2026-09-28 | Operator (Session, River 59)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-28 | Operator (Session, River 59) — session-weiter Delegations-Consent

## Offen (aufgeschlüsselt)

### ci-check-Verifikation (Jump-Detektion + konditionale TE + Membran-Parität)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `ci-check`-Lauf auf `origin/main` schließt ab (generisch, keine Run-ID pinnen).
- **Lage:** (gemessen 2026-09-28 River 59) `jump_residual_breached` auf das zweiseitige Segment-Residuum umgebaut (Rat-Verdikt 3; `AGENTS.md` + `archivar-mathematikerin.md:29/31` nachgezogen, Gate-Fixture `jump_residual_acceleration_term`, zwei diskriminierende Tests in `tests.rs`); `transfer_entropy_embedded_ksg_conditional` + Z-Phase-Surrogat (`te.rs:2514/:2704/:2729`, `#[cfg(test)]`-Gate `:8058`); `cargo check` 0/0. Der frühere `ci-check 36442797218` war cancelled (Kanten-Cancel), `ci-gate 36445552559` rot aus Parser-clippy (nicht River).
- **Blockade:** keine.
- **Braucht:** `ci_manage jobs <neuer ci-check>`; bei Rot `ci_manage log <id>`.

### konditionale TE — te-gate-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `te-gate`-Lauf schließt ab.
- **Lage:** (gemessen 2026-09-28 River 59 via `cargo check` 0/0) eingebettete konditionale Generalisierung gebaut: `transfer_entropy_embedded_ksg_conditional` (`te.rs:2514`), `z_phase_surrogate` (`:2704`), `conditional_embedded_te_phase` (`:2729`), Dimensions-Starvation-Refusal (`:2585`); Gate-Tests `gate_conditional_embedded_*` (`:8058+`). Kanal = `field`-Zeile; Z-Phase-Surrogate.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <te-gate-id>` — Jobs `ksg-k-gate` / `conditional-arx` / `fpr-ksg-arx`.

### Flyby-Path-2 (revised) — Δ/σ_recon offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neuer CDN-`release` der post-Flyby-Solution samt 1-σ-Kovarianz.
- **Lage:** (gemessen 2026-09-28 River 58) δ = 0,16847323696971178 km (DE441 vs DE442, sealed-arc ±21 d Perigäum-Fenster, 997 Stunden-Samples); beide NAIF-Kernel registriert; `docs/paper/flyby-path-2-preregistration-revised.md:85-88` auf dem Baum (δ gemessen, σ_recon `pending`).
- **Blockade:** keine.
- **Braucht:** `cargo run -p omegaflow-measure --bin flyby_ephemeris_gate -- --recon <arc> --sigma-recon <km>`; Riß gegen beide Hashes tragen.

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin | **Bindung:** termin:2026-09-29
- **Trigger:** 29.09. 00:00 UTC Snapshot.
- **Lage:** (gemessen 2026-09-28 River 58) 28.09-Snapshot `36421869230` success.
- **Blockade:** 29.09.-Snapshot fehlt.
- **Braucht:** nach 29.09. 00:00 UTC `gh workflow run flyby-path2-fill.yml`; Artefakt `flyby-path2-fill`.

## An fremde Feder (Absender-Zeilen — Aufenthalt = Eigentum)

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| fresh red `ci-gate 36445552559` — `clippy` 34 Fehler in `src/archivar/pds3_table.rs` + `pds4.rs` (`div_ceil`-Nachbau, `unwrap_or`-Nachbau, complex type), `format` + `dropped-gate` rot; `build` grün | `docs/handover/handover-2026-09-28-mountain-folge198.md` (Mountain) | River 59 (`ci_manage log 36445552559`) | Parser-clippy der pds3/pds4-Arme; Fix auf der commitenden Linie |
| orphan-doc-Träger (mountain-Natur): `survey-2026-09-03-daten-holdings-inventur`, `survey-2026-09-07-tmp-opencode-scan`, `survey-2026-09-14-kapitulationen-pendings-inventur`, `survey-2026-09-16-dead-sources-relevanz`, `survey-2026-09-17-omegaflow-legacy-konzepte` | mountain-folge198 | River 58/59 (`register_lookup --orphan-docs`) | Mycelium folge198 routet dieselbe Träger-Pflicht — je Doc eine Träger-Zeile |
| 81-Block-Fix — `format ephemeris_binary`, `ttl 86400` | mountain-folge198 | river folge51/52/53 | Prüfintervall aus Live-Release-Abständen oder 2²⁵ s ≈ 388 d |
| matrix-rotor `36436173707` Runner-Preemption (Step 7 `cancelled`, kein Code-Marker, kein Watchdog-Cancel) | mycelium-folge198 | River 58/59 (gemessen) | `.github/workflows/matrix-rotor.yml` job-level concurrency prüfen |
| dropped-gate-Umbau — selbst-messend/träger-bewusst statt handgepflegter Absolutwert | mycelium-folge198 | River 57 | Zähler 984→1060 in einem Tag |
| flyby-odf-cdn `36414284741` rot — `odf-persist` ohne `--file` → 0-Byte census → `gh release upload` HTTP 400 | mycelium-folge198 | River 57 | Fix `--file src/archivar/kernels/odf07155.dat`; dann dispatch |
| `devcontainer`-CLI in einem Actions-Job (Umgebungs-Parität) | mycelium-folge198 | River 54 (Operator-Wort) | `.devcontainer/devcontainer.json` von keinem Workflow konsumiert; Job mit `devcontainer up`/`exec` |

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort. Pfad-begrenzte
Commit-Pfade dieser Session:

`AGENTS.md` · `docs/concepts/archivar-mathematikerin.md` ·
`docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` ·
`src/archivar/main_flow.rs` · `src/archivar/tests.rs` ·
`src/gate/commit_gate.rs` · `src/gate/commit_gate_vocab.json` ·
`src/mathematikerin/te.rs` · `src/mathematikerin/doppler.rs` ·
`tools/measure/src/bin/flyby_anderson_probe.rs` ·
`tools/measure/src/bin/a_posteriori_placement_probe.rs` ·
`docs/handover/handover-2026-09-28-river-folge59.md` ·
`docs/handover/archiv/handover-2026-09-28-river-folge58.md`.
