<!--
  title: Handover — Sensory-Folge 178 (Stand 2026-09-26)
  session: Sensory-Folge 178
  class: handover
  date: 2026-09-26
  sha256: e585a78174d273e61401b6791cfe1306aa44b4c1abb2a4e57e6892498f64fc05
  status: live
-->
# Handover — Sensory-Folge 178 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur
die eigenen Hunks — committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr
von HEAD ist.

Es gibt keinen `härtesten Punkt` — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit Messstempel) / **Blockade** /
**Braucht**. Sortierung: **umsetzbar zuerst** — (1) sofort abarbeitbar (eigen,
dispatchbar), (2) operator-gebundene Vorbereitung (Kante fertig, nur das Wort fehlt),
(3) blockiert/wartend (mit Trigger), (4) extern (Dritter). Der Akteur steht pro Punkt
in `Bindung`, nicht in der Reihenfolge.

Die [redacted] ist das persönliche Gerät des Operators; ihre Kennung (MAC) und ihre Daten
bleiben lokal (`.secrets.local`, `data/`), nie getrackt, nie am CDN.

**Audit-Regel (2026-09-26):** vor jedem `operator-gebunden`-Tag wird `.secrets.local`
auf den nötigen Key geprüft — `[redacted]_MAC` und `OMEGAFLOW_CA_BUNDLE` standen bereits
dort, während die Punkte als „wartet auf Operator" gebucht waren.

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

## Träger-Zeilen (Orphan-Faltung)

Die folgenden Prosa-Dokumente tragen offene Marker und werden an ihre Trägerpunkte
gebunden (der Scanner `register_lookup --orphan-docs` erkennt den Träger nur, wenn
eine lebende Übergabe den Dateinamen nennt):

- `survey-2026-09-07-weberin-sonnensystem-kette.md`, `survey-2026-09-07-weberin-thread-matrix.md`,
  `survey-2026-09-13-weberin-quellen.md`, `survey-2026-09-13-weberin-quellen-folge.md`,
  `survey-2026-09-13-weberin-quellen-treffer.md`, `survey-2026-09-14-weberin-quellen-rerun.md`
  → Träger `docs/concepts/die-weberin.md` (Weberin-Punkte unten).

### Rest-Träger-Zeilen (Stand 2026-09-26)

Je Zeile ein zuletzt trägerloses Dokument: `Pfad` (offene Marker) → Trägerpunkt
oder descoped-Befund. Der Dateiname in dieser Übergabe ist der Träger.

- `docs/blatt/blatt-der-grat.md` (2) → ENSO-Pfeil messen: `cross_te_screen`.
- `docs/blatt/blatt-kreuz-screening-gyirong.md` (3) → Träger Punkt „Kreuz-Screening" (unten).
- `docs/blatt/blatt-solar-seconds-matrix.md` (1) → `corona_conditional_probe` auf das Paar 211A→193A.
- `docs/blatt/blatt-thuan-fragesteller.md` (4) → `termin:2026-12-02` (Gaia DR4), dann `docs/auftrag/archiv/auftrag-gaia-dr4-iapetus.md`.
- `docs/concepts/arxiv-api.md` (2) → Träger Punkt „arxiv HTTP 406" (Mountain-Linie).
- `docs/concepts/blatt-papier-resultat.md` (1) → Blatt-1-Bojen-Matrix-Rotor laufen lassen, Matrix-Zeile Σ p̂·M nachtragen (Z.63–71).
- `docs/concepts/das-eine-instrument.md` (2) → Träger Punkt „Das eine Instrument — Anomalie offen" (unten).
- `docs/concepts/der-paradigmenwechsel.md` (9) → Träger Punkt „JUICE-Erdpassage 28./29.09.2026" (unten).
- `docs/concepts/die-akteure-im-boden-und-wasser.md` (7) → Träger Punkt „Seismik-Flotte" (unten).
- `docs/concepts/fuenf-funken-anomalie-suche.md` (4) → Funke 3 via `broker_difference_probe`, Funke 5 (TDB-Koinzidenz-Fenster) bauen.
- `docs/concepts/recherche-galileo-kadenz-reconciliation.md` (1) → `archive_search --ntrs 19930010224` bzw. DSMS Services Catalog v7.5 §"Doppler count interval".
- `docs/concepts/the-seven-spheres.md` (2) → Träger Punkt „Sieben Sphären" (unten): Stern-Winkeldurchmesser-Feld (CHARM2, füllbar); gemessenes Δz je Okkultation.
- `docs/paper/causal-arrow-preregistration.md` (1) → `te_pair_probe` (Lag-Sweep {1,3,6,12,24,48}) gegen Rasuwa-Regen; DAHITI Koshi via `sfetch` (api_key).
- `docs/paper/corona-heating-ladder.md` (2) → Träger Punkt „Korona-Heizung" (unten): `aia_ladder_probe` über den vollen 613-Event-Korpus.
- `docs/paper/cross-screening-tibet.md` (1) → Träger Punkt „Kreuz-Screening" (unten).
- `docs/paper/depth-phase-echo-fleet.md` (5) → Träger Punkt „Seismik-Flotte" (unten).
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4), `docs/paper/flyby-path-2-preregistration.md` (1) → Träger Punkt „JUICE-Erdpassage 28./29.09.2026" (unten).
- `docs/paper/galileo-rotor-spin-era-floor.md` (1) → CK-Kerne jenseits `ck90341a`–`ck90344b` (Frame −77000) von `naif.jpl.nasa.gov` harvesten.
- `docs/paper/jwst-disequilibrium-survey.md` (7) → Träger Punkt „JWST Biosignatur-Kanäle pending" (unten).
- `docs/paper/laic-arrow-direction.md` (3) → `archive_search --playwright https://leos.ac.cn` (CSES SPA).
- `docs/paper/nadel-v-fresh-area-dip-scan.md` (1) → Träger Punkt „Nadel V" (unten).
- `docs/paper/probe-front-dark-matter.md` (2) → Träger Punkt „Pioneer/Dark-Matter" — **gemessen geschlossen** (vision-read Appendix A; keine erntbare Datei).
- `docs/paper/solar-seconds-matrix.md` (3) → `corona_conditional_probe` auf das Paar 211A→193A.
- `docs/paper/sturzflut-tibet-pfeil.md` (23) → Träger Punkt „Trishuli" (unten).
- `docs/paper/tonga-lamb-crosscheck.md` (3) → Träger Punkt „BGR-Matched-Filter-Ankunft (Tonga)" (unten).
- `docs/surveys/axiom-gate-broken-null-control.md` (1) → Träger Punkt „Broken-Null-Control" (geschlossen — te-gate 36228804363 success).
- `docs/surveys/axiom-gate-depth-phase-echo-fleet.md` (1) → Träger Punkt „Seismik-Flotte" (unten).
- `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md` (17) → Träger Punkte „Pioneer/Dark-Matter" + „DSN-Briefe in Flug" (unten): vier Werkzeuglücken.
- `docs/surveys/survey-2026-09-16-sonden-flotte.md` (4) → Träger Punkt „DSN-Briefe in Flug" (unten).
- `docs/surveys/survey-ein-blatt-korona-heizung.md` (1) → Träger Punkt „Korona-Heizung" (unten).
- `docs/surveys/survey-2026-09-06-codestruktur.md` (8) → Träger Stehender Pass „CI-Status am HEAD" (oben).
- `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (9) → Träger Punkt „HRV/Puls→Strahlung" (unten, wartend).
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` (2) → Träger River-/Browser-Linie (Kaltstart-Entschärfung, Versionslücke).
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` (7) → Träger „FIT-Verifikation [redacted]" (operator) + „BLE-HR-Live-Messung [redacted]" (unten).
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` (1) → Träger Punkt „Postfach" (unten): nächster Schritt MPI-FKF/LAB_A-Antwort via `smail`; die CSES-Limadou-Antwort (Sotgiu 2026-09-16, „wait a few weeks") ist Wiedervorlage.
- `docs/concepts/ein-blatt-papier.md` (2) → Lag-Sweep-Träger Punkt „causal-arrow-preregistration" (`te_pair_probe`); KDE-Bandbreiten-Gate descoped.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md`, `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md`, `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md`, `docs/concepts/mirror-research.md` (je 1) → Bestand der Mycelium-Linie (kein eigener Sensory-Schritt).
- `docs/concepts/exzellenz-konzept.md` (3) → descoped (gemessen 2026-09-26: Marker sind die Definition von pending §2/§2.5; Body final → `docs/concepts/exzellenz-konzept.md:91`).
- `docs/concepts/kybernetische-astrophysik.md` (10) → descoped (gemessen 2026-09-26: Marker sind Status-Vokabular „offen"/„pending" im Essay-Body → `docs/concepts/kybernetische-astrophysik.md:45`).
- `docs/concepts/zeugnis.md` (5) → descoped (gemessen 2026-09-26: Marker sind die Disziplin-Prosa des Handover-Abschnitts F → `docs/concepts/zeugnis.md:370`).
- `docs/paper/asmar-2005-spacecraft-doppler-tracking-noise-budget.md` (2) → descoped (gemessen 2026-09-26: Treffer sind der Substring „depending" Z.29/35, kein offener Punkt).
- `docs/paper/planet-nine-kbo-residue.md` (1) → descoped (gemessen 2026-09-26: §5(iv) auf „implementiert + getestet" nachgezogen, `spk_type1_check.rs`; der Reader ist gebaut).
- `docs/paper/terminologie-der-gegenstroemung.md` (1) → descoped (Marker „pending" im Definitions-Kopf → `:22`).
- `docs/concepts/blatt-papier-beweis.md` (3) → descoped (Membran-Bindung gebaut `src/mathematikerin/omega.rs:349`, Commit `356fa616`; pending-Kanalzellen = Quellen-Port-Stand).
- `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` (2) → descoped (Substring „wartet" in „erwartete"; Silence-Map-Probe gebaut).
- `docs/surveys/survey-messpunkt-verteilung.md` (6) → descoped (gemessen 2026-09-26: Kandidaten 1–8 verdiktet; Kandidat 9/§6 Konsultations-Einladung).
- `docs/concepts/glossar.md` (1) → descoped (Marker sind die `pending`-Definitionen des Glossars → `docs/concepts/glossar.md:24`).
- `docs/concepts/archivar-mathematikerin.md` (1) → Träger: Archivar-Pending (Bootstrap-all-bodies-Load, Per-Tick-Fetch-Loop, Katalog-Kegel auf dem Kegel — `main_flow.rs`); Bindungslinie Mountain.
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (2) → Träger: §7 offene Punkte der Membran-Ladearchitektur (River-Linie); `archivar-mathematikerin.md` verweist darauf.
- **Alt-Orphans (2026-09-26 gemessen, trägerlos):** `docs/concepts/pfeiler-der-architektur.md` (2), `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` (3), `docs/surveys/survey-fortschritt.md` (1) — Marker = Status-Vokabular; kein eigener Sensory-Schritt. Schritt: beim nächsten Pass je Datei die Marker lesen → descope oder Träger (Aufenthalt Mycelium/Mountain).

## Offen (umsetzbar zuerst)

Die Reihenfolge ist die Umsetzbarkeit; der Akteur steht pro Punkt in `Bindung`.

### 1. Sofort abarbeitbar (eigen, dispatchbar)

#### Kreuz-Screening — `is_day`-/Selbstpaar-Ausschluss gebaut, Re-Run offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `ci_manage list`) `meteo-cdn 36259731023` **success**; der `is_day`/Selbstpaar-Fix steckt in HEAD (rustfmt-Rest im Baum).
- **Blockade:** keine.
- **Braucht:** rustfmt-Hunk committen; die Träger-Docs bleiben **offen** (gemessen 2026-09-26: `cross-screening-tibet.md:42/:44`, `blatt-kreuz-screening-gyirong.md:71/:201` räumliche Kopplung Rasuwa→Gyirong) — der grüne Lauf schließt sie nicht; Kopplung messen.

#### Positive Maske — Slab2-Idempotenz-Gate gefixt, beide Läufe rot
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `ci_manage log`) beide Läufe scheitern deterministisch an `slab2_compiler.rs:552-557`: der Download-Zielordner `data/www.sciencebase.gov/` wird vor `curl` nicht angelegt → `curl: (23) Failure writing output`; `write_asset` legt ihn erst `:251-257`.
- **Blockade:** keine.
- **Lage-Fortschritt:** (2026-09-26, grind-flash) Parent-Dir-Fix gebaut — `slab2_compiler.rs:556-564` `create_dir_all` vor dem Download; `cargo check -p omegaflow-harvest --bin slab2_compiler` 0/0.
- **Braucht:** Fix committen+pushen, dann `gh workflow run slab2-cdn.yml`.

#### Seismik-Flotte — Compiler-Arme gebaut, CDN-Läufe offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `ci_manage`) `volume-cdn`, `isc-ehb-cdn`, `emc-cdn` **success**; Arme LLNL-G3D-JPS/ISC-EHB/EMC gebaut; EMC-3D-netCDF-Katalog-Arm `descoped`.
- **Blockade:** keine.
- **Braucht:** Träger-Docs bleiben **offen** (gemessen 2026-09-26: `depth-phase-echo-fleet.md:35-38` Dual-Phase-Fit pending; `die-akteure-im-boden-und-wasser.md:85` Gravimeter-SFTP pending; `axiom-gate-depth-phase-echo-fleet.md:42` CMT-Term/Kalibrier-Gate) — getrennte Paper-Pendings, nicht der Arm.

#### Weberin — cometels manifestiert, TNO offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) `cometels-cdn 36252200125` success; Katalog-Arm + Consumer gebaut.
- **Blockade:** keine.
- **Lage:** (gemessen 2026-09-26) Route existiert für eris/haumea/makemake (`kernel-flatten.yml:99-109`, `--bodies 2136199,2136108,2136472`); 10 weitere TNOs warten. Fehlend: (1) `src/archivar/kernels/naif_body_ids.tsv` trägt die 9 harvestbaren TNO-Namen (quaoar/orcus/varuna/ixion/salacia/varda/2003az84/2002aw197/2002ux25) nicht; (2) `kernel-flatten.yml:105 --bodies` listet sie nicht; 2002tx300 fehlt im GM-Katalog.
- **Lage-Fortschritt:** (2026-09-26) **gebaut** — 9 Zeilen in `src/archivar/kernels/naif_body_ids.tsv` (`2050000 quaoar` … `2055637 2002ux25`, parent 10) + `kernel-flatten.yml:105 --bodies` erweitert; 2002tx300 fehlt im GM-Katalog `asteroid_gm_sb441.φ`.
- **Braucht:** `kernel-flatten.yml` laufen lassen (CI); dann die TNO-Bins.

#### Weberin Faden-Matrix — FDSN-Endpunkt registriert
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) FDSN-Station-Endpunkt in `phi/sources.φ:5737`; Rat bestätigt die Elevations-Spalte; sub-Seespiegel-Skip benannt.
- **Blockade:** keine.
- **Lage:** (gemessen 2026-09-26) Die Broker-Routen sind **gebaut**: ANTARES (`skydirection_compiler.rs:231/:530/:615`) und Lasair (`:153/:516/:611`), in `skydirection-cdn.yml:29-30`; `antares_loci_compiler.rs`/`lasair_ztf_compiler.rs` sind Duplikate.
- **Braucht:** Fink/ALeRCE-Reader-Tests (CI-only); der `format skd1`-Archivar-Reader fehlt (nur `sky1` → `main_flow.rs:3675`), das Asset läuft lokal (`data/skydirections.bin`).

#### Sieben Sphären — Winkel-Feld gebaut, Δz-Okkultation absent
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `ci_manage`) `charm2-cdn 36259779508` **success**; CHARM2 438 JOIN-Paare → 363 Records; Δz bleibt `absent` (IOTA trägt Lichtkurven, kein Δz-Katalog).
- **Blockade:** keine.
- **Braucht:** Träger `the-seven-spheres.md`: Sphäre I (Stern-Winkeldurchmesser-Feld) **closed** (gemessen 2026-09-26, charm2 grün); Sphäre VII Δz bleibt `absent` (`:148`).

#### Nadel V — 6 Kandidaten, positive Kontrolle offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) `allwise_psd.bin` manifestiert; `lsst-live-scan 36257925460` success: 6 Kandidaten-Dips, positive-Kontroll-Zeilen = 0.
- **Blockade:** keine.
- **Lage-Fortschritt:** (2026-09-26, grind-pro) Die positive-Kontroll-Schicht ist **gebaut** (`lsst_color_coupling_probe.rs:112/:971`, VSX `:63`) — `vsx_known_couple=0` heißt: der einzige periodische Kandidat im blanken Fresh-Cone hat keinen VSX-Eintrag <3″; Coverage, kein Gate-Fehler.
- **Lage-Fortschritt:** (2026-09-26) **Wurzel gefunden — Code-Bug, kein Coverage**: `vsx_fetch_text` (`lsst_color_coupling_probe.rs:46`) setzte `-c.r` **ohne Einheiten-Flag** → VizieR liest Bogenminuten, der 3″-Cone war effektiv **0,05″** → nie ein VSX-Treffer (der wahre Grund für `vsx_known_couple=0`). **Fix:** `&-c.u=deg` ergänzt; `cargo check -p omegaflow-measure --bin lsst_color_coupling_probe` 0/0. Gültige Kontroll-Cone gemessen: `150.22889,1.39466,260,24` (COSMOS-DDF, LINEAR 21146742 RRC, FAP 3e-4, 34 Zweiband-Joins) — als zweite Stufe in `lsst-live-scan.yml` ergänzt.
- **Braucht:** `gh workflow run lsst-live-scan.yml`; dann prüfen, dass `vsx_known_couple > 0`.

#### Korona-Heizung — Feldmap + `millionths`-Arm live
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) Feldmap registriert, `millionths`→SI in `units.rs`, `cargo check` 0 Warnungen.
- **Blockade:** keine.
- **Lage-Fortschritt:** (2026-09-26) `aia-cdn` dispatcht — Lauf `36268729035` (Defaults 2014.03.01–2014.05.30, Bänder 94–335, Asset `aia2014_lines.bin`).
- **Braucht:** Lauf lesen (`ci_manage view/log 36268729035`).

#### clippy eigene Dateien — `ci-check` rot
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `ci_manage log`) `ci-check 36256060965` rot: 20 Test-Fails (`src/archivar/parquet.rs:2120/:2136`, `skydirection.rs:758`, `parse.rs:1703`, `uws.rs:282`, `tests.rs:7985`, `mathematikerin/tests.rs:639` — teils fremd), clippy `-D warnings` (`extract.rs:1139`, `tests.rs:6181`, `te.rs:4306` `manual_div_ceil`), fmt `skydirection.rs:200`.
- **Blockade:** keine.
- **Braucht:** den **eigenen** clippy-Fund `te.rs:4306` fixen; fremde rote Dateien bleiben fremde Linien; dann `gh workflow run ci-check.yml`.

#### Trishuli — S1-Footprint offen (re-messen, kein vorgetäuschtes Warten)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) TE-Sweep signifikant lag3/6. Die S1-Post-Szene **existiert**: ASF liefert 6 post-event S1D-IW-GRDH-Frames (2026-08-28 … 2026-09-24); Bahrabise (27.7868/85.8993) liegt im Footprint `lat 27.070–28.998, lon 85.231–88.039` (z. B. `S1D_IW_GRDH_1SDV_20260924T001038…`). Download auth-gated (Earthdata-Login-OAuth; `EARTHDATA_EDL_TOKEN` vorhanden), CDS-`$value` ebenso.
- **Blockade:** Earthdata-OAuth-Session (kein Key-Gap).
- **Braucht:** Szene über die Earthdata-Session laden, Footprint/Alignment messen.

#### Alt-Orphans ×3 — Marker lesen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) `pfeiler-der-architektur.md` → descoped (`:37`, `:135` Prosa); `survey-2026-09-16-fremde-parser-sammlungen.md` → descoped (`:87` „nicht meßpflichtig", `:116` Verb „wartet"=unterhält, `:121` Bemerkung); `survey-fortschritt.md` → Träger Mycelium (`:65` §C-Punkte; getragen in `handover-2026-09-25-mycelium-folge161.md:257-271`).
- **Blockade:** keine.
- **Braucht:** keine (abgeschlossen).

### 2. Operator-gebundene Vorbereitung (Kante fertig, nur das Wort/Akt fehlt)

#### RR-Kanal / Beat-to-beat — braucht einen Gurt (Trigger: Förderung)
- **Status:** operator-gebunden (Beschaffung) | **Bindung:** operator
- **Trigger:** Förderung gewährt (Operator-Wort 2026-09-26)
- **Lage:** (gemessen 2026-09-26) Live-BLE: Uhr verbunden (`dev_[redacted]`), HR-Charakteristik `0x2a37` aufgelöst, StartNotify läuft, GFDI-Transport von der Firmware abgelehnt (`NotSupported`) → **keine RR** (`decode_hr_measurement` → `intervals` leer → Beat-Kanal still). FIT: alle 25 Operator-Aktivitäten `nn=0`; nur das Sample mit Gurt trägt 7564 NN. Der Handgelenk-Sensor überträgt HF (bpm), kein RR.
- **Blockade:** kein RR-fähiger Sensor.
- **Braucht:** einen **Brustgurt (Polar H10 / Garmin HRM-Dual)** beschaffen — dann füllt RR den BLE-Kanal und jede `.fit` (`nn`>0); danach `perm_tone_probe <log>` (Histogramm/Perzentile).

#### Weberin-Quellen — HAWC-Bundle lokal vorhanden
- **Status (Vorbereitung):** eigen — 4-Zert-Bundle `/tmp/opencode/hawc-ca-bundle.pem`, Reader gebaut.
- **Status (Akt):** operator | **Bindung:** operator
- **Trigger:** `OMEGAFLOW_CA_BUNDLE` — **bereits in `.secrets.local` gesetzt** (Audit 2026-09-26)
- **Braucht:** HAWC-Fetch mit dem lokalen Bundle prüfen; CI-Secret separat setzen.

#### DEMETER/CDPP-Order-Flow — Leser gebaut, Browser-Akt offen
- **Status (Vorbereitung):** eigen — `regards_order_read.rs` gebaut; `--live` meldet 403-WAF.
- **Status (Akt):** operator | **Bindung:** operator
- **Trigger:** Operator-Browser exportiert die Order-/Datei-JSON
- **Braucht:** `regards_order_read <order.json>`; Order-Ablauf 2026-09-28.

#### Onboard-/CIQ-Bedarf benennen
- **Status:** operator | **Bindung:** operator
- **Trigger:** Operator-Wort
- **Lage:** (gemessen 2026-09-24) Host-Reader `parse_fit` verdrahtet; CIQ ohne Reader.
- **Braucht:** Bedarf Ja/Nein — Nein → released, `FIT_DIR` bleibt der FIT-Kanal.

#### Weberin-Quellen — vDEC offen (3 von 4 per Credential gelöst)
- **Status (Vorbereitung):** eigen — Entwurf `state/mail/weberin-quellen-konten-2026-09-26.md` (privat).
- **Status (Akt):** operator | **Bindung:** operator
- **Trigger:** Operator-Wort
- **Lage:** (gemessen 2026-09-26) IGETS/ONC/TNS per Credential gelöst; vDEC 403 direct+Proton, Wayback 200.
- **Braucht:** vDEC-Antrag (Webform + Projekttext); Daten bleiben **lokal**.

#### Beat-Arbitrierung — verdeckter Lauf mit zwei Beat-Quellen
- **Status (Vorbereitung):** eigen — Spawn-Arbitrierung `main_flow.rs:504`.
- **Status (Akt):** operator | **Bindung:** operator
- **Trigger:** ein zweiter Beat-Kanal (der Gurt) liegt vor
- **Braucht:** `OMEGAFLOW_HIDDEN=1`-Lauf mit zwei Quellen → genau eine `beat source:`-Zeile.

#### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25) Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut; BOM-Zeilen stehen.
- **Braucht:** LOCK-Aufhebung, dann BOM bestellen (inkl. H2).

#### ESP32-Puls-Knoten (Träger ohne Uhr)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Wort:** ESP32 separat als eigener LOCK | 2026-09-25 | Operator (Session)

### 3. Blockiert / wartend (mit Trigger)

#### Commit ohne Operator-Wort — `c6edfbe45` (Riss, 2026-09-26)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) ein Sub-Agent committete Workflow-Änderungen ohne `/commit`-Wort und pushte.
- **Blockade:** keine (kein Rückbau).
- **Braucht:** im Abschluss-Check sichtbar tragen.

#### M2c — Rust-ZNSP-Host auf dem BL808 gegen das H2
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ox64 angekommen (`LZ473049629CN`)
- **Blockade:** Ox64 liegt beim Carrier.
- **Braucht:** Buildroot-Bring-up, dann Rust-ZNSP-Host.

#### BL808-eigenes 802.15.4-Radio — `pending`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** öffentlicher 802.15.4-Treiber/Stack für den BL808 (`bouffalo_sdk_bl808`, `zephyr#112921`)
- **Lage:** (gemessen 2026-09-26) Zephyr #112921 deckt BL61x/BL70x, kein BL808; `bouffalo_sdk_bl808` ohne `lmac154`.
- **Braucht:** PR-Suche beobachten.

#### Galileo Borduhr-Sprung A/B — keine Absolutfrequenz-Reihe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Fund einer öffentlichen Absolutfrequenz-Reduktion über 1995-11-30/12-01
- **Lage:** (gemessen 2026-09-26) gemessenes `absent`.
- **Braucht:** andere Reduktion finden; notfalls absent halten.

#### JWST Biosignatur-Kanäle pending
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** JWST-Detektion + Spektrum eines dieser Kanäle (`10.1073/pnas.2416188122`)
- **Lage:** (gemessen 2026-09-26) nur Modelle/Feasibility.
- **Braucht:** bei Fund Detektion + Spektrum ins Register.

#### Das eine Instrument — Anomalie offen
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein zweiter Messkanal (VLBI-Beacon) existiert
- **Blockade:** kein Instrument misst den vollen Phasenraum der Pioniere.
- **Braucht:** VLBI-Beacon auf der nächsten interstellaren Sonde.

#### JUICE-Erdpassage 28./29.09.2026 — Siegel-Wort fehlt (termin-kritisch)
- **Status:** termin:2026-09-29 | **Bindung:** termin:2026-09-29
- **Trigger:** 2026-09-28/2026-09-29
- **Lage:** (gemessen 2026-09-26) Prädiktionskanal `flyby-path-2-preregistration.md` + Addendum nachgezogen; RTSW/Swarm/OMNI2 live. **Riss:** Kp-Route in `61e272ab0` aus `sources.φ` entfernt → Kp-Zelle kann nicht füllen.
- **Blockade:** Siegel-Wort vor dem 28.09.
- **Braucht:** Siegel-Wort; danach In-situ-Messung gegen den präregistrierten Feldzustand.

### 4. Extern (Dritter)

#### Ox64-Lieferung
- **Status:** wartend | **Bindung:** termin (Carrier)
- **Trigger:** Ankunft (`LZ473049629CN`)
- **Lage:** (gemessen 2026-09-25) zwei Ox64 versandt.
- **Braucht:** Ankunft quittieren → M2c.

#### Postfach
- **Status:** wartend | **Bindung:** extern (Mail)
- **Trigger:** neuer Eingang (`state/mail/mail_ledger.φ`)
- **Lage:** (gemessen 2026-09-26) Ledger absent (Aufbau gehört CI); CSES-Limadou-Antwort Wiedervorlage; DSN-Briefe in Flug; Konto-Verifikationen.
- **Braucht:** `smail_recv` / Ledger bei Trigger.

#### BGR-Matched-Filter-Ankunft (Tonga)
- **Status:** wartend | **Bindung:** dritter (vDEC)
- **Trigger:** vDEC-Zugang gewährt (`ctbto.org/resources/for-researchers-experts/vdec`)
- **Braucht:** nach Zugang den matched-filter-Arrival messen.

#### DSN-Briefe in Flug
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Antwort der DSN (`state/mail/mail_ledger.φ`)
- **Braucht:** Antwort quittieren.

#### Gaia DR4 + Europa-Clipper-Erdpassage
- **Status:** termin:2026-12-02 | **Bindung:** termin:2026-12-02
- **Trigger:** 2026-12-02 (Gaia DR4) / 2026-12-03 (Europa Clipper)
- **Braucht:** am Datum Epochen-Astrometrie bzw. EC-Magnetfeld ernten.

## Operator-Wort-Register (Stand 2026-09-26)

Jedes gegebene Operator-Wort steht als `Wort | Datum | Quelle`. Der nächste Pass liest
es hier und legt den Punkt **nie erneut vor** — nur eine neue Messung öffnet ihn.

- **Wort:** „ändere die agents — das Handover nach umsetzbar/nicht umsetzbar sortieren" | 2026-09-26 | Operator (Session) → ausgeführt: AGENTS.md `3672968d3`.
- **Wort:** „alles Offene bis zur Kante abarbeiten, gemessen abschließen — nicht verschleppen" | 2026-09-26 | Operator (Session).
- **Wort:** „die Compiler-Arme durch Agenten bauen lassen" | 2026-09-26 | Operator (Session) → ausgeführt: LLNL-G3D-JPS, ISC-EHB, Slab2, EMC, cometels.
- **Wort:** „alles Offene und Benannt-Ungemessene wird übernommen" | 2026-09-26 | Operator (Session).
- **Wort:** „jedes Operator-Wort steht im Handover; keine Session kaut es neu durch" | 2026-09-26 | Operator (Session).
- **Wort:** „deine Daten/Datei verlassen das Gerät nie" | 2026-09-25 | Operator (Session) — [redacted]/DEMETER.
- **Wort:** „beides" (BLE-Live + FIT-Probe) | 2026-09-26 | Operator (Session) → ausgeführt: zwei verdeckte Läufe + 25 FIT-Parses.
- **Wort:** „das mache ich erst, wenn ich gefördert werde" (Gurt-Beschaffung) | 2026-09-26 | Operator (Session).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (pfad-begrenzt committen):
`docs/handover/handover-2026-09-26-sensory-folge178.md`,
`docs/handover/archiv/handover-2026-09-26-sensory-folge177.md`,
`tools/measure/src/bin/cross_te_screen.rs` (nur der rustfmt-Hunk),
`src/mathematikerin/te.rs` (nur der `div_ceil`-Hunk `:4306`),
`tools/harvest/src/bin/slab2_compiler.rs` (nur der `create_dir_all`-Hunk `:556-564`),
`tools/measure/src/bin/lsst_color_coupling_probe.rs` (der `-c.u=deg`-Hunk `:46`),
`src/archivar/kernels/naif_body_ids.tsv` (9 TNO-Zeilen),
`.github/workflows/kernel-flatten.yml` (`--bodies`-Erweiterung `:105`),
`.github/workflows/lsst-live-scan.yml` (zweite Kontroll-Cone-Stufe).

Nicht committen (fremd im geteilten Baum): `AGENTS.md`, `docs/handover/_template.md`,
`.github/workflows/eht-uvfits-cdn.yml`, `.github/workflows/modis-cdn.yml`,
`phi/blocked_sources.φ`, `phi/harvest.φ`, `state/` (privat).
