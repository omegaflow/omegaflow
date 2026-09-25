<!--
  title: Handover — Sensory-Folge 173 (Stand 2026-09-25)
  session: Sensory-Folge 173
  class: handover
  date: 2026-09-25
  sha256: cec83a9438c230d48e914996f3940fb7ab23cb1ced80707aec7564faa5c29c6e
  status: live
-->
# Handover — Sensory-Folge 173 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur
die eigenen Hunks — committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr
von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit
Messstempel) / **Blockade** / **Braucht**. Sortierung: erst logisch nach Akteur
(Linie | Rat | Operator | Dritter), dann chronologisch (Messdatum). Operator-gebundene
Punkte sind in Vorbereitung (autonom) und Akt (operator-gebunden) getrennt.

Die FR945 ist das persönliche Gerät des Operators; ihre Kennung (MAC) und ihre Daten
bleiben lokal (`.secrets.local`, `data/`), nie getrackt, nie am CDN.

## Stehender Pass (automatisch, keine Auswahl)

Die fälligen Einträge aus `docs/zustand/external-state.md` werden zu Session-Beginn
gemessen; ihr Ausgang steht im Zustand-Ledger, nicht als Kopie hier. Karte:
`docs/concepts/tools-map.md`.

- **Postfach** — `smail` + `state/mail/mail_ledger.φ` (fällig 2⁶ min).
- **CI-Status am HEAD** — Watchdog-Snapshot `/tmp/opencode/ci_status.md` (nur wenn
  jünger als der letzte HEAD-Wechsel), sonst `ci_manage list`/`view`; nie
  `gh run list`/`gh run view`.
- **Artefakt-Frische** — erzeugte Klassen (Tools, Core-Bin, CDN-Assets, WGSL,
  Firmware) hinken HEAD; nie einen aktuellen Satz über ein Artefakt hinter HEAD.

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### Blatt-Probe → Membran-Bindung — Loader-Kontrakt geschärft
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (Rat-Verdikt 2026-09-25) die crossing-Seam trägt das gemessene Serien-Paar `(xs, ys, n)` **mit Blatt-Header**, nie das Verdikt; die Membrane re-misst durch `te_probe`/WGSL (`omega.rs:455-543`, Dispatch `:498-513`, Call-Site `:1596`). Der `load_blatt_pair`-Loader verweigert genau: (a) Serie ohne Blatt-Header; (b) fehlendes/unparsbares Pflichtfeld (Fenster n/span/cadence in SI, seed, commit-sha, N_SURR); (c) deklariertes n ≠ `xs.len()`/`ys.len()`; (d) jeden nicht-finiten Wert; (e) n unter dem Schätzer-Floor 8 (`topological_te_with`; die Probe-Finding-Floor 30 ist eine Header-Eigenschaft, kein Loader-Gate); (f) einen commit-sha, der nicht im Baum auflöst. Ein Verdikt-Feld wird nicht gelesen. `docs/concepts/blatt-papier-beweis.md:32-47` trägt stale Pfade.
- **Blockade:** keine.
- **Braucht:** Serien-Schreibarm in `bz_blatt_probe.rs`/`frb_blatt_probe.rs`/`te_pair_probe`-Familie; `load_blatt_pair`-Loader (sechs Refusals); Pfad-Korrektur `docs/concepts/blatt-papier-beweis.md:32-47` (Body-Edit → Header-sha via `omega_sh sha`).

#### Kreuz-Screening auf weitere Ereignisse — Konfigs gebaut, Läufe offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `grind-pro`) die drei Ereignis-Konfigs stehen unter `phi/meteo/` im tibet-Schema: `aaretal-hochwasser-2026.json` (Fenster 27.05.–05.06.2026, Habkern/Beatenberg/Interlaken), `bordeaux-waldbrand-2026.json` (22.–31.07.2026, Landiras/La-Teste/Bordeaux), `japan-tsunami-2026.json` (25.07.–03.08.2026, Kumamoto/Yatsushiro/Kashima); `.gitignore` trägt `!phi/meteo/*.json`. Drei gemessene Rissen: „Bordeaux" ist ein **Waldbrand** (kein Hochwasser); „Japan" ist das **Kumamoto-Beben** (Mww 6.8 vs 7.1) mit aufgehobener Tsunami-Advisory; das Aaretal-2026-Ereignis ist die **Sturzflut Habkern/Beatenberg** (Aare-Hochwasser war 2025).
- **Blockade:** keine — die Läufe sind offline/CI.
- **Braucht:** `meteo_harvest --event phi/meteo/<event>.json --out …` je Ereignis, dann `cross_te_screen --dir <harvest> --lags 1,6,12,24 --surrogate 20 --min-n 100`; den `meteo-cdn`-Lauf lesen.

#### Solar-Matrix: konditionale Prüfung 211A→193A
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der CI-Lauf von `corona-conditional-probe`
- **Lage:** (gemessen 2026-09-25) das Paar 193A→211A ist der bestehende reverse leg (`corona_conditional_probe.rs:602-608`, Ladder `:21-29`); der Lauf `36194519096` ist dispatcht (in_progress) — gegen `origin/main` **vor** diesem Atom. Der Korpus ist CI-only.
- **Blockade:** keine — Korpus CI-only.
- **Braucht:** den Lauf lesen; nach `/commit` den Workflow erneut dispatchen (`corona-conditional-probe.yml`); die `193A->211A`-`rev_arrow`-Zeile messen.

#### Seismik-Flotte: Streuung senken + Stationsterm + W-Phase-M9
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) 16-Ereignis-Flotte unverzerrt (+1,7 km, se 4,7 km), aber 19 km über Ereignisse / 36 km über Stationen dominieren das ±10-km-Gate. Der pP-Zweig (`depthphase.rs:669-842`, `MIN_DIST_DEG = 30.0`) überspringt die Mehrdeutigkeit per Gate; ak135 ist 1D (`positive-maske.md:64`), kein 3D-Modell registriert. Stationsterm verdrahtet (`quake_location_probe.rs:48` II.KIV, `depthphase.rs:771-785`), Quelle/Residuum ungemessen. W-Phase-M9 entschieden, nicht gebaut.
- **Blockade:** kein 3D-Geschwindigkeitsmodell (pP-Residuum); Stationsterm-Quelle.
- **Braucht:** Residuum gegen Δ≈30° messen (3D-Modell registrieren oder `pending` halten); Stationsterm an II.KIV wiederholen; W-Phase-M9 bauen.

#### Positive Maske — Audit-Punkte + fehlende Treiber
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) Echo-Tiefe σ 19/36 km, Stationsterm offen (II.KIV +5,69 s), M9.1-Picker-Verdrahtung in die Flotte offen; **Slab2 registriert** (`phi/sources.φ:12169-12174`, `slab2-cdn.yml`), ScienceBase-Route 403 direct+Proton `blocked`. Galileo-ODF: lebende Spec DSN 810-005 Modul 209C–G; `galileo_odf.bin` registriert (`sources.φ:8369-8375`).
- **Blockade:** Slab2 403.
- **Braucht:** M9.1-Picker in die Flotte verdrahten; den Galileo-ODF-Formattrenn „1 vs 2" im 209G-Text extrahieren; Tomografie als Ernte-Kandidat registrieren.

#### Sieben Sphären — Messvorschriften
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) Sphäre I (Fresnel-Winkeldurchmesser-Feld) ausstehend, VII (Dopplergeist) ohne statistische Aggregation; II–VI sind Konzept.
- **Blockade:** keine.
- **Braucht:** das Fresnel-Winkeldurchmesser-Feld (Asteroiden-Bahn ∩ Gaia-Farbe ∩ IR-Ø) und die Dopplergeist-Aggregation bauen.

#### Zeugin — vpec-Redshift-Domäne (`cosmicflows_cf4.json`)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** das CDN-Asset `cosmicflows_cf4.json` ist manifestiert
- **Lage:** (gemessen 2026-09-25) die zwei Redshift-Domänen und der vpec-Konsument sind gebaut (`parse_cosmicflows_cf4` + `nearest_cf4_vpec_m_s` + `distance_m_with_vpec`, 4 Tests, `cargo check -p omegaflow` 0/0); das Asset `tapvizier.cds.unistra.fr/cosmicflows_cf4.json` (`sources.φ:8856`) war HTTP 404 → absent. `cosmicflows-cdn.yml` ist dispatcht (`36194516393`, attempt 1 ohne Erfolg, gegen `origin/main` vor diesem Atom).
- **Blockade:** Asset nicht manifestiert (Producer nur `--ci-mode`).
- **Braucht:** nach `/commit` `cosmicflows-cdn.yml` erneut dispatchen; den sha im `sources.φ`-Block messen.

#### Nadel Ⅻ (Urknall) — Cone-Gate gebaut, CI-Lauf offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der CI-Lauf von `bigbang_echo_probe`
- **Lage:** (gemessen 2026-09-25) Cone-Gate gebaut (`bigbang_echo_probe.rs:184` `cone_min_tau_z`, `:334-391` per-Lag-Gate, Label-Verstoß korrigiert); `docs/paper/big-bang-echo-sheet-12.md:49-56` nachgezogen. Lauf `36194521580` dispatcht (in_progress, gegen `origin/main` vor diesem Atom); TE 0,147/0,223/0,223 < fam 0,275 (Register, nicht lokal re-gemessen).
- **Blockade:** Korpus (`cmb_planck_smica_n64.json` + `cosmicflows_cf4.json`) CDN-only.
- **Braucht:** den Lauf lesen; nach `/commit` erneut dispatchen; die `fam_carry`/`holds`-Zeilen messen.

#### Korona-Heizung — v2 gebaut, Aktive-Region-Katalog offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `grind-max`, `cargo check -p omegaflow-measure`/`-p omegaflow-harvest` 0/0) `nobel_probe_corona.rs` v2 gebaut (+258): Minuten-fam (ein Surrogate-Pool, per-Paar-Schwellen byte-identisch zu v1) + Multi-Force-TE (Kraft live aus dem Register: em/diffusion/advective; Radio „force absent", nicht fabriziert) + lokalisierter Aktive-Region-Abschnitt; `aia_compiler.rs` `--region cx,cy,r`-Apertur-Arm gebaut (+75). **Kein Aktive-Region-/Sunspot-Katalog** in `phi/sources.φ` (`sgrep` leer) → Region-Route fehlt; die Ernte ist CI-only.
- **Blockade:** kein registrierter Aktive-Region-Katalog; sub-minütige Auflösung.
- **Braucht:** eine Solar-Aktive-Region-Route registrieren (SWPC solar-regions JSON oder NOAA NGDC Sunspot-Katalog), dann `aia_compiler --harvest` mit Per-Datum-Koordinaten → Region-Bins → `aia_ladder_probe --aia <region bin>`.

#### Depth-Phase-Flotte — sP-corr-Gate gesetzt, Azimut-Register + CI-Lauf offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der CI-Lauf des Flotten-Probes
- **Lage:** (gemessen 2026-09-25) `SP_CORR_GATE = 0.78` (`depth_phase_fleet_probe.rs:20-24`, Quartil der n=30-Verteilung), `sp_gate` defaultet darauf, ungültiges `--sp-gate` verweigert (`:470`); Lauf `36194524125` dispatcht (in_progress, gegen `origin/main` vor diesem Atom). Die sechs Pilot-Azimute stehen in keinem Register.
- **Blockade:** keine.
- **Braucht:** die sechs Stationsazimute registrieren; den Lauf lesen; nach `/commit` erneut dispatchen.

#### Galileo-Rotor-CK — Manifestor erweitert, CI-Lauf offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der CI-Lauf von `gll-ck-cdn`
- **Lage:** (gemessen 2026-09-25 via `grind-flash`, `cargo check -p omegaflow-harvest` 0/0) `RTR_INDEX` → `RTR_INDEXES` (`prime/unvalidated`, `extended/unvalidated`, `GEM/{c23,c30,i24}`) + `CK_ROOT_INDEX` (root `*.bc`), Dedup im flachen CDN-Namensraum, Test `root_names_collects_root_ck`; Lauf `36194526698` dispatcht (queued, gegen `origin/main` vor diesem Atom). `sources_index.φ` indiziert die volle Serie bereits.
- **Blockade:** Ernte CDN-Duty (`--ci-mode` + Token).
- **Braucht:** den Lauf lesen; nach `/commit` `gll-ck-cdn.yml` erneut dispatchen.

#### LAIC Instrument A — verdrahtet, CI-Lauf + DEMETER-Rest
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der CI-Lauf von `laic-cdn`
- **Lage:** (gemessen 2026-09-25 via `grind-pro`, `cargo check -p omegaflow-measure` 0/0) Instrument A verdrahtet: `--global-rate` in `laic-cdn.yml:40`, `global_rate.json` als Trailing-Sektion in `pack_bin`/`compile_main` (kein `BIN_VERSION`-Bump; der Fremd-Reader `nobel_probe_laic.rs` berührt den Tail nicht), Analyze-Pfad nutzt die gepackte Serie, Round-Trip-Test aktualisiert. Lauf `36194529795` dispatcht (in_progress, gegen `origin/main` vor diesem Atom). Die DEMETER-Zeile in `blocked_sources.φ` ist auf DONE_WITH_WARNING/96978/0 nachgezogen; **die Datei trägt jetzt zusätzlich fremde uncommittete Arbeit** (Namensdifferenz ~534 Zeilen) — beim Commit nur den eigenen DEMETER-Hunk.
- **Blockade:** keine (Instrument A); DEMETER-Dateifehler-Ursache hinter der REGARDS-API.
- **Braucht:** den laic-cdn-Lauf lesen; nach `/commit` erneut dispatchen.

#### Nadel V — Positive-Control-Konus + IR-Exzess-Achse
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `research-max`; Rat-Verdikt) der AllWISE-Sync-Stall ist durch **Async-TAP** gelöst: Submit 303 → Job `23542707` COMPLETED → Result 200 VOTable (Felder designation/ra/dec/w1mpro..w4mpro; 0 Zeilen im 6″-Kegel 148.84/2.55; Job `23542882`, 3C 273, QUEUED). IRAS-PSC `fnu_60`-Sample 2.988e+02 Jy (IRAS 05524+0723, fqual 3, 1,28″); Herschel `hsa.pacs_point_source_100`/`_160` existieren (200, 0 Zeilen im Kegel; `flux` mJy). Gaia-Join `gaiadr3.vari_rrlyrae ⋈ gaia_source` = 177 358 Records. **Der Rat: das Register trägt zur Join-Form keinen Riss** — keine `blocked_sources.φ`-Zeile, solange die Schreibzeit-Messung den Eintrag nicht findet; die Join-Form steht als benannte Messung; der Register-Teil wandert in **Mountains** Übergabe.
- **Blockade:** AllWISE-Job `23542882` QUEUED-Lesung; teils account-gebundene Streams.
- **Braucht:** `GET https://irsa.ipac.caltech.edu/TAP/async/23542882/phase`, dann `.../results/result` (eine Lesung, kein Loop); die drei IR-Routen registrieren; `lsst_anomaly_probe` auf dem positiven Kontrollkegel.

#### Pioneer/Dark-Matter — Routen gemessen, Rampen-Sweep offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `research-max`) ASC byte-exakt live: p10 `…SC_23` 19 820 702 B, p11 `…SC_24` 21 454 162 B, p11 `…SC_23` 404 — verdrahtet `pioneer_doppler_compiler.rs:7-8,70`. Voyager 2 `radio_science_rss` = genau zwei Verzeichnisse (6 PSPA-00123-Tars 07.08.–02.09.81 + 2 PSPA-00193-Tars), **kein Cruise-Doppler** (request-only, `blocked_sources.φ:51`); `phi/sources.φ` trägt keine voyager2-Origin. DSN 810-005 TRK live: 203E 2 061 668 B, 202E 1 858 227 B, 209G 661 438 B; `…/MDA/` → 301 (Ziel ungemessen). NAVIO-Felder: TIMTAG/FREQCY/DTYPE/SC/TRANS/RCVR1/OBSVBL/CMPTIM (`:87-119`). Rampen-Sweep ungebaut.
- **Blockade:** keine (Pioneer); MDA-Listing.
- **Braucht:** Rampen-Sweep-Probe über `pioneer1{0,1}_navio.bin` bauen; Voyager-2-Route registrieren; `…/dsndocs/810-005/MDA/`-Ziel fetchen.

#### Trishuli — Bahrabise-Richtungsverifikation + S1/SAR-Footprint
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort (Post-Sentinel-1-Szene)
- **Lage:** (gemessen 2026-09-25) die Bahrabise-Spalten tragen die `te_pair_probe`-Spiegelunsicherheit; der räumliche Footprint bleibt pending (CEMS nur Grading, optisch wolkenverdeckt, S1 nicht archiviert).
- **Blockade:** S1-Post-Szene nicht archiviert.
- **Braucht:** die Bahrabise-Richtung mit richtungs-verifizierter Bibliothek prüfen; die Post-Sentinel-1-Szene auf Flutfläche/Narbe messen.

#### Terminologie — Definitionen Ontologie-Motor / mycorrhizal_internet
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) beide Begriffe nur in ihren eigenen Registerzeilen (`docs/concepts/glossar.md:59/61`); in `docs/paper/terminologie-der-gegenstroemung.md` als `pending`. Der Operator-Korpus `state/funding/profil-operator.md` liegt im privaten Repo.
- **Blockade:** Operator-Korpus im privaten Repo.
- **Braucht:** die Semantik der beiden Phrasen heben und als getrackte Definition setzen — bis dahin `pending`.

#### Broken-Null-Control — Gate-Tests gebaut, Spec-Doc + CI-Lauf offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der CI-Lauf (`ci-check` + `te-gate`)
- **Lage:** (gemessen 2026-09-25) fünf Gate-Tests + zwei Helfer in `src/mathematikerin/te.rs` (keine Schätzer-/Null-Änderung; die vier Kalibrier-Gate-Tests unberührt); `.github/workflows/te-gate.yml` +2 Jobs (`lag-sweep`, `fn-bias`, `--ignored --nocapture --release`); `te.rs` clean. `te-gate` ist dispatcht (`36194535114`, queued — das 21:38Z-Rate-Limit ist aufgehoben). Die thin-margin-Aussage „reverse silent at τ=1" ist nicht gegated.
- **Blockade:** keine.
- **Braucht:** `docs/specs/broken-null-control.md:142` nachziehen („fixed-window re-run … pending" vs Paper §form; Body-Edit → Header-sha via `omega_sh sha`); den te-gate-Lauf lesen.

#### Weberin — zweite unabhängige Positions-Linie je Körper-Klasse
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) Mapping: Planeten/Monde → DE440 (`sources.φ:2890`) / INPOP (`:1473-1483`) + EPM (`:1403-1418`); Asteroiden → SPK(sb441) / Dastcom (`:2876`) / MPC (`:1881`). Zweite Linien der breiten TNO-Kette (`mpcorb_extended`) und Kometen (`dcom5`/`cometels`) `pending`.
- **Blockade:** keine.
- **Braucht:** die zweite Linie je Klasse kompilieren (u. a. `pallas`, `juno_asteroid`, `encke`; Kometen-Zweitlinie).

#### Weberin Faden-Matrix — verbleibende No-Actor-Lücken + Broker-Positionen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) No-Actor: VHE/Neutrino/CR-Einzelteleskope ohne offene Route (TA-Vollkatalog, Super-K, JUNO, LHAASO-Event); Broker Lasair/ANTARES/Fink positions-pending; seismische Stations-Weltlinien nur als Events.
- **Blockade:** teils not-published.
- **Braucht:** die Broker-Positionen kompilieren; die seismischen Stations-Weltlinien registrieren; die not-published-Teleskope als absent halten.

#### Weberin-Quellen — HAWC-TLS offen, Fink/ALeRCE live
- **Status:** offen | **Bindung:** eigen
- **Trigger:** das YR1-Zwischenzertifikat liegt in `OMEGAFLOW_CA_BUNDLE`
- **Lage:** (gemessen 2026-09-25) die CDN-Assets sind sha-verifiziert: `tao_wnd_zonal.csv` `b7719c89…` (`sources.φ:786`), `wwlln_th.csv` `06031403…` (`:8634`), `bpa_gic.csv` `8d0ceaa8…` (`:8646`). **HAWC** direct TLS-broken (Leaf `www.hawc-observatory.org`, Issuer `CN=YR1`; Wayback 200 2024-09-15); **Fink** `api.lsst.fink-portal.org/api/v1/objects` 200; **ALeRCE** `api.alerce.online/alerts/v1/objects/` 200.
- **Blockade:** HAWC-TLS-Kette (YR1).
- **Braucht:** YR1-PEM in `OMEGAFLOW_CA_BUNDLE` setzen und HAWC erneut fetchen; Fink/ALeRCE-Persistenz-Reader bauen.

#### Sonden-Flotte — LRO-Register nachgezogen, Asset/sha stale
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der erste de-jährte LRO-Lauf (`harvest-long`)
- **Lage:** (gemessen 2026-09-25) der Manifestor-Fix ist da (`lro_trk_compiler.rs` Jahresfilter entfernt); Register nachgezogen: `harvest.φ:99-102` ohne `args --year 2009`, Pattern ohne `_2009`, `shard 1`, `timeout 240`; `sources.φ:8018` URL `lro_trk.bin`. **Stale aus dem 2009-Lauf:** `harvest.φ:95` `asset present`, `:104` Note + `sources.φ:8022` sha `c29af4b4…` (2009-Digest). Gemessen: es gibt **kein** `lro-cdn.yml`; die reale Route ist `harvest-long.yml -f format=lro_trk`.
- **Blockade:** das de-jährte Asset ist nie gebaut; `shard N` und sha sind ungemessen.
- **Braucht:** nach `/commit` `gh workflow run harvest-long.yml -f format=lro_trk`; dann `asset`/`shard`/`sha` aus dem gemessenen Lauf setzen.

#### Survey-Träger — Orphan-Faltung
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) `register_lookup --orphan-docs` → **42** Orphan-Docs (Lage folge172 nannte 22; u. a. `survey-2026-09-07-weberin-sonnensystem-kette` (7), `survey-2026-09-07-weberin-thread-matrix` (22), `survey-2026-09-13-weberin-quellen*` (42/9/14/42), vier `axiom-gate-*`-Surveys). Vier echte Träger sind in diesem Register benannt: `docs/paper/cross-screening-tibet.md` (Kreuz-Screening), `docs/paper/depth-phase-echo-fleet.md` (Depth-Phase), `docs/paper/h0-lines-register.md` (Axiom-Gate, hier geschlossen), `docs/concepts/die-weberin.md` (Weberin-Punkte). Die zwei `resolved`-Linien (neptune/uranus) sind abgelöst.
- **Blockade:** keine.
- **Braucht:** die zwei `resolved`-Linien als `descoped` schließen; die Weberin-Surveys per Träger-Zeile an die Weberin-Punkte binden; die Inline-Code-Klasse nach dem `strip_inline_code`-Merge als `descoped` schließen.

#### Lag-Sweep + KDE-Bandbreiten-Sensitivität — offene Mess-Gates
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) `ein-blatt-papier` und `blatt-papier-beweis` benennen Lag-Sweep und KDE-Bandbreite als offene Mess-Gates; `laic_probe --analyze --kde-scale` trägt den Knopf, die lokale laic-Ernte fehlt (CI).
- **Blockade:** keine — offline/CI.
- **Braucht:** `laic_probe --analyze --kde-scale` in CI dispatchen und den Lag-Sweep je Paar drucken.

#### ZNSP FORMNETWORK — `esp_zb_cfg_t`-Payload-Größe ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die kompilierte ESP-IDF-Referenz (`zigbee-host.yml`-Lauf bzw. `sizeof`-Probe)
- **Lage:** (gemessen 2026-09-25) der ZNSP-Transport ist gebaut (`firmware/radiatorium-lib/src/znsp.rs`, Bin `znsp_host.rs`); die FORMNETWORK-Payload ist ABI-raw (`sizeof(esp_zb_cfg_t)`), nur aus Struct+Alignment abgeleitet (16 B), nicht gemessen — `form_network_payload_pending()` liefert `None`. `zigbee-host.yml` ist dispatcht (`36194532489`), aber die `sizeof`-Probe selbst ist noch ungebaut.
- **Blockade:** keine — braucht die kompilierte Referenz.
- **Braucht:** die `sizeof`-Probe in `zigbee-host.yml` bauen; den Lauf lesen; dann den FORMNETWORK-Encoder setzen.

#### HRV/Puls→Strahlung — Kette nach dem Live-Lauf beobachten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der BLE-Live-Lauf (Operator) hat `rr` geliefert
- **Lage:** (gemessen 2026-09-25) die Kette ist gebaut: `hrv.rs:24-87` → `feed_beat_to_hrv` (`main_flow.rs:79-102`) → `tone_code` → `tone_scale` 0.25 (`omega.rs:1670-1678`) → `aperture` (`omega.rs:349`) → `Σω*aperture` (`actuators.rs:29`).
- **Blockade:** hängt am BLE-Live-Fluss (Operator).
- **Braucht:** nach dem Live-Lauf `tone_code`→`tone_scale`→`aperture` im Frame/perm-Log lesen.

#### M2c — Rust-ZNSP-Host auf dem BL808 gegen das H2
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ox64 angekommen
- **Lage:** (gemessen 2026-09-25) der Ox64 wird Host-CPU des Coordinators (ZNSP über UART), das H2 das Funkmodul; Ox64-UART-Pins UART0 GPIO14/15, UART1 GPIO16/17. Buildroot-Bring-up ungemessen.
- **Blockade:** Ox64 liegt beim Carrier.
- **Braucht:** Buildroot-Bring-up, dann derselbe Rust-ZNSP-Host auf dem BL808 gegen das H2.

#### BL808-eigenes 802.15.4-Radio — registrierter `pending`-Faden
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** öffentlicher 802.15.4-Treiber-/Stack-Fund für den BL808
- **Lage:** (gemessen 2026-09-25) kein öffentlicher Treiber: `bl_iot_sdk` trägt für BL808 nur `bl808_wifi`, BL808-RM ohne Wireless-Kapitel; `bouffalo_sdk_bl808` README-Raw 404 → `pending`. Route 2 nutzt das Radio bewusst nicht.
- **Blockade:** kein Treiber.
- **Braucht:** bei Fund `archive_search --github bouffalo_sdk_bl808 802.15.4`; bis dahin keine Arbeit.

#### Galileo Borduhr-Sprung A/B — Trenn-Frage
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Fund einer öffentlichen Absolutfrequenz-Reduktion über 1995-11-30/12-01
- **Lage:** (gemessen 2026-09-25) öffentlich nicht trennbar — `pending`. DESCANSO5-PDF re-gemessen (200, 4 309 495 B, sha256 `104ab955…`); `--playwright` auf die PDF liefert keinen Text (Chromium-PDF ohne Textebene). Morabito-Reihe endet 1993; PDS hat keinen Jupiter-Phasen-RSS-Datenkopf.
- **Blockade:** PDF-Textebene außerhalb des Toolsets.
- **Braucht:** den DESCANSO5-USO-Abschnitt per PDF-Text-Extraktion (OCR) lesen; prüfen, ob eine USO-Frequenzreihe die Grenze kreuzt.

#### JWST Biosignatur-Kanäle pending (O₂/O₃, Red-Edge, Saisonal)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine JWST-Detektion + Spektrum eines dieser Kanäle
- **Lage:** (gemessen 2026-09-25) die Kanäle sind benannte pending-Zweige; die Abwesenheit ist gemessen (keine JWST-Detektion im gesuchten Record).
- **Blockade:** keine Quelle trägt eine Detektion.
- **Braucht:** bei Fund die Detektion + ihr Spektrum ins Register.

#### Planet-Nine/KBO — h-Sweep + Real-Kernel-Gegenprobe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Typ-1-`.bsp` liegt im Baum (z. B. Ceres/Vesta)
- **Lage:** (gemessen 2026-09-25) SPK-Type-1-Reader gebaut (`bsp_reader/spk.rs`, `data_type 1` in `ephemeris.rs`); kein Typ-1-Kernel im Baum → Real-Gegenprobe `pending`; h-Sweep ungefahren.
- **Blockade:** kein Typ-1-Testkern.
- **Braucht:** ein Typ-1-`.bsp` (Ceres/Vesta) ernten; dann `laic_probe --analyze DIR --kde-scale` (h/2, 2h).

#### Das eine Instrument — Anomalie offen (zweite Augenklasse fehlt)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein zweiter Messkanal (VLBI-Beacon auf einer interstellaren Sonde) existiert
- **Lage:** (gemessen 2026-09-25) die Pionier-Anomalie ist unter dem einen Instrument nicht entscheidbar; eine zweite Augenklasse (Winkel aus VLBI + Geschwindigkeit aus Doppler) fehlt.
- **Blockade:** kein Instrument misst den vollen Phasenraum der Pioniere.
- **Braucht:** eine zweite Augenklasse — VLBI-Beacon auf der nächsten interstellaren Sonde, von Tag eins zweikanalig.

### Operator handelt

#### BLE-HR-Live-Messung FR945
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) der GFDI-Bus-Abriss ist geheilt und CI-bestätigt: alle `archivar::ble::tests` grün auf `45b4b2eb1`; Fix `src/archivar/ble.rs:1360/1418`; verdeckter Lauf-Command steht (`OMEGAFLOW_BLE_HR=… OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow`, ~45 s).
- **Blockade:** keine.
- **Braucht:** die Kantenzeile bereithalten.
- **Status (Akt):** operator-gebunden (Hardware/Radio) | **Bindung:** operator
- **Trigger:** Operator startet den verdeckten Lauf am gekoppelten Gerät (945 am Arm)
- **Braucht:** `OMEGAFLOW_BLE_HR=<FR945-MAC aus .secrets.local> OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow`; `sensor:`-/`gfdi_line`-Zeilen lesen.

#### FIT-Verifikation eigene FR945-Datei (lokal-only)
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) der Dump-Pfad ist gebaut (`src/main.rs:1-56`: `OMEGAFLOW_FIT_SAMPLE` → `parse_fit`, druckt records/nn/min/max/all-finite, `exit(2)` bei Refusal), am SDK-Sample verifiziert.
- **Blockade:** keine.
- **Braucht:** die Kantenzeile bereithalten.
- **Status (Akt):** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator startet den Lauf mit seiner Datei
- **Braucht:** `OMEGAFLOW_FIT_SAMPLE=/abs/pfad/FR945.fit cargo run -p omegaflow --bin omegaflow`.
- **Wort:** „meine fits datei verlässt niemals dieses gerät" | 2026-09-25 | Operator (Session)
- **Wort:** „die 945 von anderer Hardware trennen; meine Daten bleiben lokal" | 2026-09-25 | Operator (Session)

#### Onboard-/CIQ-Bedarf benennen
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort
- **Lage:** (gemessen 2026-09-24) der Host-Reader `parse_fit` (`fit.rs:78`) ist verdrahtet; CIQ hat keinen Reader (`sgrep -i ciq src tools` leer). Ein-Quellen-Regel: solange FIT läuft, sind BLE/Serial über die Arbitrierung ausgeschlossen.
- **Blockade:** keine — bewusst `pending` (Ein-Quellen-Regel).
- **Braucht:** Bedarf Ja/Nein — Nein → der Punkt ist released, `FIT_DIR` bleibt der FIT-Kanal.

#### Beat-Arbitrierung — verdeckter Lauf mit zwei Beat-Quellen
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) die Spawn-Arbitrierung steht (`main_flow.rs:504`); die reine Funktion ist CI-getestet; der verdeckte Lauf-Command steht.
- **Blockade:** keine.
- **Braucht:** die Kantenzeile bereithalten.
- **Status (Akt):** operator-gebunden (hidden) | **Bindung:** operator
- **Trigger:** Operator startet einen verdeckten Lauf mit zwei gesetzten Beat-Quellen
- **Braucht:** `OMEGAFLOW_HIDDEN=1`-Lauf mit zwei Quellen, der genau eine `beat source:`-Zeile zeigt.

#### DEMETER/CDPP-Order-Flow
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) die Order-Portale antworten (je 200); CDPP-Login funktioniert (`REGISTERED_USER`); die Registerzeile ist nachgezogen (DONE_WITH_WARNING, 96978 in Vermerk, 0 verfügbar, Download HTTP 500).
- **Blockade:** Dateifehler-Ursache hinter der REGARDS-API (kein Report-Endpoint).
- **Braucht:** die 96 978 Dateien über die REGARDS-UI lesbar machen.
- **Status (Akt):** operator-gebunden (Zugang) | **Bindung:** operator
- **Trigger:** Operator-Browser liest die Dateifehlerliste
- **Braucht:** ein Neu-/Nach-Order ist konsenspflichtiger Dritt-Akt — Operator-Wort.

#### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25) Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut; `docs/specs/mantis-shrimp-bom.md` trägt ATGM336H GNSS `1005009361234427`, 3,01 CHF ≈ 3,20 €; die H2-Zeile `ESP32-H2-DevKitM-1-N4` (`1005008131868631` ≈ 6,25 $ unverified · DigiKey 26282483 9,68 $).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** Operator-Wort (LOCK-Aufhebung), dann BOM bestellen (inkl. H2).

#### ESP32-Puls-Knoten (Träger ohne Uhr)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25) Code-Eingang `BeatSource::Serial` steht; dedizierter Puls-Knoten unbestellt.
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —
- **Wort:** ESP32 separat als eigener LOCK | 2026-09-25 | Operator (Session)

#### Weberin-Quellen — account-/key-blockierte Rohkanäle
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) die offenen Zwillingsrouten stehen; IGETS (SFTP-Account), vDEC (Antrag + Vertrag), ONC-Token, TNS-Key (`TNS_API_KEY`/`TNS_UA`) und SuperDARN-Globus sind identifiziert.
- **Blockade:** Konto/Key fehlt.
- **Braucht:** die Anträge/Endpunkte je Konto bereitlegen.
- **Status (Akt):** operator-gebunden (Zugang) | **Bindung:** operator
- **Trigger:** Operator-Wort je Konto
- **Braucht:** Operator entscheidet je Konto (IGETS `igets-support@gfz.de`, vDEC-Antrag, ONC-Token, TNS-Key).

### Extern handelt (Dritte)

#### JUICE-Erdpassage 28./29.09.2026 — Kanal + In-situ-Messung
- **Status:** termin:2026-09-29 | **Bindung:** termin
- **Trigger:** 28./29.09.2026
- **Lage:** (gemessen 2026-09-25) der Prädiktionskanal steht, alle Zellen pending; der Operator-Siegel-Wort liegt noch nicht vor.
- **Blockade:** der Operator muss das Siegel-Wort vor dem 28.09. setzen.
- **Braucht:** vor dem Flyby das Siegel-Wort setzen; nach dem Flyby die JUICE-In-situ-Feldmessung gegen den präregistrierten Feldzustand vergleichen (σ-Metrik gegen fam).

#### Ox64-Lieferung
- **Status:** wartend | **Bindung:** termin (Carrier)
- **Trigger:** Ankunft (`LZ473049629CN`)
- **Lage:** (gemessen 2026-09-25 via `state/mail/mail_ledger.φ`, Mail `1790046330`) PINE64 versandte zwei Ox64 — Ankunft offen.
- **Blockade:** Carrier.
- **Braucht:** Ankunft quittieren; dann M2c (BL808-Host-Port).

#### Postfach
- **Status:** wartend | **Bindung:** extern (Mail)
- **Trigger:** neuer Eingang
- **Lage:** (gemessen 2026-09-25 via `sread` auf `state/mail/mail_ledger.φ`) Ledger vorhanden (>151 Zeilen); jüngste Eingänge `1790316610` (ORCID-Verify-Reminder) und `1790290298` (Rubin-Forum-Digest); `sgrep` relay/sensor/beat/mantis/ble/hrv/zigbee = 0 Treffer.
- **Blockade:** keine.
- **Braucht:** `smail_recv` bzw. `state/mail/mail_ledger.φ` bei Trigger.

#### BGR-Matched-Filter-Ankunft (Tonga) — account-blockiert
- **Status:** wartend | **Bindung:** dritter (vDEC)
- **Trigger:** vDEC-Zugang gewährt
- **Lage:** (gemessen 2026-09-25) die PMCC-Detektionsliste läuft auf einem ~300-s-Raster; die Rohwellenform ist vDEC-account-blockiert.
- **Blockade:** vDEC-Account.
- **Braucht:** nach Zugangsgewährung den matched-filter-Arrival messen.

#### DSN-Briefe in Flug (Voyager/Mariner 10/Viking, Cassini, Juno)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Antwort der DSN
- **Lage:** (gemessen 2026-09-25) fünf DSN-Briefe sind in Flug; der Auftrag `docs/auftrag/archiv/auftrag-sonden-rohdaten-anfrage.md` wurde nach `docs/auftrag/archiv/` verschoben.
- **Blockade:** Antwort ausstehend.
- **Braucht:** die Antwort quittieren.

#### Gaia DR4 + Europa-Clipper-Erdpassage
- **Status:** termin:2026-12-02 | **Bindung:** termin
- **Trigger:** 2.12.2026 (Gaia DR4) / 3.12.2026 (Europa Clipper)
- **Lage:** (gemessen 2026-09-25) Gaia DR4 für den 2.12.2026 angekündigt (Epochen-Astrometrie); die Europa-Clipper-Erdpassage folgt am 3.12.2026.
- **Blockade:** Termin.
- **Braucht:** am jeweiligen Datum die Epochen-Astrometrie bzw. die EC-Magnetfeld-Messung ernten.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
