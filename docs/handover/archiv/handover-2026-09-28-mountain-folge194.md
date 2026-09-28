<!--
  title: Handover — Mountain-Folge 194 (Stand 2026-09-28)
  session: Mountain-Folge 194
  class: handover
  date: 2026-09-28
  sha256: 72c1db4f7002f4a87768e53cba70127b5772a427d1223c222b978b25109c6967
  status: live
-->
# Handover — Mountain-Folge 194 (2026-09-28)

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
„kannst du das bitte fixen" — der `--verdict`-Werkzeugdefekt wird im Atom geheilt, nicht als Register-Punkt gefaltet | 2026-09-28 | Operator (Session, Mountain 194)
„bitte fixen statt verschleppen" — no-cadence-Sprachloch und ttl der 81 SPK-Blöcke im Atom gebaut, nicht getragen | 2026-09-28 | Operator (Session, Mountain 194)
„was sagt der rat?" — Rat zur Design-Frage (Verdict-Semantik, no-cadence-Repräsentation) | 2026-09-28 | Operator (Session, Mountain 194)
session-weiter Consent (Delegation), nicht das Commit-Wort — Commit/Push trägt `/commit` | 2026-09-28 | Operator (Mountain-Session 194)

## Offen (aufgeschlüsselt)

### CI-check — grüne Runde auf HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf auf HEAD endet.
- **Lage:** (gemessen 2026-09-28 via `git log -1`, `ci_manage status`) HEAD ist `44558ca48` (sensory-Push während der Session); `ci-check` `36409581203` lief auf `9e17cb331` (`in_progress`, Schritt `dropped-gate`), die Vorlauf-Läufe `cancelled` — kein grüner `ci-check` auf dem aktuellen HEAD im Fenster.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <id>` des Laufs auf HEAD einmal nach Lauf-Ende lesen; grün → Punkt löschen; bleibt `dropped-gate` rot, ist mycelium der Träger.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte liegt in `state/zustand/wartend.φ`)
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-28 via `sread state/mail/mail_ledger.φ`) zwei NED-Einträge — Token-Bitte an IPAC/Cook 2026-09-25/26 —, kein `NED_BYPARAMS_TIMEOUT_TOKEN` im Ledger; jüngster Eintrag 2026-09-27 (IGETS). `X-NED-Timeout-Token` sitzt nur am Form-POST (`tools/harvest/src/bin/ned_byparams_compiler.rs`).
- **Blockade:** Token fehlt.
- **Braucht:** mit dem Token den echten ByParams-Job fahren; scheitern Poll/Fetch, Token an `http_get`/`fetch_body` ergänzen (erst messen, dann ergänzen).

### Legacy-CDN-Assets `ssd.jpl.nasa.gov` — Disposition
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** mycelium hat `spectra.bin`/`nvss.json`/`first14.json`/`curated48_spectra.bin` unter ihren Family-Tag re-manifestiert.
- **Lage:** (gemessen 2026-09-28 via `archive_search --verdict` + Code) die Register-`url`-Zeilen `phi/sources.φ:2416` (ncei spectra), `:10605` (tapvizier nvss), `:10475` (tapvizier first14), `:9207` (exoplanetarchive curated48) sind **404**; die Assets liegen 200 unter dem Legacy-Tag `ssd.jpl.nasa.gov` (`CAPPED_RELEASE`, `src/archivar/cdn.rs:6`). Kein Quellen-Identitäts-Riss: die `url`-netlocs == Compiler-Tags (`spectral_compiler.rs:209`, `tap_compiler.rs:1736`, `jwst_spectra_compiler.rs:934`), der `origin`-Root stimmt.
- **Blockade:** Re-Manifest (mycelium).
- **Braucht:** nach Re-Manifest die vier Legacy-Assets löschen oder halten (Verdikt-Zeile in `dead_sources.φ`); bis dahin kein Mountain-Schritt.

## Prosa-Träger (eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` | Kette ungefüllt, Fill-Bin fehlt | **termin** 28./29.09.2026 (JUICE-Perigäum); nächster Schritt: Addendum zum Siegel nach dem Ereignis.
- `docs/concepts/blatt-papier-beweis.md` | fehlende Kanäle (`imos_argo_sst`, ESA-CCI, SOI acoustic, CSES) | nächster Schritt: Port über `docs/SOURCE_PORT.md` (river: TE-Membran-Bindung).
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | Gegenprobe + Lizenz-Zeile | nächster Schritt: Lizenz-Zeile setzen.
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` | §7 Stern-Gitter/`enclosure_rho`/Sprung | Archivar-Ladepfad (river: ω-Loop-Verdict).

## An fremde Feder (Absender-Zeile — Aufenthalt = Eigentum)

Ziel ist die **lebende** Übergabe der fremden Linie; der Eigentümer faltet die Zeile
in seinem Pass. `mycelium-folge191`/`192` sind archiviert — Ziel ist `mycelium-folge193`.

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| DAS2 Iowa + Occultation-DB UTFPR residual `sources.φ`-Zeile | `docs/handover/handover-2026-09-28-mycelium-folge193.md` | mountain folge189 | beide Arme gebaut (`b4106e69a`); die residuale `url`/`origin`/`compiler`/Tags-Zeile ist Myceliums Feder |
| Stehender-Pass-Korrektur (Verify-Semantik) | `state/zustand/standing-pass.md` (mycelium) | mountain folge191 | die Zeile „die Verify-Semantik ist korrekt" gilt nur für JSON-deklarierte Arme und Void-Fetches; der non-JSON-Defekt ist in `src/archivar/port.rs::ci_body_verdict` geheilt |
| NDK-Feld-Entscheidung — welcher Skalar eines Moment-Tensor-Events in die 9 Kraft-Medien eingeht | Future Operator-Queue (`state/future/handover/`) | mountain folge191/192 | der GCMT-NDK-Parser (`src/archivar/ndk.rs`) lebt, der reverify-Sweep überspringt `format == "ndk"` (`src/archivar/main_flow.rs:1462`), und `extract()` trägt keinen `ndk`-Arm (`format-void`). Frage: `mw` am Centroid, `m0`, oder kein Skalar? |
| LLNL_G3D_JPS/S40RTS `volume.bin` — falsche `origin`-Direktive in `sources.φ` | `docs/handover/handover-2026-09-28-mycelium-folge193.md` | mountain folge193 | die `LLNL_G3D_JPS.volume.bin`- und `S40RTS.volume.bin`-Blöcke in `phi/sources.φ` tragen die EMC/AFRP-Sammel-`origin`; die Manifestation `.github/workflows/volume-cdn.yml` holt korrekt `https://media.githubusercontent.com/media/tom-new/tomography-models/main/<name>.nc`. Myceliums Feder (Manifestations-Direktive). |
| Register↔CDN-Tag-Drift (Copilot-Punkte: spectra · nvss · first14 · curated48 · twomass_psc · jwst_spectra) | `docs/handover/handover-2026-09-28-mycelium-folge193.md` | mountain folge194 (Check) | **Kein Quellen-Identität-Riss** (gemessen 2026-09-28 via `archive_search --verdict` gegen Baum+Compiler): Register-`origin`/`compiler` stimmen mit den Produzenten — `tap_compiler`→`tapvizier.cds.unistra.fr`, `spectral_compiler`→`ncei.noaa.gov`, `jwst_spectra_compiler`→`exoplanetarchive.ipac.caltech.edu`, `vlies_density_compiler`→`ssd.jpl.nasa.gov-vlies`. Der Riss ist die **Manifestations-/Tag-Direktive** (Myceliums Feder): (a) die 4 registrierten Quellen sind 404 unter ihrem Tag, das Asset liegt legacy unter `ssd.jpl.nasa.gov` (je 206) → Re-Manifest dispatch; (b) `twomass_psc.bin` liegt unter `irsa.ipac.caltech.edu` (206), Produzent lädt `ssd.jpl.nasa.gov-vlies` (404); (c) `jwst_spectra.bin` liegt unter `ssd.jpl.nasa.gov` (206), Produzent `exoplanetarchive…` (404), Watchlist `ssd.jpl.nasa.gov`. `jwst_spectra.bin`/`twomass_psc.bin` tragen keine Register-Zeile (Compiler-Intermediates; die Quelle `curated48_spectra.bin` ist registriert) — Admission unberührt. **Aufgelöst (Messung 2026-09-28):** der 404 ist **kein** Quellen-Identitäts-Riss — die `url`-netloc == Compiler-Tag für alle vier, das Asset liegt unter dem Legacy-`ssd.jpl.nasa.gov`-Capped-Release. Myceliums Schuld: `spectra.bin` hat **keinen** ausführenden Workflow (`spectral_compiler` wird von keinem `.yml` gerufen); `nvss-cdn.yml`/`first14-cdn.yml`/`kernel-flatten.yml`(Job `jwst-spectra`) re-dispatch. Der Stehende-Pass-Eintrag „kanonisch mountain" ist der Fehlschluss (`url`/`origin`/`compiler`/Tags = Myceliums Feder). |
| format-Findings (vizier asu-tsv / ldeo NDK) — gemessen geschlossen | `docs/handover/handover-2026-09-28-mycelium-folge193.md` | mountain folge194 (Messung) | (a) die 6 `asu-tsv`-Zeilen bleiben `descoped` (`phi/blocked_sources.φ:297-319`, format-basiert, kein stale); (b) `phi/sources.φ:7256` `format ndk`: Parser `src/archivar/ndk.rs` lebt, `extract()` trägt keinen `ndk`-Arm, der Live-Sweep klassifiziert `format-gap` (`src/archivar/fetch.rs:664`), der Haupt-Loop überspringt `ndk` (`src/archivar/main_flow.rs:1462`). Die `JSON parse void`-Meldung ist geheilt (`src/archivar/port.rs:2592` `ci_body_verdict` → `FormatGap`). Kein Mountain-Bau; die Feld-Entscheidung ist operator-gebunden. |
| orphan-doc-Träger (fremde) | `docs/handover/handover-2026-09-28-river-folge55.md` · Future Operator-Queue · `docs/handover/handover-2026-09-28-mycelium-folge193.md` | mountain folge194 (Zensus) | river: `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` (Complexity-Term, Silence-Map-Probe), `docs/surveys/survey-messpunkt-verteilung.md` (2D-Voronoi, River-Folge 47). future: `docs/auftrag/auftrag-gic-einreichung.md` (Paper-Einreichung, Operator-Hand). mycelium: `docs/paper/gic-causal-driver.md` (zitierbare DOI). Ohne offenen Akt: `docs/concepts/kybernetische-astrophysik.md` (Manifest), `docs/concepts/pfeiler-der-architektur.md` (kein Marker), `docs/surveys/survey-fortschritt.md` (Inhalt `descoped`). |

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
