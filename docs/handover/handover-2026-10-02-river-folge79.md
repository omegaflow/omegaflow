<!--
  title: Handover — River-Folge 79 (2026-10-02)
  session: River-Folge 79
  class: handover
  date: 2026-10-02
  sha256: d83b7c029a066c0a3ff628b51773d2c05c3a8bd154e148f86abff1aa1fef60cb
  status: live
-->
# Handover — River-Folge 79 (2026-10-02)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„kannst du bitte einen benschmark mit allen zur verfügung stehenden sinnvollen stimmen machen …" | 2026-10-01 | Operator (Session, River 76)
„warum nutzt du nur die schlechten stimmen wir brauchen wirklich fähige senior reviewer …" | 2026-10-01 | Operator (Session, River 76)
„… bitte z.ai + arena noch fahren und prüfe welcher z.ai ui chat besser funktioniert" | 2026-10-01 | Operator (Session, River 76)
„future ist schon dabei eine voices agenten lösung zu bauen bitte spreche dich mit ihr ab und bitte a und b" | 2026-10-01 | Operator (Session, River 76)
„braucht es pro?" (Dispatch-Profil-Rückfrage) | 2026-10-01 | Operator (Session, River 77)

## An future

Origin: river folge79 (Faltung aus folge78, noch nicht gefaltet).

- **GIC-Schluss-Gegenlesung / `the riss stands`:** Rivers Teil bleibt der eine Satz —
  nach dem `wy-max-t`-Lauf wird `docs/paper/gic-causal-driver.md:17` („the riss stands")
  durch die gemessene Fassung ersetzt; §3.2 (`:157-158`) offene Konstruktion, α (`:169`)
  unbenannt bis dahin. Kein Satz vor dem Messergebnis.
- **ENSO-Design / Flyby-σ-Metrik / TE-Novelty (nemotron-Stimmen):** gefaltet — sie
  speisen die ENSO-Positivkontrolle (Desaisonalisierung vor der TE) bzw. den
  GIC-Paper-Text; keine neue Antwort nötig.
- **Frontier-Kanäle + Kanal-Routing:** bestätigt. Kimi K3 + GLM-5.3 ohne Login
  (tryingopen/together), erste Route `chat.z.ai` (per-Akt-Wort), arena als benannter
  Fallback; die hängenden UI-Routen entfallen. Roster-Update ist deins.

## An mountain

Origin: river folge79 (Faltung aus folge78; `station ABK` gefaltet in `7fce797df`, die Namens-Schuld steht).

- **Namens-Schuld (gemessen 2026-10-02):** `field intermagnet_dbdt` ist am ABK- und am SOD-Block
  doppelt deklariert (`phi/sources.φ:1793` + `:1802`) — dieselbe Zeichenkette für zwei Stationen
  (`station ABK` `:1791`, `station SOD` `:1800` gesetzt); das Register muss die Namen trennen oder
  die Doppelung als gewollt benennen. Der `station_code` matcht `station-{code}`; der Metas-Schlüssel
  bleibt der Kanalname, daher ist die Doppelung heute funktional, aber namens-blind.
- **`jwst_spectra`-Richtung bei absentem Abstand (gemessen 2026-10-02, `## An river` gefaltet):** der
  Consumer trägt extragalaktische Records jetzt am deklarierten `at`-Anker (`main_flow.rs`,
  `jwst_spectrum_motion`); der Wire hat keinen Richtungs-Slot ohne Abstand. Eine Richtungs-Führung
  bräuchte ein `z` im JWS1-Bin (Compiler-Arm) — Register-/Contract-Entscheidung deine Feder.

## Offen (aufgeschlüsselt)

### tapvizier-TAP-Klasse — rave/sb9/first14 `*-cdn`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende des nächsten `rave-cdn`/`sb9-cdn`/`first14-cdn`-Dispatches
- **Lage:** (gemessen 2026-10-02) die drei CDN-Läufe `36944700194`/`36944703250`/`36944707063`
  enden `failure`; der Fehler ist `tap_query http exit status: 22: curl: (22) … 400` im ersten
  rave-Slice. **Riss:** die generierte ADQL ist gültig — lokal gegen `tapvizier` gemessen liefert
  der rave-Slice-0-Lineage-Query und der sb9-Join-Query **HTTP 200**; der 400 tritt nur auf dem
  Runner auf (Quelle-seitige Drossel/Sperre der Cloud-IP oder ein runner-spezifischer Zustand).
  `tap_compiler.rs` verwirft mit `-sSf` den Fehler-Body — die Ursache ist damit blind.
- **Blockade:** kein runner-naher Messpunkt für den 400-Body
- **Braucht:** `tools/harvest/src/bin/tap_compiler.rs` trägt jetzt `--fail-with-body` und druckt
  `body=…` im Fehlerfall; nach Push `gh workflow run rave-cdn.yml` (sb9/first14 ebenso), dann
  `ci_manage log <id>` liest den 400-Body. Bei bestätigter IP-Sperre: Zugangs-Zustand
  (`ip-blocked`) registrieren (Mycelium), nicht weiter bauen.

### σ-Asset `dr3_stars.bin` — σ-Zensus
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Census-Lauf (keiner dispatched)
- **Lage:** (gemessen 2026-10-02 via `archive_search --sniff`) das Asset ist am CDN:
  `ssd.jpl.nasa.gov/dr3_stars.bin`, 75 001 828 Bytes, sha256 `fb9a1408…`; `gaia-cdn 36943970102`
  endete `success`.
- **Blockade:** kein Census-Schritt benannt
- **Braucht:** σ-Zensus über `dr3_stars.bin` messen — Werkzeug wählen
  (`vlies_density_probe` / `star_dmax_probe` lesen `dr3_stars`); erste Messung: `sgrep dr3_stars tools/measure/src/bin`.

### Kalibrierte Null (Westfall–Young max-T) — CI-Messung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `wy-max-t 36943967388`
- **Lage:** (gemessen 2026-10-02 via `ci_manage jobs`) der Lauf `36943967388` endete `cancelled`
  wie seine zwei Vorgänger (`36867148250`, `36843561883`) — **drei Cancellations in Folge**, kein
  Messergebnis; nur der `selftest`-Job wurde `success`. Timeout 420 min, `cancel-in-progress: false`.
- **Blockade:** die wy-Jobs werden extern abgebrochen (Watchdog/Alter) — Ursache ungemessen
- **Braucht:** den Abbruchgrund des letzten wy-Jobs lesen (`ci_manage jobs`-Step/Alter); dann
  entscheiden, ob Timeout/Sharding oder der Watchdog der Auslöser ist. Danach erneut messen.

### GIC-Treiber-Kanäle Bs · Clock · P_dyn · M_A
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `bz-retro-probe 36943972951`
- **Lage:** (gemessen 2026-10-02 via `ci_manage jobs`) 15/16 Quartals-Jobs `success`, nur
  `hourly (sod-2025-q1)` `failure`; dessen Job-Log ist per API `unread` (job `110641675747`,
  Log-Endpoint HTTP 404). Die vier Kanäle sind verdrahtet (`COMP_BX`, `M_A`, `d9baca435`).
- **Blockade:** kein Log für den einen roten Quartals-Job
- **Braucht:** `gh workflow run bz-retro-probe.yml` neu dispatchen (oder den einen Job via
  `ci_manage rerun 36943972951`), dann die vier Kanäle aus den erfolgreichen Artefakten in den
  Verdict-Baum aufnehmen.

### ENSO — Positivkontrolle + Desaisonalisierung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `enso-probe 36943976395`
- **Lage:** (gemessen 2026-10-02 via `ci_manage view`) der Lauf endete `success`; das
  Ergebnis-Artefakt (`enso-probe`) ist noch nicht gelesen.
- **Blockade:** keiner
- **Braucht:** das `enso-probe`-Artefakt lesen (τx/SOI als Kontrollkanäle, Desaisonalisierung vor
  der TE) und den Befund in den GIC/ENSO-Text aufnehmen.

### Frühwarnsystem — Präregistrierung (Rat, zerlegt 2026-10-01)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `wy-max-t` (α-Ebene)
- **Lage:** (gemessen 2026-10-02) Träger `docs/blatt/fruehwarnsystem-praeregistrierung.md`
  (sha `e0d2f181…`, `status: unsealed`) committet (`d9baca435`); α `pending`.
- **Blockade:** das Siegel setzt die α-Ebene der kalibrierten Null voraus (Siegeln gegen 10⁻¹ verboten)
- **Braucht:** nach dem max-T-Lauf α setzen; das Siegel ist der Operator-Akt.

### Weberin — Produzent-Blob-Persistenz
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Contract-Wort (welcher Produzent ist kanonisch)
- **Lage:** (gemessen 2026-10-02) `station-convergence 36944080015` endete `success`, doch
  `station-convergence.yml:10` hat `permissions: contents: read` und lädt nur
  `station-convergence.txt` (`:31`) — `data/weberin_verdicts.bin` bleibt auf dem Runner. Der
  kanonische CDN-Produzent ist `weberin_verdicts_compiler` (`weberin-verdicts-cdn.yml`), der die
  **registrierten Body-Zeilen** faltet (`ssd.jpl.nasa.gov-weberin`); ein zweiter Upload derselben
  Datei durch den Live-Probe würde ihn überschreiben.
- **Blockade:** zwei Produzenten für `weberin_verdicts.bin` — der Live-Probe schreibt den Blob,
  der Compiler schreibt den kanonischen
- **Braucht:** Contract-Wort (Rat/Operator): ist der Live-Probe-Blob lokal oder muss er als
  Body-Zeile registriert werden, damit der Compiler ihn faltet? Danach `station-convergence.yml`
  bzw. `main_flow.rs:1036` entsprechend bauen.

### Flyby-path-2-Kette
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** OMNI2 CDAS publiziert ≥2026-09-03; kp.gfz.de-JSON-API 200 (definitiv); ESOC-Recon publiziert
- **Lage:** (gemessen 2026-10-01) OMNI2 `OMNI2_H0_MRG1HR` Verfügbarkeit endet ~2026-09-03;
  `kp.gfz.de` liefert 200 (41 Werte, status `pre`); ESOC `ephemeris_juice_recon.bin` CDN 404.
- **Blockade:** externe Datenvorläufe
- **Braucht:** bei Fälligkeit `flyby_path2_fill` (CI) lesen, Addendum mit `m_eff`/Joint-Permutation
  + gesiegeltem Zeitraster fortschreiben.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Baum ist mit parallelen
Linien-Sessions geteilt — **Commit als Letzter**. Pfad-begrenzte Commit-Pfade dieser Session:

- `tools/utils/src/bin/volume_builder.rs` · `tools/harvest/src/bin/tap_compiler.rs`
- `docs/handover/handover-2026-10-02-river-folge79.md`, und `…-folge78.md` → `archiv/` (Move)

**Nicht meine Hunks (gemessen 2026-10-02, `git status`):** `kernel-flatten.yml` (M),
die Mycelium-Handover-Moves/Neuanlagen (`folge219` → `archiv/`, `folge220`), die neuen
`dsn-cdn.yml`/`jades-cdn.yml` — alle unangetastet.

## Burn: open 0.0 · close 0.2351 · cap 0.5 · Grund: volume_builder-E0063-Heilung + tap_compiler-Fehler-Body + 10 CI-Messungen, ein Pass
