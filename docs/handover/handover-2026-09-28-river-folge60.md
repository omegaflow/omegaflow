<!--
  title: Handover — River-Folge 60 (2026-09-28)
  session: River-Folge 60
  class: handover
  date: 2026-09-28
  sha256: 8819e962143fe862507cb188880f1ab675cdab6f6daeece6b0190bb0ba202727
  status: live
-->
# Handover — River-Folge 60 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`. Nur eigene Arbeit: pfad-begrenzter Commit,
fremde uncommittete Arbeit unangetastet (in dieser Session concurrent in
`src/archivar/extract.rs` + `src/archivar/pds4_binary.rs` + `src/gate/commit_gate.rs`
+ `phi/blocked_sources.φ` + `docs/handover/handover-2026-09-28-mountain-folge198.md`).

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
„bist du dir sicher dass das alles ist was du bis zur kante abarbeiten kannst?" | 2026-09-28 | Operator (Session, River 60)
„Flyby-Path-2 Δ/σ_recon … wird das bearbeitet?" | 2026-09-28 | Operator (Session, River 60)
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-28 | Operator (Session, River 60) — session-weiter Delegations-Consent

## Offen (aufgeschlüsselt)

### ci-check-Verifikation (Jump-Detektion + konditionale TE + Membran-Parität)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der laufende `ci-check`-Lauf auf HEAD `dfd2a4a19` schließt ab.
- **Lage:** (gemessen 2026-09-28 River 60 via `ci_manage status`/`view`) `ci-check 36454281654` in_progress auf HEAD `dfd2a4a19`, Job `test` → `cargo test --release --features browser_relay`; `ci-gate 36454281741` failure, aber nur `clippy` + `dropped-gate` rot (Parser-clippy pds3/pds4, nicht River), `format` + `build` grün. Der Jump-Umbau (zweiseitiges Segment-Residuum) + `transfer_entropy_embedded_ksg_conditional` (`te.rs:2514`) + Z-Phase-Surrogat (`:2704`) sind gebaut, `cargo check` 0/0.
- **Blockade:** keine.
- **Braucht:** `ci_manage jobs 36454281654`; bei Rot `ci_manage log 36454281654`.

### konditionale TE — te-gate-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der laufende `te-gate`-Lauf schließt ab.
- **Lage:** (gemessen 2026-09-28 River 60 via GH-API/`ci_manage view`) `te-gate 36442790162` (run 52, `workflow_dispatch`, head `59c820a2`) in_progress seit 15:20Z; Jobs `ksg-k-gate`/`conditional-arx`/`fpr-ksg-arx` (`te-gate.yml:42/74/90`). `te-gate` läuft nur per `workflow_dispatch` + Wochen-Cron — der Lauf ist angestoßen, es fehlt nur das Ende.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36442790162`.

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin | **Bindung:** termin:2026-09-29
- **Trigger:** 29.09. 00:00 UTC Snapshot.
- **Lage:** (gemessen 2026-09-28) 28.09-Snapshot `36421869230` success.
- **Blockade:** 29.09.-Snapshot fehlt.
- **Braucht:** nach 29.09. 00:00 UTC `gh workflow run flyby-path2-fill.yml`; Artefakt `flyby-path2-fill`.

## An fremde Feder (Absender-Zeilen — Aufenthalt = Eigentum)

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| fresh red `ci-gate 36445552559` — `clippy` 34 Fehler in `src/archivar/pds3_table.rs` + `pds4.rs`; `format` gefixt (river folge59 `c874dc587`), `dropped-gate` rot; `build` grün | `docs/handover/handover-2026-09-28-mountain-folge198.md` (Mountain) | River 59/60 (`ci_manage log 36445552559` + `jobs 36454281741`) | Parser-clippy der pds3/pds4-Arme; Fix auf der commitenden Linie |
| orphan-doc-Träger (mountain-Natur): `survey-2026-09-03-daten-holdings-inventur`, `-tmp-opencode-scan`, `-kapitulationen-pendings-inventur`, `-dead-sources-relevanz`, `-omegaflow-legacy-konzepte` | mountain-folge198 | River 58/59 (`register_lookup --orphan-docs`) | je Doc eine Träger-Zeile |
| 81-Block-Fix — `format ephemeris_binary`, `ttl 86400` | mountain-folge198 | river folge51/52/53 | Prüfintervall aus Live-Release-Abständen oder 2²⁵ s ≈ 388 d |
| matrix-rotor `36436173707` Runner-Preemption (Step 7 `cancelled`, kein Code-Marker, kein Watchdog-Cancel) | mycelium-folge198 | River 58/59 | `.github/workflows/matrix-rotor.yml` job-level concurrency prüfen |
| dropped-gate-Umbau — selbst-messend/träger-bewusst statt handgepflegter Absolutwert | mycelium-folge198 | River 57 | Zähler 984→1060 in einem Tag |
| flyby-odf-cdn `36414284741` rot — `odf-persist` ohne `--file` → 0-Byte census → `gh release upload` HTTP 400 | mycelium-folge198 | River 57 | Fix `--file src/archivar/kernels/odf07155.dat`; dann dispatch |
| `devcontainer`-CLI in einem Actions-Job (Umgebungs-Parität) | mycelium-folge198 | River 54 (Operator-Wort) | `.devcontainer/devcontainer.json` von keinem Workflow konsumiert; Job mit `devcontainer up`/`exec` |
| lokaler Post-Flyby-Kernel `data/esa/juice_cog_000114_230416_261003_v01.bsp` (Vorhersage, gültig bis 2026-10-03) ist **unregistriert**; die registrierte `juice_cog_000112_…_260919` (`phi/sources.φ:7151`) endet **vor** dem Perigäum | mycelium (url/compiler) · mountain (Quellen-Identität) | river folge60 (gemessen) | Quellen-Zeile auf den gültigen Kernel ziehen **oder** den Wait `flyby-path2-recon` (`state/zustand/wartend.φ`, Aufnehmer river) bedienen |
| `## An river` (mountain folge198): Jump-Detektion + ω-Loop-Verdict-Term **geschlossen** (survey-Nachtrag River 59, `8748a39cd`); pds3/pds4 `*_fixed_width` **gebaut** (river folge60: Namens-Join `series_field_name` + Format-Liste in `main_flow.rs`, Gate-Test) | mountain-folge198 | river folge60 (gemessen) | Block beim mountain-Pass entfernbar |
| `## An river` (future-folge149): format rot `a_posteriori_placement_probe.rs:162/:244` **gefixt** (river folge59 `c874dc587`; `ci-gate 36454281741` job `format` = success) | future-folge149 | river folge59/60 | Block beim future-Pass entfernbar |

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort. Pfad-begrenzte
Commit-Pfade dieser Session:

`src/archivar/main_flow.rs` · `docs/concepts/kybernetische-astrophysik.md` ·
`docs/handover/handover-2026-09-28-river-folge60.md` ·
`docs/handover/archiv/handover-2026-09-28-river-folge59.md`.
