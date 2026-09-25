<!--
  title: Handover — Sensory-Folge 168 (Stand 2026-09-25)
  session: Sensory-Folge 168
  class: handover
  date: 2026-09-25
  sha256: dd03b6c8baeb7ca952b41bcc1eb1a938e51a7ee5dffd611be212084460dfa950
  status: live
-->
# Handover — Sensory-Folge 168 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage**
(mit Messstempel) / **Blockade** / **Braucht**. Sortierung: **erst logisch nach
Akteur (wer handelt) — Linie | Rat | Operator | Dritter —, dann chronologisch
(Messdatum)** (Operator-Wort 2026-09-25). Ein Punkt trägt genau einen Akteur.

Die FR945 ist das persönliche Gerät des Operators; ihre Kennung (MAC) und ihre
Daten bleiben lokal (`.secrets.local`, `data/`), nie getrackt, nie am CDN. Der
getrackte Baum trägt nur die Rolle „Träger", nie die Kennung.

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### ZNSP FORMNETWORK — `esp_zb_cfg_t`-Payload-Größe ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die kompilierte Referenz (der `zigbee-host.yml`-Lauf bzw. eine ESP-IDF-`sizeof`-Probe)
- **Lage:** der ZNSP-Transport ist gebaut (gemessen 2026-09-25 via `grind-flash`/`cargo check`): `firmware/radiatorium-lib/src/znsp.rs` (SLIP, `Frame`, CRC, `cmd`, `NetworkMachine`), Bin `firmware/radiatorium/src/bin/znsp_host.rs`; die FORMNETWORK-Request-Payload ist ABI-raw (`sizeof(esp_zb_cfg_t)`), gemessen nur aus dem Struct + Default-Alignment (16 B abgeleitet, nicht direkt) — `NetworkMachine::form_network_payload_pending()` liefert `None`.
- **Blockade:** keine — braucht eine kompilierte ESP-IDF-Referenz.
- **Braucht:** `sizeof(esp_zb_cfg_t)` gegen eine kompilierte Referenz messen (eine `sizeof`-Probe in `zigbee-host.yml` ergänzen), dann den FORMNETWORK-Encoder setzen; keine geratene Struct-Kopie.

#### Neu gebaute Artefakte — erste CI-Läufe beobachten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `esp32-firmware.yml`-Lauf (zweites Bin `znsp_host`) und der erste `zigbee-host.yml`-Lauf
- **Lage:** Workflow `zigbee-host.yml` und das zweite Bin `firmware/radiatorium/src/bin/znsp_host.rs` sind gebaut, aber uncommitted — kein Lauf gestartet (gemessen 2026-09-25 via `git status`).
- **Blockade:** keine.
- **Braucht:** nach `/commit`+Push `gh workflow run esp32-firmware.yml` und `gh workflow run zigbee-host.yml`; Ergebnis aus dem Watchdog-Snapshot `/tmp/opencode/ci_status.md` bzw. `ci_manage view <id>` — nie pollen.

#### HRV/Puls→Strahlung — Kette nach dem Live-Lauf beobachten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der BLE-Live-Lauf (Operator) hat `rr` geliefert
- **Lage:** die Kette ist gebaut (gemessen 2026-09-25 via `sgrep`): `src/archivar/hrv.rs:24-87` → `feed_beat_to_hrv` (`src/archivar/main_flow.rs:79-102`) → `tone_code` → `tone_scale` 0.25 (`src/mathematikerin/omega.rs:1670-1678`) → `aperture = field_permeability*tone_scale` (`omega.rs:349`) → Strahlung `Σω*aperture` (`src/mathematikerin/actuators.rs:29`).
- **Blockade:** hängt am BLE-Live-Fluss (Operator).
- **Braucht:** nach dem Live-Lauf `tone_code`→`tone_scale`→`aperture` im Frame / perm-Log lesen.

#### M2c — Rust-ZNSP-Host auf dem BL808 gegen das H2
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ox64 angekommen
- **Lage:** der Weg steht (Rat-Verdikt 2026-09-25): der Ox64 wird Host-CPU des Coordinators (ZNSP über UART), das H2 das Funkmodul; der portable ZNSP-Kern (`firmware/radiatorium-lib/src/znsp.rs`) wird vom `std`+`serialport`-Host mitbenutzt. Ox64-UART-Pins gemessen via PINE64-Wiki (UART0 GPIO14/15 = Pin 1/2, UART1 GPIO16/17 = Pin 32/31). Buildroot-Bring-up ungemessen.
- **Blockade:** Ox64 liegt beim Carrier.
- **Braucht:** Buildroot-Bring-up (Wiki-Flashing-Pfad), dann derselbe Rust-ZNSP-Host (`std` + `serialport`) auf dem BL808 gegen das H2.

#### BL808-eigenes 802.15.4-Radio — registrierter `pending`-Faden
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** öffentlicher 802.15.4-Treiber-/Stack-Fund für den BL808
- **Lage:** kein öffentlicher Treiber (gemessen 2026-09-25 via research-max `archive_search`): `bl_iot_sdk` trägt für BL808 nur `bl808_wifi` (kein 802.15.4), BL808-RM ohne Wireless-Kapitel, PAC+SVD existieren; `openbouffalo/bouffalo_sdk_bl808` (README-Raw 404, Inhalt ungemessen → `pending`). Route 2 nutzt das Radio bewusst nicht (Rat-Verdikt); der Punkt fällt nie auf 0.0.
- **Blockade:** kein Treiber
- **Braucht:** bei Treiber-Fund `sgrep`/`archive_search --github bouffalo_sdk_bl808 802.15.4`; bis dahin keine Arbeit.

#### ENSO-Kausalpfeil — Blatt I (Wind↔SST) ungemessen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) das ENSO-Blatt ist als Handover/Messauftrag geführt, aber ungemessen; `blatt-der-grat` führt den ENSO-Pfeil als ausgeschlossenen Kandidaten ohne TE-gegen-Schwelle-Verdikt.
- **Blockade:** das Becken-Windfeld (TAO/TRITON, Scatterometer) ist nicht registriert/geerntet.
- **Braucht:** Becken-Windfeld über `docs/SOURCE_PORT.md` registrieren, dann `te_pair_probe` Wind↔SST mit Lag-Sweep {1,3,6,12,24,48} fahren.
- **Quelle:** docs/concepts/ein-blatt-papier.md, docs/concepts/blatt-papier-beweis.md, docs/concepts/blatt-papier-resultat.md, docs/concepts/der-kausalpfeil.md, docs/blatt/blatt-der-grat.md

#### Lag-Sweep + KDE-Bandbreiten-Sensitivität — offene Mess-Gates
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) `ein-blatt-papier` und `blatt-papier-beweis` benennen Lag-Sweep und KDE-Bandbreite als offene Mess-Gates; `laic_probe --analyze --kde-scale` trägt den Knopf, die lokale laic-Ernte fehlt (CI).
- **Blockade:** keine — die Läufe sind offline/CI.
- **Braucht:** `laic_probe --analyze --kde-scale` in CI dispatchen und den Lag-Sweep je Paar drucken.
- **Quelle:** docs/concepts/ein-blatt-papier.md, docs/concepts/blatt-papier-beweis.md, docs/concepts/blatt-papier-resultat.md, docs/paper/laic-arrow-direction.md

#### Blatt-Probe → Membran-Bindung
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) `blatt-papier-beweis` §1: das Blatt entsteht im Offline-Probe; „Die Membran-Bindung bleibt pending."
- **Blockade:** keine.
- **Braucht:** die Offline-Blatt-Probe an den Membran-Pfad (`te_compute`, WGSL) binden.
- **Quelle:** docs/concepts/blatt-papier-beweis.md

#### Kreuz-Screening auf weitere Ereignisse (Bordeaux, Aaretal, Japan)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) ein Fenster/ein Ereignis je Becken; „keine Verallgemeinerung über Ereignisse (Bordeaux, Aaretal, Japan offen)."
- **Blockade:** keine.
- **Braucht:** dieselben Serien für Bordeaux/Aaretal/Japan ernten und `cross_te_screen` fahren.
- **Quelle:** docs/blatt/blatt-kreuz-screening-gyirong.md, docs/paper/cross-screening-tibet.md

#### Solar-Matrix: konditionale Prüfung 211A→193A
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) der einzige intra-AIA-Pfeil 211A→193A trägt nur einen Zeugen (die Matrix selbst); die konditionale Prüfung ist pending.
- **Blockade:** keine.
- **Braucht:** die konditionale Sonde auf 211A→193A anwenden (die Matrix siebt, die konditionale Sonde schlichtet).
- **Quelle:** docs/blatt/blatt-solar-seconds-matrix.md, docs/paper/solar-seconds-matrix.md, docs/surveys/axiom-gate-solar-seconds-matrix.md

#### Seismik-Flotte: Streuung senken + Stationsterm + W-Phase-M9
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die 16-Ereignis-Flotte ist unverzerrt (+1,7 km, se 4,7 km), aber 19 km über Ereignisse / 36 km über Stationen dominieren das ±10-km-Gate; Stationsterm (+5,69 s) offen; W-Phase-M9 entschieden, nicht gebaut.
- **Blockade:** keine.
- **Braucht:** besseres Picken / den mehrdeutigen pP-Zweig bei Δ≈30° auflösen und den Stationsterm an II.KIV wiederholen; W-Phase-M9 bauen.
- **Quelle:** docs/concepts/die-akteure-im-boden-und-wasser.md

#### ETOPO1-Gitter — CDN-Manifestation (395 MB)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die CDN-Manifestation des ETOPO1-Gitters (395 MB) steht in der offenen Reihenfolge der Boden-/Wasser-Akteure.
- **Blockade:** keine.
- **Braucht:** ETOPO1 als CDN-Asset registrieren und über den CI-Manifestator bringen.
- **Quelle:** docs/concepts/die-akteure-im-boden-und-wasser.md

#### Fünf Funken der Anomalie-Suche — Bau
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) alle fünf Funken sind ungebaut; Ordnung „Fertig vor neu": Funke 3 (Broker-Differenz) und 5 (TDB-Fenster) billig zuerst.
- **Blockade:** keine.
- **Braucht:** Funke 3 (Broker-Differenz, Abfrage-Logik) und 5 (TDB-Koinzidenz-Fenster) bauen, dann 4 (Deredden-Baseline) und 1 (Verschwindens-Suche), zuletzt 2 (TE zwischen Quellen).
- **Quelle:** docs/concepts/fuenf-funken-anomalie-suche.md

#### Positive Maske — Audit-Punkte + fehlende Treiber
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) Echo-Tiefe σ 19/36 km, Stationsterm offen (II.KIV +5,69 s), M9.1-Picker-Verdrahtung in die Flotte offen, Galileo-ODF Format 1 vs 2 ungemessen; Slab2 und Tomografie nicht registriert.
- **Blockade:** keine.
- **Braucht:** M9.1-Picker in die Flotte verdrahten; Galileo-ODF Format 1/2 messen; Slab2/Tomografie als Ernte-Kandidaten registrieren.
- **Quelle:** docs/concepts/positive-maske.md

#### Galileo Borduhr-Sprung A/B — Trenn-Frage pending
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Fund einer öffentlichen Absolutfrequenz-Reduktion über 1995-11-30/12-01
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) das öffentliche Material ist unzureichend, (A) gegen (B) zu trennen; die USO-Absolutfrequenz-Reihe endet Nov 1991, PDS-RSS-Datenköpfe ungeprüft.
- **Blockade:** fehlende Quelle.
- **Braucht:** `archive_search` auf eine Morabito-Fortsetzung / die Pass-Schätzung der PDS GO-J-RSS-* Datenköpfe / ein Einweg-Doppler-Residual über die Grenze.
- **Quelle:** docs/concepts/recherche-extern-galileo-ruck-borduhr-modell.md

#### Galileo-resid Kadenz — welcher Schritt setzte die 60-s-Zählintervalle?
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) zwei native Kadenzen koexistieren im Archiv (1 s über den Bestand, nativ 60 s im Dez-1990-Zweiweg-Fenster der 70-m-Stationen 14/43/63); offen bleibt, welcher Reduktions-/Tracking-Schritt die 60-s-Zählintervalle setzte.
- **Blockade:** keine.
- **Braucht:** den Tracking-/Reduktionsschritt der Dez-1990-Zweiweg-Pässe in TRK-2-25/ATDF-Doku suchen.
- **Quelle:** docs/concepts/recherche-galileo-kadenz-reconciliation.md

#### Sieben Sphären — Messvorschriften
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) Sphäre I (Fresnel-Winkeldurchmesser-Feld) ist ausstehend, VII (Dopplergeist) hat keine statistische Aggregation; II–VI sind Konzept.
- **Blockade:** keine.
- **Braucht:** das Fresnel-Winkeldurchmesser-Feld (Asteroiden-Bahn ∩ Gaia-Farbe ∩ IR-Ø) und die Dopplergeist-Aggregation bauen.
- **Quelle:** docs/concepts/the-seven-spheres.md

#### Zeugin — vpec-Redshift-Domäne (`cosmicflows_cf4.json` ungenutzt)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die Architektur verrechnet `z*C_LIGHT/HUBBLE_H0`; das vpec-Asset `cosmicflows_cf4.json` liegt ungenutzt vor.
- **Blockade:** keine.
- **Braucht:** die zwei Redshift-Domänen implementieren (naher Fluss `vpec/H0` + Hubble-Anteil, ferner Fluss kosmologisch).
- **Quelle:** docs/concepts/zeugnis.md

#### Nadel Ⅻ (Urknall) — Form-Entscheidung der Reihen-Paarung
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die Reihen-Paarung (Winkelserie × z-Reihe) ist eine offene Form-Entscheidung; CSES ist descoped.
- **Blockade:** keine.
- **Braucht:** die Reihenachse für die CMB×Struktur-Paarung festlegen.
- **Quelle:** docs/concepts/kybernetische-astrophysik.md

#### Korona-Heizung — ortsaufgelöster Aktive-Region-Pfad
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) kein Instrument misst eine fam- und bandbreiten-feste Koronal-Sprosse; der sun-as-a-star-Ladder dämpft den Flare ~10×. Drei Lücken: sub-minütige Auflösung (nur unter fam), Minuten-fam, Multi-Force-TE.
- **Blockade:** keine.
- **Braucht:** den ortsaufgelösten Aktive-Region-Pfad bauen; die Minuten-fam und die Multi-Force-TE (`nobel_probe_corona` v2) ergänzen.
- **Quelle:** docs/paper/corona-heating-ladder.md, docs/surveys/survey-ein-blatt-korona-heizung.md

#### Depth-Phase-Flotte — sP-corr-Gate + Stations-Azimut-Register
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) der Dual-Phase-Fit ist pending (das sP-corr-Gate ist ungesetzt, die Verteilung gemessen); die sechs Pilot-Azimute stehen in keinem Register.
- **Blockade:** keine.
- **Braucht:** das sP-corr-Gate aus der gemessenen Verteilung setzen und die sechs Stationsazimute registrieren.
- **Quelle:** docs/paper/depth-phase-echo-fleet.md, docs/surveys/axiom-gate-depth-phase-echo-fleet.md

#### Galileo-Rotor-CK — volle Spin-Historie ernten
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) nur das 4-Tage-EGA-1-Fenster (1990-12-07…11) der frame -77000 all-spin-bus CK-Serie ist geerntet; die Spin-Historie der übrigen Missionsepochen ist Register-Pflicht.
- **Blockade:** keine.
- **Braucht:** die frame -77000 CK-Serie über die EGA-1-Grenze hinaus harvesten.
- **Quelle:** docs/paper/galileo-rotor-spin-era-floor.md

#### JWST Biosignatur-Kanäle pending (O₂/O₃, Red-Edge, Saisonal)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine JWST-Detektion + Spektrum eines dieser Kanäle
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die Kanäle sind benannte pending-Zweige; die Abwesenheit ist gemessen (keine JWST-Detektion im gesuchten Record); die XUV-Re-Erklärung bleibt pending.
- **Blockade:** keine Quelle trägt eine Detektion.
- **Braucht:** bei Fund die Detektion + ihr Spektrum in das Register aufnehmen.
- **Quelle:** docs/paper/jwst-disequilibrium-survey.md

#### LAIC — Instrument A + DEMETER-Order
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) Instrument A (Ereignisrate) ist named/unbuilt; DEMETER-Daten sind order-gated (Produktdownload `online:false` → GET 500); CSES ist SMS-CN-blockiert.
- **Blockade:** DEMETER-Order-Flow; CSES-Login.
- **Braucht:** Instrument A bauen; den DEMETER-Order-Flow prüfen.
- **Quelle:** docs/paper/laic-arrow-direction.md

#### Nadel V — Positive-Control-Konus + IR-Exzess-Achse
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) der variable-rich Positive-Control-Konus ist der pending next control; die IR-Exzess-Achse 10–60 µm ist eine separate, unberührte Messung; die credentialisierten Alert-Streams (Fink Kafka, ANTARES Key, Rubin) fehlen.
- **Blockade:** credentialisierte Streams sind account-gebunden.
- **Braucht:** den Positive-Control-Konus (echte RR Lyrae/EB) und die IR-Exzess-Achse messen.
- **Quelle:** docs/paper/nadel-v-fresh-area-dip-scan.md

#### Planet-Nine/KBO — SPK-Type-1-Reader + h-Sweep
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) der SPK-Type-1-Reader (modified difference arrays) ist nicht implementiert — die merged Voyager-Kernels warten; die KDE-Bandbreiten-Sensitivität (h/2, 2h) ist ungemessen.
- **Blockade:** keine.
- **Braucht:** den SPK-Type-1-Reader implementieren und den h-Sweep fahren.
- **Quelle:** docs/paper/planet-nine-kbo-residue.md

#### Pioneer/Dark-Matter — ASCII-Vollmission-Reduktion + Voyager-Doppler + DSN 810-005
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die ASCII-Vollmission trägt den programmierten Uplink-Rampen-Sweep und ist die verbleibende offene Reduktion; ein Voyager-Doppler bei SPDF ist unverifiziert; 810-005 (MDA-Resolver) ist die benannte nächste Suche.
- **Blockade:** keine.
- **Braucht:** die ASCII-Vollmission-Reduktion (Rampen-Sweep) bauen; die Voyager-Doppler-Route bei SPDF prüfen; 810-005 suchen.
- **Quelle:** docs/paper/probe-front-dark-matter.md

#### Trishuli — Bahrabise-Richtungsverifikation + S1/SAR-Footprint
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort (Post-Sentinel-1-Szene)
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die Bahrabise-Spalten tragen die `te_pair_probe`-Spiegelunsicherheit; der räumliche Footprint bleibt pending (CEMS nur Grading, optisch wolkenverdeckt, S1 noch nicht archiviert).
- **Blockade:** S1-Post-Szene noch nicht archiviert.
- **Braucht:** die Bahrabise-Richtung mit richtungs-verifizierter Bibliothek prüfen; die Post-Sentinel-1-Szene auf Flutfläche/Narbe messen.
- **Quelle:** docs/paper/sturzflut-tibet-pfeil.md

#### Terminologie — Definitionen Ontologie-Motor / mycorrhizal_internet
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) für beide Begriffe existiert kein getrackter Definit; die Messung ist der Exklusivitäts-Zensus, die Definition `pending`.
- **Blockade:** keine.
- **Braucht:** die getrackten Definitionen schreiben.
- **Quelle:** docs/paper/terminologie-der-gegenstroemung.md

#### Broken-Null-Control — offene Spec-Pendings
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) max-T-Korrektur über die 20-Paar-Matrix, Lag-Sweep (nur τ ∈ {0, 60, 120}), Bandbreiten-Sensitivität, Fenster-Drift und Rest-FN bei n = 300 sind als Pendings benannt.
- **Blockade:** keine.
- **Braucht:** die fünf Messungen nachziehen (max-T, Lag-Sweep, h, Fenster-Drift, Rest-FN).
- **Quelle:** docs/surveys/axiom-gate-broken-null-control.md

#### Exzellenz-Konzept — Body/Header-sha-Widerspruch
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) der Header trägt einen sha256, der Body nennt den eigenen sha256 `PENDING`.
- **Blockade:** keine.
- **Braucht:** den Body-Satz auf den gesetzten sha256 nachziehen (oder den Header leeren).
- **Quelle:** docs/concepts/exzellenz-konzept.md

#### Weberin — zweite unabhängige Positions-Linie je Körper-Klasse
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) Planeten/Monde (INPOP-vs-DE/Astrometrie), Raumsonden (Doppler), breite TNO-Kette (`mpcorb_extended`) und Kometen (`dcom5`/`cometels`) tragen je nur eine Linie.
- **Blockade:** keine.
- **Braucht:** die zweite Linie je Klasse ernten/kompilieren (u. a. `pallas`, `juno_asteroid`, `encke`).
- **Quelle:** docs/surveys/survey-2026-09-07-weberin-sonnensystem-kette.md

#### Weberin Faden-Matrix — verbleibende No-Actor-Lücken + Broker-Positionen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) No-Actor: VHE/Neutrino/CR-Einzelteleskope ohne offene Route (TA-Vollkatalog, Super-K, JUNO, LHAASO-Event); Broker Lasair/ANTARES/Fink positions-pending; seismische Stations-Weltlinien nur als Events.
- **Blockade:** teils not-published.
- **Braucht:** die Broker-Positionen kompilieren; die seismischen Stations-Weltlinien registrieren; die not-published-Teleskope als absent halten.
- **Quelle:** docs/surveys/survey-2026-09-07-weberin-thread-matrix.md

#### Weberin-Quellen — offene, ungebaute Routen (WWLLN, BPA-GIC, HAWC, Fink/ALeRCE)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) vier Klassen haben eine offene, noch nicht gebaute Route (WWLLN-netcdf, BPA-GIC, HAWC, LHAASO); HAWC braucht das Let's-Encrypt-Intermediate im CA-Bundle; Fink/ALeRCE-Persistenz offen; TA-Vollkatalog not-published.
- **Blockade:** HAWC-TLS-Kette; Fink/ALeRCE antworten instabil.
- **Braucht:** den WWLLN-netcdf-Compiler + CDN-Manifestation und den BPA-GIC-Compiler bauen; das HAWC-CA-Intermediate setzen; Fink/ALeRCE re-messen.
- **Quelle:** docs/surveys/survey-2026-09-13-weberin-quellen.md, docs/surveys/survey-2026-09-13-weberin-quellen-treffer.md, docs/surveys/survey-2026-09-14-weberin-quellen-rerun.md

#### Sonden-Flotte — MAVEN-TNF + nativer Serien-Arm
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) Cassini/DART sind registriert, MAVEN (1 167 TNF-Produkte, anonym) bleibt offen; der native Serien-Arm fehlt (`tnf_compiler` schreibt CSV); Ulysses/BepiColombo/LRO ohne Register-Eintrag.
- **Blockade:** keine.
- **Braucht:** MAVEN-TNF ernten und registrieren; den nativen Serien-Arm bauen; Ulysses/BepiColombo/LRO den Registereintrag prüfen.
- **Quelle:** docs/surveys/survey-2026-09-16-sonden-flotte.md

#### Gelesen — keine offene Arbeit (descoped)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die Dokumente tragen keinen handlungsfähigen offenen Punkt: die Blätter/H0-Register erklären ihre Pendings als geschlossen bzw. als reinen Registerbefund; `die-weberin` und `blatt-kreuz-screening-kollab` haben ihre Stufen beantwortet.
- **Blockade:** keine.
- **Braucht:** — (descoped mit Befund)
- **Quelle:** docs/blatt/blatt-h0-linien-register.md, docs/blatt/blatt-kreuz-screening-kollab.md, docs/concepts/das-eine-instrument.md, docs/concepts/die-weberin.md, docs/paper/asmar-2005-spacecraft-doppler-tracking-noise-budget.md, docs/paper/causal-arrow-preregistration.md, docs/paper/h0-lines-register.md, docs/surveys/axiom-gate-cross-screening-tibet.md, docs/surveys/axiom-gate-h0-lines-register.md, docs/surveys/axiom-gate-neptune-rift-ephemerides.md, docs/surveys/axiom-gate-uranus-rift-ephemerides.md

### Operator handelt

#### BLE-HR-Live-Messung FR945
- **Status:** operator-gebunden (Akt: Hardware/Radio) | **Bindung:** operator
- **Trigger:** Operator startet den verdeckten Lauf am gekoppelten Gerät (945 am Arm)
- **Lage:** der GFDI-Bus-Abriss ist geheilt und **CI-bestätigt** (gemessen 2026-09-25 via `ci_manage log 36116592391`): alle `archivar::ble::tests` grün auf `45b4b2eb1`; Fix `src/archivar/ble.rs:1360/1418`. Live-Bestätigung offen (kein BlueZ/Gerät in der Session).
- **Blockade:** Hardware/Radio — der verdeckte Lauf braucht das Operator-Wort.
- **Braucht:** `OMEGAFLOW_BLE_HR=<FR945-MAC aus .secrets.local> OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow` (~45 s); `sensor:`-/`gfdi_line`-Zeilen lesen.

#### FIT-Verifikation eigene FR945-Datei (lokal-only)
- **Status:** operator-gebunden (Akt) | **Bindung:** operator
- **Trigger:** Operator startet den Lauf mit seiner Datei
- **Lage:** (gemessen 2026-09-25 via `cargo run -p omegaflow --bin omegaflow`) der Dump-Pfad ist gebaut (`src/main.rs:1-56`: liest `OMEGAFLOW_FIT_SAMPLE`, ruft `parse_fit`, druckt records/nn/min/max/all-finite, `exit(2)` bei Refusal); am SDK-Sample verifiziert. `emit_nn` (`src/archivar/fit.rs:264`) begrenzt `nn` auf `[NN_MIN_MS, NN_MAX_MS]` — eine Event-Timestamp-Diskontinuität ist `absent`, kein Riesen-`nn`; Test `hr_event_timestamp_discontinuity_is_absent`.
- **Blockade:** keine (Vorbereitung gebaut).
- **Braucht:** `OMEGAFLOW_FIT_SAMPLE=/abs/pfad/FR945.fit cargo run -p omegaflow --bin omegaflow` — Datei bleibt auf diesem Gerät.
- **Wort:** „meine fits datei verlässt niemals dieses gerät" | 2026-09-25 | Operator (Session)
- **Wort:** „die 945 von anderer Hardware trennen; meine Daten bleiben lokal" | 2026-09-25 | Operator (Session)

#### Onboard-/CIQ-Bedarf benennen
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort
- **Lage:** (gemessen 2026-09-24 via `sgrep`) der Host-Reader `parse_fit` (`src/archivar/fit.rs:78`) ist verdrahtet; CIQ hat keinen Reader (`sgrep -i ciq src tools` leer). Ein-Quellen-Regel: solange FIT läuft, sind BLE/Serial über die Arbitrierung ausgeschlossen.
- **Blockade:** keine — bewusst `pending` (Ein-Quellen-Regel).
- **Braucht:** Bedarf Ja/Nein — Nein → der Punkt ist released, `FIT_DIR` bleibt der FIT-Kanal.

#### Beat-Arbitrierung — verdeckter Lauf mit zwei Beat-Quellen
- **Status:** operator-gebunden (hidden) | **Bindung:** operator
- **Trigger:** Operator startet einen verdeckten Lauf mit zwei gesetzten Beat-Quellen
- **Lage:** die Spawn-Arbitrierung steht (`src/archivar/main_flow.rs:504`); kein Lauf hat die Verdict-Zeile (`beat source: …`) gemessen erzeugt (gemessen 2026-09-24 via `sgrep`). Die reine Funktion ist CI-getestet.
- **Blockade:** Heavy compute (CI/Operator) — eine Live-Messung braucht CI oder Operator-Wort.
- **Braucht:** `OMEGAFLOW_HIDDEN=1`-Lauf mit zwei Quellen, der genau eine `beat source:`-Zeile zeigt.

#### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25) Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut. Die GNSS-BOM-Lücke ist geschlossen: `docs/specs/mantis-shrimp-bom.md` trägt ATGM336H GNSS `1005009361234427`, 3,01 CHF ≈ 3,20 €* (via `archive_search --playwright`). Die BOM trägt die H2-Zeile `ESP32-H2-DevKitM-1-N4` (NCP-Funkmodul, Coordinator-Radio; `1005008131868631` ≈ 6,25 $ unverified · DigiKey 26282483 9,68 $) — ein Beschaffungsakt, ein LOCK.
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** Operator-Wort (LOCK-Aufhebung), dann Bestellung der BOM (inkl. H2).

#### ESP32-Puls-Knoten (Träger ohne Uhr)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25 via `sgrep`) Code-Eingang `BeatSource::Serial` steht; dedizierter Puls-Knoten unbestellt. Vom Rat unberührt (anderes Gegenüber/Ziel — Route 2 berührt ihn nicht).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —
- **Wort:** ESP32 separat als eigener LOCK | 2026-09-25 | Operator (Session)

#### Weberin-Quellen — account-/key-blockierte Rohkanäle
- **Status:** operator-gebunden | **Bindung:** operator (Zugang)
- **Trigger:** Operator-Wort je Konto
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) IGETS-Zeitreihen (SFTP-Account), vDEC-Rohwellenform (Antrag + Vertrag), ONC-Token, TNS-Key (`TNS_API_KEY`/`TNS_UA`) und SuperDARN-Globus sind account-/key-blockiert; die offenen Zwillingsrouten stehen.
- **Blockade:** Konto/Key fehlt.
- **Braucht:** Operator entscheidet je Konto (IGETS `igets-support@gfz.de`, vDEC-Antrag, ONC-Token, TNS-Key).
- **Quelle:** docs/surveys/survey-2026-09-13-weberin-quellen.md, docs/surveys/survey-2026-09-13-weberin-quellen-folge.md, docs/surveys/survey-2026-09-14-weberin-quellen-rerun.md

#### DEMETER/CDPP-Order + CSES-Zugang
- **Status:** blockiert | **Bindung:** operator (Zugang)
- **Trigger:** Order-Freigabe / chinesische Mobilnummer
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) DEMETER-Produktdownload `online:false` → GET 500, der Order-Flow ist nötig; CSES `leos.ac.cn` antwortet 000 und verlangt SMS-CN-Login — kein anonymer Zugang.
- **Blockade:** order-gated / SMS-CN.
- **Braucht:** den DEMETER-Order stellen bzw. den CSES-Zugang klären (Operator-Wort).
- **Quelle:** docs/paper/laic-arrow-direction.md, docs/concepts/kybernetische-astrophysik.md

### Extern handelt (Dritte)

#### Ox64-Lieferung
- **Status:** wartend | **Bindung:** termin (Carrier)
- **Trigger:** Ankunft (`LZ473049629CN`)
- **Lage:** PINE64 versandte zwei Ox64 (gemessen 2026-09-25 via `state/mail/mail_ledger.φ`, Mail `1790046330`) — Ankunft offen.
- **Blockade:** Carrier.
- **Braucht:** Ankunft quittieren; dann M2c (BL808-Host-Port).

#### Postfach
- **Status:** wartend | **Bindung:** extern (Mail)
- **Trigger:** neuer Eingang
- **Lage:** (gemessen 2026-09-25 via `sread` auf `state/mail/mail_ledger.φ`) Ledger **vorhanden** (155 Zeilen), jüngster Eingang `1790340479` (Globus SuperDARN-Transfer FAILED — nicht sensory-relevant); `sgrep` relay/sensor/beat/mantis/ble/hrv/zigbee = 0 Treffer.
- **Blockade:** keine.
- **Braucht:** `smail_recv` bzw. `state/mail/mail_ledger.φ` bei Trigger.

**PII (im Auftrag):** die FR945-MAC ist aus HEAD entfernt (Wert nur lokal in `.secrets.local`); der History-Rewrite (die Historie trägt sie weiter) + die umgesetzte MAC-Gate-Klasse (Platzhalter ausgenommen) leben in `docs/auftrag/auftrag-pii-history-rewrite.md` (Nachtrag 2026-09-25).

#### BGR-Matched-Filter-Ankunft (Tonga) — account-blockiert
- **Status:** wartend | **Bindung:** dritter (vDEC)
- **Trigger:** vDEC-Zugang gewährt
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) die PMCC-Detektionsliste läuft auf einem ~300-s-Raster; die Rohwellenform ist vDEC-account-blockiert; „Next step: none before access is granted."
- **Blockade:** vDEC-Account.
- **Braucht:** nach Zugangsgewährung den matched-filter-Arrival messen.
- **Quelle:** docs/paper/tonga-lamb-crosscheck.md, docs/concepts/die-akteure-im-boden-und-wasser.md

#### DSN-Briefe in Flug (Voyager/Mariner 10/Viking, Cassini, Juno)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Antwort der DSN
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) fünf DSN-Briefe sind in Flug (Entscheid-Handover §Warten).
- **Blockade:** Antwort ausstehend.
- **Braucht:** die Antwort quittieren.
- **Quelle:** docs/surveys/survey-2026-09-16-sonden-flotte.md

#### JUICE-Erdpassage 28./29.09.2026 — Kanal + In-situ-Messung
- **Status:** termin:2026-09-29 | **Bindung:** termin
- **Trigger:** 28./29.09.2026
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) der Prädiktionskanal steht bereit und alle Zellen sind pending; der Operator-Siegel-Wort liegt noch nicht vor.
- **Blockade:** der Operator muss das Siegel-Wort vor dem 28.09. setzen.
- **Braucht:** vor dem Flyby das Siegel-Wort setzen; nach dem Flyby die JUICE-In-situ-Feldmessung gegen den präregistrierten Feldzustand vergleichen (σ-Metrik gegen fam).
- **Quelle:** docs/paper/flyby-path-2-preregistration.md, docs/paper/flyby-path-2-falsification-metric-addendum.md, docs/concepts/der-paradigmenwechsel.md

#### Gaia DR4 + Europa-Clipper-Erdpassage
- **Status:** termin:2026-12-02 | **Bindung:** termin
- **Trigger:** 2.12.2026 (Gaia DR4) / 3.12.2026 (Europa Clipper)
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung) Gaia DR4 ist für den 2.12.2026 angekündigt (Epochen-Astrometrie, das Jeans-Residuum wird ein 4D-Feld); die Europa-Clipper-Erdpassage folgt am 3.12.2026.
- **Blockade:** Termin.
- **Braucht:** am jeweiligen Datum die Epochen-Astrometrie bzw. die EC-Magnetfeld-Messung ernten.
- **Quelle:** docs/concepts/der-paradigmenwechsel.md, docs/blatt/blatt-thuan-fragesteller.md

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
