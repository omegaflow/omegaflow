<!--
  title: Handover — River-Folge 79 (2026-10-02)
  session: River-Folge 79
  class: handover
  date: 2026-10-02
  sha256: 50035ca77f0bd44bacb18f633f62b5f90544243f4f8e597b7d5e437b76ecfbab
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
„braucht es pro?" (Wiederholung; gemessen: nein — flash löste die GIC-Kanal-Faltung identisch) | 2026-10-02 | Operator (Session, River 79)
„braucht es pro und max?" (Architektur-Umsetzung; gemessen: nein — der Rat entschied, flash setzte um) | 2026-10-02 | Operator (Session, River 79)
„glm 5.3 ist auch stark" (externe Zweitstimme z.ai/GLM-5.3) | 2026-10-02 | Operator (Session, River 79)
„nutze den rat aber auch die ui chats (z.ai, kimi, claude, tryopenly, togetherai)" | 2026-10-02 | Operator (Session, River 79)

## An future

Origin: river folge79 (Faltung aus folge78, noch nicht gefaltet).

- **GIC-Schluss-Gegenlesung / `the riss stands`:** Rivers Teil bleibt der eine Satz —
  nach dem `wy-max-t`-Lauf wird `docs/paper/gic-causal-driver.md:17` („the riss stands")
  durch die gemessene Fassung ersetzt; §3.2 offene Konstruktion, α unbenannt bis dahin.
  Kein Satz vor dem Messergebnis.
- **ENSO-Design / Flyby-σ-Metrik / TE-Novelty (nemotron-Stimmen):** gefaltet — sie
  speisen die ENSO-Positivkontrolle (Desaisonalisierung vor der TE) bzw. den
  GIC-Paper-Text; keine neue Antwort nötig.
- **Frontier-Kanäle + Kanal-Routing:** bestätigt. Kimi K3 + GLM-5.3 ohne Login
  (tryingopen/together), erste Route `chat.z.ai` (per-Akt-Wort), arena als benannter
  Fallback; die hängenden UI-Routen entfallen. Roster-Update ist deins.

## An mountain

Origin: river folge79 (Faltung aus folge78; `station ABK` gefaltet in `7fce797df`, die Namens-Schuld steht).

- **Namens-Schuld (gemessen 2026-10-02):** `field intermagnet_dbdt` ist am ABK- und am SOD-Block
  doppelt deklariert (`phi/sources.φ:1793` + `:1802`) — dieselbe Zeichenkette für zwei Stationen;
  das Register muss die Namen trennen oder die Doppelung als gewollt benennen. Funktional heute
  (der Metas-Schlüssel ist der Kanalname), aber namens-blind.
- **`jwst_spectra`-Richtung bei absentem Abstand:** der Consumer trägt extragalaktische Records
  jetzt am deklarierten `at`-Anker (`main_flow.rs`, `jwst_spectrum_motion`); der Wire hat keinen
  Richtungs-Slot ohne Abstand. Eine Richtungs-Führung bräuchte ein `z` im JWS1-Bin — deine Feder.

## Offen (aufgeschlüsselt)

### tapvizier-TAP-Klasse — Verifikationslauf (geheilt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende der Re-Dispatches `rave-cdn 36984925478` / `sb9-cdn 36984928491` / `first14-cdn 36984931299`
- **Lage:** (gemessen 2026-10-02) Wurzel gemessen: river-77 („ADQL quoting") setzte in der
  `CONTAINS`-Join-Klausel ein literales `t.`/`j.` **vor** `alias_col` → `t.t."RAJ2000"`,
  `j.j."RA_ICRS"` → CDS 400 „unresolved identifiers"; die ADQL war sonst gültig. Beide
  Format-Positionen (`--crossmatch` und `--crossmatch-z`) korrigiert; lokal gegen
  `tapvizier` re-generiert: sb9 → 13 Spalten/5 Zeilen, rave-Slice-0 (`--where`+pm) → 12/5.
- **Blockade:** keiner
- **Braucht:** nach Push `gh workflow run rave-cdn.yml` (sb9/first14 ebenso); `ci_manage log <id>`
  bestätigt grün. `tap_compiler.rs` trägt `--fail-with-body` + `body=…` (1600 B) im Fehlerfall.

### σ-Asset `dr3_stars.bin` — falscher Release-Tag, Re-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `gaia-cdn 36984934208`
- **Lage:** (gemessen 2026-10-02) der 56-B-Bin (`1 704 003×56 = 95 424 168 B`) wurde vom Lauf
  `36943970102` in den **falschen** Release `tapvizier.cds.unistra.fr` hochgeladen
  (`gh api … updated 2026-10-02T00:48:55Z`); der registrierte Release `ssd.jpl.nasa.gov` trägt
  noch das **legacy 44-B**-Asset (`75 001 828 B`, updated `2026-08-23`, sha `fb9a1408…` → keine
  σ-Slots). Ursache: `gaia-cdn.yml` übergab kein `--release-tag` → Compiler-Default
  `tapvizier.cds.unistra.fr`. Fix: `--release-tag ssd.jpl.nasa.gov` gesetzt.
- **Blockade:** keiner
- **Braucht:** nach Push `gh workflow run gaia-cdn.yml`; dann `ssd.jpl.nasa.gov`-Asset snifen
  (56-B-Stride, 95 424 168 B) und den 3-Slot-σ-Zensus rechnen. Ein σ-Zensus-Bin existiert **nicht**
  — `star_dmax_probe`/`vlies_density_probe` lesen den Katalog ohne die σ-Felder
  (`spatial.rs` `sigma_plx_mas`/`sigma_pm_ra_masyr`/`sigma_pm_de_masyr` bleiben unkonsumiert);
  minimales neues Bin nötig (Layout `tap_compiler.rs:437-488`, 0.0 = absent).

### GIC-Quartal `sod-2025-q1` — fehlendes 16. Artefakt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `bz-retro-probe 36984937798`
- **Lage:** (gemessen 2026-10-02) in `36943972951` waren 15/16 Quartals-Jobs `success`; nur
  `sod-2025-q1` failed, sein Job-Log ist per API `unread` (HTTP 404). Die 15 vorhandenen Quartale
  sind als §4.6 des GIC-Papers gefaltet (vier neue Kanäle, alle family-bound).
- **Blockade:** keiner
- **Braucht:** nach dem Re-Dispatch `ci_manage log 36984937798` lesen; bei Grün §4.6 auf 16
  Artefakte erweitern, sonst den einen Job-Grund benennen.

### Kalibrierte Null (Westfall–Young max-T) — Sharding gebaut, Verifikationslauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende des nächsten `wy-max-t` (Re-Dispatch @diesem Atom)
- **Lage:** (gemessen 2026-10-02 via GH-API) die drei wy-Jobs `36943967388` wurden bei exakt
  **6 h 00 m** gekappt — GitHub-Hosted-Hartgrenze, nicht `timeout-minutes: 420`. Rat-Entscheid
  umgesetzt (flash): `wy_max_t_probe.rs` schreibt je Shard die volle B×m-Null-Matrix
  (`--perm-from/--perm-to/--out-null`), `--combine` poolt die Replikate, studentisiert
  **mitgliedsweise** vor dem Within-Replicate-Max und zieht dasselbe μ/σ für die Observed-Werte;
  `wy-max-t.yml` fährt 3 Shards/Station-Jahr + Combine-Job, `timeout-minutes 180`. Test
  `shard_split_pools_to_single_run` bit-identisch zur Einzellauf-Null. GLM-5.3 bestätigt die
  Validität und genau diese Ordnungsbedingung.
- **Blockade:** keiner
- **Braucht:** `gh workflow run wy-max-t.yml`; nach Grün `docs/paper/gic-causal-driver.md:17`
  („the riss stands") durch die gemessene Fassung ersetzen, §4/§5 fortschreiben. Named pending:
  KDE-Terme der Zielrandverteilung je Mitglied cachen (permutationsinvariant) — der eigentliche
  Kostenhebel.

### Weberin — getrennter Station-Blob + Membran-Merge (gebaut, Verifikationslauf)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `station-convergence` + Membran-Load
- **Lage:** (gemessen 2026-10-02) Rat-Entscheid „ein Produzent je Pfad" umgesetzt (flash): die
  Live-Probe schreibt/defaultet `data/weberin_verdicts_station.bin` und manifestiert bei
  `--ci-mode` auf `ssd.jpl.nasa.gov-weberin`; `station-convergence.yml` trägt `contents: write`;
  der Compiler bleibt kanonisch für die Body-Zeilen; `main_flow.rs:1036ff` lädt beide mit
  CDN-Fallback (`load_weberin_verdicts_or_cdn`) und merged mit expliziter Präzedenz
  (`merge_verdict_lines`: Station > Body, deterministisch, nicht load-order). Ein frischer Rechner
  zieht beide Assets aus dem Release.
- **Blockade:** keiner
- **Braucht:** `gh workflow run station-convergence.yml`; den Blob am CDN snifen. Named follow-ups
  (GLM-5.3/Claude-Kritik): Pair-Epoch-Konsistenz beider Blobs, und der fehlende Station-Blob als
  benannter Zustand (≠ „keine Messung").

### Flyby-path-2-Kette
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** OMNI2 CDAS publiziert ≥2026-09-03; kp.gfz.de liefert `def`; ESOC-Recon publiziert
- **Lage:** (gemessen 2026-10-02) OMNI2 H0 1-hr: neuester Echtwert `2026-09-03T00:30Z`, danach nur
  Fill (HAPI: „last … data Sep 02, 2026"). kp.gfz.de: bare Pfad 500, parameterisiert 200 (11/2 d,
  49/Woche), alle `status:"pre"` — `def` noch nicht publiziert. ESOC `ephemeris_juice_recon.bin`:
  CDN 404, lokal absent — **und in `phi` nicht registriert** (Riss: `sgrep recon.bin phi` = 0).
- **Blockade:** externe Datenvorläufe; fehlende ESOC-Registerzeile
- **Braucht:** bei Fälligkeit `flyby_path2_fill` (CI) lesen. ESOC-Arm bleibt `pending` (Rats-Beschluss):
  die Warte steht in `state/zustand/wartend.φ` (`ephemeris-juice-recon`, Aufnehmer river); die
  `phi/sources.φ`-Zeile schreibt Mountain im Atom der ersten Manifestation — keine tote url-Zeile.
  Named follow-up (GLM-5.3-Kritik): Probe-Kadenz/Deadline für den Warte-Trigger.

### Frühwarnsystem — Präregistrierung (Rat, zerlegt 2026-10-01)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** α-Ebene der kalibrierten Null (`wy-max-t`)
- **Lage:** (gemessen 2026-10-02) Träger `docs/blatt/fruehwarnsystem-praeregistrierung.md`
  (`status: unsealed`) committet; α `pending`.
- **Blockade:** das Siegel setzt die α-Ebene der kalibrierten Null voraus (Siegeln gegen 10⁻¹ verboten)
- **Braucht:** nach dem max-T-Lauf α setzen; das Siegel ist der Operator-Akt.

### ENSO — Desaisonalisierung gebaut, Verifikationslauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende des nächsten `enso-probe`
- **Lage:** (gemessen 2026-10-02 via `gh run download 36943976395`) die Positivkontrolle ist
  **bestanden** — die gepflanzte Kopplung wird bei lag 3 detektiert (TE `7.6483e-1` > per-lag-Thr
  `2.7245e-1` und > fam `3.0247e-1`); Bz↔SST bleibt `silent`. Rat-Entscheid „monatliche
  Klimatologie, uniform, nach `bin_monthly`" umgesetzt (flash), **erweitert um die sd-Division**
  (GLM-5.3/Claude-Kritik: auch die zweite Moment-Modulation ist ein gemeinsamer Jahrestreiber);
  Test `deseasonalize_removes_common_annual_cycle` grün.
- **Blockade:** keiner
- **Braucht:** `gh workflow run enso-probe.yml`. Named follow-ups: Surrogate auf der Anomalie-Skala
  bauen (sonst Null-Mismatch) und eine saisonale Positivkontrolle als Fixture; danach die
  Bz↔SST-Zeile im GIC-Paper fortschreiben.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Baum ist mit parallelen
Linien-Sessions geteilt — **Commit als Letzter**. Pfad-begrenzte Commit-Pfade dieser Session:

- `tools/harvest/src/bin/tap_compiler.rs` · `.github/workflows/gaia-cdn.yml`
- `docs/paper/gic-causal-driver.md`
- `tools/measure/src/bin/wy_max_t_probe.rs` · `.github/workflows/wy-max-t.yml`
- `tools/measure/src/bin/station_convergence_probe.rs` · `.github/workflows/station-convergence.yml`
- `src/archivar/main_flow.rs` · `src/archivar/weberin_verdicts.rs`
- `tools/measure/src/bin/enso_blatt_probe.rs`
- `docs/handover/handover-2026-10-02-river-folge79.md`, und `…-folge78.md` → `archiv/` (Move)

**Nicht meine Hunks (gemessen 2026-10-02, `git status`):** `kernel-flatten.yml`, `src/archivar/fit.rs`,
`tools/harvest/src/bin/ceers_spectra_compiler.rs`, die Mycelium-Handover-Moves/Neuanlagen,
`dsn-cdn.yml`/`jades-cdn.yml` — alle unangetastet.

## Burn: open 0.0 · close 0.3700 · cap 0.5 · Grund: 4 Rat-Entscheidungen + externe Stimmen (GLM-5.3/Claude) + 4 Fixes (wy-Sharding, Weberin-Merge, ENSO-Standardisierung, ESOC-Warte), ein Pass
