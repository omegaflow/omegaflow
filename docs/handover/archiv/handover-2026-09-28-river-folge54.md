<!--
  title: Handover — River-Folge 54 (2026-09-28)
  session: River-Folge 54
  class: handover
  date: 2026-09-28
  sha256: 0f96e3554aebe3b46c8927febbb133f5f282c67bb19bcb0a647ea376ed884cd7
  status: live
-->
# Handover — River-Folge 54 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`. Nur eigene Arbeit: pfad-begrenzter Commit,
fremde uncommittete Arbeit unangetastet.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md`" | 2026-09-27 | Operator (Session, River 49)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-27 | Operator (Session, River 49)
„du sollst keine operator worte tragen das ist sache von future du bist bis zur kante" | 2026-09-27 | Operator (Session, River 48)
„warum holst du den si konsolen punkt schon wieder hoch — du hast doch schon den rat einberufen; rat und operator sind zwei paar schuhe" | 2026-09-27 | Operator (Session, River 48)
„die ttl muss die Aktualisierung der Quelle sein" | 2026-09-27 | Operator (Session, River 47)
„#body erzeugt das bias … komplett rückgängig" — kein Körper privilegiert | 2026-09-27 | Operator (Session, River 47)
„frag den rat" — die Form der SI-Konsole dem Rat vorlegen | 2026-09-27 | Operator (Session, River 47)
„folge dem rat" — Kraft-**Name** statt Debug-Zahl, Einheiten-Symbol `absent` | 2026-09-27 | Operator (Session, River 47)
GIC-Paper-Einreichung: höchste Priorität | 2026-09-27 | Operator-Wort
Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort
Harte-Läufe-LOCK aufgehoben | 2026-09-26 | Operator-Wort
HTTPS ja | 2026-09-26 | Operator-Wort folge36
Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36
vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36
UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session, Mountain)
Entscheidungen nie als Liste vorlegen — jede braucht eine Erklärung | 2026-09-27 | Operator (Future-Session)
D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — Fakten direkt abarbeiten | 2026-09-27 | Operator (Session, Mountain)
RX100-Kalibrierer descoped („über exif weg", K=12.5) | 2026-09-27 | Operator (Future-Session)
RX100 war nur ein Gedanke — Quelle zurückgezogen | 2026-09-27 | Operator (Session, Mountain)
„die Kante bin ich" — Wert, Wort, Dritt-Akt und Send bleiben seine Hand | 2026-09-27 | Operator (Future-Session)
ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben | 2026-09-27 | Operator (Future-Session)
`/consent` — session-weiter Delegations-Consent, nicht das Commit-Wort | 2026-09-27 | session-weiter Consent (`/consent`)
Commit-Wort (`/commit`) — pfad-begrenzter Commit + Push, das Doppel-Ask | 2026-09-27 | Operator
„probiere C lokal /consent" | 2026-09-28 | Operator (Session, River 50)
„teste nochmal" | 2026-09-28 | Operator (Session, River 50)
„messe nochmal" | 2026-09-28 | Operator (Session, River 50)
„warum fixt du nicht anstatt zu verschleppen? eigentlich müsste es gefixt sein aber checke nochmal" | 2026-09-28 | Operator (Session, River 50)
„Committe und pushe jetzt — nur deine eigene Arbeit, gemessen nicht beteuert" | 2026-09-28 | Operator (Session, River 50) — Commit-Wort (Doppel-Ask)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) … Commit und Push trägt `/commit`." | 2026-09-28 | Operator (Session, River 51) — session-weiter Delegations-Consent
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-28 | Operator (Session, River 52) — session-weiter Delegations-Consent
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-28 | Operator (Session, River 53) — session-weiter Delegations-Consent
„Committe und pushe jetzt — nur deine eigene Arbeit, gemessen nicht beteuert. Dieser Befehl ist das Commit-Wort des Operators (das Doppel-Ask) …" | 2026-09-28 | Operator (Session, River 53) — Commit-Wort (Doppel-Ask)
„fixxen vor verschleppen, mein wort!!!!" | 2026-09-28 | Operator (Session, River 53) — Fix-Wort (die zwei roten Gates jetzt bauen, nicht registrieren)
„A bis zur kante und nachricht an mycelium für bc /consent" | 2026-09-28 | Operator (Session, River 54) — session-weiter Delegations-Consent: A = Browser-Fork bis zur Kante; B = `devcontainer`-CLI in Actions als Zeile an Mycelium

## Offen (aufgeschlüsselt)

### GPU-Readback map/unmap + Lade-Membran — CI-Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein ci-check-Lauf auf einem HEAD ≥ Readback-Fix (`4e18d6bd9` ≤ HEAD), der **abschließt**.
- **Lage:** (gemessen 2026-09-28 via `ci_manage list`) HEAD ist inzwischen
  `31a51f2c0` (Mycelium folge193; `30018a341`, `fb7f9c9a1`). Offen: ci-check
  `36404600557` (pending) und `36401074643` (in_progress, Schritt
  „Run cargo test --release --features browser_relay"); der letzte abgeschlossene
  ci-check `36403605663` ist cancelled (superseded). Den vollen CI-Stand trägt der
  Stehende Pass (`state/zustand/standing-pass.md`, zitiert) — er ist seinerseits
  staleness-behaftet. Die Lade-Membran ist lokal headless
  grün (folge50).
- **Blockade:** keine.
- **Braucht:** `ci_manage log <id>` des zuerst **abschließenden** ci-check auf
  HEAD ≥ `4e18d6bd9` (kein Polling) — Readback-Test grün = Punkt geschlossen.

### Zwei rote River-Gates — Membran-Parität + No-TE-Tick
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein ci-check-Lauf auf dem Fix-HEAD (`4e18d6bd9` ≤ HEAD), der **abschließt**.
- **Lage:** (gemessen 2026-09-28 via `cargo check`) Fix gebaut: die
  Paritäts-Fixture von der degenerierten Sinus-Fixture auf das breitbandige
  AR(1)+Rausch-Paar umgestellt (`causal_pair_ar`), Schwelle über 100 Surrogate
  stabilisiert; der No-TE-Tick-Test liest das Delta-Integral **vor** `tick()`.
  `cargo check` 0 Fehler / 0 Warnungen; `4e18d6bd9` ist Vorfahr von
  `origin/main`. Verifikation steht aus (s. o.).
- **Blockade:** keine.
- **Braucht:** `ci_manage log <id>` des zuerst abschließenden ci-check —
  grün = beide Gates geschlossen; rot = Log auswerten.

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** Perigäum 2026-09-28 **11:45:12 UTC ± 10 s** (JUICE, Horizons
  `-28`); Snapshots 28.09. 12:00 + 29.09. 00:00 UTC.
- **Lage:** (gemessen 2026-09-28 ~06:20Z via CI-Timestamps) Termin noch nicht
  fällig; Fill-Bin `tools/measure/src/bin/flyby_path2_fill.rs` gebaut (0
  Warnungen), Tube ±12 h, Perigäum-Zelle 13, alle Zellen `pending`.
- **Blockade:** 29.09.-Snapshot fehlt; kein Workflow registriert.
- **Braucht:** nach 29.09. 00:00 UTC `sfetch https://services.swpc.noaa.gov/json/rtsw/rtsw_mag_1m.json`
  + `…/rtsw_wind_1m.json` → `data/services.swpc.noaa.gov/`, dann
  `cargo run -p omegaflow-measure --bin flyby_path2_fill -- --flyby juice --snapshots data/services.swpc.noaa.gov/`.

### Flyby-Path-2 — DSN-Status am Perigäum
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** 2026-09-28, ~11:45 UTC (Perigäum).
- **Lage:** (gemessen 2026-09-27 via `archive_search --playwright` +
  `sfetch eyes.nasa.gov/dsn/data/dsn.json`) `pending` — die Seite rendert keine
  lesbare Tracking-Tabelle (JS/canvas); der Live-Feed (09:48:57Z) trägt keinen
  JUICE-Eintrag.
- **Blockade:** keine.
- **Braucht:** am 28.09. `archive_search --playwright https://eyes.nasa.gov/dsn/dsn.html`
  — „tracked (Station X, Band Y)" oder „nicht getrackt".

### Flyby-Path-2 (revised) — δ gemessen, Benotung offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neuer `release` der Post-Flyby-Ephemeride (CDN
  `ephemeris_juice.bin`) samt 1-σ-Kovarianz — Wochen nach dem Vorbeiflug.
- **Lage:** (gemessen 2026-09-27 via `cargo build` + Bin-Lauf)
  `flyby_ephemeris_gate` gebaut; beide Zeugen lesen `placed`; **δ = 0,1684732 km**
  (168 m, DE441 vs DE442; `phi/sources.φ:3448`/`:3469`). Δ/σ_recon `pending`.
- **Blockade:** keine.
- **Braucht:** nach dem Flyby `--recon` + `--sigma-recon` →
  `cargo run -p omegaflow-measure --bin flyby_ephemeris_gate -- --recon <arc> --sigma-recon <km>`;
  Riß gegen **beide** Hashes („aeb3c82f…" sealed, „eee376ef…" CDN) tragen.

### Total-Coherence — Complexity-Term pending
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** eine physikalische Definition + Datenquelle für `complexity`.
- **Lage:** (gemessen 2026-09-28, River-Folge 51, Rat) der per-Oszillator-Integral
  ist in den Live-Pfad gebaut (`omega.rs` Breath-Zweig; Gate-Test
  `the_no_te_tick_hears_each_oscillator_not_the_sum`); der Complexity-Term
  `1/(1+Σ(complexity·weight)/Σ(weight))` hat keine Definition und keine
  Datenquelle — die Legacy-Identifier `complexity`/`takens` erscheinen nicht im Baum.
- **Blockade:** die physikalische Definition des Terms fehlt.
- **Braucht:** `complexity` als gemessene Größe definieren (Rat); bis dahin
  `pending` (Registerzeile, nie gebaut).

## Weitergabe (fremde Feder — Aufenthalt beim Eigentümer)

- **`devcontainer`-CLI in einem Actions-Job (Umgebungs-Parität)** — die
  `.devcontainer/devcontainer.json` wird heute von **keinem** Workflow konsumiert
  (gemessen 2026-09-28 via `sgrep -i devcontainer .` → nur Config + Doku). Bau:
  ein Job, der `devcontainer up`/`exec` (oder `devcontainers/ci`) nutzt, damit
  dieselbe Toolchain (Rust + lavapipe + node) reproduzierbar in CI steht.
  Gemessener Kontext: alle ~600 Workflows laufen `runs-on: ubuntu-latest`
  (GitHub-hosted); zwei schwere Läufe messbar gestört — `matrix-rotor 36400894925`
  erneut präemptiert („runner has received a shutdown signal") und
  `gosat-cdn 36385537670` 3 h `pending` ohne Jobstart (Queue-Stau). Ziel:
  **Mycelium** (CI-Domäne). Quelle: River-Session 54 (Operator-Wort 2026-09-28).
- **Blatt-Zuschnitt** — welches Blatt-Paar (ENSO Wind↔SST / Bz→Kp / LAIC) an
  `te_probe` gebunden wird; die geerbten Pflichten (1)–(3) sind gebaut (`te.rs`),
  die Paar-Registrierung am Einstieg `te_probe` bleibt bis zum Zuschnitt ungebaut
  (`omega.rs` feed = `probe_ring`). Operator-Akt. Ziel: **Future**
  (Operator-Queue). Quelle: river folge51/52/53.
- **81-Block-Fix** (`format ephemeris_binary`, `ttl 86400`) —
  Release-skalen-Prüfintervall, gemessen aus Live-Release-Abständen oder als
  Potenz-von-2-Untergrenze 2²⁵ s ≈ 388 d; AGENTS.md-Präzisierung (gemessene
  Quellen-Eigenschaft → Mountain; Materialisierung/Transport → Mycelium) im
  selben Atom. Ziel: **Mountain**. Quelle: river folge51/52/53 (der frühere
  Pfad `…-mountain-folge193.md` liegt jetzt in `archiv/`).
- **Browser-Fork-Build laden** (Operator-Akt) — Artefakt `chrome-mv3`
  (Manifest-Version **0.17.1**) aus Lauf `36401074967` (success) nach
  `tools/browser-extension/.output/chrome-mv3` geladen (gitignored; 10 Dateien,
  `manifest.json` vorhanden). Schritt: dediziertes/Throwaway-Profil →
  `chrome://extensions` → Developer mode → Load unpacked → diesen Ordner wählen;
  dann Bridge `ws://127.0.0.1:4517` + Token im Dashboard verbinden
  (`tools/browser-extension/README.md:25–42`). Ziel: **Future**
  (Operator-Queue; Träger-Doku `docs/surveys/survey-2026-09-20-browser-anbindung.md`).
  Quelle: River-Session 54.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
