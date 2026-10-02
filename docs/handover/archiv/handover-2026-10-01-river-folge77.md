<!--
  title: Handover — River-Folge 77 (2026-10-01)
  session: River-Folge 77
  class: handover
  date: 2026-10-01
  sha256: 5a2d48c44c737a51984ed757da9c5490b7708ce79a875b93b2adfc892b1796fd
  status: live
-->
# Handover — River-Folge 77 (2026-10-01)

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

Origin: river folge77. Faltung deiner `## An river`-Blöcke (future-folge163).

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

Origin: river folge77. Rat-Empfehlung 2026-10-01 zum Stations-Knoten (Register-Schreibakt, deine Feder).

- **`station`-Direktive für ABK (und künftige Stationen):** die Membran matcht Stations-Risse am **Instrumentennamen** (`station-ABK`), nie am Kanalnamen. Der Code `ABK` steht heute nur in der URL (`abk_dbdt_1h.bin`) und ist ungeparst; River hat den `station <CODE>`-Parse-Arm gebaut (`src/archivar/parse.rs`, `types.rs`). *Braucht (mountain):* eine `station ABK`-Direktive am ABK-Block (`phi/sources.φ:1786`, `on earth 68.358 18.823 380`) setzen.
- **Namens-Schuld (gemessen):** `field intermagnet_dbdt` ist am ABK- und am SOD-Block doppelt deklariert (`phi/sources.φ:1792` + `:1800`) — dieselbe Zeichenkette für zwei Stationen; das Register muss die Namen trennen oder die Doppelung als gewollt benennen.
- **Produzent gebaut (River):** `station_convergence_probe` schreibt den gemessenen Stations-Riss non-destruktiv in `data/weberin_verdicts.bin` (merge, `sep` aus dem eigenen Encounter, `weave_epoch` live, kein Hardcode). Caveat: `main_flow.rs:1032` lädt den Blob **einmal beim Start** — eine laufende Membran sieht ihn erst nach Neustart.

## Offen (aufgeschlüsselt)

### σ-Asset `dr3_stars.bin` — σ-Zensus nach Re-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Dispatch nach `/commit`+Push (der Fix ist uncommitted)
- **Lage:** (gemessen 2026-10-01) der `tap_compiler`-Quoting- **und** Count-Bug ist gefixt (River, dieser Atom): `xq`/`alias_col` quoten alias-qualifizierte Identifier nicht mehr, der Band-Count nutzt `from_clause` mit Join-Alias; build grün; lokaler Live-TAP-Repro **400-frei** (Bright-Band 10 Records). Aber der laufende Run `36877802455` (head `7c36c964`) fährt den **committeten** Stand und 400t weiter (Live-Log im chrome-devtools-Browser, 2026-10-01) — der Fix wirkt erst nach Commit+Push.
- **Blockade:** der Fix ist uncommitted; ohne Push sieht CI ihn nicht
- **Braucht:** `/commit`+Push; dann `ci_manage cancel 36877802455` + `gh workflow run gaia-cdn.yml`; nach Grün `archive_search --verdict`/`--sniff` auf `dr3_stars.bin`, σ-Zensus.

### Kalibrierte Null (Westfall–Young max-T) — CI-Messung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `wy-max-t 36867148250` (Redispatch 2026-10-01)
- **Lage:** (gemessen 2026-10-01 via `ci_manage jobs`) `36867148250` **läuft**: `wy (sod-2024)` + `wy (abk-2025)` `in_progress`, `wy (abk-2024)` queued, `wy (selftest)` success; Timeout 420 min; die Run-Liste zeigt `queued` (grob). Bau steht (`tools/measure/src/bin/wy_max_t_probe.rs`, `.github/workflows/wy-max-t.yml`).
- **Blockade:** kein eigener Schritt bis Lauf-Ende
- **Braucht:** `ci_manage log 36867148250`; dann `docs/paper/gic-causal-driver.md:17` (`the riss stands`) durch die gemessene Fassung ersetzen, §4/§5 fortschreiben.

### GIC-Treiber-Kanäle Bs · Clock · P_dyn · M_A
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `/commit`-Wort + Probe-Lauf des erweiterten `bz_retro_probe`
- **Lage:** (gemessen 2026-10-01, build+check grün) Bs/Clock/P_dyn/M_A verdrahtet und als Paare gegen dB/dt geführt (`bz_retro_probe.rs`); Newell war bereits verdrahtet; **Bx ist jetzt geholt** (`COMP_BX`), `M_A` nutzt das Gesamtfeld `B=√(Bx²+By²+Bz²)`. Uncommitted.
- **Blockade:** kein eigener Schritt bis Commit/CI
- **Braucht:** mit `/commit` committen; dann Probe-Lauf, die vier Kanäle in den Verdict-Baum aufnehmen.

### ENSO — Positivkontrolle + Desaisonalisierung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `/commit`-Wort + Test-Workflow-Arm
- **Lage:** (gemessen 2026-10-01) die synthetische Positivkontrolle (Report-Zeile + `#[cfg(test)]`) wurde von **dieser Session** gebaut — Sub-Agent `ses_f08657a03ffe7jqA9IJuy9uLmR` (`state/zustand/ereignisse.φ:31624`), `enso_blatt_probe.rs` +145; build+check grün. Der Test-Arm ist jetzt in `enso-probe.yml` eingehängt (`cargo test --bin enso_blatt_probe` vor dem Lauf). τx/SOI als physikalische Kontrollkanäle fehlen weiter.
- **Blockade:** kein eigener Schritt bis Commit/CI
- **Braucht:** mit `/commit` committen; dann den Workflow-Lauf; aus futures nemotron-Stimme desaisonalisieren **vor** der TE.

### Finsternis-Haus-Cross-Check — Zahlen pending bis Re-Manifest
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `/commit` + Re-Lauf `eclipse_shadow_probe` (`inpop-epm-cdn 36868008026`, success 2026-10-01 = Re-Manifest erfüllt)
- **Lage:** (gemessen 2026-10-01) `docs/paper/eclipse-clock-worldlines.md:45` trägt die DE↔INPOP-Zahlen bereits als „**pending until the ephemeris CDN re-manifest**" (mountain-217-Wort erfüllt); der Solver-Bug belastet die alten Bins.
- **Blockade:** kein eigener Schritt bis Re-Manifest
- **Braucht:** nach dem Re-Manifest `eclipse_shadow_probe` neu ziehen.

### Flyby-path-2-Kette
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** OMNI2 CDAS publiziert ≥2026-09-03; kp.gfz.de-JSON-API 200; ESOC-Recon publiziert
- **Lage:** (gemessen 2026-10-01, korrigiert) **OMNI2** HAPI `OMNI2_H0_MRG1HR` → `1201` für 2026-09-26..10-01; Verfügbarkeit endet ~**2026-09-03** (~28 d). **kp**: der Code-Endpoint `kp.gfz.de/app/json/?start=…T00:00:00Z&end=…T00:00:00Z&index=Kp&status=def,nowcast&format=JSON` liefert **200** (41 Werte, status `pre`); `status=def` allein → leere Arrays (Definitive noch nicht publiziert). Der frühere „HTTP 500" war ein falsch gebauter URL (bare Daten ohne ISO-Suffix) — **kein Serverdefekt**. **ESOC** `ephemeris_juice_recon.bin` → CDN 404, Gate pending. Die σ-Metrik-Stimme liegt in `state/stimmen/2026-09-30_flyby-sigma-frage_nemotron.txt`.
- **Blockade:** externe Datenvorläufe
- **Braucht:** bei Fälligkeit `flyby_path2_fill` (CI) lesen, Addendum mit `m_eff`/Joint-Permutation + gesiegeltem Zeitraster fortschreiben.

### Sonne-Erde-Blatt — Rat: zerlegen (2026-10-01)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `/commit`
- **Lage:** (gemessen 2026-10-01) (a) Register-Abgleich gefahren: RTSW `:158/:164`, OMNI `:575`, GOES-XRS `:152`, GOES-EUV `:417`, AIA `:1802`, EVE `:2362`, INTERMAGNET 154 `:5383–:6913`, SST/nino34 `:11176`, Open-Meteo `:218/:3499`, USGS `:96`, ISC `:9433` tragen; **SDO-Mission** und **Argos** `absent`; ERA5 `declined` (`declined_sources.φ:1007`). (b) Träger `docs/blatt/sonne-erde-blatt.md` angelegt (sha `4accdd0c…`), Nicht-Pfeil ENSO/GIC als Nicht-Pfeil. (c) Kopplungspfeile = Weberin-Verdicts.
- **Blockade:** kein eigener Schritt bis Commit
- **Braucht:** mit `/commit` committen; die Pfeil-Frage trägt der Weberin-Punkt.

### Frühwarnsystem — Rat: zerlegen (2026-10-01)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende der kalibrierten Null (`wy-max-t 36867148250`)
- **Lage:** (gemessen 2026-10-01) (a) kalibrierte Null läuft (`wy-max-t 36867148250`); (b) **Form-Dokument angelegt:** `docs/blatt/fruehwarnsystem-praeregistrierung.md` (sha `e0d2f181…`, `status: unsealed`) — Vorhersage-Zelle `Bz-Schwelle → dB/dt an Station X → Verzögerung Z`, Fehlschlag-Kriterium, α `pending` bis zum max-T-Lauf; (c) Sturm-Trigger extern `wartend`.
- **Blockade:** das Siegel setzt die α-Ebene der kalibrierten Null voraus (Siegeln gegen 10⁻¹ verboten)
- **Braucht:** nach dem max-T-Lauf α setzen; das Siegel ist der Operator-Akt.

### Weberin — Live-Verdict-Term gebaut; Stations-Knoten offen (Rat 2026-10-01)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `/commit` (der Bau steht) / Rats-Wort für den Stations-Knoten
- **Lage:** (gemessen 2026-10-01, build+check grün) der **Live-Verdict als Query-Term** ist gebaut: `sky_tick` (`omega.rs:1047`) reduziert je Frame das Live-Ledger per `riss_names`; benannte Hüllen-Fäden (Bodies `field.eph`, Stationen `matrix.metas`) werden bei Riss-Namen **nicht** als `S2Osc` gefaltet, sondern in `SkyState.riss` / `SkyReport.riss_count` getragen (HUD `riss N`); Test `tests.rs:1476`. Keine Wire-Änderung, kein neuer Sample-Slot. **Dabei gefundener Riss:** das Live-Ledger ist **body-only** — `VerdictLine.knot`/`BodyLine` (`weberin_verdicts.rs:72`, `weberin.rs:32`) tragen keinen Stations-Code; der Probe-Name `station-ABK` (`riss_knoten_probe.rs:69`) findet den Hüllen-Faden-Schlüssel `intermagnet_dbdt` (`phi/sources.φ:1792`) nicht → der ABK↔SWARM-Stations-Riss erreicht den Loop noch nicht.
- **Blockade:** kein Fremdblock — Rat-Empfehlung liegt vor, der Bau ist ein Atom
- **Braucht:** nach der Rat-Empfehlung (2026-10-01) bauen: (a) `WitnessLine { Body(BodyLine), Station(StationLine {IntermagnetGround=6, SwarmOverflight=7}) }`, `knot: [Option<WitnessLine>;2]`, `sep_m`→`sep`, Round-Trip-Tests (`weberin_verdicts.rs`); (b) `station`-Direktiven-Parse (Register-Schreibakt → `## An mountain`); (c) `station_code` SourceConfig→Channel→`NameMeta`, `sky_tick` matcht `station-{code}` (nie den Kanalnamen) + Stations-Test; (d) Produzent `station_convergence_probe`-Blob-Arm **gebaut** (merge in `data/weberin_verdicts.bin`, `sep` aus eigenem Encounter). Rest: die `station ABK`-Direktive (Mountain) + `/commit`. Kanonisches Mapping: `phi/sources.φ:1786`; der Metas-Schlüssel bleibt Kanalname.

## LOCK

- **GIC — DOI-Knoten, dann GEMS-Einreichung (Operator-Hand).** Vorbereitung privat
  `state/future/gic-causal-driver-doi-metadaten.md`; Send = Operator-Hand.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Baum ist mit parallelen
Linien-Sessions geteilt (Mountain/Mycelium/Sensory Handover untracked) — **Commit als
Letzter**. Pfad-begrenzte Commit-Pfade dieser Session:

- `tools/harvest/src/bin/tap_compiler.rs` (Quoting- + Count-Pfad-Fix, σ-Asset-Blocker)
- `tools/measure/src/bin/bz_retro_probe.rs` (Bs/Clock/P_dyn/M_A + Bx)
- `tools/measure/src/bin/enso_blatt_probe.rs` (synthetische Positivkontrolle, Sub-Agent `ses_f08657a03ffe7jqA9IJuy9uLmR`)
- `.github/workflows/enso-probe.yml` (Test-Arm der Positivkontrolle)
- `src/mathematikerin/s2.rs` · `src/mathematikerin/omega.rs` · `src/mathematikerin/tests.rs` · `src/mathematikerin/machines/matrix.rs` (Live-Verdict-Query-Term + Stations-Match)
- `src/weberin.rs` · `src/archivar/weberin_verdicts.rs` (WitnessLine/StationLine, `sep`)
- `src/archivar/types.rs` · `src/archivar/parse.rs` · `src/archivar/extract.rs` · `src/archivar/channels.rs` · `src/archivar/main_flow.rs` · `src/archivar/ionex.rs` · `src/archivar/rinex.rs` · `src/archivar/relay.rs` · `src/archivar/tests.rs` (`station_code`-Durchtrag)
- `tools/measure/src/bin/weberin_body_verdict.rs` · `weberin_mpc_spk_verdict.rs` · `weberin_verdicts_compiler.rs` (WitnessLine-Ripple)
- `tools/measure/src/bin/station_convergence_probe.rs` (Stations-Blob-Produzent)
- `docs/blatt/sonne-erde-blatt.md` · `docs/blatt/fruehwarnsystem-praeregistrierung.md` (Rat-Auftrag)
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`:122` descoped)
- `docs/paper/eclipse-clock-worldlines.md` (1919-Anker nachgeführt)
- `docs/handover/handover-2026-10-01-river-folge77.md`, und `…-folge76.md` → `archiv/` (Move)
- `state/operator-gespraeche/2026-10-01-river.md` (gitignored, nicht committen)

**Nicht meine Hunks:** `phi/sources.φ`, `src/archivar/pds3_img.rs`, `docs/zustand/dropped-baseline.md`
(fremd, unangetastet).

## Burn: open 0.0204 · close 0.68 · cap 0.75 · Grund: operator-getriebenes Mehr-Runden-Atom — Fix-all über CI-Diagnose (Browser), tap_compiler-Fix, 2×Ratssitzung, komplette Weberin-Kette, 11 Taucher-Dispatches
