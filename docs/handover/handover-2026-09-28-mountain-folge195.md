<!--
  title: Handover — Mountain-Folge 195 (Stand 2026-09-28)
  session: Mountain-Folge 195
  class: handover
  date: 2026-09-28
  sha256: e4ebf3c4082ecf080e95eabe7de5faf34b7e8164ee46adbd4d5c864320221a4b
  status: live
-->
# Handover — Mountain-Folge 195 (2026-09-28)

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

## Offen (aufgeschlüsselt)

### CI-check — grüne Runde auf HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf auf HEAD endet.
- **Lage:** (gemessen 2026-09-28 via `git log -1`, `ci_manage status`) HEAD ist `0da70c65d`; `ci-check 36422750525` pending, `36420817056` in_progress (Schritt `test`); die Vorlauf-Läufe `cancelled` — kein grüner `ci-check` auf dem aktuellen HEAD im Fenster.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <id>` des Laufs auf HEAD einmal nach Lauf-Ende lesen; grün → Punkt löschen; bleibt `dropped-gate` rot, ist mycelium der Träger.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte liegt in `state/zustand/wartend.φ`)
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-28 via `sread state/mail/mail_ledger.φ`) zwei NED-Einträge — Token-Bitte an IPAC/Cook 2026-09-25/26 —, kein `NED_BYPARAMS_TIMEOUT_TOKEN` im Ledger. `X-NED-Timeout-Token` sitzt nur am Form-POST (`tools/harvest/src/bin/ned_byparams_compiler.rs`).
- **Blockade:** Token fehlt.
- **Braucht:** mit dem Token den echten ByParams-Job fahren; scheitern Poll/Fetch, Token an `http_get`/`fetch_body` ergänzen (erst messen, dann ergänzen).

### Legacy-CDN-Assets `ssd.jpl.nasa.gov` — Disposition
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** mycelium hat `spectra.bin`/`nvss.json`/`first14.json`/`curated48_spectra.bin` unter ihren Family-Tag re-manifestiert.
- **Lage:** (gemessen 2026-09-28 via `archive_search --verdict` + Code) die Register-`url`-Zeilen `phi/sources.φ:2416`/`:10605`/`:10475`/`:9207` sind **404**; die Assets liegen 200 unter dem Legacy-Tag `ssd.jpl.nasa.gov` (`CAPPED_RELEASE`, `src/archivar/cdn.rs:6`). Kein Quellen-Identitäts-Riss.
- **Blockade:** Re-Manifest (mycelium).
- **Braucht:** nach Re-Manifest die vier Legacy-Assets löschen oder halten (Verdikt-Zeile in `dead_sources.φ`); bis dahin kein Mountain-Schritt.

### Sonden-Flotte Asien/Russland — Parser-Arme (`blocked parser-def`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der jeweilige Reader-Arm steht (`docs/SOURCE_PORT.md`; sensory-folge197).
- **Lage:** (gemessen 2026-09-28 via `commit_check`, uncommitted) die Klassen-Träger `phi/blocked_sources.φ::gap:pds3-binary ×1`, `phi/blocked_sources.φ::gap:pds3-fixed-width ×2`, `phi/blocked_sources.φ::gap:pds3-img ×1`, `phi/blocked_sources.φ::gap:pds4-binary ×2`, `phi/blocked_sources.φ::gap:pds4-fixed-width ×1` (7 Einträge, owner mountain) tragen keinen Reader-Arm; die `url`/`origin`-Zeilen sind teils gesetzt (`phi/sources.φ` Venera 15/16).
- **Blockade:** Parser-Arme (Raster/binär/fixed-width) fehlen.
- **Braucht:** je Gap ein Reader-Arm nach `docs/SOURCE_PORT.md` (`gap:`-Token → Arm).

## Prosa-Träger (eigene)

- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | `## Lizenz` gesetzt (8 gemessene Lizenzen; HAPI `unverified` — 403-Rate-Limit); offen: Gegenprobe der Spalten/Quellen. Nächster Schritt: Gegenprobe.
- `docs/concepts/blatt-papier-beweis.md` | offen: Wind-advective-Kanal `TAO/ERA5 pending` (:77), CSES `Zugang blockiert` (SSDC account/PI authorization, Trigger 2026-10-02) (:79), Membran-Bindung `pending` (:42). `imos_argo_sst` → `decline superseded-by-integrated`, `SOI acoustic` → `decline aggregate-index` (beide 2026-09-28). Nächster Schritt: TAO/ERA5-Port über `docs/SOURCE_PORT.md`; CSES an die Operator-Queue (Zugang).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` | §7-Reste: ω-Loop-Verdict-Term (river), Bootstrap/fetch-Loop, Sprung-Radius-Reconcile `Φ·JUMP_GRID·2ⁿ` vs `Φ·grid_step`, Weberin-Lücke `format vlde` (absent in `phi/sources.φ`). Stern-Gitter/`enclosure_rho`/Sprung sind gebaut und aus §7 gestrichen. Nächster Schritt: je Punkt.

## An mycelium (fremde Feder — Aufenthalt beim Eigentümer)

- **register-coverage-Arm** (gemessen 2026-09-28 via `ci_manage log 36417886014`): der `UNVERIFIABLE_PRIVATE`-Arm in `tools/register/src/bin/register_lookup.rs` ist mit diesem Atom gebaut und committet — Job `orphans` soll grün sein. Quelle: mountain folge195. Herkunft: mycelium-folge195 (`Weitergabe`).
- **Copilot-Streichung an die Linien**: mountain folge195 führt keine Copilot-Stimme; die Cloud-Grenze (private Daten nie) + public-CI-Ausnahme (`bin/ci_triage`) gilt. Quelle: mycelium-folge195.
- **Register↔CDN-Tag-Drift** (`spectra.bin` · `nvss.json` · `first14.json` · `curated48_spectra.bin` · `twomass_psc` · `jwst_spectra`): Manifestations-/Tag-Direktive ist Myceliums Feder; `spectra.bin` hat keinen ausführenden Workflow. Quelle: mountain folge194.
- **DAS2 Iowa + Occultation-DB UTFPR** residuale `url`/`origin`/`compiler`/Tags-Zeile (beide Arme gebaut, `b4106e69a`). Quelle: mountain folge189.
- **LLNL_G3D_JPS/S40RTS `volume.bin`** — falsche `origin`-Direktive in `phi/sources.φ` (EMC/AFRP-Sammel-`origin`; `volume-cdn.yml` holt korrekt `media.githubusercontent.com/media/tom-new/tomography-models`). Quelle: mountain folge193.

## An river (fremde Feder)

- **Träger-Orphans (river)**: `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` (Complexity-Term, Silence-Map-Probe), `docs/surveys/survey-messpunkt-verteilung.md` (2D-Voronoi). Quelle: mountain folge194.
- **ω-Loop-Verdict-Term** in `survey-2026-09-26-membran-ladearchitektur.md` §7 (Verdict erreicht nur den Relay, `src/archivar/main_flow.rs:1050-1068`). Quelle: mountain folge195.

## An future (Operator-Queue, private)

- **NDK-Feld-Entscheidung** — welcher Skalar eines Moment-Tensor-Events in die 9 Kraft-Medien eingeht: `mw` am Centroid, `m0`, oder kein Skalar? (GCMT-NDK-Parser `src/archivar/ndk.rs` lebt; `extract()` trägt keinen `ndk`-Arm.) Quelle: mountain folge191/192.
- **Sonden-Flotte Asien/Russland** — sensory-folge197 (Zeile 212 ff.) adressiert Mountain: je neuem Kanal ein `url`/`origin`/`compiler`-Port-Schritt nach `docs/SOURCE_PORT.md` (Akatsuki-RS, Hayabusa, Kaguya/SELENE, Chandrayaan-1, Venera 15/16, Vega 1/2, Phobos 2, ExoMars TGO, Danuri/KASI, CNSA). Eingetragen als `ausstehend kandidat` in `phi/pipeline/ledger.φ`. Quelle: sensory-folge197.
- **CSES-Zugang** (SSDC account/PI authorization, Trigger 2026-10-02). Quelle: mountain folge195.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
