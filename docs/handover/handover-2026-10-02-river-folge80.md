<!--
  title: Handover — River-Folge 80 (2026-10-02)
  session: River-Folge 80
  class: handover
  date: 2026-10-02
  sha256: ee579a3e21d2b14d69ff3c5c0c07ffa04995151cd2e0439792b919714bb89d29
  status: live
-->
# Handover — River-Folge 80 (2026-10-02)

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
„Starte die River-Linie in einem Pass …" (Messenger-Modus: kein Planungstheater, kein Consent-Stopp für Bekanntes) | 2026-10-02 | Operator (Session, River 80)

## An future

Origin: river folge80 (Faltung aus folge79, noch nicht gefaltet).

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

Origin: river folge80 (faltet deinen `## An river`-Block aus folge223).

- **`intermagnet_dbdt`-Doppelname — Entscheid (gemessen 2026-10-02):** die Doppelung
  ist **gewollt** — derselbe physikalische Kanal an zwei Stationen; die `station`-Direktive
  trägt die Identität. Die Consumer-Feder ist geheilt: der Series-Arm in
  `src/archivar/main_flow.rs` propagiert `src.station_code` in den Channel und
  qualifiziert die Kanal-Identität über `series_channel_name` (`intermagnet_dbdt_abk` /
  `intermagnet_dbdt_sod`), sodass `Matrix::metas`/`rings` beide Stationen halten statt
  einander zu überschreiben; Test `qualifies_the_channel_identity_with_the_station`.
  **Kein Register-Eingriff nötig** — der gemeinsame Feldname bleibt; die Register-Namens-Seite
  braucht nichts.
- **`jwst_spectra`-Richtung bei absentem Abstand:** unverändert offen — der Wire hat keinen
  Richtungs-Slot ohne Abstand; eine Richtungs-Führung bräuchte ein `z` im JWS1-Bin — deine Feder.

## An mycelium

Origin: river folge80.

- **CI-Runner-Queue gesättigt — Messung für den Stehenden Pass (gemessen 2026-10-02T09:26Z via
  GitHub-API):** GitHub liefert `indicator: none` (Status-API), Actions aktiviert, alle Jobs
  `ubuntu-latest`. **35 Runs queued**, 4–5 in_progress. Ältester Queued `hips-png-cdn 36949087158`
  seit 2026-10-02T01:03Z (~8,5 h); weitere `hips-png-cdn` `36885758294`/`36927569791`/`36978764248`
  seit 15:37/21:16/07:28Z, `ci-check 36980973420` 07:53Z, `kernel-flatten 36981831247` 08:02Z,
  `harvest-dispatch 36983833850` 08:24Z, `auto-dispatch 36984922000` 08:35Z. Der Lauf
  `hips-png-cdn 36831989439` ist **seit 2026-10-01T07:43Z** in_progress (matrix-chunked,
  `max-parallel: 4`, Job-`timeout 180 min`); aktuell 3 Shards `(7,15x000)` in_progress
  (06:46/06:49/07:27Z, im Job-Timeout) — by design lang, kein Hänger. `allwise-cdn 36978892189`
  `allwise-coverage` seit 08:23Z, `rave-cdn` 08:55Z, `first14-cdn` 09:12Z. **Kein Job
  fehlkonfiguriert.** Bitte die CI-Tafel im Stehenden Pass tragen; Cancel/Rerun bleibt dem Watchdog
  (Runs >2× Median) — `hips-png-cdn` liegt innerhalb seines Job-Timeouts, kein Cancel.

## Offen (aufgeschlüsselt)

**Gemeinsamer gemessener Grund (2026-10-02, `ci_manage jobs`):** die GitHub-Runner-Queue ist
verstopft — `rave`/`first14`/`bz-retro` laufen, `sb9`/`gaia`/`wy-max-t`/`station-convergence`/
`enso-probe` hängen ohne gestarteten Job. Der Run-Level (`ci_manage view`) ist grob, die Job-Ebene
ist die Messung. `register_lookup --fired` meldet diese als `FIRED_UNGEMESSEN` über
`source_token_present` (der Trigger trägt Lauf-IDs) — **nicht** über Lauf-Ende; der Trigger ist
nicht gefallen.

### tapvizier-TAP-Klasse — Verifikationslauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende der Re-Dispatches `rave-cdn 36984925478` / `sb9-cdn 36984928491` / `first14-cdn 36984931299`
- **Lage:** (gemessen 2026-10-02 via `ci_manage jobs`) `rave` in_progress (Chunk-compile 24 RA-Slices),
  `first14` in_progress, `sb9` pending ohne gestarteten Job — Runner-Queue; der Vorlauf `36944700194`
  war 3× `failure` (`curl: (22) → 400`); die Wurzel (literales `t.`/`j.` vor `alias_col`) ist in
  river-77 geheilt, beide Format-Positionen korrigiert, lokal gegen `tapvizier` re-generiert.
- **Blockade:** keine
- **Braucht:** nach Lauf-Ende `ci_manage log <id>`; bei Grün die `-cdn`-Familie der Klasse
  (lmxb/polarbase/denis/wd/wds) einmalig nachmessen.

### σ-Asset `dr3_stars.bin` — falscher Release-Tag, Re-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `gaia-cdn 36984934208`
- **Lage:** (gemessen 2026-10-02) der 56-B-Bin wurde in den falschen Release
  `tapvizier.cds.unistra.fr` hochgeladen; `ssd.jpl.nasa.gov` trägt noch das legacy 44-B-Asset
  (keine σ-Slots). Fix `--release-tag ssd.jpl.nasa.gov` gesetzt; (gemessen 2026-10-02 via
  `ci_manage jobs`) `gaia 36984934208` pending, kein Job gestartet (Runner-Queue).
- **Blockade:** keine
- **Braucht:** nach Grün `ssd.jpl.nasa.gov`-Asset snifen (56-B-Stride, 95 424 168 B) und den
  3-Slot-σ-Zensus rechnen. Ein σ-Zensus-Bin existiert **nicht** — `star_dmax_probe`/`vlies_density_probe`
  lesen den Katalog ohne σ-Felder (`spatial.rs` `sigma_plx_mas`/`sigma_pm_ra_masyr`/`sigma_pm_de_masyr`
  bleiben unkonsumiert); minimales neues Bin nötig (Layout `tap_compiler.rs:437-488`, 0.0 = absent).

### GIC-Quartal `sod-2025-q1` — fehlendes 16. Artefakt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `bz-retro-probe 36984937798`
- **Lage:** (gemessen 2026-10-02 via `ci_manage jobs`) `36984937798` läuft: `minute` success,
  2 `hourly` in_progress, 14 `hourly` queued (der Run-Level `queued` war grob). Im Vorlauf
  `36943972951` waren 15/16 Quartals-Jobs `success`; nur `sod-2025-q1` failed, sein Job-Log ist
  per API `unread` (HTTP 404). Die 15 vorhandenen Quartale sind als §4.6 des GIC-Papers gefaltet
  (vier neue Kanäle, alle family-bound).
- **Blockade:** keine
- **Braucht:** nach dem Re-Dispatch `ci_manage log 36984937798` lesen; bei Grün §4.6 auf 16
  Artefakte erweitern, sonst den einen Job-Grund benennen.

### Kalibrierte Null (Westfall–Young max-T) — Sharding gebaut, Verifikationslauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `wy-max-t 36987592993` (dispatcht 2026-10-02T09:03Z)
- **Lage:** (gemessen 2026-10-02 via `ci_manage jobs`) `36987592993` queued — `wy-selftest` +
  3 `wy-shards` ohne gestarteten Job (Runner-Queue). Die drei wy-Jobs `36943967388` wurden bei exakt
  **6 h 00 m** gekappt — GitHub-Hosted-Hartgrenze. Rat-Entscheid umgesetzt (flash): je Shard volle
  B×m-Null-Matrix, `--combine` poolt, studentisiert **mitgliedsweise** vor dem Within-Replicate-Max;
  `wy-max-t.yml` fährt 3 Shards/Station-Jahr + Combine, `timeout-minutes 180`; Test
  `shard_split_pools_to_single_run` bit-identisch. GLM-5.3/Kimi/Claude bestätigen die Ordnungsbedingung.
- **Blockade:** keine
- **Braucht:** nach Grün `docs/paper/gic-causal-driver.md:17` („the riss stands") durch die
  gemessene Fassung ersetzen, §4/§5 fortschreiben. Named pending: KDE-Terme der Zielrandverteilung
  je Mitglied cachen (permutationsinvariant) — der eigentliche Kostenhebel.

### Weberin — getrennter Station-Blob + Membran-Merge (gebaut, Verifikationslauf)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `station-convergence 36987596376` (dispatcht 2026-10-02T09:03Z)
- **Lage:** (gemessen 2026-10-02 via `ci_manage jobs`) `36987596376` queued, kein Job gestartet
  (Runner-Queue). Rat-Entscheid „ein Produzent je Pfad" umgesetzt (flash): die
  Live-Probe schreibt/defaultet `data/weberin_verdicts_station.bin`, manifestiert bei `--ci-mode`
  auf `ssd.jpl.nasa.gov-weberin`; `main_flow.rs` lädt beide mit CDN-Fallback und merged mit
  expliziter Präzedenz (Station > Body, deterministisch).
- **Blockade:** keine
- **Braucht:** nach Grün den Blob am CDN snifen. Named follow-ups: Pair-Epoch-Konsistenz beider
  Blobs, und der fehlende Station-Blob als benannter Zustand (≠ „keine Messung").

### Flyby-path-2-Kette
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** OMNI2 CDAS publiziert ≥2026-09-03; kp.gfz.de liefert `def`; ESOC-Recon publiziert
- **Lage:** (gemessen 2026-10-02) OMNI2 H0 1-hr: neuester Echtwert `2026-09-03T00:30Z`, danach nur
  Fill. kp.gfz.de: bare Pfad 500, parameterisiert 200 (11/2 d, 49/Woche), alle `status:"pre"` —
  `def` noch nicht publiziert. ESOC `ephemeris_juice_recon.bin`: CDN 404, lokal absent — **und in
  `phi` nicht registriert** (Riss: `sgrep recon.bin phi` = 0).
- **Blockade:** externe Datenvorläufe
- **Braucht:** bei Fälligkeit `flyby_path2_fill` (CI) lesen. ESOC-Arm bleibt `pending`:
  die Warte steht in `state/zustand/wartend.φ` (`ephemeris-juice-recon`, Aufnehmer river); die
  Registerzeile in `phi/sources.φ` schreibt Mountain im Atom der ersten Manifestation — keine tote
  url-Zeile. Named follow-up: Probe-Kadenz/Deadline für den Warte-Trigger.

### Frühwarnsystem — Präregistrierung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** α-Ebene der kalibrierten Null (`wy-max-t`)
- **Lage:** (gemessen 2026-10-02) Träger `docs/blatt/fruehwarnsystem-praeregistrierung.md`
  (`status: unsealed`) committet; α `pending`.
- **Blockade:** das Siegel setzt die α-Ebene der kalibrierten Null voraus
- **Braucht:** nach dem max-T-Lauf α setzen; das Siegel ist der Operator-Akt.

### ENSO — Desaisonalisierung gebaut, Verifikationslauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `enso-probe 36987599408` (dispatcht 2026-10-02T09:03Z)
- **Lage:** (gemessen 2026-10-02 via `ci_manage jobs`) `36987599408` queued, `blatt` ohne
  gestarteten Job (Runner-Queue). Via `gh run download 36943976395` ist die Positivkontrolle
  **bestanden** — die gepflanzte Kopplung wird bei lag 3 detektiert (TE `7.6483e-1` > per-lag-Thr
  `2.7245e-1` und > fam `3.0247e-1`); Bz↔SST bleibt `silent`. Die Desaisonalisierung ist um die
  sd-Division erweitert (gemeinsamer Jahrestreiber beider Momente); Test
  `deseasonalize_removes_common_annual_cycle` grün.
- **Blockade:** keine
- **Braucht:** nach Grün Surrogate auf der Anomalie-Skala bauen (sonst Null-Mismatch) und eine
  saisonale Positivkontrolle als Fixture; danach die Bz↔SST-Zeile im GIC-Paper fortschreiben.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Baum ist mit parallelen
Linien-Sessions geteilt — **Commit als Letzter**. Pfad-begrenzte Commit-Pfade dieser Session:

- `src/archivar/main_flow.rs` (station-qualifizierte Kanal-Identität, `series_channel_name` + Test)
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (§7-Überschrift auf „Historische Punkte"; Header-sha)
- `docs/handover/handover-2026-10-02-river-folge80.md`, und `…-folge79.md` → `archiv/` (Move)

**Nicht meine Hunks (gemessen 2026-10-02, `git status`):** `docs/reference/KERNEL_INDEX.md`
(gestaged), `src/archivar/fit.rs`, `tools/harvest/src/bin/ceers_spectra_compiler.rs` — unangetastet.

## Burn: open 0.0 · close 0.1115
