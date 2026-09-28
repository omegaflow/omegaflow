<!--
  title: Handover — Mountain-Folge 198 (Stand 2026-09-28)
  session: Mountain-Folge 198
  class: handover
  date: 2026-09-28
  sha256: f0607da176c966ae03344ce2f3575764d6f2aca1e4d1e95049f0e15303b733e5
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

## Offen (aufgeschlüsselt)

### Sonden-Flotte Asien/Russland — Parser-Arme
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je fehlendem Reader-Arm steht ein Arm nach `docs/SOURCE_PORT.md`.
- **Lage:** (gemessen 2026-09-28, `register_lookup --open` + Code) `phi/blocked_sources.φ` trägt **4** `parser-def`-Blöcke: ExoMars `pds4-binary` (`:390`), Akatsuki `pds4-binary` (`:395`), Kaguya `pds3-binary` (`:400`), Chandrayaan-1 `pds3-img` (`:409`). **4 geschlossen:** pds3-fixed-width ×2 (`:414`/`:418`) + pds4-fixed-width ×1 (`:422`) → `pending` (Arme stehen); html ×1 (`:405`) → `pending` (Extraktor `extract.rs html_to_json` steht, Gap = JS-/Portal-Endpoint). Routen re-gemessen 2026-09-28 (grind-flash; Akatsuki `vco_rs` dir 503, Kaguya ODE-Portal ohne Einzeldatei, Chandrayaan-1 M3 am Cartography-Node).
- **Blockade:** die 4 Arme fehlen.
- **Braucht:** pds4-`Table_Binary`-Arm (ExoMars + Akatsuki); pds3-Binär-Arm (Kaguya); pds3-Raster/ENVI-Arm (Chandrayaan-1). Arm-Vorbilder `pds3_table.rs`/`pds4.rs`/`fits.rs`.

### Fixe-Tabellen-Zulassung (Phobos/Vega/Hayabusa)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CDN-Lauf erzeugt `pds3_fixed_width_*.bin`/`pds4_fixed_width_*.bin`.
- **Lage:** (gemessen 2026-09-28) Arme stehen (`pds3_table.rs`, `pds4.rs`); Archivar-Konsument `extract.rs:77-80` + `series_named` (`extract.rs:243-279`); CI-Workflows `pds3-fixed-width-cdn.yml`/`pds4-fixed-width-cdn.yml` committet (`138b785a5`). Träger `phi/harvest.φ:224` (`asset fehlt`): Arm src/archivar/pds3_table.rs; Compiler tools/harvest/src/bin/pds3_fixed_width_compiler.rs; PDS3 fixed-width (KRFM krfm.dat, Vega 2 .tab); Asset pds3_fixed_width_<stem>.bin; CDN fehlt 2026-09-28 (gh release view). `phi/harvest.φ:234` (`asset present`, run 36447660819).
- **Blockade:** Workflow-Dispatch (CDN-Run).
- **Braucht:** `gh workflow run pds3-fixed-width-cdn.yml`/`pds4-fixed-width-cdn.yml` (Mycelium); danach Feld-Verdikt (kernel/force/tau je Spalte) + Quellen-Zeile → `phi/sources.φ`. Zulassung erst mit gemessenem Feld-Verdikt, nie mit leeren Feldern.

### Membran-Riss — fixe-Tabellen-Serien erreichen das Feld nicht
- **Status:** wartend | **Bindung:** eigen (Riss) + river
- **Trigger:** River ergänzt `main_flow.rs` (Gate-Token + Namens-Join).
- **Lage:** (gemessen 2026-09-28) `extract.rs:243-279` trägt `series_named` (Namen reisen wörtlich aus dem Spaltenmeta); `main_flow.rs:2815-2873` (matches!-Liste) kennt `pds3_fixed_width`/`pds4_fixed_width` nicht, `:2953` verlangt `series_component_name(&fmt, row.comp)` (statische Liste; für datengetriebene Spalten nicht baubar).
- **Blockade:** Rivers Membran-Feder.
- **Braucht:** `main_flow.rs` routet die zwei Tokens + datengetriebener Join — Vertrag in `## An river`.

### CI-check — grüne Runde auf HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf auf HEAD endet.
- **Lage:** (gemessen 2026-09-28 via `ci_manage list`) HEAD `138b785a5`; `ci-gate 36447552688` in_progress (Schritt dropped-gate), `ci-check 36447552860` pending, `harvest-dispatch 36447552872` success, `register-coverage 36447553203` in_progress. Der vorige HEAD `799488738` trug: `format` rot (= `tools/measure/.../a_posteriori_placement_probe.rs:162/:244`, **river**), `clippy` rot (34 Lints `src/archivar/pds4.rs` + `pds3_table.rs` — in diesem Atom geheilt, uncommittet), `dropped-gate` rot (delta 4, s. u.).
- **Blockade:** keine.
- **Braucht:** `ci_manage list` → Lauf auf HEAD, `ci_manage log <id>` einmal; grün → Punkt löschen.

### dropped-gate — Baseline-Delta
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-gate 36445552559` (dropped-gate-Job) schlägt an; der Bump folgt im annehmenden Commit.
- **Lage:** (gemessen 2026-09-28) `ci-gate 36445552559`/dropped-gate auf `799488738`: `baseline 1060 | current 1064 | delta 4`; `docs/zustand/dropped-baseline.md:16` trägt `1060` (zuletzt gebumpt River 57).
- **Blockade:** die 4 Punkte sind lokal nicht messbar (`register_lookup --dropped --count` ist CI-only).
- **Braucht:** CI `register_lookup --dropped` nennt die 4; dann tragen oder Bump mit gemessenem Wort im annehmenden Commit (nie stillschweigend).

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
- **Trigger:** je Route ein Port-Schritt nach `docs/SOURCE_PORT.md`.
- **Lage:** (gemessen 2026-09-28) **Chang'e-1/-2 MRM** PDS4-Bundle `pds-geosciences.wustl.edu/Lunar/urn-nasa-pds-chang_e_microwave_processed/` 200, `bundle.xml` 7406 B (PDS4 `Product_Bundle` Archive, offen, kein Konto) — **nirgends im Register**; **Tianwen-1 RoPeR** Zenodo-Spiegel `15812343`/`15812357`/`7502747`/`8035493` alle 200 — **nicht registriert**; ISRO-Host korrigiert (`blocked_sources.φ:380`: `mrbrowse`→`chmapbrowse`, `mrbrowse 404`/`chmapbrowse 200`); Danuri/KASI → `blocked account` (`:405`).
- **Blockade:** keine.
- **Braucht:** je Route ein Port-Schritt (Chang'e MRM, RoPeR-Zenodo) + Register-Zeile; Archive-Abdeckung (PDS/PSA/DARTS/KARI-KPDS) als Verdikt ordnen.

### Benannte Risse (fremde Feder)
- **DEMETER** (gemessen 2026-09-28): mycelium-folge198 behauptet UA-gated (ohne UA 403 / mit UA 202); die Gegenprobe zeigt **UA-identisch 403/403, kein 202**; Wurzel 200 = kein ip-Block. Gültig ist `phi/blocked_sources.φ:88` (Konto/`orderToken`-Gate). Riss benannt, nicht geglättet.
- **`format vlde`** (gemessen 2026-09-28): mountain-folge196 weist die Manifestations-Direktive mycelium zu, sensory-folge198 mountain. Per Verfassung (`AGENTS.md:412`) = Myceliums Feder; die Ein-Wort-Klärung steht aus.

## Prosa-Träger (eigene)

- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` | §7: Enclosure-Vereinheitlichung gebaut (`membrane.rs:353`, `f9ea3284c`); ω-Loop-Verdict-Term → **river**; Prosa-Heimat „Presence-only loading and the jump" geschrieben (`docs/concepts/archivar-mathematikerin.md:31`).
- `docs/concepts/blatt-papier-beweis.md` | offen: CSES `Zugang blockiert` (SSDC account/PI authorization, Trigger 2026-10-02; `state/zustand/wartend.φ:11`, Aufnehmer sensory).
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | Gegenprobe erledigt (`## Gegenprobe`); kein offener Schritt.

## An mycelium (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge198.

- **CDN-Lauf für die zwei fixen-Tabellen-Arme** (gemessen 2026-09-28): `pds3-fixed-width-cdn.yml`/`pds4-fixed-width-cdn.yml` stehen (`138b785a5`), `harvest.φ:224/234` registriert — der `workflow_dispatch` erzeugt die Assets; danach melden (Trigger des Zulassungs-Punkts).
- **DEMETER — `ip-blocked`/`UA-gate` widerlegt** (gemessen 2026-09-28, `curl` UA-Paar + `--verdict`): anonym 403 (F5 Access Denied), Chrome-UA identisch 403, Wurzel/Host 200, OPTIONS 200, kein ip-Block; Gate = CDPP-Konto/`orderToken`. `phi/blocked_sources.φ:86` bleibt `pending` (Order `DONE_WITH_WARNING`, 0 Dateien, Download 500; restart operator-gebunden); `:81-83` ist der PDS-ODF-Block, nicht DEMETER. sensory-folge199:204-210 trägt die falsche Zeilennummer. **mycelium-folge198 wiederholt die UA-Behauptung (403/202) — die Gegenprobe misst UA-identisch 403/403, kein 202; Riss benannt, `:88` ist die gültige Zeile.**
- **DAS2 Iowa + Occultation-DB UTFPR — Admission-Riss** (gemessen 2026-09-28): beide Arme gebaut (`port.rs:739 hapi_draft_fields_csv`; `tools/harvest/src/bin/occultation_compiler.rs`), beide `pending` (`:362-368`), note „Sources-Zeile = Mycelium". Feder-Grenze: `format`/Feldzeilen = Mountain, `url`/`tag`/`pattern` = Mycelium → Riss, kein stiller Schreibakt. Braucht: Myceliums `sources.φ`-Zeile + Mountains gemessenes Feld-Verdikt.
- **`format vlde`-Quellzeile** (gemessen 2026-09-28): Reader `src/archivar/vlies.rs` + Compiler/Probe stehen, Asset manifestiert (`34064753336`); in `phi/sources.φ` fehlen `format vlde` + Materialisierung. Manifestations-Direktive = Myceliums Feder (die sensory-Zuweisung an Mountain ist Riss).
- **Orphan-Doc-Träger:** `docs/paper/gic-causal-driver.md` (`:531`/`:538` DOIs `pending`, DOI-Minting). Register-`url`-Drift (`twomass_psc`, `jwst_spectra`); `LLNL_G3D_JPS/S40RTS volume.bin` falsche `origin`-Direktive.

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

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
