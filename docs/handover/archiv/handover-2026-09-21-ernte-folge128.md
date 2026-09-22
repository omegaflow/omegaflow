<!--
  title: Handover — Ernte-Folge 128 (Stand 2026-09-21)
  session: Ernte-Folge 128
  class: handover
  date: 2026-09-21
  sha256: d34414c3fe4ab8021c6851608bab2ec423d705fa792b30f0c12b0de7c5922b7a
  status: live
-->
# Handover — Ernte-Folge 128 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder offene Punkt trägt seinen nächsten
Schritt in derselben Zeile; Status-Tag (`wartend` | `operator-gebunden` |
`blockiert` | `termin`). Wartestellungen sind kein Auswahlpunkt.

## Stehender Pass (gemessen 2026-09-21, Folge 128)

- **HEAD** `9cc2b96c` beim Start (geteilter Baum, fremde Commits laufen
  gleichzeitig); eigener Commit folgt. Arbeitsbaum: eigener Pfad-Satz (unten);
  fremde Arbeit (forschung/entscheid) ist uncommittet im Baum.
- **CI** — `rpw-cdn` `35571712600` @`2e7fa73b` **success** (P1-Verifikation,
  siehe unten); `ps1-cdn` `35569486280` in_progress; `demeter-cdn` `35567568429`
  queued; `ci-check` cancelled-Churn; `hyperscanning-te` `35570480672` failure
  (Forschungs-Linie).
- **Zustand** — `docs/zustand/external-state.md` fortgeschrieben (P1-Zeile,
  Asset-Cap-Zeile); CI-Zeile von Forschung-Folge 128 bereits gemessen.

## P1 RPW-CDN-Rotation — verifiziert und geschlossen

- Lauf `35571712600` @`2e7fa73b` success. Assets gemessen (`archive_search
  --sniff`, 2026-09-21): `amda.irap.omp.eu/rpw_efield.bin` 2 931 048 B sha256
  `f87c77ec…`; `rpw-lira.obspm.fr/rpw_efield_lira.bin` 3 983 048 B sha256
  `d398dba7…`.
- Alt-Kopie `ssd.jpl.nasa.gov/rpw_efield.bin` gelöscht (`gh release
  delete-asset`, bestätigt „not found"); die zweite Alt-Kopie
  (`rpw_efield_lira.bin` auf `ssd.jpl.nasa.gov`) war bereits 404.
- **Verpasste 7. Site:** `phi/pipeline/frame_registry.φ:262` zeigte noch auf den
  alten Tag. Nachgezogen (amda + rpw-lira eingetragen, ssd-Zeile entfernt) —
  die Datei ist generiert/gitignored, der nächste `--port`-Lauf erzeugt sie
  identisch neu.

## Offen (aufgeschlüsselt)

### Generischer `upload_asset`/Familien-Tag-Umbau
- **Status:** offen | **Bindung:** eigen
- **Lage:** `src/archivar/cdn.rs:40` hängt am 1000-Asset-Cap des Release
  `ssd.jpl.nasa.gov` (id 367063539); die RPW-Familie ist rotiert, ~74 weitere
  Aufrufer nicht (`eve_compiler.rs:8`, `ps1-cdn.yml:149`).
- **Blockade:** keine (Architektur-Entscheid nötig).
- **Braucht:** Rat/Architektur, dann `grind-pro` (jeder Aufrufer benennt seinen
  Familien-Tag).

### HFRNet-RTV-Gitter-Compiler
- **Status:** offen | **Bindung:** eigen
- **Lage:** `hfrnet-tds.ucsd.edu` tot (`dead_sources.φ:519`); lebende Gitter-Route
  `coastwatch.pfeg.noaa.gov/erddap/griddap/ucsdHfrW1_Lon0360` + `ucsdHfrW2`
  (200, `water_u`/`water_v` m/s) ist als `ausstehend kandidat` in
  `phi/pipeline/ledger.φ` registriert; kein Compiler.
- **Blockade:** keine.
- **Braucht:** `grind-flash` (ERDDAP-griddap-Parser + Compiler + CDN).

### PS1 Order-10-Final
- **Status:** wartend | **Bindung:** termin (Run `35569486280`)
- **Lage:** `ps1_dr2_coverage.fp01` auf `ssd.jpl.nasa.gov` absent; Lauf
  `35569486280` in_progress („final combine not reached"); Größe `Pending`
  (`phi/footprints.φ:19`).
- **Blockade:** Lauf-Abschluss.
- **Braucht:** `ci_manage view 35569486280` / `ci_manage log`; bei Final-Combine
  Größe → `footprints.φ:19`.

### DEMETER
- **Status:** wartend | **Bindung:** termin (Run `35567568429`)
- **Lage:** Re-Dispatch `demeter-cdn.yml` queued; `rs-catalog` 500 (eigener
  Register-Befund).
- **Blockade:** Lauf-Abschluss.
- **Braucht:** `ci_manage view 35567568429`; bei success 77 `url`+`sha256`-Zeilen
  registrieren.

### Quaoar sha256
- **Status:** wartend | **Bindung:** termin/dritter (Zenodo)
- **Lage:** `zenodo.21185812` (572 467 032 B) — Zenodo 504, sha256 `pending`.
- **Blockade:** Zenodo-Rückkehr.
- **Braucht:** `archive_search --verdict https://zenodo.org/api/records/21185812`.

### Babamul / IA2 TAP — Register-Korrektur
- **Status:** offen | **Bindung:** eigen
- **Lage:** beide `pending` mit „kein gebauter Konsument"
  (`phi/blocked_sources.φ:3,35`), obwohl die Routen mit Token HTTP 200 liefern.
  Nach AGENTS.md ist der Konsument kein Quellen-Verdikt, sondern eine
  Bau-Reihenfolge.
- **Blockade:** keine.
- **Braucht:** auf `ernte` re-taggen oder den Konsumenten-Bau als Schritt
  benennen (`grind-flash`).

### SSDC Limadou
- **Status:** operator-gebunden | **Bindung:** operator (→ `entscheid`)
- **Lage:** CAS-Login lädt, „Permission Denied" für user omegaflow; Follow-up an
  den PI ist per-act consent.
- **Blockade:** Operator-Wort.
- **Braucht:** als `An entscheid`-Post gesetzt (`post.md`); Antwort des Operators.

### Wartend (kein Auswahlpunkt)

- **TAP-Backends** dachs.fai.kz + pithia.cbk.waw.pl (`ledger.φ`) — Backend down
  (PostgreSQL tot), Trigger sync-QUERY/tables 500→200.
- **Lasair-LSST API** — Backend 502; Trigger Erholung.
- **Sonden-Antworten** (Voyager/Mariner/Viking/Juno/Cassini) `blocked_sources.φ`
  — Anfragen 2026-09-16 raus, Antwort offen.
- **BepiColombo bc_mpo_more** — Freigabe ~April.
- **NRS02-10,12,13 SHAPE** `nrs_stations.φ:17` — Trigger neuer Stations-Prefix.
- **EMODnet HFRADAR NADR** — nächste Re-Messung **2026-10-19**.

## Geschlossen in dieser Session

- **Split-Routing-Verifikation** — nachgemessen (2026-09-21, `--verdict` direkt →
  Proton-Exit 169.150.218.57): `api.alerce.online/alerts/v1/objects/` **live**
  (200, der 000 war ein `--verdict`-Tool-Fehlalarm, per `curl` widerlegt),
  `zenodo.org/api/records/10594301` live, `api.lsst.fink-portal.org` live;
  `data.superdarn.ca`, `hfrnet-tds.ucsd.edu`, `api.alerce.online` (nackte URL)
  beidseitig gleich. **Kein Geo-Block** — die toten Hosts sind echt tot, der
  Tunnel öffnet keine Tür zusätzlich.

## Benchmark

- **Ernte-Folge 128** — 3 × `research-max` (ALeRCE / SuperDARN / HFRNet), vom
  Operator mit „harten Bandagen" angefordert. Kein flash-Doppel-Lauf; die zwei
  bereits entschiedenen Hosts (SuperDARN, HFRNet) waren flash-Klasse. Burn nicht
  gemessen.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `phi/declined_sources.φ`,
  `phi/pipeline/ledger.φ`, `phi/pipeline/frame_registry.φ` (generiert/gitignored),
  `docs/handover/post.md` (`An bau` Tool-Gap, `An entscheid` SSDC neu),
  `docs/zustand/external-state.md` (P1/Asset-Cap), neues Handover
  `handover-2026-09-21-ernte-folge128.md`, Move
  `handover-2026-09-21-ernte-folge127.md` → `archiv/`.
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
