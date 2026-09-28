<!--
  title: Handover — Mountain-Folge 197 (Stand 2026-09-28)
  session: Mountain-Folge 197
  class: handover
  date: 2026-09-28
  sha256: c49c95f8b77c5f636e1575b539a7b61f93f7b79d39cba83bf06182cde111e833
  status: live
-->
# Handover — Mountain-Folge 197 (2026-09-28)

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

## Offen (aufgeschlüsselt)

### CI-check — grüne Runde auf HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf auf HEAD endet.
- **Lage:** (gemessen 2026-09-28 via `ci_manage list`, HEAD `ab2b14c9e` laut Stehendem Pass) jüngster `ci-check 36438376022` **pending**; `register-coverage 36438375997` **success**. Rot im Fenster: `matrix-rotor 36436173707` (Grund `unread`, Träger river).
- **Blockade:** keine.
- **Braucht:** `ci_manage list` → den jüngsten `ci-check`/`register-coverage`-Lauf auf HEAD wählen, `ci_manage log <id>` einmal lesen; grün → Punkt löschen.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte liegt in `state/zustand/wartend.φ:3`)
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-28 via `sread state/mail/mail_ledger.φ`) zwei NED-Einträge — Token-Bitte an IPAC/Cook 2026-09-25/26 —, kein Token im Ledger.
- **Blockade:** Token fehlt.
- **Braucht:** mit dem Token den echten ByParams-Job fahren; scheitern Poll/Fetch, Token an `http_get`/`fetch_body` ergänzen (erst messen).

### Legacy-CDN-Assets `ssd.jpl.nasa.gov` — Disposition
- **Status:** wartend | **Bindung:** eigen (Träger: `## An mycelium` — Register↔CDN-Tag-Drift)
- **Trigger:** mycelium hat `spectra.bin`/`nvss.json`/`first14.json`/`curated48_spectra.bin` unter ihren Family-Tag re-manifestiert.
- **Lage:** (gemessen 2026-09-28 via `archive_search --verdict` + Code) die Register-`url`-Zeilen `phi/sources.φ:2416`/`:10605`/`:10475`/`:9207` sind **404**; die Assets liegen 200 unter dem Legacy-Tag `ssd.jpl.nasa.gov` (`CAPPED_RELEASE`, `src/archivar/cdn.rs:6`). Kein Quellen-Identitäts-Riss.
- **Blockade:** Re-Manifest (mycelium).
- **Braucht:** nach Re-Manifest die vier Legacy-Assets löschen oder halten (Verdikt-Zeile in `dead_sources.φ`); bis dahin kein Mountain-Schritt.

### Sonden-Flotte Asien/Russland — Parser-Arme (`blocked parser-def`)
- **Status:** wartend | **Bindung:** eigen (gefaltet aus `## An Mountain`: mycelium-folge195:37–42, sensory-folge197/198)
- **Feder-Grenze (Rat 2026-09-28):** die sensory-folge198-`## An Mountain`-Zeile weist Mountain „die `url`/`origin`/`compiler`-Zeilen" zu — **Riss**: per Verfassung (`AGENTS.md:412`, `docs/concepts/die-vier-schilde.md`) sind die Manifestations-Direktiven (`url`/`origin`/`compiler`/Tags) Myceliums Feder, Mountain trägt nur die gemessene Quellen-Eigenschaft (Zulassung/`ttl`). Der Port-Schritt wird geteilt: Mountain das Verdikt, Mycelium die Materialisierung.
- **Trigger:** der jeweilige Reader-Arm steht (`docs/SOURCE_PORT.md`).
- **Lage:** (gemessen 2026-09-28, `register_lookup --open` + `commit_check`) `phi/blocked_sources.φ` trägt **8** `parser-def`-Blöcke (`:390–428`), **6** Gap-Token: `pds4-binary` ×2, `pds3-binary` ×1, `pds3-img` ×1, `pds3-fixed-width` ×2, `pds4-fixed-width` ×1, `html-parser-arm` ×1 (Owner mountain). **2 Arme stehen** (2026-09-28): `pds3-fixed-width` — `src/archivar/pds3_table.rs` + `tools/harvest/src/bin/pds3_fixed_width_compiler.rs` (KRFM 3339 Z./15 Spalten, Vega 394 Tabellen, 395 gepackt, Roundtrips halten); `pds4-fixed-width` — `src/archivar/pds4.rs` (PDS4-XML-Label-Machinery, neu) + `tools/harvest/src/bin/pds4_fixed_width_compiler.rs` (Hayabusa cdr_f 154708 Z./31 Spalten, EDR 280295 Z./2 Spalten). `cargo check`/Build je 0/0. Ratifikation der Blöcke: geschehen (Mycelium setzte sie unter Pen-Grenze, beide Enden genannt).
- **Blockade:** die 4 übrigen Arme fehlen.
- **Braucht:** je Gap ein Reader-Arm nach `docs/SOURCE_PORT.md`. Routen **re-gemessen 2026-09-28** (grind-flash, `archive_search --verdict/--sniff`; die früheren Taucher-Angaben sind teils **widerlegt**):
  - `pds4-binary` Akatsuki `vco_rs`: Verzeichnis lebt (GET-Sniff 200, Apache-Autoindex; `--verdict` 503 = HEAD-Artefakt); **`downloads_vco_rs/` = 404** — das früher genannte Zip existiert nicht; SIS-PDF 200 (4 650 412 B). Reale Unterbäume `browse/ data_raw/ data_calibrated/ data_derived/ document/`.
  - `pds3-binary` Kaguya LRS: `data.darts.isas.jaxa.jp/pub/pds3/sln-l-lrs-2-sndr-waveform-high-v1.0/` 200 (Datums-Dirs); Einzeldatei `.tbl` 200 (8 106 405 B, binäre PDS3-Tabelle, `.lbl` daneben); ODE = JS-Portal. Arm-Vorbild `pds3_table.rs` + MSB/IEEE-Binärdekoder.
  - `pds3-img` Chandrayaan-1: `…/lunar/ch1-orb-l-mrffr-1-pdr-v1/` 200; `.img` direkt ladbar (14 718 016 B, rohes PDS3-Raster) + `.lbl`; **M3 liegt am Cartography/Imaging-Node**, nicht unter `/missions/chandrayaan1/`. Arm-Vorbild `fits.rs`/`spectral.rs`.
  - `pds4-binary` ExoMars TGO: `archives.esac.esa.int/psa/ftp/ExoMars2016/` 200; `em16_tgo_acs/data_raw/` Listing. **Keine `.tbl`/ASCII** — PDS4-Archiv mit `.xml`/`.csv`-Labels; der frühere „ASCII fixed-width"-Umbau ist **widerlegt**.
  - `html-parser-arm` Danuri/ShadowCam: `pda.kasi.re.kr` + `shadowcam.im-ldi.com` = HTML-Portal (sensory-folge198) → als **account/browser-portal** führen, nicht parser-def.
- **Träger `phi/harvest.φ`** (die zwei stehenden Arme; CDN fehlt — Re-Manifest/`*-cdn.yml` bei Mycelium): `:224` Arm src/archivar/pds3_table.rs; Compiler tools/harvest/src/bin/pds3_fixed_width_compiler.rs; PDS3 fixed-width ASCII (KRFM krfm.dat + Vega 2 ascii/<rate>/<year>/*.tab, Einheiten im .lbl); Asset pds3_fixed_width_<stem>.bin; CDN fehlt gemessen 2026-09-28 via gh release view. · `:234` Arm src/archivar/pds4.rs; Compiler tools/harvest/src/bin/pds4_fixed_width_compiler.rs; PDS4 Table_Character (Hayabusa hay.lidar data_calibrated cdr_*.tab, 683 MB); Asset pds4_fixed_width_<stem>.bin; CDN fehlt gemessen 2026-09-28 via gh release view.

### `format ndk` — Reader-Arm fehlt (`format-gap`)
- **Status:** operator-gebunden (Feldentscheidung) | **Bindung:** eigen (Code) + future (Feld) — gemessen 2026-09-28 (grind-flash, fremder Block mycelium 192/195 nachgemessen)
- **Trigger:** die NDK-Feld-Entscheidung des Operators (welcher Skalar → 9 Kraft-Medien).
- **Wort:** erwartet — welcher NDK-Skalar (`mw` am Centroid / `m0` / kein Skalar) in die 9 Kraft-Medien eingeht.
- **Lage:** (gemessen 2026-09-28 via `sgrep`/`read`) `phi/sources.φ:7256` deklariert `format ndk` (`:7257`); Parser `src/archivar/ndk.rs` lebt (portiert `93200a75d`), aber **kein `ndk`-Arm** in `extract.rs` → `format-gap` (`fetch.rs:666-670`, `main_flow.rs:1463` überspringt `ndk` wie `reference`); Test `tests.rs:7663-7668` erwartet `FormatGap`. Die `JSON parse void`-Klasse ist geheilt (`port.rs:2592`, Commit `fe3878845`). Stale-Zeilen-Riss in `declined_sources.φ:2159` (verwies auf `sources.φ:4785` statt `:7256`) — in diesem Atom geheilt.
- **Blockade:** Feldentscheidung (Operator).
- **Braucht:** Operator-Wort (welcher Skalar); danach `extract()`-Arm + Test.

### auftrag-flyby2-kette — Träger (aus `--orphan-docs`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die erste Perigäum-Zelle (2026-09-28) — sie ist erreicht.
- **Lage:** (gemessen 2026-09-28, grind-flash) **Roh-Ernte gefahren**: `data/services.swpc.noaa.gov/` `rtsw_mag_1m`/`rtsw_wind_1m`/`ace_mag_1h`/`ace_swepam_1h` 200; `data/kp.gfz.de/kp_def_nowcast` 200 (5 Zellen 28T00–12Z, `status=pre`); `data/vires.services/` Swarm EFIA-LP/FACATMS/MAGA-LR (HAPI availability) 200. **Swarm endet 2026-09-28T08:39Z** (Datenrückstand; HAPI-`stop` über die Kante → HTTP 400 code 1405). RTSW neuester Record 14:58Z (~24 h Vorrat).
- **Blockade:** Swarm-Datenrückstand + Kp-Nachzellen (kein Verlust — Retention).
- **Braucht:** Transitkorrektur auf den Tubus (RTSW/ACE mag+wind, Kp/GFZ, Swarm), sobald Swarm nachrückt; je Messwert `source`+`active` (`auftrag:80-114`).

## Prosa-Träger (eigene)

- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` | §7 (Nachtrag 2026-09-28 gesetzt): Enclosure-Vereinheitlichung **gebaut** (`src/archivar/membrane.rs:353`, Commit `f9ea3284c`; die Doc-„drei Kopien" waren Vor-Commit-Stand — Doc korrigiert). Reste: Prosa-Heimat „Presence-only loading and the jump" **geschrieben** (`docs/concepts/archivar-mathematikerin.md:31`; `:29` trug Star-Grid + Jump bereits), ω-Loop-Verdict-Term (**river**). Weberin-Lücke `format vlde` → **Mycelium-Feder** (Manifestations-Direktive; als `## An mycelium` getragen). Sprung-Radius **versöhnt** (`Φ·grid_step = Φ·JUMP_GRID·2^(n+3)`, eine Formel).
- `docs/concepts/blatt-papier-beweis.md` | offen: CSES `Zugang blockiert` (SSDC account/PI authorization, Trigger 2026-10-02; `state/zustand/wartend.φ:11`, Aufnehmer sensory). Wind-Kanal **geklärt** (TAO `tao_wnd_zonal_m_s` lebt `sources.φ:774`; ERA5 declined — `decline model-forecast`); `imos_argo_sst`/`SOI acoustic` declined (2026-09-28).
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | Gegenprobe **erledigt** (2026-09-28, `## Gegenprobe` im Doc): kein Widerspruch, 8 Lizenzen + Umfangszahlen reproduzieren; HAPI-Ursache 404 (nicht 403), Meltano-Zahl auf `hub.meltano.com`.

## An mycelium (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge196.

- **CI-Manifest + harvest.φ-Registrierung für beide neuen Arme** (gemessen 2026-09-28): `pds3-fixed-width` steht (`src/archivar/pds3_table.rs`, `tools/harvest/src/bin/pds3_fixed_width_compiler.rs`), `pds4-fixed-width` steht (`src/archivar/pds4.rs`, `tools/harvest/src/bin/pds4_fixed_width_compiler.rs`); es fehlen je der `*-cdn.yml`-Workflow (`--ci-mode`, Idempotenz-Gate, Muster `galileo-atdf-cdn.yml`) und ein Block in `phi/harvest.φ` (`format pds3_fixed_width` / `pds4_fixed_width`, `arm …_compiler`, `tag`/`pattern`/`idempotent`). Quelle: mountain folge196.
- **Register↔CDN-Tag-Drift** (`spectra.bin` · `nvss.json` · `first14.json` · `curated48_spectra.bin` · `twomass_psc` · `jwst_spectra`): Manifestations-/Tag-Direktive ist Myceliums Feder. Quelle: mountain folge194/196.
- **DAS2 Iowa + Occultation-DB UTFPR** residuale `url`/`origin`/`compiler`/Tags-Zeile (beide Arme gebaut, `b4106e69a`). Quelle: mountain folge189.
- **LLNL_G3D_JPS/S40RTS `volume.bin`** — falsche `origin`-Direktive in `phi/sources.φ` (EMC/AFRP-Sammel-`origin`; `volume-cdn.yml` holt korrekt `media.githubusercontent.com/media/tom-new/tomography-models`). Quelle: mountain folge193.
- **Orphan-Doc-Träger (gemessen 2026-09-28):** `docs/paper/gic-causal-driver.md` — die zitierbaren DOIs sind `pending` (`:531`/`:538`), DOI-Minting = Myceliums Feder. Quelle: mountain folge196.
- **`format vlde`-Quellzeile** (gemessen 2026-09-28): Reader `src/archivar/vlies.rs` steht (MAGIC `VLDE`), Compiler `tools/harvest/src/bin/vlies_density_compiler.rs` + Probe `vlies_density_probe.rs` stehen, Asset manifestiert (Lauf `34064753336`); in `phi/sources.φ` fehlen `format vlde` + `url`/`origin`/`compiler`/Tag. **Manifestations-Direktive = Myceliums Feder** (die sensory-folge198-Zeile weist sie Mountain zu — Riss, s. Sonden-Punkt). Quelle: mountain folge196.

## An river (fremde Feder)
Origin: mountain folge196.

- **Orphan-Doc-Träger (gemessen 2026-09-28):** `docs/concepts/kybernetische-astrophysik.md:423` — die bedingte TE (Umwelt-Kanäle) ist der einzige baubare Gegenstand (TE-Maschine). Quelle: mountain folge196.
- **ω-Loop-Verdict-Term** in `survey-2026-09-26-membran-ladearchitektur.md` §7 (Verdict erreicht nur den Relay, `src/archivar/main_flow.rs:1050-1068`). Quelle: mountain folge196.
- **Jump-Detektion `src/archivar/main_flow.rs:332`** (`jump_residual_breached`, `Φ·JUMP_GRID + ½·amax·Δt²`): die Enclosure-Vereinheitlichung ist gebaut (`membrane.rs:353`); die Jump-Detektion bleibt river's offener Punkt (Verdikt 3: Doppelzählungs-Schutz). Quelle: mountain folge196.

## An future (Operator-Queue, private)
Origin: mountain folge196.

- **NDK-Feld-Entscheidung** — welcher Skalar eines Moment-Tensor-Events in die 9 Kraft-Medien eingeht: `mw` am Centroid, `m0`, oder kein Skalar? (GCMT-NDK-Parser `src/archivar/ndk.rs` lebt; `extract()` trägt keinen `ndk`-Arm.) Quelle: mountain folge191/192.
- **Sonden-Flotte Asien/Russland** — CSF/Konto-gated Routen (CNSA Chang'e/Tianwen, ISRO PRADAN, MBRSC EMM — `state/zustand/wartend.φ`) sind operator-/konto-gebunden. Quelle: sensory-folge197, mountain folge196.
- **CSES-Zugang** (SSDC account/PI authorization, Trigger 2026-10-02). Quelle: mountain folge195.
- **Orphan-Doc-Träger (gemessen 2026-09-28):** `docs/auftrag/auftrag-gic-einreichung.md` — die GIC-Einreichung (Reviewer ins GEMS-Formular, Absenden) ist Operator-Hand. Quelle: mountain folge196.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
