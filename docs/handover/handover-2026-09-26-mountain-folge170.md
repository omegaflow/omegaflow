<!--
  title: Handover — Mountain-Folge 170 (Stand 2026-09-26)
  session: Mountain-Folge 170
  class: handover
  date: 2026-09-26
  sha256: ce5ef0647e3f625840997d7fa68140f6758b77829e7c300dd6f99588d1f97674
  status: live
-->
# Handover — Mountain-Folge 170 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Sortierung: erst Akteur (Linie | Rat | Operator |
Dritter), dann in keiner Rangfolge. Jeder Punkt aufgeschlüsselt:
Trigger / Lage / Blockade / Braucht.

## Stehender Pass

- **Postfach:** (gemessen 2026-09-26) `state/` führt keine Dateien, `state/mail/mail_ledger.φ` absent → kein Eingang.
- **CI-Status am HEAD `3eaa60de7`:** (gemessen 2026-09-26 via `ci_manage` + Browser) **rot** — der
  E0583 `file not found for module llnl_g3d` ist behoben (die Datei ist committet, `6670a601b`);
  `ci-check` `36244460936` bleibt rot an einem `clippy -D warnings`-/`format`-Schwall über den
  ganzen Core (`extract.rs`×8, `port.rs`×6, `main_flow.rs`×2, `spatial.rs`, `uws.rs`, `te.rs`×3,
  `atdf.rs`, `cometels.rs`, `llnl_g3d.rs`, `skydirection.rs`). Mountain-Anteil gefixt:
  `parquet.rs`, `grib2.rs`, `ionocal.rs`, `voyager_occlt.rs` + die drei Probes unter `tools/measure`.

## Offen (aufgeschlüsselt)

### Linie handelt (eigen)

#### Planck-SZ-Query — Register-Akt nach Spaltenkorrektur
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** `phi/blocked_sources.φ` frei (fremd-dirty).
- **Lage:** (gemessen 2026-09-26 via grind-pro) korrigierte ADQL
  `SELECT TOP 5000 name,ra,dec,snr FROM planck.com_pccs2_sz_mmf3` gegen
  `https://irsa.ipac.caltech.edu/TAP/sync` → HTTP **200**, echte PSZ2-SZ-Zeilen;
  18/18 Spalten bestätigt; Verdikt **register** — kein descope. **Arm gebaut:**
  `tools/harvest/src/bin/planck_psz2_compiler.rs` (CSV, header-getrieben, 1271 Zeilen live;
  Distanz-Crossmatch `com_pccs2_sz_union.redshift` über `name` → **925/1271** mit `z`,
  Sentinels `-1.0` → absent); `cargo check`/`cargo test` 0/0. Wire-fertiger Block für
  `phi/sources.φ` + `blocked_sources.φ`-Ersetzung liegen in `/tmp/opencode/planck-register-block.φ`
  (`at sun`/`cmap .`/`ra ra`/`dec dec`/`z z`, kein `field` — `snr` dimensionslos).
- **Blockade:** `phi/blocked_sources.φ` ist fremd-dirty (`MM`; mycelium Lasair/
  SuperDARN/sensor.community/GOSAT) → Eintrag `:333` nicht sicher überschreibbar; der
  `sources.φ`-Block darf erst mit der `blocked`-Ersetzung atomar landen.
- **Offen ungemessen:** die Blockform des Artefakts (`at sun`/`cmap .`/`z z`) ist gegen den
  `cmap`-Vertrag in `src/archivar/extract.rs` **nicht verifiziert** — beim Anwenden gegen eine
  echte Zeile prüfen (der Arm selbst lief live: 1271 Zeilen, 925 mit `z`).
- **Braucht:** `/tmp/opencode/planck-register-block.φ` anwenden, sobald `blocked_sources.φ`
  frei ist (Block nach `sources.φ`, `:333`-Eintrag → `pending`).

#### CI-Verify TAP/Parquet/GRIB — wartet auf grünen ci-check
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `ci-check` am HEAD (Mountain-Anteil grün).
- **Lage:** (gemessen 2026-09-26) die vier TAP-Blöcke (`gavo.aip.de:7458`,
  `padc-tap-rcsed:7468`, `voparis:7480`, `skvo:9528`) korrigiert und committet; Parquet-Arme
  (`parquet.rs`) + GRIB-2-Arme (`grib2.rs`) gebaut, `cargo check`/`--tests` 0/0. Der E0583
  (`mod llnl_g3d`, Owner sensory) ist behoben (`6670a601b`); Mountain-eigene clippy-Lints
  (`parquet.rs:1079/1180`, `grib2.rs:594`, `ionocal.rs:303/411`, `voyager_occlt.rs:269`) +
  `grib2.rs`-Format gefixt. `ci-check` `36244460936` bleibt rot an **fremden/shared** Lints
  (`extract.rs`, `port.rs`, `main_flow.rs`, `spatial.rs`, `uws.rs`, `te.rs`, `atdf.rs`,
  `cometels.rs`, `llnl_g3d.rs:431`, `skydirection.rs:200`).
- **Blockade:** `clippy`-/`format`-Lints in fremdem Core: `extract.rs`+`uws.rs` (mycelium),
  `spatial.rs`+`main_flow.rs` (river), `te.rs`+`cometels.rs`+`skydirection.rs`+`llnl_g3d.rs` (sensory).
- **Offen ungemessen:** der Mountain-Anteil (format/clippy) ist lokal `cargo check`-rein, die
  **clippy-Wirkung ist CI-ungemessen** (lokales `cargo clippy` verweigert).
- **Braucht:** nach grünem `ci-check` die `epoch obs_time mjd`-Direktive (ogle) bestätigen und
  Parquet/GRIB-Tests als gelesen abhaken; die fremden Lint-Dateien sind an ihre Owner getragen
  (sensory/river) bzw. für mycelium hier benannt (deren Handover ist fremd-dirty).

#### Harvest-Assets — `asset fehlt` (OGIMET/NOHRSC/HAMQSL)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ogimet-cdn` 36242356691 / `nohrsc_snowfall-cdn` 36242617755 / `hamqsl-cdn`.
- **Lage:** (gemessen 2026-09-26) der OGIMET-Parser ist **nicht** defekt — der Header trägt
  lat/lon (40-47-59N / 124-10-00W / 13 m); die CI-Meldung „no latitude/longitude" stammt aus
  einem OGIMET-Soft-Block (HTTP 200, 25 B `Status: 403 Forbidden`, kein `<h4>`). Ehrliche Meldung
  `no report header ({n} B): "Status: 403 Forbidden"` in `tools/harvest/src/bin/ogimet_compiler.rs`
  gebaut (`cargo check` 0/0); `ogimet-cdn` re-dispatcht. `phi/harvest.φ:95/145/164` offen.
- **Blockade:** keine — der Soft-Block ist intermittierend.
- **Braucht:** Lauf lesen; nach Manifestation die drei `asset fehlt`-Zeilen schließen;
  `gh workflow run nohrsc_snowfall-cdn.yml` (falls nicht gelaufen).

#### gll.rss — ODR-Shard fehlt, übrige shas nachgetragen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gll-rss-odr-cdn` 36242358527.
- **Lage:** (gemessen 2026-09-26) `gll_rss_odr.bin` → **404**, Lauf-Vorlauf failure
  (Runner-Shutdown). `gll_rss_atdf.bin` (710 527 112 B, `b762373c…`), `gll_rss_atdf_x.bin`
  (13 000 B, `e63bc3ea…`), `messenger_tnf.bin` (193 905 440 B, `04211c8e…`),
  `gll_rss_rsr.bin` (**590 720 008 B**, `baee1a81…`) gemessen und in `phi/sources.φ` als
  `sha256`-Direktive nachgetragen. Riss zur Alt-Zahl: die frühere Größe 85 956 014 B war ein
  Partial-Download, nicht der Asset.
- **Blockade:** keine.
- **Braucht:** nach odr-Manifestation den sha in den `sources.φ`-Block `:8431` nachtragen.

#### Atom D — Beat-Paar, Kandidat gemessen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-26 via research-max + grind-flash) Bau steht
  (`odf.rs::tnf_phase_series`, WGSL `beat_pair`). Zwei-Station-Kandidat **gefunden**:
  EHT 2017 `2016.1.01404.V`, **Baseline ALMA (`Aa`)–APEX (`Ap`)** im Full-Array-uvfits,
  Maser-Zeitbasis; Fringe-Rate `df` → Gate `df·dt<0.5` plausibel. Lizenz **ODC-PDDL 1.0**.
  Direkter Abruf über `almascience.org/almadata/ec/eht/2016.1.01404.V/` (HTTP 200, 302 → ESO;
  kein Login/Key). Beispielmodul `e17a10-7-hi-na-1921-293-fits.tgz` **2,1 GB**,
  sha `ebff01cc…` (`.sha256sums` Z. 11). CyVerse-Weg nur über Proton-Exit (WAF-Interstitial).
- **Blockade:** keine — ALMA direkt offen.
- **Braucht:** Modul laden + **ALMA–APEX-Baseline aus dem uvfits selektieren** (neuer
  Atom: uvfits-Reader, Aa–Ap-Baseline → `df`), dann durch `beat_pair`/`tnf_phase_series`.

### Operator handelt (operator-gebunden)

#### `epochrange` Wire-Slot (MJD-Breite)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort.
- **Lage:** (gemessen 2026-09-26 via grind-pro) Entwurf fertig: Slot 20 (`r_eq`) als Kopf des
  Mess-Blocks; Einheit TDB-Sekunden; `0.0` = Punktmessung (null-echt); im WGSL ungenesen →
  null GPU-Wirkung. Änderungsfläche `types.rs:41-62` + `:183-199`, `parse.rs:1053-1058`,
  `extract.rs:4406`, `spatial.rs:495` + `:613`, `constants.js:112/143`; Record bleibt 26×f64.
- **Blockade:** Architektur-Akt (Wire-Version) → Operator-/Rat-Wort.
- **Braucht:** Operator-Wort; danach Implementierung nach dem Entwurf.

#### Chrome-DevTools-MCP
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort.
- **Lage:** (gemessen 2026-09-26) MCP antwortet `-32001 timeout`; Vorbereitung in
  `docs/concepts/tools-map.md:294-297` (Flags `--no-usage-statistics --no-performance-crux`).
- **Blockade:** Operator-Wort (Debugger-Rechte am live Chrome/Neustart).
- **Braucht:** Operator-Wort; dann Chrome mit den Flags starten und MCP anbinden.

#### S3-Scheme (Token)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (bzw. Token).
- **Lage:** (gemessen 2026-09-26) SigV4+Handshake gebaut; `blocked key` (alter 401 = Register-Umbuchung).
- **Blockade:** Operator-Wort (Token).
- **Braucht:** Token bzw. Wort; danach die Route freischalten.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
