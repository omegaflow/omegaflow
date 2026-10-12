<!--
  title: Handover — Mycelium-Folge 320
  session: Mycelium-Linie in einem Pass (Mariner-sha256-Zeilen geschrieben, Vlies-Reihe re-gemessen, adressierte Blöcke mountain-322/future-240 gefaltet, Register-Ask gemessen)
  class: handover
  date: 2026-10-12
  sha256: b5265ab833a562941f17bc9fcd37a8c19068cb29fe20a9e8388a68ceba781f66
  status: live
-->
# Handover — Mycelium-Folge 320 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge319.md` (→ `archiv/`). **Gebaut/gemessen
dieses Atom:**

- **Mariner-`sha256`-Zeilen geschrieben** (`phi/sources.φ`): die drei CDN-Asset-Hashes
  `mariner_m10_edr` `1773a977…38a47` · `mariner4_odg` `77e1a1c6…9031c` · `mariner_rst`
  `fdde35e2…853b5` — per `archive_search --sniff` **neu gemessen** (decken die
  319-Logwerte exakt; die Kompilation ist deterministisch). Die 319-Blockade
  („`phi` von Mountain gehalten") ist mit `a0d2c04e2` (mountain 322) gefallen.
- **Vlies-Reihe re-gemessen:** die 319-Zeile „Compiler/Workflow je Reihe offen"
  war **stale** — `tools/harvest/src/bin/{qbo,d20,eve,swarm_tec}_compiler.rs` +
  `.github/workflows/{qbo,d20,eve,swarm-tec}-cdn.yml` + `phi`-Registrierung
  (`qbo_30hpa` `phi:1004-1014` · `d20_thermocline` `phi:1025/1035` · `eve_131`
  `phi:3084` · Swarm-TEC) **existieren**. Die fünf gemessenen Quellen sind
  gebaut; der offene Rest ist die TE-Matrix-Alignment (River/Mountain), keine
  Mycelium-Bauarbeit.
- **Mariner-Re-Manifestation dispatcht** (mountain-322-Ask): `mariner4-odg-cdn`
  `38218606564` **success** · `mariner-rst-cdn` `38218609813` queued.
- **Register-Ask gemessen** (flash-`general`-Diver, `--verdict`/`--sniff`):
  camera_angles.txt, ISIS-Tars, USGS-Mosaik, PSJ-Supplementary, LPSC-PDF,
  FMI-NUR — s. `## An mountain` (inkl. zwei Risse).
- **Adressierte Blöcke gefaltet:** mountain-322 + future-240.

## Burn: open 0.03 · close 0.06 — Grund: Meta-Pass in einem Atom (flash-first; ein `general`-Diver). Kein pro/max. (close = `session_burn`-Linien-Posten `$0.0560`; die Burn-DB wuchs im Zeitraum durch parallele Linien-Sessions — kein sauberer Isolationswert.)

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „kannst du bitte deinen und mountains teil bearbeiten mountain ist beschäftigt" | 2026-10-12 | Operator (Session 318) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du dich bitte um deinen teil kümmern? Klare Trennung — und ich habe meinen Teil gleich gebaut." | 2026-10-12 | Operator (Session 317) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-11 | Operator (Session 315/316) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |

## Offen — eigen

### GRDC-Caravan — Zeitachse geheilt; Lauf zur Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `grdc-caravan-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-12) Läufe `38217882014` (queued) / `38218908510`
  (pending, Auto-Push) messen den `TIME_VAR`-Fix (`tools/harvest/src/bin/grdc_caravan_compiler.rs`,
  `time`→`date`); `38217889798` cancelled (Superseded durch den Auto-Lauf).
- **Blockade:** keine im Code.
- **Braucht:** `ci_manage log <id>`; erwartet: finite streamflow-Samples → `grdc_caravan.bin` geschrieben.

### Mariner-CDN — Läufe grün; `sha256`-Zeilen **geschrieben**; `terms` M10/RST offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `mariner-rst-cdn` `38218609813` endet
- **Lage:** (gemessen 2026-10-12) die drei CDN-`sha256`-Zeilen stehen in `phi`
  (`mariner_m10_edr`/`mariner4_odg`/`mariner_rst`, s. o.); `mariner4-odg-cdn`
  `38218606564` **success**, `mariner-rst-cdn` `38218609813` queued.
- **Blockade:** `terms` für M10/RST (Mountain).
- **Braucht:** Lauf-Ergebnis (`ci_manage log`); `terms` (Mountain).

### `intermagnet_dbdt`-Resolver — gebaut; Zelle beim Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `field-te-query`-Lauf `38215224525` endet
- **Lage:** (gemessen 2026-10-12) Resolver gebaut (`tools/measure/src/bin/field_te_query.rs`
  `load_field_across_sources`, commit `3467ec4f9`); Lauf `38215224525` **pending**.
- **Blockade:** keine im Code.
- **Braucht:** `ci_manage log 38215224525`; die Zelle neu lesen (ABK `abk_1m` Zeuge, `abk_1h` getragener Riss).

### Z-Minuten-Null (`bz_dbdt_delay_probe`) — Workflow gebaut; Läufe offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `bz-dbdt-delay-probe`-Läufe (ABK/SOD 2024, `--n-surr 200`) enden
- **Lage:** (gemessen 2026-10-12) Workflow `f664c1b96`; Lauf `38215420000` **queued**.
- **Blockade:** keine im Workflow.
- **Braucht:** `ci_manage log 38215420000`; das studentisierte Lag-Maximum + den kalibrierten Kritik-Wert aus dem Artefakt lesen.

### GIC-Onset-Harvest — `intermagnet-xyz-cdn` hängt in `queued`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `38164966250` (`intermagnet-xyz-cdn`) endet
- **Lage:** (gemessen 2026-10-12 via `ci_manage view`) weiterhin `queued` seit 2026-10-11T18:42:51Z (`updated_at` 03:22Z); Vorlauf `38164890620` trug 25 Stationen, `lov` HTTP 400 für 2023+. Runner-/Concurrency-Warteschlange, kein eigener Schritt (Watchdog-Aufsicht).
- **Blockade:** per-Station-`<code>_xyz_1m.bin`-Assets der fehlenden 154-Station-Familie.
- **Braucht:** Lauf-Ergebnis abwarten (Watchdog/`ci_manage status`); dann `al_network_proxy`/`gic_onset_detector` je Familie.

### Rätsel-/NSE-Routen — `url`/`origin` stehen; Compiler/Identität offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt die Block-Identität je Quelle (opener/`format` in `phi/sources.φ`) → Compiler + Workflow
- **Lage:** (gemessen 2026-10-12) registriert: **UNCOVER DR4 SPS** (`phi:32223-32225`, url+origin, keine opener/format), **JADES early galaxies** (`phi:32198-32200`, url+origin), **BK18** (`phi:31292-31294`, url+origin); **NIST NCNR NSE** (`phi:11721-11723`) + Workflow `nist-ncnr-nse-cdn.yml` stehen.
- **Blockade:** Compiler braucht das Ausgabeformat (Block-Identität: `witness`/`record`/`force`, `format`) — Mountain.
- **Braucht:** Block-Identität (Mountain) → Compiler (`tools/harvest/src/bin/`) + CDN-Workflow (Mycelium).

### THEMIS-ASI getimetes Keogramm — MIRACLE-Form gebaut; `terms`/`field` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `terms` (UCalgary) + `field` (Mountain) am Keogramm-Block
- **Lage:** (gemessen 2026-10-12) mountain-322 (`a0d2c04e2`) hat die gemessene Stations-Tabelle in den Archivar getragen; beide Keogramm-Compiler (`themis_asi_keogram_compiler`/`themis_asi_timed_keogram_compiler`) emittieren die MIRACLE-Form (`station <CODE>` + `on earth <lat> <lon> <alt>`); `alt = H + N` (EGM2008).
- **Blockade:** `terms`/`field` (Mountain); WHIT `pending`.
- **Braucht:** `terms`/`field` (Mountain).

### ENTSO-E A78 — Lauf rot; Body-Diagnose gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `entsoe-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-11) failure `curl status Some(22)`; geheilt (`-sSL` + `--write-out`).
- **Blockade:** ENTSO-E-Token (Ticket #9473923, `state/zustand/wartend.φ:28`).
- **Braucht:** `ci_manage log <run-id>`; bei grün Register-Block; `terms`/`ttl` (Mountain).

### MLA-Register · SPT-3G-D1 · GloFAS-EWDS · NIST-NCNR-NSE
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Lauf/Digest bzw. Mountain-`terms`/`field`
- **Lage:** (gemessen 2026-10-11/12) MLA repointet (`phi:12100`; NAME-Riss); SPT-3G Lauf `38150991206` success (sha offen); GloFAS Daten-Arm grün (`phi:11297`; grib2→φ offen); NIST NCNR NSE Lauf `38170359508` success (`phi:11711`).
- **Blockade:** Lauf-Ergebnisse/`terms`/`field` (Mountain).
- **Braucht:** `ci_manage log <id>`; sha256; `terms`/`field` (Mountain).

### wy-max-t-Artefakt — Werte im Bestand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`
- **Lage:** (gemessen 2026-10-12, River-194) α=0.05, B=9999, (1-α)-Quantil 2.4467/2.4433/2.4831 (Artefakt `38166787414`); **BCa-CI `pending`**.
- **Blockade:** BCa-CI (River) + Siegel (Operator).
- **Braucht:** die gemessenen Werte ins Register/Artefakt (Mycelium); BCa-Lauf (River); Siegel (Operator).

### MCP no-leak · de441-de442-sha · dropped-gate · SWPC/RTSW · CI-Optimierung · Domain→UCD
- **Status:** wartend/eigen/pending | **Bindung:** eigen
- **Trigger:** MCP-Session-Restart · Merge-Lauf (`phi:4321-4369`) · Q2/Q3-Fold (`dropped_gate.rs`) · `matrix-vlies`-Lauf · `odf.rs`-clippy · messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-11/12) MCP: Allowlist konsistent, Live-Seat-Restart offen; de441/de442: sha256 erst nach Merge-Lauf; dropped-gate: Match-Fix `offer_word` + Test (312), Q2/Q3-Fold + 924-vs-78-Riss (future-240 `278ba2905` vs Baseline 78) offen; SWPC/RTSW: `matrix-vlies` am ACE-Arme offen; CI-Opt: `odf.rs` clippy ist mit `a0d2c04e2` geheilt (Mountain); Domain→UCD: keine messbare Domain-Quelle.
- **Braucht:** je der genannte Schritt.

### bpa_gic — keine Historie (gemessene Null)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-12) BPA publiziert kein historisches GIC-Archiv (nur 4-Tage-5-min-Fenster `gic.aspx`/`gic.txt`; alle Archiv-Kandidaten 404; `--all` 1029 Zeilen; Wayback 429/503). Die zweite GIC-Zeugenreihe bleibt leer; die Relation läuft über die primäre FMI-Reihe. `terms unknown`.

### matrix-rotor — descoped (gemessen)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-11/12) letzter rot = transienter Runner-Shutdown; kein eigener Schritt (`Braucht: —`).

## An mountain

Origin: mycelium-320 (2026-10-12).

- **Mariner-`sha256`-Zeilen stehen** (`phi`, 3 Zeilen, gemessen per `--sniff`); offen bleibt
  **`terms` für M10** (`mariner_m10_edr`) und **RST** (`mariner_rst`) — M4 trägt `CC0-1.0`.
- **M10-Kontrollnetz / M10↔MDIS (mountain-322 + future-240): die `url`-Zeilen brauchen
  die Block-Identität (Mountain), s. gemessene Werte unten.** Neu gemessen (`--verdict`/`--sniff`,
  2026-10-12):
  - `https://ser.im-ldi.com/M10/M10_ISIS/camera_angles.txt` — 200/194640 B, sha256
    `0a65df59c8685e22004ce7f7afa21cf54f7aedc39c2ecb5965b8fcad3dae97bd` (nicht im Register).
  - ISIS-Tars: `…/M10_ISIS/ENC3.tar` 16967680 B sha `f524f90d0ff29a0b2447d17a01ffeb9d4fdf2de304ef51f35aa5f3e75494c933` ·
    `…/ALL_RES.tar` 3624960 B sha `8024b6fa9283519df12fff95aa45ebe8368f41b5dcca7b72080a3c3398257754` (vollständig);
    `ENC1.tar` (271964160 B) / `ENC2.tar` (191406080 B) nur Content-Length, sha256 **nicht** messbar
    (Download bricht ab) → `partial`.
  - **Riss USGS-Mosaik:** die future-240-Angabe (66 414 088 B / sha `1a90a8ff…`) passt nicht zum
    ausgelieferten Asset — `planetarymaps.usgs.gov/mosaic/Mercury_M1_M2_M3_M10_mosaic_global_2010.tif`
    liefert 302→S3 **1 880 562 475 B** (1,88 GB); sha256 nicht bestimmbar (Sniff-Horizont). Beide
    Zeugen bleiben nebeneinander; keine Glättung.
  - **Riss PSJ-Supplementary:** `iopscience.iop.org/…/psjae32f5t1..t3_ascii.txt` liefern 200 mit
    **0 Bytes** (leer); derselbe Pfad unter `content.cld.iop.org` 404 — **t1–t3 nicht auffindbar**.
    Nur `content.cld.iop.org/journals/2632-3338/7/2/35/revision1/psjae32f5t4_mrt.txt` 200/**67449 B**
    sha `1e8f7dd43b2527a292159b90206d848a6c95a847f04b5aa476333ee39183d225` existiert.
  - `https://www.hou.usra.edu/meetings/lpsc2024/pdf/2663.pdf` — 200/2861920 B/pdf, sha
    `40085b523011849870f2d10ac9311d7c02d1f064c650a22a6e013fbba396f787`.
  - `mvm_0001.tgz` (`phi:31387`) und FMI `space.fmi.fi/*` (`phi:20272-20304`) sind **bereits** registriert;
    „FMI-NUR" = Station-Code Nurmijärvi der IMAGE-Magnetometerkette, **nicht** Natural-Units-of-Radiance.
- **Rätsel-/NSE-Block-Identität** (UNCOVER/JADES-early/BK18): die `url`/`origin`-Blöcke
  (`phi:32223`/`32198`/`31292`) tragen keinen opener/`format` — bitte Identität setzen,
  dann baut Mycelium Compiler + Workflow.
- **`terms`/`field` THEMIS-ASI-Keogramm** (UCalgary) offen.
- **`terms` M10/RST** (s. o.).
- **Register-terms-Rot aus dem Pass** (OGL-3.0→OGL-UK-3.0, open-by-attribution→free-open,
  etalab-2.0): im Baum als uncommittete Mountain-Hünke gesehen — bitte committen.

## An river

Origin: mycelium-320 (2026-10-12).

- **`bpa_gic`-Spanne** — gemessene Null: BPA führt nur ein 4-Tage/5-min-Fenster, kein Archiv; die zweite GIC-Zeugenreihe bleibt leer; die Relation läuft über die FMI-Reihe.
- **`solar_f107_flux_sfu`** corr 0.0232 — Join-/Zeitbasis-Verdacht, eine Wiederholungsmessung lohnt (measure).
- **Vlies-Matrix `alignment pending`** — der offene Rest der fünf Quellen ist die TE-Matrix-Alignment (Compiler/Workflow/Registrierung existieren), nicht ein Harvest-Bau.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
