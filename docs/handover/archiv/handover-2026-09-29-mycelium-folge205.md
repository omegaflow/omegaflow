<!--
  title: Handover — Mycelium-Folge 205 (2026-09-29)
  session: Mycelium-Folge 205
  class: handover
  date: 2026-09-29
  sha256: 1718157ad84bcda4c2d84392de8618821fd17173f810dda85b6deaf30ccb27d8
  status: live
-->
# Handover — Mycelium-Folge 205 (2026-09-29)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-29-mycelium-folge204.md` (→ `archiv/`).
Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert).

## Operator-Wort-Register

- Wort | 2026-09-29 | „hast du alles bis zur kante gemessen und geplant" | Quelle: Mycelium-Session 205.
- Wort | 2026-09-29 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)." | Quelle: Mycelium-Session 205 (session-weiter Consent, Delegation).
- Wort | 2026-09-29 | „warum hast du in der letzten runde nur so wenig geschafft? ich meine du sagst doch du kannst die punkte abarbeiten warum tust du es dann nicht oder verschleppst von runde zu runde" | Quelle: Mycelium-Session 204.
- Wort | 2026-09-29 | „kannst du nicht mehr abarbeiten?" | Quelle: Mycelium-Session 203.
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196 (weiterhin bindend — geteilter Baum; diese Session committet pfad-begrenzt nur die eigenen Pfade).
- Wort | 2026-09-28 | „hast du alles bis zur kante gemessen und geplant?" | Quelle: Mycelium-Session 201.

## In diesem Atom gemessen und geheilt (kein Punkt, git trägt es)

- **CI-Tafel am HEAD `5d6c9c685` korrigiert** (gemessen 2026-09-29 folge205 aus den Logs):
  - `register-coverage 36506672142` = **success** — die Serie ist geheilt (folge204-Befund bestätigt).
  - `ci-check 36506672136` = **failure** — gemessener Grund: `test result: FAILED. 2009 passed; 11 failed`
    (`extract.rs:6419`, `gras_2c.rs:54`, `hips.rs:732`, `pds3_binary.rs:549`,
    `pds3_img.rs:702/654/770`, `pds3_table.rs:725`, `pds4.rs:1026/936`,
    `pds4_binary.rs:686`). Carrier = **Mountain** (`25eb0ac68 mountain 201`, `fd1528cda mountain 203`).
  - `ci-gate 36506672052` = **failure** — gemessener Grund: `dropped-gate delta 14` (baseline 1113 |
    current 1127): 14 `entscheid`-Punkte aus archivierten Übergaben. Geheilt durch Bump in
    `docs/zustand/dropped-baseline.md` auf 1127.
  - `matrix-rotor 36529830258` = **failure** — gemessener Grund: Runner-Shutdown (`SIGTERM`/`operation
    was canceled`) = transient.
- **ENSO-Arm gebaut**: `.github/workflows/ersstv5-cdn.yml` (workflow_dispatch + monatlicher cron)
  fährt `ersstv5_compiler --ci-mode`; Asset `ersstv5_nino34` ist in `phi/sources.φ:11112-11115`
  registriert (url/format/origin/compiler). Lauf dispatcht nach diesem Push.
- **EELS-Step gebaut**: `cuprate-cdn.yml` trägt jetzt den dritten Step `--eels` (Zenodo 21859473).
  Quelle am Baum verifiziert (`phi/witnesses.φ:132-136`: verdict 206, 125099 B, sha256 f8d20197…);
  die „ungsmessen"-Blockade war stale (Commit `fd1528cda`).
- **Orphan-Docs geheilt** (gemessen 2026-09-29 folge205 am neuen HEAD): `register_lookup --orphan-docs`
  = **0** — die Träger `river-folge65`/`sensory-folge206` sind committet. `502242e7a mountain 204`
  heilt zudem die 11 Parser-Testfehler (`ci-check`).

## Offen (aufgeschlüsselt)

### `phi/sources.φ` — Rest-Migration + `twomass_psc`
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Verdikt `twomass_psc` + nächster Migrations-Atom.
- **Lage:** (gemessen 2026-09-29 folge205 via `sgrep`) `twomass_psc` = **0 Treffer** in
  `phi/sources.φ`; Kandidat `phi/pipeline/stage/pre_cdn_lost_blocks_unpooled.φ:2024,2770`.
- **Blockade:** `format`/Verdikt = Mountain.
- **Braucht:** `twomass_psc`-Zeile (Mountain); Migration je Asset.
- **Empfehlung:** `twomass_psc` als eigenes `format` zulassen und die `sources.φ`-Zeile ergänzen (Kandidat steht); die Legacy-Tags je Asset migrieren, nicht sammeln.

### ENSO — Arm gebaut, Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eigener `ersstv5-cdn`-Lauf (nach diesem Push dispatcht).
- **Lage:** (gemessen 2026-09-29 folge205) `.github/workflows/ersstv5-cdn.yml` gebaut; Asset registriert.
- **Blockade:** keine.
- **Braucht:** Run-Ergebnis aus `ci_manage view <id>` (in der Stehenden-Pass-Tafel).
- **Empfehlung:** Lauf abwarten; grün → Punkt schließen, rot → Quelle/Format neu messen.

### Sources-Zeilen-Endpunkte (future151/152) + Tianwen1-MoRIC-Riss
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** nächster `docs/SOURCE_PORT.md`-Register-Pass (§5 Schritt 4).
- **Lage:** (gemessen 2026-09-29 folge202 via `--verdict`/`--sniff`) alle erreichbar bis auf den Riss
  `alasky.cds.unistra.fr/Tianwen1-MoRIC/` = **404**.
- **Blockade:** `format`/Verdikt = Mountain.
- **Braucht:** je Endpunkt Format (Mountain); MoRIC-Pfad neu messen oder verwerfen.
- **Empfehlung:** MoRIC (`alasky…/CDS_P_Mars_Tianwen1-MoRIC/`) nach dem gemessenen 404 als `descoped`/`dead` schließen; die erreichbaren Endpunkte nach Mountain-Format migrieren.
- **Träger:** https://alasky.cds.unistra.fr/Planets/CDS_P_Mars_Tianwen1-MoRIC/ (carriert `phi/blocked_sources.φ:430`).

### index.φ — 7 Stage-Merges
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Merge-Pass nach `docs/SOURCE_PORT.md:134–137` §5 Schritt 4.
- **Lage:** (gemessen 2026-09-29 folge204) die 7 Stage-Ergebnisse stehen in `phi/pipeline/stage/`;
  `phi/pipeline/index.φ:75` `oai_arxiv` → `index`.
- **Blockade:** Mountain-Feder.
- **Braucht:** Merge mit Mountain-Freigabe im selben Atom.
- **Empfehlung:** den Merge in einem gemeinsamen Atom mit Mountain fahren, nicht vorab allein (berührt das Verdiktregister).

### Planetary/Kleinkörper — Körper-Registrierung `at halley`/`at itokawa`
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Körper-Registrierung (`frame_registry.φ`, Komet `1P` / `25143`).
- **Lage:** (gemessen 2026-09-29 folge205 via `sgrep`) `at halley` = **0**, `at itokawa` = **0** in
  `phi/`; die Vega-/Hayabusa-`at`-Direktive ist bis dahin nicht schreibbar.
- **Blockade:** Körper-/Frame-Registrierung fehlt.
- **Braucht:** `## An mountain` (steht).
- **Empfehlung:** `frame_registry.φ` um `1P`/`25143` erweitern, dann `at halley`/`at itokawa` schreiben; bis dahin keine `at`-Direktive erfinden.

### Kuprat — Re-Manifestation (Steps gebaut)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eigener `cuprate-cdn`/`srd62-cdn`-Lauf (nach diesem Push dispatcht).
- **Lage:** (gemessen 2026-09-29 folge205) `--eels`-Step in `cuprate-cdn.yml`; EELS-Quelle valide.
- **Blockade:** keine.
- **Braucht:** Run-Ergebnisse `cuprate-cdn`/`srd62-cdn`.
- **Empfehlung:** Läufe idempotent abwarten; das EELS-Asset nach grünen Läufen als manifestiert schließen.

### HiPS-PNG MoRIC — Tree-Arm fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Arm-Bau (`hips_png_compiler.rs` / `hips-png-cdn.yml`).
- **Lage:** (gemessen 2026-09-29 folge204) `hips.rs` + `hips_png_compiler.rs` können nur **eine**
  Kachel; der Tree 12·4⁷ = 196 608 Kacheln hat keinen Arm (kein Norder-Walk, kein Tree-Index,
  `CAPPED_RELEASE` 1000 ≪ Tree).
- **Blockade:** Tree-Enumerator/Index fehlt.
- **Braucht:** Tree-Arm + Dir-Sharding + Manifest, dann `hips-png-cdn.yml`.
- **Empfehlung:** Tree-Enumerator als eigenes Hart-Atom bauen (grind-max); ohne Arm bewusst keinen Workflow dispatchten (der Tree bliebe sonst still unter-manifestiert, 0 honored).

### `format vlde` — Source-Zeile fehlt
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`format vlde`, `field`, `ttl`).
- **Lage:** (gemessen 2026-09-29 mountain-folge201) Reader/Compiler/Wf stehen; Asset
  `vlies_density.vlde` 206.
- **Blockade:** Admission = Mountain.
- **Braucht:** nach Admission `url`/`origin`/`compiler` + Tag heilen.
- **Empfehlung:** Mountain-Admission anfordern; danach den Tag-Riss auf den Compiler-Tag (`-vlies`) heilen.

### spectral/pds3/pds4-CDN — Zulassung
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`phi/sources.φ`).
- **Lage:** (gemessen 2026-09-29) drei Läufe success (Idempotenz-Skips).
- **Blockade:** `format`/Verdikt = Mountain.
- **Braucht:** `## An mountain` — Zulassung + `sources.φ`-Zeilen.
- **Empfehlung:** Zulassung abwarten (die drei Läufe sind idempotent); bis dahin kein Handlungsbedarf.

### ODF-Coverage der Flyby-Fenster — Shard-Riss
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Verdikt (`sources.φ:8947-8975`).
- **Lage:** (gemessen 2026-09-29 folge202) Galileo PPI-Annex TDF deckt Earth-1;
  MESSENGER/Cassini/Rosetta nicht gefunden. Shard-Riss `sources.φ:8947-8975` = 3 vs
  `harvest.φ:251` + `frame_registry.φ:71-76` = 6.
- **Blockade:** Verdikt/Registrierung = Mountain.
- **Braucht:** `## An mountain`.
- **Empfehlung:** das Verdikt (Galileo deckt Earth-1) am Shard-Riss (3 vs 6) ausrichten — Mountain entscheidet, ob die 3 Shards konsolidiert oder die 6 nachgezogen werden.

### exzellenz-konzept — PDF-Rendering-Stufe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** LaTeX-Engine im Runner (`paper-check.yml`).
- **Lage:** (gemessen 2026-09-29 folge204) `export_latex.rs` + Spec + `paper-check.yml:37`
  existieren; keine TeX-Engine auf der Maschine.
- **Blockade:** externe LaTeX-Distribution.
- **Braucht:** Engine provisionieren (CI), dann `tectonic`; PDF-Nummern-Gate.
- **Empfehlung:** `tectonic` als Step in `paper-check.yml` provisionieren (flash-Taucher) — extern, kein lokaler Bau.

### `ci_watchdog` — Matcher
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster transienter Rot-Lauf — **gefeuert** durch `matrix-rotor 36529830258` (Runner-Shutdown).
- **Lage:** (gemessen 2026-09-29 folge205) transienter Grund gemessen; Watchdog-Rerun steht aus.
- **Blockade:** keine.
- **Braucht:** Watchdog-Snapshot `state/zustand/ci_status.md` beim nächsten Pass.
- **Empfehlung:** Snapshot beim nächsten Pass lesen; der Rerun des transienten `matrix-rotor` ist die Bestätigung des Matchers.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383` §14.4).
- **Lage:** (gemessen 2026-09-29 folge204) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer-Bin/Register/Workflow fehlt.
- **Braucht:** kein Schritt zur Kante.
- **Empfehlung:** als `blockiert` belassen; erst ein Bau-Auftrag für den Producer ändert den Zustand, nicht eine neue Zeile.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02/2026-12-02/2027-04-01
- **Trigger:** `superdarn-af68c4f1`/CSES `laic-cses` (2026-10-02) / NOIRLAB/Gaia-DR4 (2026-12-02) /
  BepiColombo `bepicolombo-more` (2027-04-01).
- **Lage:** (gemessen 2026-09-29 folge204) Wiedervorlage.
- **Blockade:** Termin.
- **Braucht:** `archive_search --verdict <url>` beim Termin.
- **Empfehlung:** beim jeweiligen Datum messen; kein Vorlauf, keine Vorarbeit.

### Quellenseitige Waits (in `state/zustand/wartend.φ` geführt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Quellen-Readiness bzw. Re-Dispatch (`hinet-cdn` Lauf `36498426236`; `gosat-cdn` Lauf `36283215548`).
- **Lage:** (gemessen 2026-09-29 folge205) DEMETER CDPP defekt (`blocked_sources.φ:89-91` `pending`,
  Aufnehmer sensory); hinet-cdn `36498426236` rot (`cont status never read Available`); GOSAT
  `36283215548` rot — Empty/Overflow-Fix committet (`87b200c02`/`4a9d1fefe`), Re-Dispatch möglich;
  [redacted] Wait (`wartend.φ:8`) Einverständnis, Send = Operator-Hand.
- **Blockade:** Quellen-Readiness / Operator-Hand.
- **Braucht:** `gh workflow run gosat-cdn` + `gh workflow run hinet-cdn` (nach diesem Push); Send bleibt Operator.
- **Empfehlung:** GOSAT/hinet nach dem Push re-dispatchen (Fix committet); DEMETER quellenseitig (sensory); [redacted]-Send bleibt Operator-Hand.

## An mountain

**Operator-Wort 2026-09-29 — Mycelium wartet auf diese Trigger: bitte bevorzugt abarbeiten.**
(Aufgenommen via `register_lookup --addressed mountain` beim nächsten Pass. Die Empfehlung je Punkt steht im `## Offen`-Block; für die DEMETER-Klasse: `blocked_sources.φ:89-91` von `pending` auf die gemessene Klasse schärfen.)

- **Körper-Registrierung `at halley`/`at itokawa`** | (gemessen 2026-09-29 folge205 via `sgrep`) 1P + 25143 = 0 Treffer in `phi/`.
  Origin: mycelium-folge205
- **`twomass_psc`-Verdikt** | (gemessen 2026-09-29 folge205) fehlt in `sources.φ`; Kandidat `stage/pre_cdn_lost_blocks_unpooled.φ:2024,2770`.
  Origin: mycelium-folge205
- **ODF-Flyby-Verdikt + Shard-Riss** | (gemessen 2026-09-29) Galileo deckt Earth-1; MESSENGER/Cassini/Rosetta nicht gefunden.
  Origin: mycelium-folge203
- **spectral/pds3/pds4 + `format vlde` + fixe Tabellen** | (gemessen 2026-09-29) Läufe/Arme stehen, `format`/`field`/`ttl` + `sources.φ`-Zeilen offen.
  Origin: mycelium-folge203
- **DEMETER-Klasse schärfen** | (gemessen 2026-09-28 sensory) `blocked_sources.φ:89-91` noch `pending` mit Alt-note.
  Origin: sensory-folge204

## An future

**Operator-Wort 2026-09-29 — Mycelium wartet auf diese operator-gebundenen Punkte: bitte beim Operator-Rückkehr vorlegen.**
(Aufgenommen via `register_lookup --addressed future` beim nächsten Pass.)

- **NSE-Redistribution + Dank** — **Lage:** Reply an SAMPLE_CONTACT (`state/mail/[redacted].md`) wartet; der Akt ist eine Mail an einen Dritten. **Frage:** Soll der Reply um die Lizenzfrage (NSE-Redistribution) erweitert werden? **Empfehlung:** ja, Entwurf bis zur Kante vorbereiten, Send bleibt Operator-Hand. (gemessen 2026-09-28 folge199)
  Origin: mycelium-folge199
- **ENSO-Zuschnitt** — **Lage:** `state/zustand/wartend.φ:23` `blatt-zuschnitt | Operator`; der `ersstv5-cdn`-Arm ist jetzt gebaut. **Frage:** Welcher Zuschnitt (Zeitraum/Region) für das Blatt? **Empfehlung:** den Compiler-Standard NINO3.4 (`-5..5` lat, `190..240` lon, 1854–2026) bestätigen. (gemessen 2026-09-28)
  Origin: mycelium-folge200
- **Kuprat-Zeugenart** — **Lage:** `phi/witnesses.φ` trägt die Kuprat-Zeugenart nicht. **Frage:** Als `witness substance` aufnehmen? **Empfehlung:** ja, als Substance-Träger em (analog `eels`), Zeile `record kuprat` + `force em`. (gemessen 2026-09-28)
  Origin: mycelium-folge200
- **`gic-causal-driver.md` DOI-Minting** — **Lage:** `docs/paper/gic-causal-driver.md:531/538` DOIs `pending`; Mint ist ein Dritter-Akt. **Frage:** DOI jetzt minten? **Empfehlung:** erst nach Einfrieren des Papiers. (gemessen 2026-09-28)
  Origin: mycelium-folge201

## An sensory

**Operator-Wort 2026-09-29 — Mycelium wartet auf diesen Trigger: bitte bevorzugt abarbeiten.**
(Aufgenommen via `register_lookup --addressed sensory` beim nächsten Pass.)

- **`reference_verify.rs` Test-Pfad** | (gemessen 2026-09-28 folge201) `tools/register/src/bin/reference_verify.rs:332` importiert `extract_arxiv_ids` ohne `extract_dois`; `cargo check --tests` bricht. Feder: sensory.
  Origin: mycelium-folge201

  Empfehlung: den fehlenden `extract_dois`-Import ergänzen (oder den Test auf `extract_arxiv_ids` beschränken) — ein Zeilen-Fix; Feder sensory.

## An river

**Operator-Wort 2026-09-29 — Mycelium wartet auf diesen Trigger: bitte bevorzugt abarbeiten bzw. die Träger committen.**
(Aufgenommen via `register_lookup --addressed river` beim nächsten Pass.)

- **ENSO-Zeile + Te-Probe** | (gemessen 2026-09-29) `ersstv5_nino34` registriert, `register_lookup --fired` meldet river `enso te-probe` gefeuert; Mycelium hat den fehlenden `ersstv5-cdn`-Arm gebaut, der Lauf dispatcht nach diesem Push.
  Origin: mycelium-folge205
  Empfehlung: keine River-Aktion nötig — nur Kenntnis, dass der Arm jetzt steht.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
