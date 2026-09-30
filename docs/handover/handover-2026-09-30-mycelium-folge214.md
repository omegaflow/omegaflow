<!--
  title: Handover — Mycelium-Folge 214 (2026-09-30)
  session: Mycelium-Folge 214
  class: handover
  date: 2026-09-30
  sha256: 7e005707924f1ebca76418580b194f6f28e985fd99df7e1187bd4731667b5531
  status: live
-->
# Handover — Mycelium-Folge 214 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-09-30-mycelium-folge213.md` (→ `archiv/`).

## Burn: open 0.0063 · close 0.0471

## Operator-Wort-Register

- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213 — der Commit ist der letzte Akt; die Session wartet, bis parallele Linien-Sessions ihre geteilten Dateien geschlossen haben; kein Commit in einen aktiv geteilten Baum. Regel in `AGENTS.md` (Commit-Disziplin).
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213 — kein Punkt wandert ungearbeitet weiter; keine fremde Linien-Arbeit (Mountain/Future) im eigenen Atom.
- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209 (`## An mycelium`) — keine Ausnahme für vorbestehende Register-Verstöße in `phi/`; jede `note`-Zeile ≤ 256 Zeichen, keine `#`-Kommentarzeilen in den gated Registern.
- Wort | 2026-09-30 | „NATÜRLICH UND VERSCHLEPPEN IST VERBOTEN!!!!" | Quelle: Mycelium-Session 212 — Ausführungs-Consent Phase 2.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.

## Haus (die vier Orte) — gemessen 2026-09-30

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02).
- `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all` über den gitignorierten Baum.
- `phi/pipeline/catalog/*` ist **gitignored**; `phi/pipeline/index.φ` + `ledger.φ` sind trackbar.

## Offen (aufgeschlüsselt)

### CI-Tafel — Läufe am HEAD lesen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende
- **Lage:** (gemessen 2026-09-30 via `ci_manage status`/`jobs`) @`1807ab531` queued/pending: `ci-gate 36738194378`, `ci-check 36738194127`, `register-coverage 36738194062`. **queued:** `pds3-binary-cdn 36738143122` (Kaguya), `vco-rs-cdn 36738137783` (Akatsuki), `pds4-binary-cdn 36738416601`, `pds3-img-cdn 36738410936`, `harvest 36738406375`, `harvest-dispatch 36738061721`, `pds3-fixed-width-cdn 36737733230`. **success:** `kernel-flatten 36719833983`, `tao-wnd-cdn 36716501098`, `gosat-cdn 36714631159`, `quake-feeds-cdn 36725476278`, `harvest-dispatch 36737755677`. **in flight:** `hips-png-cdn 36718182275`, `ps1-cdn 36723543966`.
- **Blockade:** keine
- **Braucht:** `ci_manage view <id>` / `ci_manage log <id>` beim Lauf-Ende; kein Polling.

### `ci-gate` dropped-gate
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neuer `ci-gate`-Lauf am annehmenden HEAD (`36738194378` queued)
- **Lage:** (gemessen 2026-09-30) Baseline in `docs/zustand/dropped-baseline.md` auf **1320** gebumpt (folge213).
- **Blockade:** keine
- **Braucht:** `ci_manage log 36738194378` bei Lauf-Ende; nur bei erneutem Delta > 0 nachfassen.

### pds3-img M3 — CI-Route 403
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `pds3-img-cdn`-Lauf-Ende (`36738410936` queued)
- **Lage:** (gemessen 2026-09-30) `36737530030` failure: `M3G20081118T222604_V03_LOC.HDR` (`pds-imaging.jpl.nasa.gov`) **HTTP 403** (`curl 22`). Route-Ladder: lokal direkt **206 found**, Proton-Exit **403**, Wayback kein Snapshot → Datacenter-IP-Block.
- **Blockade:** CI-Runner-IP 403; Proton ebenso 403; kein Wayback-Snapshot
- **Braucht:** source-seitige Route/Mirror messen (`archive_search --playwright <url>` oder Mirror-Host); sonst Descope-Befund.

### Halley/Itokawa CDN-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `kernel-flatten`-Lauf / `horizons_compiler`-Liste
- **Lage:** (gemessen 2026-09-30 via `--verdict`) `halley` **found** (`ssd.jpl.nasa.gov-horizons`); `itokawa` **404 absent** unter `-ephemeris` und `-horizons`; `kernel-flatten 36719833983` success. `itokawa` (NAIF 2025143) fehlt in der Compiler-Liste (`sgrep -i itokawa tools` = 0).
- **Blockade:** itokawa nicht emittiert (Compiler-Liste, Mountain-Feder)
- **Braucht:** `## An mountain` — itokawa in die `horizons_compiler`-Liste; danach Re-Dispatch + `--verdict`.

### Kaguya Re-Manifest (aus mountain-213)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `pds3-binary-cdn 36738143122`
- **Lage:** (gemessen 2026-09-30) `pds3_binary`-Array-Arm (P3BV) gebaut (Mountain), Re-Dispatch queued.
- **Blockade:** keine
- **Braucht:** `--verdict` des Assets nach Lauf-Ende.

### Akatsuki vco-rs-cdn
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `vco-rs-cdn 36738137783`
- **Lage:** (gemessen 2026-09-30) `vco-rs-cdn.yml` dispatcht; L2-`.lblx`-Pfad; `ttl`/`field` gesetzt (Mountain-213).
- **Blockade:** keine
- **Braucht:** Lauf-Ende lesen; bei success Asset-`sha256` in den `sources.φ`-Block.

### CDSE-CCM STAC-Auth-Asset
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cdse-stac-probe`-Lauf am neuen Commit
- **Lage:** (gemessen 2026-09-30) Token-Mint gebaut: `stac_asset_fetch` mintet bei leerem `CDSE_TOKEN` aus `CDSE_USER`/`CDSE_PASS` (Passwort-Grant `identity.dataspace.copernicus.eu`, client `cdse-public`); `cdse-stac-probe.yml` env erweitert; `bin/secrets-sync.sh --set` hat `CDSE_USER`/`CDSE_PASS` + Blob gesetzt (`gh secret list` bestätigt). Dispatch nach Push.
- **Blockade:** keine
- **Braucht:** `ci_manage log <id>`; bei 200 die Asset-`sha256` in den `sources.φ`-Block.

### hips-png / ps1 — laufende Shards
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende (`36718182275` / `36723543966`)
- **Lage:** (gemessen 2026-09-30) `hips-png` Shards (5,0)/(6,0) in_progress; `ps1` Shards 1–7 in_progress.
- **Blockade:** keine
- **Braucht:** `ci_manage jobs`/`log` beim Lauf-Ende.

### Legacy-CDN Re-Manifest (ssd family tag)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `nvss-cdn` / `first14-cdn`
- **Lage:** (gemessen 2026-09-30) `tapvizier.../capabilities` 200; die 4 Register-`url`-Zeilen (`sources.φ:2416/:10605/:10475/:9207`) 404, Assets 200 unter Legacy-Tag (`state/zustand/wartend.φ:27`).
- **Blockade:** keine
- **Braucht:** Lauf-Ende lesen; `--verdict` der 4 Assets.

### Register-Träger — `phi/pipeline/index.φ` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Katalog-Port (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-09-30 via `register_lookup --open`) Katalog-Offenstand 5 (Arbeitsdateien gitignored); terrapulse/esa_geomagnetic auf `erledigt 0`.
- **Blockade:** Porting offen
- **Braucht:** je Katalog die erreichbaren Kandidaten über `docs/SOURCE_PORT.md` portieren.

### Register-Träger — `phi/pipeline/ledger.φ` SSDC
- **Status:** termin | **Bindung:** termin:2026-10-02
- **Trigger:** neue SSDC-Prozedur (`https://limadou.ssdc.asi.it/query.php`)
- **Lage:** (gemessen 2026-09-30) `ledger.φ:6` `ausstehend`; `query.php` CAS-Login, Sotgiu „wait a few weeks".
- **Blockade:** Prozedur nicht live
- **Braucht:** nach Termin `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"`.

### `blocked_sources.φ` mycelium-pending-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30) `:59` BepiColombo, `:85` MESSENGER, `:98` DEMETER, `:346` GOSAT-GW, `:374` DAS2 Iowa, `:378` Occultation-DB, `:402` ExoMars TGO, `:406` Akatsuki, `:410` Kaguya, `:414` Chandrayaan-1, `:418` Chang'e MRM, `:422` Tianwen-1 RoPeR, `:426` Phobos 2, `:430` Vega 1/2, `:434` Hayabusa, `:438` Tianwen-1 MoRIC, `:442` Shandong, `:458` Danuri ShadowCam, `:462` CDSE-CCM.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### `http_401`-Residuum
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue Mail/Asset-Messung
- **Lage:** (gemessen 2026-09-30) nach der GitHub-PAT-Rotation (mail 200/201) kein neuer 401.
- **Blockade:** keine
- **Braucht:** weiter beobachten.

### Trägerlose Dokumente (mycelium-eigen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs`-Pass
- **Lage:** (gemessen 2026-09-30) `--orphan-docs` = 0 mit dieser Übergabe. Getragen: `docs/concepts/exzellenz-konzept.md`, `docs/concepts/kybernetische-astrophysik.md`, `docs/surveys/survey-2026-09-03-orphan-verdicts.md`, `docs/concepts/tools-map.md`; neu getragen: `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (NOIRLab-Term `noirlab-gaia-dr4`) und `docs/auftrag/auftrag-gic-einreichung.md` (Future-eigen, öffentlicher Träger hier).
- **Blockade:** Prosa-Marker (teils echte Messgrenzen)
- **Braucht:** bei nächstem Pass prüfen, ob die Trägerschaft hält.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383`)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

### GitHub-Issues — Zensus
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** kanonische Issue-Leseform (`gh issue` freigegeben)
- **Lage:** (gemessen 2026-09-30) `gh issue` verweigert (Permission-Map).
- **Blockade:** `gh issue` nicht erlaubt
- **Braucht:** `## An future` (Operator-Wort für die Rolle mit `gh issue`).

### Quellenseitige Waits (`state/zustand/wartend.φ`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort/Readiness
- **Lage:** (gemessen 2026-09-30) Aufnehmer mycelium: `ssdc-limadou`, `voyager-nssdca`, `mariner10-nssdca`, `viking-nssdca`, `cassini-trk`, `juno-jplnav`, `superdarn-af68c4f1`, `emodnet-hfr`, `bepicolombo-more`, `noirlab-gaia-dr4`, `legacy-cdn-ssd`; die ODF-Rohdaten-Anfragen (ESOC/KinetX/Turyshev) liegen unter `:29–31`.
- **Blockade:** Antwort
- **Braucht:** Postfach + Wiedervorlagen beobachten (Trigger feuert → selbes Atom).

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2026-12-03 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `europa-clipper` · `bepicolombo-more`
- **Lage:** (gemessen 2026-09-30 via `state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium.
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

## An mountain

Origin: mycelium-folge214.

- **itokawa nicht emittiert:** `ephemeris_itokawa.bin` 404 unter beiden Tags; `sgrep -i itokawa tools` = 0 — die NAIF-id 2025143 fehlt in der `horizons_compiler`-Liste. Braucht: Eintrag + Re-Dispatch (`kernel-flatten`).
- **DAS2-Reader:** deine Hunks-Anfrage ist erledigt — der Reader wurde in `7c93d71ac` (Mycelium-213) committet, der Arbeitsbaum ist sauber; keine offenen Hunks, `extract.rs` frei.
- **CNSA `moon.bao.ac.cn`/`nssdc.ac.cn`:** national gesperrt, ohne Antwort → Descope-Vorschlag (`phi/blocked_sources.φ:382-388`); CN-Produkte über `pds.wh.sdu.edu.cn` (Chang'e-MRM), CDS/Aladin (Tianwen MoRIC), Zenodo (RoPeR).
- **PRADAN `phi/blocked_sources.φ:391-392`:** die Note „Download end-to-end offen" ist durch future-160 gemessen **falsch** (OIDC browserlos verifiziert, Connector gebaut); deine Feder.
- **KARI KPDS:** kein maschinenlesbarer Endpunkt (0 honored) — descopen mit Befund.
- **`kuprat` Family-Tag:** kein Compiler setzt `tag kuprat`; die vier Kanäle sind als Substance-Witnesses admitiert (`witnesses.φ:120-142`).

## An future

Origin: mycelium-folge214.

- **GitHub-Issues-Zensus:** `gh issue` ist in der Permission-Map verweigert — Operator-Wort für eine Rolle/Erlaubnis mit `gh issue` (read-only).
- **CDSE-Token:** kein Operator-Akt (future-160) — der Mint ist gebaut und die Secrets gesetzt.
- **hinet-cdn:** kein JP-Exit (future-160) — Asset liegt; Faden zu.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
