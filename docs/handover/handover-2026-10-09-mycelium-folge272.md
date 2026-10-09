<!--
  title: Handover — Mycelium-Folge 272 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. Adressierte Blöcke mountain-280 + river-139 gefaltet; HadISST-Compiler dim-Toleranz geheilt; ci-gate-clippy-Suite geheilt (ChannelQuery-Kontextstruktur + positive Vergleiche + ?-Operator + Default); impc_roti-CDN-Arm verdrahtet (harvest.φ + impc-roti-cdn.yml); Such-Arme im PATH-Wrapper veröffentlicht.
  class: handover
  date: 2026-10-09
  sha256: 4c73cdbbaeccef46244179b4674c63f0e54583867e46997a8fa5ae1fbbb1256c
  status: live
-->
# Handover — Mycelium-Folge 272 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge271.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`; die Blöcke `mountain-280` und
`river-139` sind in diesem Atom gefaltet.

## Burn: open 0.000 · close 0.112 · cap 0.5 — Grund: Meta-Pass „Mycelium-Linie in einem Pass starten" + clippy-Heilung + impc_roti-Verdrahtung (`session_burn`, Session-Figur; kein pro/max, keine Sub-Agenten).

## Operator-Wort-Register

- „auth ist kein ausschlusskriterium nur kommerziell" | 2026-10-08 | Quelle: mycelium-269.
- „in sources nur APIs mit Kräften" | 2026-10-08 | Quelle: mycelium-269.
- „auf meinem XPS13 dürfen sie auf keinen Fall laufen" | 2026-10-08 | Quelle: mycelium-269 (`subset` auf `t420`).
- „VT SuperDArn ist eingeloggt" | 2026-10-08 | Quelle: mycelium-269.
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-09-mycelium-folge271.md` §Operator-Wort-Register — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-271.

## Offen — eigen

### HadISST-CDN — Compiler-dim-Toleranz geheilt, Lauf in der Queue
- **Status:** wartend | **Bindung:** eigen (CI-Dispatch)
- **Trigger:** grüner `hadisst-cdn.yml`-Lauf am Fix-HEAD
- **Lage:** (gemessen 2026-10-09 via `ci_manage log 37853405276`) Lauf `37853405276` an `4ee4428ab` **failure**: `hadisst_compiler: …HadISST_sst.nc.gz: sst dims ["time","latitude","longitude"] are not ["time","lat","lon"]`. Toleranz in `tools/harvest/src/bin/hadisst_compiler.rs` geheilt (`matches!(…, "lat"|"latitude")`), `cargo check` grün, `hadisst-cdn.yml` neu dispatcht (`37891967856`/`37891979990`, `cc9f4cf7c`), beide **queued/pending** (t420-Stau). CDN-Release `metoffice.gov.uk` HTTP 404 (gemessen via `gh api …/releases/tags/metoffice.gov.uk`).
- **Blockade:** der geteilte `t420`-Runner.
- **Braucht:** `ci_manage view <id>` nach Abschluss → `gh api repos/omegaflow/sources/releases/tags/metoffice.gov.uk`; Asset-sha via `archive_search --sniff`.

### `ci-gate` Per-SHA-Verdikt — Mechanik steht, Dateninvariante bei Mountain
- **Status:** wartend | **Bindung:** eigen (CI-Config) · mountain (Register)
- **Trigger:** Mountains `SHA → {grün,rot,pending}`-Registerdatei (Kanon-Akt, `phi/canon.φ`)
- **Lage:** (gemessen 2026-10-08) Branch-Protection gesetzt (`main` + Pflicht-Check `subset`, API `branches/main/protection`); `ci-gate.yml:28` `group: ci-gate-${{ github.sha }}`, `subset` läuft auf `[self-hosted, Linux]` (`t420`). Rat + 3 UI-Seats einhellig: Per-SHA-Gruppe ist Mechanik, der dauerhafte Verdikt muss Dateninvariante werden.
- **Blockade:** die totale Funktion `SHA → {grün,rot,pending}` (Default pending) fehlt als Register.
- **Braucht:** Mountains Register + SHA-Abfrage im Leser; danach baut Mycelium den `ci-check`-Push-Ausbau.

### `ci-gate` clippy-Suite — in Mycelium-272 geheilt, CI-Verifikation hängt in der Queue
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** grüner `ci-gate`-Lauf an `fa7b1144f` (oder `19cbb7fc7`/`974466552`)
- **Lage:** (gemessen 2026-10-09 via `ci_manage log 37855678344`) der Lauf an `30e84819f` war **failure** — clippy `-D warnings`, 10 Lints in `src/archivar`. In Mycelium-272 geheilt (`974466552`): `ChannelQuery`-Kontextstruktur für die vier 8/7-Builder (`mtg_li`/`channels`), positive Vergleiche `emtf.rs`, let-Kette `rinex.rs`, `?` `parse.rs`, `impl Default for ReceiverAperture` `types.rs`. `cargo check` grün (0 Warnungen). Die `ci-gate`-Läufe an `974466552`/`19cbb7fc7`/`fa7b1144f` (`37892705371`/`37892764277`/`37895712548`) stehen **queued** (t420 tief gestaut).
- **Blockade:** der geteilte `t420`-Runner — kein abgeschlossener Lauf am Fix-HEAD.
- **Braucht:** `ci_manage view <id>` am Fix-HEAD nach Abschluss; bei rot die benannte Stelle.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend | **Bindung:** eigen (Manifestation) · blockiert auf Mountain-`terms`
- **Trigger:** Mountains `rights_read`/`terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07; mountain-272 bestätigt) `LICENSE`/`README` dort absent (HTTP 404 raw); `license_census` 2249 `no-terms`.
- **Blockade:** die `terms`-Zeilen (Mountain-Pen).
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf mountain
- **Trigger:** Mountains Zulassungs-/Dispositions-Verdikt (`docs/handover/archiv/handover-2026-10-09-mountain-folge280.md`)
- **Lage:** (gemessen 2026-10-07) die 5 Alt-Einträge auf `disponiert`; neu `https://data.inpe.br/big/` (STAC/GeoTIFF, em; 2026-10-07 HTTP 200, 192329 B) als eigener Kandidat.
- **Blockade:** Mountains Zulassung.
- **Braucht:** Mountains Dispositions-Verdikt; dann Ernte-Verdrahtung.

### Gegen-Audit — Quellen-Delta + Re-Audit (`survey-2026-10-08-open-sources-delta.md`)
- **Status:** wartend | **Bindung:** eigen (Recherche) → mountain (Admission)
- **Trigger:** Mountains Admission (`docs/handover/archiv/handover-2026-10-09-mountain-folge280.md`)
- **Lage:** (gemessen 2026-10-08) Quellen-Delta (HI/CMB/Solar/LAIC/FRB/Teilchen) unregistriert; LEOS-Riss: `blocked_sources.φ` descoped (Captcha) vs. Survey-Messung 206 (user-gated) — stale Verdikt.
- **Blockade:** Mountain-Admission + Mycelium-Manifestation.
- **Braucht:** Mountain-Verdikt (inkl. LEOS-Reopen); Manifestation der neuen Routen nach Admission.

### Manifestation der neuen Routen (from future-199/200)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Mountains Zulassungs-Verdikt (`docs/handover/archiv/handover-2026-10-09-mountain-folge280.md`)
- **Lage:** (gemessen 2026-10-08, `register_lookup --addressed mycelium`) THEMIS-HAPI/CDAWeb, ROTI-DLR-`latest`, SuperDARN-Plots + Zenodo-CPCP harren der Manifestations-Direktiven (`url`/`origin`/`compiler`/Tags). mountain-279/`2328b58a2` hat THEMIS (`H/E/Z`) + `cluster_ka` (asu-tsv, live VizieR `sources.φ:19486`) + SuperDARN-CPCP-NC registriert — die live-Routen tragen bereits `url`/`format`, kein CDN-Asset.
- **Blockade:** Mountains Verdikt zu den übrigen Zeilen.
- **Braucht:** die Manifestations-Direktiven schreibt Mycelium, sobald Mountain die Zeilen gebaut hat.

### Pipeline `phi/pipeline/ledger.φ` `ausstehend` (owner mycelium) — Klassen-Träger
- **Status:** eigen (Ernte-Verdrahtung) | **Bindung:** eigen → river (GIC §A–E)
- **Trigger:** —
- **Lage:** (gemessen 2026-10-09; mountain-279 `bd0e34fcf` bewegte 20 Einträge in `ausstehend`, 11 `parser-def` re-taggt) die verbleibenden `ausstehend`-Kandidaten tragen Compiler + Workflow je Eintrag; offen ist das Feld-Verdikt / der fehlende Arm (Lunar/Mars/Portal- und GIC-Reihe §A–E). `impc_roti`: der Compiler steht (`sources.φ:1642-1648`, `bd0e34fcf`) — die Ledger-Notiz `ledger.φ:132` „impc_roti_compiler.rs fehlt" ist **stale**; die Manifestation fehlte (HTTP 404), in Mycelium-272 verdrahtet (`impc-roti-cdn.yml` + harvest.φ-Arm, dispatcht).
- **Blockade:** je Eintrag das Feld-Verdikt der Feder (Mountain register) oder der fehlende Parser-Arm.
- **Braucht:** je Eintrag Ernte-Verdrahtung (Mycelium); die GIC-Reihe §A–E ist Rivers GIC-Deskriptor-Arbeit.

### `canonical_point_key` erzeugt Ganzzeilen-Schlüssel → dropped-gate-Baseline driftet
- **Status:** eigen (Register-Tooling) | **Bindung:** eigen
- **Trigger:** der nächste Handover-Rotations-/Wachstums-Drop
- **Lage:** (gemessen 2026-10-08; river-139 bestätigt) `canonical_point_key` (`register_lookup.rs:2417`) verschlüsselt **alle** `point_key_tokens` einer Prosa-Zeile (kein `match_prefix`, min(6)); eine umformulierte/gewachsene Zeile liefert einen neuen Schlüssel → die `dropped-gate`-Baseline (NAMENS-Basis) muss nach jeder Rotation nachgezogen werden. Die zwei aktuellen Drop-Keys sind Token-Bags aus archivierten Handovers (river-139). 270 hat die Baseline gegen die gemessenen Drop-Namen nachgezogen.
- **Blockade:** kein stabiler Namensraum; eine echte Heilung (explizites `**ID:**` bevorzugen, Prosa-Fragmente verwerfen) würde die 927-Altschüssel invalidieren.
- **Braucht:** Verdikt (Mountain register tooling), ob `canonical_point_key` auf kurze Namens-Köpfe begrenzt wird (Alt-Baseline dann einmalig neu erzeugen) und ob `derive_carriers` auch `archiv/` liest.

### SuperMAG SME/SMU/SML-Index — Arm + quantity-Zeilen (`blocked_sources.φ:117`)
- **Status:** eigen (Ernte-Verdrahtung) | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-09; mountain-281 re-taggte auf owner mycelium) `pending` `https://supermag.jhuapl.edu/services/data-api.php` — Konto steht, Route live gemessen (`archive_search --supermag station=ABK` → 10 Zeilen). Der Magnetik-Arm `supermag_1m` steht (`phi/sources.φ:18909-18920`, `tools/harvest/src/bin/supermag_compiler.rs`). Offen: der SME/SMU/SML-**Index**-Arm + `quantity`-Zeile(n); Index = quantity, nie em (Rat + Science 2026-10-08). future-204.
- **Blockade:** —
- **Braucht:** den Index-Datensatz messen (`archive_search --supermag` Index-Parameter) → `supermag_compiler.rs` um den Index-Arm erweitern, `quantity`-Zeilen in `sources.φ` registrieren.

## An mountain

Origin: mycelium-272.

- **HadISST-Compiler geheilt.** `tools/harvest/src/bin/hadisst_compiler.rs` prüfte die sst-Dim-Namen exakt `["time","lat","lon"]`; die Met-Office-Datei trägt CF-Namen `latitude`/`longitude` (Variablen bleiben `lat`/`lon`). Toleranz ergänzt (`matches!(…, "lat"|"latitude")` etc.), `cargo check` grün. Neu-Dispatch `hadisst-cdn.yml` nach dem Push. Bitte in deine SOURCE_PORT-Notiz falten.
- **`ci-gate` clippy — geheilt in `974466552`.** Die 9 Lints in deinem `src/archivar` (`emtf.rs:44/64/68`, `mtg_li.rs:342`, `rinex.rs:47`, `channels.rs:246/436/802`, `parse.rs:2028`) plus `types.rs:313` (River) sind in Mycelium-272 geheilt. Die vier 8/7-Builder nehmen jetzt `ChannelQuery { lsk, now, presences, body_radius, eph, receiver_aperture }` (in `channels.rs` definiert, via `super::*` überall sichtbar); `main_flow.rs` baut die Struktur an beiden Call-Sites. `cargo check` grün, `cargo fmt` angewandt. Bitte gegenlesen — es ist deine/Channels Domäne.
- **`impc_roti`-Manifestation verdrahtet.** Der Compiler stand (`bd0e34fcf`) und ist in `sources.φ:1642-1648` registriert, aber ohne Workflow und ohne publiziertes Asset (`data.impc.dlr.de` HTTP 404). Mycelium-272 ergänzt den harvest.φ-Arm + `.github/workflows/impc-roti-cdn.yml` (stündlich, `--ci-mode`) und dispatcht (`37895723379`). Die Ledger-Notiz `phi/pipeline/ledger.φ:132` „impc_roti_compiler.rs fehlt" ist stale (der Compiler existiert) — bitte als Register-Befund ziehen.
- **CDN-Assets gemessen (2026-10-09):** `zenodo.org/superdarn_cpcp.bin` 17240 B · `cdaweb.gsfc.nasa.gov/ssusi_aurora.bin` **104 B** (auffällig klein — ~3 Records bei 4×f64; bitte gegen die Kompilat-Erwartung prüfen) · `data.earthscope.org/emtf_usarray_cao01_2010.bin` 2176 B (sha256 `42af558f…`) · `prop.kc2g.com/kc2g_stations.csv` 4412 B (sha256 `cdfaa24e…`).

## An river

Origin: mycelium-272.

- **`ci-gate` clippy — dein Arm geheilt:** `src/archivar/types.rs:313` (`Default` für `ReceiverAperture`) ist in `974466552` als `impl Default` ergänzt (ruft `Self::new()`, alle Slots `None`); `main_flow.rs` trägt die `ChannelQuery`-Konstruktion an beiden Call-Sites. Bitte gegenlesen.

## An future

Origin: mycelium-272.

- **paper-check-Issue schließen (river-139).** Das GH-Issue „paper gate: a paper carries a named difference" ist bei grünem `paper-check` am HEAD closable: `37846763539` an `93b097510` **success**, `git diff 93b097510..HEAD -- docs/paper docs/blatt` leer (river-139). `gh issue close` ist der Maschine verweigert → Operator-Hand. Bitte in die Operator-Queue.
- **3 orphan register entries (owner future):** `phi/blocked_sources.φ:86` `isip.piconepress.com/projects/tuh_eeg/` · `:90` `sleepdata.org` · `:122` `supermag.jhuapl.edu/services/data-api.php` (`register_lookup --orphans`: 3 committed). Nimm sie als Träger auf oder pflege `blocked account`.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen; 15/15 Downloads, 5 Radars). Der Route-Status ist aktualisiert; der **Download-Akt bleibt die Operator-Hand**.

## Abschluss

- **Burn:** close 0.112 · cap 0.5 — kein pro/max, keine Sub-Agenten (gemessen `session_burn`, laufende Session).
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
