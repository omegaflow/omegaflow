<!--
  title: Handover — Mountain-Folge 199 (Stand 2026-09-28)
  session: Mountain-Folge 199
  class: handover
  date: 2026-09-28
  sha256: e7c6465531939e288f3e5a84acd756782113606e8bcb72ad190eb7e6e3370012
  status: live
-->
# Handover — Mountain-Folge 198 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert
(`state/zustand/standing-pass.md`); gemessen wird nur, was der eigene Trigger
für fällig erklärt.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja aus dem register aufstellen" — die offene Liste streng aus dem Register | 2026-09-27 | Operator (Session, Mountain 187)
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Session, Mountain 187)
„warum fixt du nicht anstatt zu verschleppen?" — arbeitbare Schritte werden im Atom gebaut, nicht getragen | 2026-09-28 | Operator (Session, Mountain 190)
„kannst du das bitte fixen" — der `--verdict`-Werkzeugdefekt wird im Atom geheilt | 2026-09-28 | Operator (Session, Mountain 194)
„bitte fixen statt verschleppen" — no-cadence-Sprachloch und ttl der 81 SPK-Blöcke im Atom gebaut | 2026-09-28 | Operator (Session, Mountain 194)
„was sagt der rat?" — Rat zur Design-Frage (Verdict-Semantik, no-cadence-Repräsentation) | 2026-09-28 | Operator (Session, Mountain 194)
Hier ausführen, keine Rangfolge, flash-first delegieren — session-weiter Consent (Delegation), nicht das Commit-Wort | 2026-09-28 | Operator (Session, Mountain 195)
Führe den Plan aus, delegiere an alle Sub-Agenten, höre die Stimmen bei Architektur/Abschluss — eine Session ist ein abgeschlossenes Atom | 2026-09-28 | Operator (Session, Mountain 196)
Committe und pushe jetzt — nur eigene Arbeit, gemessen nicht beteuert | 2026-09-28 | Operator (Session, Mountain 197)
Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent; session-weiter Consent (Delegation), nicht das Commit-Wort | 2026-09-28 | Operator (Session, Mountain 198)
„schau mal auf den desktop und die mails da ist was für dich angekommen" — NSE/SAMPLE_AUTHOR-Sendung | 2026-09-28 | Operator (Session, Mountain 198)
„mach erstmal eine archeologie warum ich die daten überhaupt angefragt habe" | 2026-09-28 | Operator (Session, Mountain 198)
„nach cdn veröffentlicht wird es natürlich nicht solange ich kein einverständnis habe" — NSE/SAMPLE_AUTHOR bleibt privates Holding | 2026-09-28 | Operator (Session, Mountain 198)
„mycelium kümmert sich drum" — die NSE-Warte/Datenrechte (Aufnehmer mycelium) | 2026-09-28 | Operator (Session, Mountain 198)
„kannst du bitte deine arbeit committen?" — Commit-Wort | 2026-09-28 | Operator (Session, Mountain 198)

## Offen (aufgeschlüsselt)

### Sonden-Flotte Asien/Russland — Parser-Arme
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je fehlendem Compiler-Bin/Sample ein Port-Schritt nach `docs/SOURCE_PORT.md`.
- **Lage:** (gemessen 2026-09-28) **Arme gebaut:** `src/archivar/pds4_binary.rs` (PDS4 `Table_Binary`, ExoMars + Akatsuki), `src/archivar/pds3_binary.rs` (Kaguya), `src/archivar/pds3_img.rs` (Chandrayaan-1 `.img`/ENVI) — alle `cargo check` 0/0, synthetische Tests, dispatcht in `extract.rs` (`series_parse_bin`/`series_named`). Register: die 4 `parser-def`-Blöcke (`:390`/`:395`/`:400`/`:409`) auf `pending` gesetzt (Arm steht, Compiler/Sample/Asset fehlen); gap-Token `:13`/`:14`/`:16` entsprechend. Routen 2026-09-28: Akatsuki `vco_rs` dir 503, Kaguya ODE-Portal ohne Einzeldatei, Chandrayaan-1 M3 am Cartography-Node.
- **Blockade:** kein Live-Sample; Compiler-Bin fehlt für alle drei.
- **Braucht:** Compiler-Bin je Arm + CDN-Workflow-Trigger (Mycelium); Sample/Verifikation, sobald eine Route eine Einzeldatei liefert.

### Fixe-Tabellen-Zulassung (Phobos/Vega/Hayabusa)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Feld-Verdikt je Spalte steht → Quellen-Zeile `phi/sources.φ`.
- **Lage:** (gemessen 2026-09-28 am Baum) **Trigger gefeuert:** `phi/harvest.φ:224` `asset present` (run 36447656801 success) und `:234` `asset present` (run 36447660819); Arme + Konsument stehen (`extract.rs:77-80`/`:243-279`); Workflows `pds3-fixed-width-cdn.yml`/`pds4-fixed-width-cdn.yml` committet. Die Assets sind da — das Feld-Verdikt (kernel/force/tau) je Spalte fehlt.
- **Blockade:** keine.
- **Braucht:** Feld-Verdikt aus den Assets + Quellen-Zeile → `phi/sources.φ`; Zulassung erst mit gemessenem Verdikt, nie mit leeren Feldern.

### LAB_A/MLZ NSE I(q,t) — Datensatz angekommen (SAMPLE_AUTHOR et al. 2010)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-/Operator-Wort zur Feld-/Wire-Karte der NSE-Serie (bis dahin kein eigener Schritt).
- **Herkunft (Archäologie, gemessen 2026-09-28 in `omegaflow` + `omegaflow-legacy` + Rätsel-Brett):** Das Labor-Pendant der zwölf Himmels-Nadeln ist das **Kuprat-Blatt** — die kausale DAG der drei Kanäle Spin (`em`) × Gitter (`acoustic`) × Suprastrom (`electric`) über der Dotierungs-Achse (`kybernetische-astrophysik.md:396-398` „Quanten-Cluedo im Labor (Kuprat, Atom D)"; `tools/measure/src/bin/rixs_cuprate_probe.rs:7-11`). Der tiefere Grund liegt im **Phasen-Slot** (Protokoll v9, 26×f64, `meta[13]`): PSD-Archive tragen |S(q,ω)|² ohne Phase — die **einzige Herkunft der Phase ist die komplexe FFT der Zeitreihen** (NSE-I(q,t), LISA) (`omegaflow-legacy/docs/TODO.md:3025-3028`). NSE I(q,t) ist das exakte Fourier-Paar von S(q,ω) (`docs/specs/spectral-oscillator.md:193-197`). Legacy-Track: `S15 — quanten-track` (fünf Atome C→D→komplexe TE→crystal_compiler→Kuprat-Blatt durchlaufen), das Blatt trägt „keine Aussage", nur der Spin-Kanal geerntet (`omegaflow-legacy` git `349a8f27`/`bb082a43`, `omegaflow-legacy/docs/status/lose-enden.md:499-510`). Anfrage-Kette: [redacted]/MPI-FKF 2026-08-23 + 2026-09-08 (`mail_ledger.φ:8,50`), `handover-2026-09-12-entscheid-folge3.md:59`, `handover-2026-09-14-forschung.md:99-106`, [redacted]/MLZ 2026-09-16 (`[redacted]-datenmanagement.md`), SAMPLE_CONTACT-Zusage 2026-09-17, Sendung 2026-09-28. Die Autorenanfrage war der **einzige** Pfad (arXiv 1008.4298 nur TeX+8 Figuren; IOP-Suppdata Bot-gated; iMPULSE nur Metadaten; Diss. print-only).
- **Lage:** (gemessen 2026-09-28) Antwort von Thomas SAMPLE_CONTACT (LAB_A/MLZ) mit [RETRACTED-SAMPLE]-NSE-I(q,t) an Q=(0.5, 0.5, 1.000), 13 Läufe 3.47–32.52 K: `.dat` (Echo-Fit: tau, FREQ1/2, I0/DI0, Pol/dPol, x0/Dx0, dL, Chisq) + `.log` (Fit P0/dP0, Gamma/dGamma, ChiSqr, QH/QK/QL, KF, TTAMEAN) + Instrumenten-Log + Analyse-ODP. Roh-Anlage sha256 `SOURCE_SHA…e6d864`, **privates Holding** `data/lab_a.data/SAMPLE_NSE_[RETRACTED-SAMPLE]/` (gitignored, PROVENANCE.txt). Reader + Compiler gebaut: `src/archivar/lab_reader.rs` (format `lab_reader`, dispatcht `extract.rs`), `tools/harvest/src/bin/lab_reader_compiler.rs`; `cargo check` 0/0, Compiler gegen die echten 13 Läufe gelaufen (Roundtrip hält). Register leer zur Quelle.
- **Blockade:** Wire-Lücke — die NSE-Polarisation trägt keine ICRS-Position/keinen Kraftkanal (passt nicht in den 26×f64-Record); Compiler schreibt eine flache Serie (`LABR`-Magic).
- **Braucht:** (a) Feld-/Wire-Entscheidung (welcher NSE-Skalar, welche Karte) — Rat/Operator; (b) **kein CDN** — privates Holding, Manifestation nur mit ausdrücklichem Operator-Einverständnis (SAMPLE_CONTACT teilt für Forschung, Zitatpaper+Instrument+Provenienz); (c) Antwort-Dank an SAMPLE_CONTACT (Operator-Hand, Future).

### Kuprat-Kanäle — Zeugen-Art `Substance` (gebaut)
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium re-kompiliert/re-manifestiert die vier Bins unter den neuen Magics und heilt die Tag-Drift.
- **Lage:** (gemessen 2026-09-28) Die vier Kanäle sind **Zeugen**, kein Feld-Source (zwei Außenstimmen Claude/GLM + Rat bestätigen; `src/archivar/tests.rs:8088-8091`). **Gebaut:** `WitnessKind::Substance` in `src/archivar/witness.rs` (`magic_identity`: `RIXS`/`RIXC`/`EELS`/`SRD6` → `Witness(Substance)`, Gate-Tests); die Bins auf die Hausform `[4-Byte-Magic][Version]` gebracht (`rixs.rs`, `suprastrom.rs`, `crystal_compiler.rs` charge/EELS); `cargo check` 0/0. Die **laufenden CDN-Bins** tragen noch den alten `0xCF86`-Kopf.
- **Blockade:** das Re-Manifest fehlt — die alten Bins würde der neue Reader ablehnen.
- **Braucht:** Mycelium: vier Bins re-kompilieren + unter Produzenten-Tags re-manifestieren; dann die vier `witness substance`-Zeilen in `phi/witnesses.φ` (`record rixs`/`rixc`/`eels`/`srd62`, `force` **absent**).

### `decline spectral-series` — Archäologie (2) [Operator-Frage]
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-/Rat-Wort zur Reklassifikation.
- **Lage:** (gemessen 2026-09-28, `sgrep`) nur **2** Einträge: `declined_sources.φ:1409` ONC-Hydrophon (1921-Bin-Schalldruck-Serie), `:4009` NOAA-NODD NRS (1195-Bin-Spektrum). Beide declined den **Feld-Anspruch** (Serie ≠ Skalar-Feldwert, Council C1 `c3f33f6`), der CDN-Record bleibt (`witnesses.φ:1`). **Kein** `rixs`/`eels`/`srd62` in declined (0 Treffer).
- **Blockade:** keine.
- **Braucht:** Operator-/Rat-Wort, ob die zwei als Zeugen reklassifiziert werden (`Substance` benennt Materie — sie sind Umwelt/Akustik, evtl. eine andere Art).

### Membran-Riss — fixe-Tabellen-Serien erreichen das Feld nicht
- **Status:** wartend | **Bindung:** eigen (Riss) + river
- **Trigger:** River ergänzt `main_flow.rs` (Gate-Token + Namens-Join).
- **Lage:** (gemessen 2026-09-28) `extract.rs:243-279` trägt `series_named` (Namen reisen wörtlich aus dem Spaltenmeta); `main_flow.rs:2815-2873` (matches!-Liste) kennt `pds3_fixed_width`/`pds4_fixed_width` nicht, `:2953` verlangt `series_component_name(&fmt, row.comp)` (statische Liste; für datengetriebene Spalten nicht baubar).
- **Blockade:** Rivers Membran-Feder.
- **Braucht:** `main_flow.rs` routet die zwei Tokens + datengetriebener Join — Vertrag in `## An river`.

### commit_check-Riegel Ereignis→Folge — gebaut, Alters-Riss offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-Wort zur Alters-/Session-Begrenzung des Riegels.
- **Lage:** (gemessen 2026-09-28) Fixture gebaut: `src/gate/commit_gate.rs` `ereignis_folge_violations` + Helfer, `commit_gate_vocab.json` (2 `feedback` + 6 `fixtures`), `tools/gate/src/bin/commit_check.rs` Riegel, 6 `#[cfg(test)]`-Tests; `cargo check -p omegaflow-gate` und `--features commit_gate --tests` je 0/0. Ereignis-Register `state/zustand/ereignisse.φ` (gitignored) führt die Klassen `account`/`send`; heute keine Zeile dieser Klasse.
- **Blockade:** Riss — der Riegel liest das append-only-Register **ungebunden** (jede Zeile, egal wie alt): eine aufgelöste Warte ohne bleibendes Verdikt blockt dauerhaft; zudem greift er nur im öffentlichen Repo (dort wird das state-Verzeichnis nicht gestaged), nicht im privaten `state`-Commit.
- **Braucht:** Rat-Wort zur Begrenzung (Session/Alter) oder bewusste Ungebundenheit; sonst bleibt der Riegel wie gebaut.

### CI-check — grüne Runde auf HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf auf HEAD endet.
- **Lage:** (gemessen 2026-09-28 via `ci_manage list`) HEAD `dfd2a4a19`; `ci-check 36454281654` pending, `ci-gate 36454281741` in_progress, `tools-build 36454281909` in_progress, `register-coverage 36454281665` success, `harvest-dispatch 36454281737` success. Voriger HEAD `799488738`: `format` rot (`tools/measure/src/bin/a_posteriori_placement_probe.rs:162/:244`, **river**), `clippy` rot (34 Lints) — am HEAD `dfd2a4a19` geheilt und committet (Arbeitsbaum == HEAD), `dropped-gate` rot (delta 4, s. u.). Neues Rot (mycelium): 5 Ephemeriden-CDN-Workflows `failure` (galileo/cassini/rosetta/messenger/near).
- **Blockade:** keine.
- **Braucht:** `ci_manage list` → Lauf auf HEAD; `ci_manage log <id>` einmal bei Rot; grün → Punkt löschen.

### dropped-gate — Baseline-Delta
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-gate 36445552559` (dropped-gate-Job) schlägt an; der Bump folgt im annehmenden Commit.
- **Lage:** (gemessen 2026-09-28) `ci-gate 36445552559`/dropped-gate auf `799488738`: `baseline 1060 | current 1064 | delta 4`; `docs/zustand/dropped-baseline.md:16` trägt `1060` (zuletzt gebumpt River 57).
- **Blockade:** die 4 Punkte sind lokal nicht messbar (`register_lookup --dropped --count` ist CI-only).
- **Braucht:** CI `register_lookup --dropped` nennt die 4; dann tragen oder Bump mit gemessenem Wort im annehmenden Commit (nie stillschweigend).

### Adressierte Reste (future-folge150, sensory-folge195)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Route ein Port-Schritt nach `docs/SOURCE_PORT.md`.
- **Lage:** (gemessen 2026-09-28) future-folge150: **CDS/Aladin** Tianwen-1 MoRIC (`alasky.cds.unistra.fr/Planets/CDS_P_Mars_Tianwen1-MoRIC/`, 76-m-Mosaik) und **Shandong-Univ.** `pds.wh.sdu.edu.cn` fehlen im Register (200/206). sensory-folge195/200: Sonden-Flotte Asien/RU anonym offen — Akatsuki-RS, Hayabusa PDS4, Venera 15/16, Vega-Ballons, Phobos-2, Danuri/ShadowCam (Details `docs/surveys/survey-2026-09-16-sonden-flotte.md ## Nachtrag 2026-09-28`).
- **Blockade:** keine.
- **Braucht:** Registrierung/Port je Route; die in `phi/pipeline/ledger.φ` als `ausstehend kandidat` eingetragenen lesen.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte in `state/zustand/wartend.φ:3`)
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-28 via `sread state/mail/mail_ledger.φ`) zwei NED-Einträge (Token-Bitte 2026-09-25/26), kein Token im Ledger.
- **Blockade:** Token fehlt.
- **Braucht:** mit dem Token den echten ByParams-Job fahren; scheitern Poll/Fetch, Token an `http_get`/`fetch_body` ergänzen (erst messen).

### Legacy-CDN-Assets `ssd.jpl.nasa.gov` — Disposition
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** mycelium hat `spectra.bin`/`nvss.json`/`first14.json`/`curated48_spectra.bin` unter den Family-Tag re-manifestiert.
- **Lage:** (gemessen 2026-09-28 via `archive_search --verdict` + Code) die Register-`url`-Zeilen `phi/sources.φ:2416`/`:10605`/`:10475`/`:9207` sind 404; die Assets liegen 200 unter dem Legacy-Tag `ssd.jpl.nasa.gov` (`CAPPED_RELEASE`, `src/archivar/cdn.rs:6`). Kein Quellen-Identitäts-Riss.
- **Blockade:** Re-Manifest (mycelium).
- **Braucht:** nach Re-Manifest die vier Legacy-Assets löschen oder halten (Verdikt-Zeile in `dead_sources.φ`); bis dahin kein Mountain-Schritt.

### `format ndk` — Reader-Arm fehlt
- **Status:** operator-gebunden (Feldentscheidung) | **Bindung:** eigen (Code) + future (Feld)
- **Trigger:** die NDK-Feld-Entscheidung des Operators.
- **Wort:** erwartet — welcher NDK-Skalar (`mw` am Centroid / `m0` / kein Skalar) in die 9 Kraft-Medien eingeht.
- **Lage:** (gemessen 2026-09-28 via `sgrep`/`read`) `phi/sources.φ:7256` deklariert `format ndk`; Parser `src/archivar/ndk.rs` lebt, kein `ndk`-Arm in `extract.rs` → `format-gap`. `JSON parse void`-Klasse geheilt (`port.rs:2592`, `fe3878845`); Stale-Zeilen-Riss in `declined_sources.φ:2159` geheilt.
- **Blockade:** Feldentscheidung (Operator).
- **Braucht:** Operator-Wort → `extract()`-Arm + Test.

### auftrag-flyby2-kette — Träger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Swarm rückt über `2026-09-28T08:39Z` nach (HAPI-`stop`).
- **Lage:** (gemessen 2026-09-28, grind-flash) Roh-Ernte gefahren: `rtsw_mag_1m`/`rtsw_wind_1m`/`ace_mag_1h`/`ace_swepam_1h` 200, `kp_def_nowcast` 200, Swarm HAPI 200; Swarm endet 2026-09-28T08:39Z, RTSW ~24 h Vorrat.
- **Blockade:** Swarm-Datenrückstand + Kp-Nachzellen.
- **Braucht:** Transitkorrektur auf den Tubus (RTSW/ACE mag+wind, Kp/GFZ, Swarm), sobald Swarm nachrückt; je Messwert `source`+`active`.

### Offene Asien/Russland-Kandidaten (future-folge148)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Route der Reader-Arm/Compiler nach `docs/SOURCE_PORT.md`.
- **Lage:** (gemessen 2026-09-28, general-Diver) **registriert:** Chang'e-1/-2 MRM PDS4+FITS als `parser-def binary` / `gap pds4-fits` (bundle.xml 206/7406 B sha256 `92f8cfc5…`; data/ce{1,2}_mrm.fits 206; 24 Maps 32ppd, ce2_t*_temp 2654904960 B > 2-GiB-Cap); Tianwen-1 RoPeR Zenodo als `parser-def binary` / `gap gras-2c` (15812343/15812357 je 84 Files ~150 MB; 8035493 `.mat` 393996707 B → `mat-v5`). ISRO-Host: `blocked_sources.φ` chmapbrowse 200 / mrbrowse 404; Danuri/KASI `:405` `blocked account`. Vorher Nicht-Registrierung belegt (sgrep 0 Treffer).
- **Blockade:** Reader-Arme `pds4-fits` / `gras-2c` / `mat-v5` fehlen.
- **Braucht:** je Arm ein Port-Schritt; Archive-Abdeckung (PDS/PSA/DARTS/KARI-KPDS) als Verdikt ordnen. Klassen-Träger: `phi/blocked_sources.φ::gap:pds4-fits ×1`, `phi/blocked_sources.φ::gap:gras-2c ×1`.

### Benannte Risse (fremde Feder)
- **Status:** wartend | **Bindung:** eigen (nur Benennung)
- **Trigger:** kein eigener — fremde Feder (Beleg: `phi/blocked_sources.φ:88`).
- **Lage:** (gemessen 2026-09-28) **DEMETER:** mycelium-folge198 behauptet UA-gated (ohne UA 403 / mit UA 202); die Gegenprobe zeigt UA-identisch 403/403, kein 202; Wurzel 200 = kein ip-Block. Gültig ist `phi/blocked_sources.φ:88` (Konto/`orderToken`-Gate). **`format vlde`:** mountain-folge196 weist die Manifestations-Direktive mycelium zu, sensory-folge198 mountain; per Verfassung (`AGENTS.md:412`) = Myceliums Feder.
- **Blockade:** fremde Feder.
- **Braucht:** DEMETER-Riss an mycelium; `format vlde`-Ein-Wort-Klärung Mycelium/Mountain.

## Prosa-Träger (eigene)

- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` | §7: Enclosure-Vereinheitlichung gebaut (`membrane.rs:353`, `f9ea3284c`); ω-Loop-Verdict-Term → **river**; Prosa-Heimat „Presence-only loading and the jump" geschrieben (`docs/concepts/archivar-mathematikerin.md:31`).
- `docs/concepts/blatt-papier-beweis.md` | offen: CSES `Zugang blockiert` (SSDC account/PI authorization, Trigger 2026-10-02; `state/zustand/wartend.φ:11`, Aufnehmer sensory).
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | Gegenprobe erledigt (`## Gegenprobe`); kein offener Schritt.
- `docs/specs/livefeed-gate.md` | Livefeed-Korrelations-Gate (Spec); Sprache-Angleichung 2026-09-28 (`--quelle`→`--source`, `quelle`→`source`); offene Marker = die `pending`-Felder der Ereignis-Tabelle (Spec-Inhalt); Träger Mountain.

## An mycelium (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge198.

- **CDN-Lauf für die zwei fixen-Tabellen-Arme** (gemessen 2026-09-28): `pds3-fixed-width-cdn.yml`/`pds4-fixed-width-cdn.yml` stehen (`138b785a5`), `harvest.φ:224/234` registriert — der `workflow_dispatch` erzeugt die Assets; danach melden (Trigger des Zulassungs-Punkts).
- **DEMETER — `ip-blocked`/`UA-gate` widerlegt** (gemessen 2026-09-28, `curl` UA-Paar + `--verdict`): anonym 403 (F5 Access Denied), Chrome-UA identisch 403, Wurzel/Host 200, OPTIONS 200, kein ip-Block; Gate = CDPP-Konto/`orderToken`. `phi/blocked_sources.φ:86` bleibt `pending` (Order `DONE_WITH_WARNING`, 0 Dateien, Download 500; restart operator-gebunden); `:81-83` ist der PDS-ODF-Block, nicht DEMETER. sensory-folge199:204-210 trägt die falsche Zeilennummer. **mycelium-folge198 wiederholt die UA-Behauptung (403/202) — die Gegenprobe misst UA-identisch 403/403, kein 202; Riss benannt, `:88` ist die gültige Zeile.**
- **DAS2 Iowa + Occultation-DB UTFPR — Admission-Riss** (gemessen 2026-09-28): beide Arme gebaut (`port.rs:739 hapi_draft_fields_csv`; `tools/harvest/src/bin/occultation_compiler.rs`), beide `pending` (`:362-368`), note „Sources-Zeile = Mycelium". Feder-Grenze: `format`/Feldzeilen = Mountain, `url`/`tag`/`pattern` = Mycelium → Riss, kein stiller Schreibakt. Braucht: Myceliums `sources.φ`-Zeile + Mountains gemessenes Feld-Verdikt.
- **`format vlde`-Quellzeile** (gemessen 2026-09-28): Reader `src/archivar/vlies.rs` + Compiler/Probe stehen, Asset manifestiert (`34064753336`); in `phi/sources.φ` fehlen `format vlde` + Materialisierung. Manifestations-Direktive = Myceliums Feder (die sensory-Zuweisung an Mountain ist Riss).
- **Orphan-Doc-Träger:** `docs/paper/gic-causal-driver.md` (`:531`/`:538` DOIs `pending`, DOI-Minting). Register-`url`-Drift (`twomass_psc`, `jwst_spectra`); `LLNL_G3D_JPS/S40RTS volume.bin` falsche `origin`-Direktive.
- **Warte `[redacted]` — Trigger gefeuert** (gemessen 2026-09-28): `state/zustand/wartend.φ:8` (Aufnehmer mycelium) wartete auf „Mail-Eingang"; die Mail ist da (`state/mail/mail_ledger.φ:181`, SAMPLE_CONTACT/LAB_A), die NSE-Daten sind angekommen und von Mountain privat gesichert (`data/lab_a.data/SAMPLE_NSE_[RETRACTED-SAMPLE]/`). Der Aufnehmer kann die Warte-Zeile schließen.
- **4 `pending`-Konten-Träger** (gemessen 2026-09-28, Rat): `phi/blocked_sources.φ:373/377/381/385` (CNSA/GRAS, NSSDC, ISRO/ISSDC, MBRSC/EMM) von `blocked account` → `pending` — Konto per Operator-Hand registriert, Ernte ausstehend; die Duty wandert zu mycelium (Netz). Träger: `phi/blocked_sources.φ::pending ×4`.
- **Kuprat-Kanäle — Zeugen, nicht `sources.φ`** (Operator-Hinweis 2026-09-28): die vier Kanäle (RIXS spin/charge, EELS, srd62-Suprastrom) gehören als `witness`-Zeilen nach `phi/witnesses.φ` — kein `ttl`/`field`, kein Feld-Source (`src/archivar/tests.rs:8088`). Eine Kuprat-Zeugen-Art fehlt noch (Operator-Wort ausstehend); bis dahin keine Registrierung. Unabhängig davon: **die Tag-Drift heilen** — die Probes/Workflows hardkodieren `…/releases/download/ssd.jpl.nasa.gov/{rixs_spin,srd62_suprastrom}.bin`, der Produzent ist Zenodo/NIST. Kein `tag kuprat`.

## An river (fremde Feder)
Origin: mountain folge198.

- **Membran-Riss — fixe-Tabellen-Serien erreichen das Feld nicht** (gemessen 2026-09-28): Archivar-Seite steht (`extract.rs:77-80` Arme; `extract.rs:243-279` `series_named` — die datengetriebenen Spaltennamen reisen wörtlich mit). Der Riss: `main_flow.rs:2815-2873` kennt `pds3_fixed_width`/`pds4_fixed_width` nicht; `:2953` verlangt `series_component_name(&fmt, row.comp)`. **Rat 2026-09-28 (Konsens (b)):** exakter Namens-Join; `series_component_name(&fmt, row.comp).or_else(|| names.get(row.comp as usize).map(String::as_str))`, dann das bestehende `fields.iter().find(|fc| fc.name == name)`. Die Namen kommen aus dem Archivar, **nie** aus einer zweiten statischen Liste (Duplikation = Fabrikation). Unverbundene Spalte → benannte Diagnostik, nie 0.0; Einheiten-Mismatch ist ein Port-Gate, nie Laufzeit-Konvertierung.
- **ω-Loop-Verdict-Term** in `survey-2026-09-26-membran-ladearchitektur.md` §7 (Verdict erreicht nur den Relay, `main_flow.rs:1050-1068`).
- **Jump-Detektion `main_flow.rs:332`** (`jump_residual_breached`): die Enclosure-Vereinheitlichung ist gebaut (`membrane.rs:353`); die Jump-Detektion bleibt Rivers Punkt (Doppelzählungs-Schutz).
- **Orphan-Doc-Träger:** `docs/concepts/kybernetische-astrophysik.md:423` (bedingte TE, der einzige baubare Gegenstand).

## An future (Operator-Queue, private)
Origin: mountain folge198.

- **NDK-Feld-Entscheidung** — welcher Skalar eines Moment-Tensor-Events in die 9 Kraft-Medien eingeht: `mw` am Centroid, `m0`, oder kein Skalar? (`src/archivar/ndk.rs` lebt; `extract()` trägt keinen `ndk`-Arm.)
- **Sonden-Flotte CSF/Konto-gated** (CNSA Chang'e/Tianwen, ISRO PRADAN, MBRSC EMM — `state/zustand/wartend.φ`).
- **CSES-Zugang** (SSDC account/PI authorization, Trigger 2026-10-02).
- **GIC-Einreichung** (`docs/auftrag/auftrag-gic-einreichung.md`, Operator-Hand).
- **KARI/ISRO-Konten** — Danuri/KASI (`pda.kasi.re.kr`, Login), Chandrayaan-2/3 + MOM (`chmapbrowse.issdc.gov.in`, Login/Register), Aditya-L1: `blocked account`; Konten-Entscheid.
- **NSE/SAMPLE_AUTHOR-Datenrechte** — SAMPLE_CONTACT (LAB_A/MLZ) teilt die YBCO-NSE-I(q,t)-Daten für Forschung (Zitatpaper+Instrument+Provenienz); soll das Asset CDN-manifestiert werden (öffentlich) oder privates Holding bleiben? Zudem ein Dank an SAMPLE_CONTACT (Operator-Hand).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
