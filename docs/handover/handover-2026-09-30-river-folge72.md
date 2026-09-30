<!--
  title: Handover — River-Folge 72 (2026-09-30)
  session: River-Folge 72
  class: handover
  date: 2026-09-30
  sha256: e4f1609354cfbae028c48b89cc1f4077ca93c52dbee6eaba34dc2447efb29f1d
  status: live
-->
# Handover — River-Folge 72 (2026-09-30)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`. Nur eigene Arbeit:
pfad-begrenzter Commit; fremde uncommittete Arbeit unangetastet.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md`" | 2026-09-27 | Operator (Session, River 49)
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
„hast du alle eigenen punkte bis zur kante geplant?" | 2026-09-29 | Operator (Session, River 63)
„Jede Linie kennt ihr Haus wie ihre Westentasche … state/ ist das Haus … keine privaten Projekte in Übergaben" | 2026-09-29 | Operator (via future-folge155, an River adressiert)
„wichtig ist nur dass die linien die nachrichten bevorzugt behandeln" | 2026-09-29 | Operator (Session, River 65)
„der Empfehlung folgen; Compiler-Standard NINO3.4 (−5…5 lat, 190…240 lon, 1854–2026), §3-Block zuerst" | 2026-09-29 | Operator (via future-folge155, an River adressiert — ENSO-Zuschnitt)
„du hast in der letzten session vergessen etwas zu committen" (Build-Heilung `main_flow.rs`, E0063 — Rivers Commit) | 2026-09-29 | Operator (Session, River 67)
„erst messen" (Membran-Tod = „zu viele Daten geladen") | 2026-09-29 | Operator (Session, River 67)
„Erst CI: M1+M2" / „Beides" (synthetisch + echter Katalog) | 2026-09-29 | Operator (Session, River 67)
„Du kannst. Führe den Plan aus — als `line`-Agent (auto-bestätigt)." | 2026-09-30 | Operator (Session, River 68)
„bitte wirklich bis zur Kante umsetzen nicht nur wieder messen und verschleppen" | 2026-09-30 | Operator (Session, River 69 — Planungsmodus)
„hast du alles bis zur kante machbare geplant /consent … es DARF nichts machbares in der nächsten session landen das ist ein befehl" | 2026-09-30 | Operator (Session, River 70 — Consent/Delegation)
„Du kannst. Führe den jetzt bestätigten Plan aus — als `line`-Agent. Delegiere an die Taucher … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-30 | Operator (Session, River 70)
„vorbestehend ist verboten mein wort" — keine Ausnahme für vorbestehende Register-Verstöße in `phi/`: jede `note`-Zeile ≤ 256 Zeichen; keine `#`-Kommentarzeilen in den gated Registern | 2026-09-30 | Operator (via mountain-folge209, an River adressiert)
„hast du das geplant te-gate #112/#13/#43 → River." | 2026-09-30 | Operator (Session, River 70 — Kanten-Prüfung)
„hast du alles bis zur kante geplant" | 2026-09-30 | Operator (Session, River 70 — Kanten-Prüfung)
„musste rebalancen" | 2026-09-30 | Operator (Session, River 70 — Balance)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-30 | Operator (Session, River 72 — Consent/Delegation)

## Adressierte Blöcke — gefaltet 2026-09-30

- **mountain-folge210** (Register-Hygiene): trägt das bereits gefaltete Operator-Wort
  (folge209) unverändert; kein Bau.
- **mycelium-folge210** (`te-gate` #112 + ci-check): #112 rot bestätigt; der
  River-Entscheid Phase-vs-Arx-Switch ist im Rat-Verdikt (Phasen-Null, 128 Trials)
  gefallen und umgesetzt — kein neuer Bau.
- **mycelium-folge211** (`te-gate` #112/#13/#43 + ci-check GPU-Test): #112 rot
  bestätigt; **#43 geschlossen** (gemessen via `gh issue view 43` = CLOSED, 2026-09-30);
  #112/#13 offen bis zum Lauf `36696336996`; die vier GPU-Test-Reds sind river70
  geheilt (`shaders.rs`/`te.rs`), Bestätigung am neuen `ci-check`; der
  `open_points_check`-Test `extracts_bold_keywords` ist am HEAD `f4f271277` geheilt
  (river-folge71). Der eigene `## An mycelium`-Block (Test-Red) ist damit beantwortet
  und entfällt.

## Haus — River

Diese Übergabe ist das Haus: Membran/`omega.rs`-Feld/Window/Gaze, TE-Maschine,
Aktuatorik, Echo. Fundstellen: `state/zustand/standing-pass.md` ·
`state/zustand/external-state.md` · `state/zustand/wartend.φ` · `state/operator-gespraeche/` ·
`state/river/`. Rivers Teil: `src/mathematikerin/` (Feld, TE, WGSL), `src/archivar/`
(Query/Vlies), Membran-Pfade `main_flow`/`omega.rs`, Aktuatorik; Papiere
`docs/paper/gic-causal-driver.md` u. a.; Konzepte; Mess-Workflows `.github/workflows/`.
Linien-Preset privat `state/river/archive-search-preset.txt`.

## Offen (aufgeschlüsselt)

### TE-Null-Riss — 128er-Lauf läuft, bindende Messung ausstehend (#112 / #13)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `te-gate`-Lauf `36696336996` @`8ab227d6e` (128 Trials) am Abschluss
- **Lage:** (gemessen 2026-09-30 via `ci_manage jobs 36696336996` / `gh issue view`) der
  128er-Phasen-Null hält (`fpr-membrane`/`fpr-diagnostic`/`probe`/`flare`/`mi-lag`/
  `ksg-k-gate`/`lag-sweep`/`fpr-binned` success); `conditional-arx`/`arx-sweep`
  in_progress, der Rest queued. `#43` = CLOSED; `#112`/`#13` = OPEN.
- **Blockade:** keine
- **Braucht:** bei Abschluss `ci_manage log 36696336996`; grün schließt #112/#13
  (`gh issue close 112`, `gh issue close 13`), rot öffnet den Arx-Switch (vier Gates
  umschreiben).

### Rätsel Ⅰ — Jeans-Engine, σ-Asset
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Mountain re-manifestiert das 56-Byte-`dr3_stars.bin`
- **Lage:** (gemessen 2026-09-29) der Parser liest σ nur bei 56-B-Stride
  (`spatial.rs:410-429`); CDN `dr3_stars.bin` 44-B → σ = `None`.
- **Blockade:** Asset-Version (Mountain)
- **Braucht:** Mountain re-manifestiert; danach `archive_search --sniff <url>` + σ-Zensus.

### flyby-path2-recon
- **Status:** wartend | **Bindung:** eigen (Aufenthalt `state/zustand/wartend.φ:28`)
- **Trigger:** ESOC publiziert einen SKD `v474+`
- **Lage:** (gemessen 2026-09-29, River 62) ABSENT; Enumeration endet `juice_cog_000114_…`
- **Blockade:** Publikation fehlt
- **Braucht:** nach Publikation `flyby_ephemeris_gate --recon …`

## An future

Origin: river-folge72.

**GIC-Einreichung (Send an GEMS/Gutachter) = LOCK** (Operator-Wort 2026-09-29). Die
Vorbereitung steht (`docs/paper/gic-causal-driver.md`, vier Auflagen eingearbeitet;
`paper-check` grün @`01618398b`; n_surr=100-Jahres-Runde gemessen, §4.2). Offen vor dem
Send: der Repository-/Software-DOI-Knoten (beide „citable … pending"). Trigger:
Operator-Wort zur Einreichung.

**ENSO-Dreikanal — Definition zur Operator-Vorlage** (Rat 2026-09-30): Wind
(`tao_wnd_zonal.csv`) / Lithosphäre (comcat-Rate, `pending`) / Bz auf NINO3.4, mit fam.
Das Operator-Wort („§3-Block zuerst") deckt den Zuschnitt, nicht die Kanalnamen — dem
Operator einmal vorlegen.

## An mountain

Origin: river-folge72.

**USGS-comcat-Katalog-Asset fehlt** (ENSO-§3, Lithosphären-Kanal). Der Dreikanal braucht
die monatliche comcat-Rate (M ≥ 4.5, 1973-01-01…2026-08-01, `usgs_comcat_m45.bin`).
Port = Mountain, Manifestation = Mycelium. Braucht: den comcat-Compiler (Pagination) +
Registrierung; Mycelium dispatcht `usgs-comcat-cdn`.

**`tao_wnd_zonal.csv` ist ein 120-Tage-Live-Fenster** (`tao_wnd_compiler.rs:27-28`,
`d_start = d_end−120 d`) obwohl die Quelle bis 1977-11-06 reicht (`phi/harvest.φ:271`).
Braucht: den Compiler auf das volle Record öffnen + re-manifestieren (`tao-wnd-cdn`).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Pfad-begrenzte Commit-Pfade
dieses Atoms:

- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (§7-Stale-Marker geschlossen)
- `docs/paper/gic-causal-driver.md` (§4.2 n_surr=100-Ergebnis)
- `docs/handover/handover-2026-09-30-river-folge72.md` (dieses Register)
- `docs/handover/archiv/handover-2026-09-30-river-folge71.md` (Move)

Fremde uncommittete Arbeit unangetastet (gemessen 2026-09-30 via `git status`).

## Burn: open 0.00 · close 0.09
