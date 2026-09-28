<!--
  title: Handover — Mountain-Folge 196 (Stand 2026-09-28)
  session: Mountain-Folge 196
  class: handover
  date: 2026-09-28
  sha256: fa7c054469221db496d6203d6eef610409aaf3a9723f8e263233f4f1f6abc80b
  status: live
-->
# Handover — Mountain-Folge 196 (2026-09-28)

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

## Offen (aufgeschlüsselt)

### CI-check — grüne Runde auf HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf auf HEAD endet.
- **Lage:** (gemessen 2026-09-28 via `ci_manage list`) jüngster `ci-check 36423924944` **pending**; `register-coverage 36423924967` success. HEAD rückte auf `8748a39cd` (river folge57).
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
- **Status:** wartend | **Bindung:** eigen (gefaltet aus `## An Mountain`: mycelium-folge195:37–42, sensory-folge197:212–225)
- **Trigger:** der jeweilige Reader-Arm steht (`docs/SOURCE_PORT.md`; sensory-folge197).
- **Lage:** (gemessen 2026-09-28, `register_lookup --open` + `commit_check`) `phi/blocked_sources.φ` trägt **8** `parser-def`-Blöcke (`:390–428`), **6** Gap-Token: `pds4-binary` ×2, `pds3-binary` ×1, `pds3-img` ×1, `pds3-fixed-width` ×2, `pds4-fixed-width` ×1, `html-parser-arm` ×1 (Owner mountain). **2 Arme stehen** (2026-09-28): `pds3-fixed-width` — `src/archivar/pds3_table.rs` + `tools/harvest/src/bin/pds3_fixed_width_compiler.rs` (KRFM 3339 Z./15 Spalten, Vega 394 Tabellen, 395 gepackt, Roundtrips halten); `pds4-fixed-width` — `src/archivar/pds4.rs` (PDS4-XML-Label-Machinery, neu) + `tools/harvest/src/bin/pds4_fixed_width_compiler.rs` (Hayabusa cdr_f 154708 Z./31 Spalten, EDR 280295 Z./2 Spalten). `cargo check`/Build je 0/0. Ratifikation der Blöcke: geschehen (Mycelium setzte sie unter Pen-Grenze, beide Enden genannt).
- **Blockade:** die 4 übrigen Arme fehlen.
- **Braucht:** je Gap ein Reader-Arm nach `docs/SOURCE_PORT.md`. (Sample-Jagd 2026-09-28, research-max — Befund ist Behauptung; Session-Spot-Check ergab Riss, beide Linien stehen):
  - `pds4-binary` Akatsuki `vco_rs`: Zip-Endpoint data.darts.isas.jaxa.jp/pub/pds4/data/vco/downloads_vco_rs/pre-review/vco_rs_v001-*.zip (Taucher: 200·zip); in-tree 503 (WAF flappt). Re-Messung + Zip-Pfad (`inflate.rs`).
  - `pds3-binary` Kaguya LRS: Taucher nannte DARTS /pub/pds3/sln-l-lrs-2-sndr-waveform-high-v1.0/…/LRS_SW_WF_60N_072733E.tbl = 200 (direkt+Proton); **Session-`--verdict`: 503 (WAF-Range)** → Riss, mit einer Methode nachmessen. Arm-Vorbild `pds3_table.rs` + MSB/IEEE-Binärdekoder.
  - `pds3-img` Chandrayaan-1: Taucher nannte pds-geosciences.wustl.edu/lunar/ch1-orb-l-mrffr-1-pdr-v1/…/fsb_…_v1.img + M3-ENVI; **nicht gegengeprüft**. Vorbild `fits.rs`/`spectral.rs`.
  - `pds4-binary` ExoMars TGO: Taucher nannte ACS-NIR `…EA__4_0.tbl` als **ASCII fixed-width** (Token-Umbau auf `pds4-fixed-width`); **Session-`--verdict`: 404** → Riss, vor Umbau nachmessen.
  - `html-parser-arm` Danuri: Taucher: JS-Formular + `login.php`, kein maschinenlesbarer Endpoint (Register-Lage bestätigt) → als account/browser-portal führen, nicht parser-def. ShadowCam hat einen offenen Baum (pds.shadowcam.im-ldi.com, 2026-09-28 reachable) → eigene Zeile, Format ungemessen.

### auftrag-flyby2-kette — Träger (aus `--orphan-docs`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die erste Perigäum-Zelle (2026-09-28) — sie ist erreicht.
- **Lage:** (gemessen 2026-09-28 via `--orphan-docs` + Doc-Lesung) `docs/auftrag/auftrag-flyby2-kette.md:16-17/:119` trägt einen echten offenen Marker (die Kette selbst ist ungemessen); bislang ohne Träger.
- **Blockade:** keine.
- **Braucht:** Fill-Run ≤ 24 h ab der Perigäum-Zelle starten (RTSW/ACE 1 m, Kp/GFZ, Swarm/HAPI rohdatiert ernten, transitkorrigiert auf den Tubus legen, je Messwert `source`+`active` mitprotokollieren; `:80-114`).

## Prosa-Träger (eigene)

- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` | §7-Reste: ω-Loop-Verdict-Term (river), Prosa-Heimat „Presence-only loading and the jump", Bootstrap/fetch-Loop/Katalog-Branches, Weberin-Lücke `format vlde` (`vlies.rs`-Reader + `vlies_density_compiler.rs` stehen; die `format vlde`-Quellzeile in `phi/sources.φ` fehlt). Sprung-Radius **versöhnt** (2026-09-28: `Φ·grid_step = Φ·JUMP_GRID·2^(n+3)`, eine Formel). Nächster Schritt: je Punkt.
- `docs/concepts/blatt-papier-beweis.md` | offen: CSES `Zugang blockiert` (SSDC account/PI authorization, Trigger 2026-10-02; `state/zustand/wartend.φ:11`, Aufnehmer sensory). Wind-Kanal **geklärt** (TAO `tao_wnd_zonal_m_s` lebt `sources.φ:774`; ERA5 declined — `decline model-forecast`); `imos_argo_sst`/`SOI acoustic` declined (2026-09-28).
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | Gegenprobe **erledigt** (2026-09-28, `## Gegenprobe` im Doc): kein Widerspruch, 8 Lizenzen + Umfangszahlen reproduzieren; HAPI-Ursache 404 (nicht 403), Meltano-Zahl auf `hub.meltano.com`.

## An mycelium (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge196.

- **CI-Manifest + harvest.φ-Registrierung für beide neuen Arme** (gemessen 2026-09-28): `pds3-fixed-width` steht (`src/archivar/pds3_table.rs`, `tools/harvest/src/bin/pds3_fixed_width_compiler.rs`), `pds4-fixed-width` steht (`src/archivar/pds4.rs`, `tools/harvest/src/bin/pds4_fixed_width_compiler.rs`); es fehlen je der `*-cdn.yml`-Workflow (`--ci-mode`, Idempotenz-Gate, Muster `galileo-atdf-cdn.yml`) und ein Block in `phi/harvest.φ` (`format pds3_fixed_width` / `pds4_fixed_width`, `arm …_compiler`, `tag`/`pattern`/`idempotent`). Quelle: mountain folge196.
- **Register↔CDN-Tag-Drift** (`spectra.bin` · `nvss.json` · `first14.json` · `curated48_spectra.bin` · `twomass_psc` · `jwst_spectra`): Manifestations-/Tag-Direktive ist Myceliums Feder. Quelle: mountain folge194/196.
- **DAS2 Iowa + Occultation-DB UTFPR** residuale `url`/`origin`/`compiler`/Tags-Zeile (beide Arme gebaut, `b4106e69a`). Quelle: mountain folge189.
- **LLNL_G3D_JPS/S40RTS `volume.bin`** — falsche `origin`-Direktive in `phi/sources.φ` (EMC/AFRP-Sammel-`origin`; `volume-cdn.yml` holt korrekt `media.githubusercontent.com/media/tom-new/tomography-models`). Quelle: mountain folge193.
- **Orphan-Doc-Träger (gemessen 2026-09-28):** `docs/paper/gic-causal-driver.md` — die zitierbaren DOIs sind `pending` (`:531`/`:538`), DOI-Minting = Myceliums Feder. Quelle: mountain folge196.

## An river (fremde Feder)
Origin: mountain folge196.

- **Orphan-Doc-Träger (gemessen 2026-09-28):** `docs/concepts/kybernetische-astrophysik.md:423` — die bedingte TE (Umwelt-Kanäle) ist der einzige baubare Gegenstand (TE-Maschine). Quelle: mountain folge196.
- **ω-Loop-Verdict-Term** in `survey-2026-09-26-membran-ladearchitektur.md` §7 (Verdict erreicht nur den Relay, `src/archivar/main_flow.rs:1050-1068`). Quelle: mountain folge196.

## An future (Operator-Queue, private)
Origin: mountain folge196.

- **NDK-Feld-Entscheidung** — welcher Skalar eines Moment-Tensor-Events in die 9 Kraft-Medien eingeht: `mw` am Centroid, `m0`, oder kein Skalar? (GCMT-NDK-Parser `src/archivar/ndk.rs` lebt; `extract()` trägt keinen `ndk`-Arm.) Quelle: mountain folge191/192.
- **Sonden-Flotte Asien/Russland** — CSF/Konto-gated Routen (CNSA Chang'e/Tianwen, ISRO PRADAN, MBRSC EMM — `state/zustand/wartend.φ`) sind operator-/konto-gebunden. Quelle: sensory-folge197, mountain folge196.
- **CSES-Zugang** (SSDC account/PI authorization, Trigger 2026-10-02). Quelle: mountain folge195.
- **Orphan-Doc-Träger (gemessen 2026-09-28):** `docs/auftrag/auftrag-gic-einreichung.md` — die GIC-Einreichung (Reviewer ins GEMS-Formular, Absenden) ist Operator-Hand. Quelle: mountain folge196.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
