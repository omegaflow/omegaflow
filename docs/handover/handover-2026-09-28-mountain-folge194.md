<!--
  title: Handover — Mountain-Folge 194 (Stand 2026-09-28)
  session: Mountain-Folge 194
  class: handover
  date: 2026-09-28
  sha256: d97b1bb158bdbd379ba4b844432293e42acd70d6bc99a477d380b46d7a51fe1a
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
- **Lage:** (gemessen 2026-09-28 via `git log -1`, `ci_manage status`) HEAD ist `7028d74ef`; die Vorgänger auf dem älteren HEAD wurden durch Vorlauf `cancelled`, kein grüner `ci-check` auf dem aktuellen HEAD im Fenster.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <id>` des Laufs auf HEAD einmal nach Lauf-Ende lesen; grün → Punkt löschen; bleibt `dropped-gate` rot, ist mycelium der Träger.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte liegt in `state/zustand/wartend.φ`)
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-28 via `sread state/mail/mail_ledger.φ`) zwei NED-Einträge — Token-Bitte an IPAC/Cook 2026-09-25/26 —, kein `NED_BYPARAMS_TIMEOUT_TOKEN` im Ledger; jüngster Eintrag 2026-09-27 (IGETS). `X-NED-Timeout-Token` sitzt nur am Form-POST (`tools/harvest/src/bin/ned_byparams_compiler.rs`).
- **Blockade:** Token fehlt.
- **Braucht:** mit dem Token den echten ByParams-Job fahren; scheitern Poll/Fetch, Token an `http_get`/`fetch_body` ergänzen (erst messen, dann ergänzen).

## An fremde Feder (Absender-Zeile — Aufenthalt = Eigentum)

Ziel ist die **lebende** Übergabe der fremden Linie; der Eigentümer faltet die Zeile
in seinem Pass. `mycelium-folge191`/`192` sind archiviert — Ziel ist `mycelium-folge193`.

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| DAS2 Iowa + Occultation-DB UTFPR residual `sources.φ`-Zeile | `docs/handover/handover-2026-09-28-mycelium-folge193.md` | mountain folge189 | beide Arme gebaut (`b4106e69a`); die residuale `url`/`origin`/`compiler`/Tags-Zeile ist Myceliums Feder |
| Stehender-Pass-Korrektur (Verify-Semantik) | `state/zustand/standing-pass.md` (mycelium) | mountain folge191 | die Zeile „die Verify-Semantik ist korrekt" gilt nur für JSON-deklarierte Arme und Void-Fetches; der non-JSON-Defekt ist in `src/archivar/port.rs::ci_body_verdict` geheilt |
| NDK-Feld-Entscheidung — welcher Skalar eines Moment-Tensor-Events in die 9 Kraft-Medien eingeht | Future Operator-Queue (`state/future/handover/`) | mountain folge191/192 | der GCMT-NDK-Parser (`src/archivar/ndk.rs`) lebt, der reverify-Sweep überspringt `format == "ndk"` (`src/archivar/main_flow.rs:1462`), und `extract()` trägt keinen `ndk`-Arm (`format-void`). Frage: `mw` am Centroid, `m0`, oder kein Skalar? |
| LLNL_G3D_JPS/S40RTS `volume.bin` — falsche `origin`-Direktive in `sources.φ` | `docs/handover/handover-2026-09-28-mycelium-folge193.md` | mountain folge193 | die `LLNL_G3D_JPS.volume.bin`- und `S40RTS.volume.bin`-Blöcke in `phi/sources.φ` tragen die EMC/AFRP-Sammel-`origin`; die Manifestation `.github/workflows/volume-cdn.yml` holt korrekt `https://media.githubusercontent.com/media/tom-new/tomography-models/main/<name>.nc`. Myceliums Feder (Manifestations-Direktive). |
| Register↔CDN-Tag-Drift (Copilot-Punkte: spectra · nvss · first14 · curated48 · twomass_psc · jwst_spectra) | `docs/handover/handover-2026-09-28-mycelium-folge193.md` | mountain folge194 (Check) | **Kein Quellen-Identität-Riss** (gemessen 2026-09-28 via `archive_search --verdict` gegen Baum+Compiler): Register-`origin`/`compiler` stimmen mit den Produzenten — `tap_compiler`→`tapvizier.cds.unistra.fr`, `spectral_compiler`→`ncei.noaa.gov`, `jwst_spectra_compiler`→`exoplanetarchive.ipac.caltech.edu`, `vlies_density_compiler`→`ssd.jpl.nasa.gov-vlies`. Der Riss ist die **Manifestations-/Tag-Direktive** (Myceliums Feder): (a) die 4 registrierten Quellen sind 404 unter ihrem Tag, das Asset liegt legacy unter `ssd.jpl.nasa.gov` (je 206) → Re-Manifest dispatch; (b) `twomass_psc.bin` liegt unter `irsa.ipac.caltech.edu` (206), Produzent lädt `ssd.jpl.nasa.gov-vlies` (404); (c) `jwst_spectra.bin` liegt unter `ssd.jpl.nasa.gov` (206), Produzent `exoplanetarchive…` (404), Watchlist `ssd.jpl.nasa.gov`. `jwst_spectra.bin`/`twomass_psc.bin` tragen keine Register-Zeile (Compiler-Intermediates; die Quelle `curated48_spectra.bin` ist registriert) — Admission unberührt. |

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
