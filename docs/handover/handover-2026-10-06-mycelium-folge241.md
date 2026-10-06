<!--
  title: Handover — Mycelium-Folge 241 (2026-10-06)
  session: Mycelium-Linie — Meta-Pass: Exposition-CDN-Workflows gebaut, Stehender Pass
  class: handover
  date: 2026-10-06
  sha256: 0b0569fbdd5b5ace1f0c6431eb3da44d9f7ff4b3dc3acb95d4e72fad2fdc9517
  status: live
-->
# Handover — Mycelium-Folge 241 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-06-mycelium-folge240.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0337

`session_burn`: diese Session ~**$0.0337** (line „Mycelium-Linie in einem Pass starten").
Rolling-Fenster (7 Sessions) bei Schluss **$0.1562**.
`bin/.tools_ensure archive_search|sgrep|sfetch|smail`: frisch (keine Meldung).

## Operator-Wort-Register

- Wort | 2026-10-06 | „Genau ein Zulassungskriterium (Presence-Hülle) und ein deklarierter Beobachter je Messung; ein Body-Name, den der Code wählt, ist der Bias" | Quelle: Operator-Session 2026-10-06 — als Regel in `AGENTS.md` `## Block Universe Physics`.
- Wort | 2026-10-06 | JAXA-G-Portal-Bestellungen (`download_limit=1` je, Fenster `2026/01/01`); die Abholung (fetch) ist der Vollzug desselben Worts | Quelle: Operator-Session 2026-10-06.

## Offen — eigen

### Exposom-CDN-Workflows — sha256-Nachzug (Pollen, FARA, GHSL)
- **Status:** eigen
- **Trigger:** grüner Lauf + `sha256` aus dem Release → `phi/sources.φ`
- **Lage:** (gemessen 2026-10-06 11:52 via `ci_manage list`) vier Workflows gebaut (mountain-folge242 addressed): `openmeteo-pollen-cdn.yml` (stündlich), `usda-fara-cdn.yml` (jährlich), `ghsl-cdn.yml` (jährlich, `--stride 300`), `vnp46a3-cdn.yml` (täglich). **Läufe:** `openmeteo-pollen-cdn 37459099065` = **success**, `usda-fara-cdn 37459103439` = **success**, `ghsl-cdn 37459107131` in_progress (Ausgang offen). `sources.φ`-Blöcke `:17357` (Pollen), `:17373` (FARA), `:17365` (GHSL).
- **Blockade:** keine.
- **Braucht:** `sha256` je Zeile in `phi/sources.φ` (Mountain-Verdiktzeile) nach dem Lauf.

### VNP46A3-CDN — Lauf leer, per-Zelle-Arm messen
- **Status:** blockiert
- **Trigger:** per-Zelle-Arm/Verdikt (Mountain) → Re-Dispatch
- **Lage:** (gemessen 2026-10-06 via `ci_manage log 37459111673`) `vnp46a3-cdn 37459111673` = **failure**; der CMR-Resolver liefert die Granule (`VNP46A3.A2026213.h17v01.002.2026252141443.h5`), aber `vnp46a3_compiler` meldet `no measured VNP46A3 cell left the harvest — the bin stays unwritten (0 honored)`.
- **Blockade:** der per-Zelle-Pfad liefert auf der Granule 0 Zellen — `format black_marble_vnp46a3_nightlight`-Arm (Mountain).
- **Braucht:** `vnp46a3_compiler <granule-url> --inspect` im CI (SDS-Liste) → Arm/Verdikt Mountain; ggf. andere Granule/Stride wirksam.

### Exposom-Quellenmatrix — Matrix-Lauf-Workflow
- **Status:** eigen
- **Trigger:** Sources-Zeilen je pending Domäne + `.te`-Descriptor + Workflow-YAML → `gh workflow run`
- **Lage:** (gemessen 2026-10-06) öffentlich `docs/surveys/survey-2026-10-04-exposom-matrix.md` (12 Domänen); 4 Kern-x-Serien erreichbar/registriert (OpenAQ 206, Open-Meteo 200, NASA POWER 206, OMNIWeb 200); 2 Arme gebaut (WQP + EEA-noise, `mycelium-folge231:70-75`). Mountain hat 4 x-Homes feld-aufgenommen (Licht/Pollen/Gebaute Umwelt/Ernährung, `handover-2026-10-06-mountain-folge242.md:81-86`) — die CDN-Workflows dazu stehen jetzt (s. o.). Descriptor-Form `phi/pipeline/descriptors/solar_seconds_matrix.te`, Parser `field_te_query.rs:580-684`.
- **Blockade:** die y-Serien der Matrix-Klassen sind unregistriert; eine Aufnahme braucht das Mountain-Verdikt + die Mycelium-Manifestations-Direktive.
- **Braucht:** je pending Domäne die Sources-Zeile (Verdikt + `url`/`origin`/`compiler`); dann `.te` je Klasse + `.github/workflows/exposom-matrix-te.yml`.

### Träger `survey-2026-09-03-orphan-verdicts` — Step 5 CDN-kanonisch
- **Status:** eigen
- **Trigger:** je `*-cdn.yml` die Release-Menge aus `phi/sources.φ` lesen
- **Lage:** (gemessen 2026-10-06) `:103-151` — 13 Netlocs. **Bindung umgesetzt:** Helfer `.github/workflows/scripts/register_release_set.sh <netloc>`; `de44-cdn.yml` verifiziert `ssd.jpl.nasa.gov-de` (9 registriert == 9 im Release, `sources.φ:3501-3557`). **Bindung umgesetzt (scoped):** Helfer `.github/workflows/scripts/register_release_set.sh <netloc> [<prefix>]`; `de44-cdn.yml` verifiziert `ssd.jpl.nasa.gov-de` 9/9, `inpop-epm-cdn.yml` (`ephemeris_inpop`) + `noe4-cdn.yml` (`ephemeris_noe4`) verifizieren `ftp.imcce.fr` 10+2. **Reg==Release noch offen:** `pds-rings.seti.org` 19/19, `naif.jpl.nasa.gov` 575/575. **Mismatch-Ursache gemessen — Roh-/Probe-/Tag-Hygiene, nicht fehlende Zulassung:** (1) **Roh-/Intermediär-Uploads** — `zenodo.org` trägt `data.zip`, `Proudfoot23_TNBFits.zip.000-013`, `rixs_spin_*.txt`, ein PDF; Produzenten `kyoto-pressure-cdn.yml`/`tnbfits-cdn.yml` (Compiler-Uploads). (2) **Probe-Junk** — `TAP-sync-REQUEST-*`, `TAPVizieR-tap-sync-REQUEST-*`, `nph-query.json`, `sync.json`, `.sha256`-Sidecars: **0 in `sources.φ`** und **kein Workflow-Literal** → Compiler-/Probe-Uploads. (3) **Tag-Divergenz** — das Release `ssd.jpl.nasa.gov` trägt 999 Assets aus vielen Workflows, das Register dort nur 4 (`f107_penticton.bin`, `first14.json`, `goes_xrs.bin`, `jwst_spectra.bin`) — geteiltes Misc-Release.
- **Blockade:** Bindung je Workflow nur scoped möglich (Compiler→Asset); Own-CDN-Löschung des Junk braucht vorher den `sgrep`-Konsumenten-Check. Zulassung fehlt nur bei den echten Neuen (`pioneer10/11_navio`, `galileo_receiver`, `allwise_part_*` sharded/dynamisch) — Mountains Zeile.
- **Braucht:** (a) `pds-rings.seti.org`/`naif.jpl.nasa.gov` scoped binden (Prefix je Workflow); (b) Compiler-Hygiene — nur kanonische Assets hochladen, Roh-Intermediäre nicht auf die CDN; (c) Mischtag entwirren (jeder Workflow schreibt sein eigenes Netloc); (d) Own-CDN-Junk nach Konsumenten-Check entfernen; (e) Bindung auf die je Workflow eigenen registrierten Assets scopen, wo kein Allein-Writer (de44).

### Träger `survey-2026-09-03-daten-holdings-inventur` — Ziel-Layout-Migration
- **Status:** eigen
- **Trigger:** Move je Datensatz (Operator-Wort Ziel-Layout steht 2026-09-30) → `du`-Nachmessung
- **Lage:** (gemessen 2026-10-06) Byte-Messung Schritt 2 steht (`:167-183`, 2026-09-30: `archive/knowledge` 29 Gi, `archive-state` 9,7 Gi, Repo-`data` 77 Gi); Registry-first Schritt 3 für die Staging-Kandidaten gemessen erfüllt (Ephemeriden registriert, `omni2_serie.bin` `phi/sources.φ:1420`); auf diesem Host existiert `~/knowledge`/`~/backups` nicht mehr — konsolidiert nach `~/archive/` (2026-10-06 `du`: `archive/knowledge` 29 G, `archive-state` 9,8 G, `archive/archive-root` 974 M).
- **Blockade:** Move/Löschung braucht das Operator-Wort je Datensatz (`0 honored`: nichts löschen ohne Nachbau-Quelle).
- **Braucht:** Operator-Wort je Datensatz (Queue: `## An future`) → Schritt 4/5 Unique-Byte-Move je Holding.

### `ledger.φ:2`/`:6` — Port-Runner (gemessen: vorhanden)
- **Status:** wartend
- **Trigger:** Korpus-Input `phi/pipeline/queue/<korpus>.φ` am Datenträger → `omegaflow --port`
- **Lage:** (gemessen 2026-10-06) Der Runner ist **nicht** verloren: `omegaflow --port <in> <out>` läuft über `port_mode` (`src/archivar/main_flow.rs:732`, `src/archivar/port.rs:625`), dokumentiert (`docs/SOURCE_PORT.md:121`) und in CI benutzt (`port-count.yml:24`). Die frühere Zeile „Port-Runner verloren" war ein **Falschbefund** — die Archäologie suchte einen Dateinamen `port*`; der Runner ist der Core-Bin-Modus. `phi/pipeline/stage/*` leer, aber regenerierbar (`docs/SOURCE_PORT.md:24`).
- **Blockade:** die Korpus-Eingaben (`phi/pipeline/queue/*.φ`) sind am Datenträger absent (`queue/master.φ` gitignored, `docs/SOURCE_PORT.md:21`).
- **Braucht:** Korpus-Input wiederherstellen → `omegaflow --port` über die 825/63 Blöcke.

### `phi/blocked_sources.φ` — Mycelium-Klasse
- **Status:** je eigen
- **Trigger:** Arm-Bau/Manifestation je Eintrag
- **Lage:** (gemessen 2026-10-06) ExoMars TGO ACS / Viking gravity / Hayabusa LIDAR / Phobos-2 KRFM — Workflows dispatched, queued; EUMETSAT MTG-LI — `37437023048` queued; Chandrayaan-1 Mini-RF — **blockiert** (`pds3_img` ohne Feld-Arm); Tianwen-1 MoRIC / ShadowCam / JAXA G-Portal — Sample/Record-Download = Operator-Hand; `:78` SuperDARN — LOCK.
- **Blockade:** Chandrayaan-`pds3_img`-Arm (Mountain); Sample-/Record-Downloads (Operator/per-act).
- **Braucht:** `pds3_img`-Feld-Arm (Mountain); Consent für Record-Downloads (Operator/per-act).

### iEEG-Ernte — Dienst antwortet 500
- **Status:** wartend
- **Trigger:** `www.ieeg.org/services` erholt sich → `gh workflow run ieeg-cdn.yml -f dataset=09_14_limbic_seizure_374`
- **Lage:** (gemessen 2026-10-06) Zugang steht — `IEEG_USER`/`IEEG_PASS` im `.secrets.local` (`bin/secrets_keys`), exportiert `.github/workflows/ieeg-cdn.yml:22-23`, gelesen `ieeg_compiler.rs:211-212`; `phi/sources.φ:3584` bindet `www.ieeg.org` (Mountain). `archive_search --verdict https://www.ieeg.org/services` = **HTTP 500** (Stufe 1+2, 2026-10-06).
- **Blockade:** Dienst 500 (nicht der Zugang).
- **Braucht:** Re-Dispatch bei Erholung.

### JAXA G-Portal — Abholung (Reihe queued)
- **Status:** wartend
- **Trigger:** `jaxa-gportal-cdn`-Reihen-Ausgang → `ci_manage list`
- **Lage:** (gemessen 2026-10-06T11:5x via `ci_manage view 37441304379`) Fix committet (`jaxa-gportal-cdn.yml:3-5`); der Re-Dispatch-Lauf `37441304379 @129cd0e3e` steht **queued** im Runner-Stau.
- **Blockade:** Runner-Kapazität (Stau).
- **Braucht:** Reihen-Ausgang; Reader-Feld-Verdikt je Produkt (Mountain).

### Rand ohne Rubin — Fink-Cutout-/FP-Manifestation
- **Status:** wartend
- **Trigger:** Mountains Fink-Admission im Baum → `url`/`origin`/`compiler`/Tags setzen
- **Lage:** (gemessen via future-182 addressed block, 2026-10-06) die geharvesteten FP-Assets brauchen die Manifestations-Direktiven neben Mountains Fink-Admission; verwandte bestehende Quelle ALeRCE ZTF (`phi/sources.φ:561`).
- **Blockade:** Fink-Admission (Mountain).
- **Braucht:** `phi/sources.φ`-Direktiven (Mycelium) nach Admission.

## An future

Origin: mycelium-folge241.

- **Tavily-Quota 80 %** (`mail_ledger.φ`, ts 1791121265) → Fallback `--mwmbl`/`--marginalia`.
- **Kimi-K3-Gratis-Route** (NVIDIA NIM `moonshotai/kimi-k3`, kein Kartenzwang): Developer-Account/Key = Operator-Akt → Operator-Queue.
- **Self-hosted Runner (Operator-Queue):** Wort 2026-10-06 „ok ich schaue ob ich einen gaming pc von 2011 zum laufen bekomme und da linux mint drauf installiere um ihn als runner laufen zu lassen". Mehrere Jobs binden `[self-hosted, Linux]` (`de44-cdn.yml:18`). **Braucht:** Runner-Registrierung (Labels) + Routing-Entscheidung für die `hyperscanning-te`-Screen-Jobs — Operator-Vollzug.
- **Daten-Holdings-Ziel-Layout-Migration (Operator-Queue):** Move/Löschung je Holding braucht das Operator-Wort je Datensatz (`0 honored`: kein Löschen ohne Nachbau-Quelle); Lage `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md:167-183`.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.

## Abschluss

Der Stehende Pass wird **nach** Commit + Push am neuen HEAD neu gestempelt
(`state/zustand/standing-pass.md`). Detail der Runde: der Pass.
