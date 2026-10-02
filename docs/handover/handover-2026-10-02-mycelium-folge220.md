<!--
  title: Handover — Mycelium-Folge 220 (2026-10-02)
  session: Mycelium-Folge 220
  class: handover
  date: 2026-10-02
  sha256: 020199d6e9c665e1ee82394398d8fff9265106c80a3a82daf84e95102f7f58cc
  status: live
-->
# Handover — Mycelium-Folge 220 (2026-10-02)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-02-mycelium-folge219.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.2281 · cap 0.50 Grund: jwst-Manifest-Ursache + JADES-/DSN-Workflow + 8 CI-Läufe gemessen (gemessen `session_burn` $2.2130 → $2.4411)

## Operator-Wort-Register

- Wort | 2026-10-01 | „ich habe dir nicht erlaubt zu committen und zu pushen" | Quelle: Mycelium-Session 216.
- Wort | 2026-10-01 | „stehen lassen aber das wort ist du bist die letzte linie die committed das muss sitzen" | Quelle: Mycelium-Session 216 — Mycelium committet **als letzte** Linie, nur mit dem `/commit`-Wort.
- Wort | 2026-10-01 | „bitte nicht nur messen und verschleppen sondern bearbeiten messen und bearbeiten ist die prämisse mein dauerhaftes wort" | Quelle: Mycelium-Session 216.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209.
- Wort | 2026-10-01 | „ja möchte ich" | Quelle: Mycelium-Session 215 — VCO-rs-Register auf das PDS4-20190704-Asset umstellen.

## Haus (die vier Orte) — gemessen 2026-10-02

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02); `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all`; `phi/pipeline/catalog/*` ist gitignored, `phi/pipeline/index.φ` + `ledger.φ` trackbar.
- Manifestations-Direktiven (`url`/`origin`/`compiler`/`sha256`/Tags) schreibt Mycelium; die Verdikt-Zeilen (`ttl`/Zulassung/Disposition/`note`) schreibt Mountain exklusiv.

## Offen (aufgeschlüsselt)

### CDN-Workflows für JADES + DSN — gebaut, Push + Erstlauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `/commit` (Push) → `gh workflow run jades-cdn.yml` / `gh workflow run dsn-cdn.yml`
- **Lage:** (gemessen 2026-10-02) `.github/workflows/jades-cdn.yml` + `.github/workflows/dsn-cdn.yml` geschrieben; die Compiler `tools/harvest/src/bin/jades_spectra_compiler.rs` (`--out`/`--lsk`/`--workdir`/`--budget`/`--ci-mode`, Upload-Tag `jades.herts.ac.uk`) und `tools/harvest/src/bin/dsn_compiler.rs` (`--out`/`--ci-mode`, Upload-Tag `eyes.nasa.gov`) stehen, die Register-`url`-Zeilen `phi/sources.φ:9485` (`jades_spectra.bin`) und `:109` (`dsn_snapshot.bin`) zeigen auf die Releases; `gh workflow run` liest nur den default branch.
- **Blockade:** uncommittet
- **Braucht:** nach Push die zwei Läufe dispatchen und deren Lauf-Ende lesen; manifestiert `…/jades.herts.ac.uk/jades_spectra.bin` und `…/eyes.nasa.gov/dsn_snapshot.bin`; danach `archive_search --verdict` beider `url`-Zeilen (206 → schließen).

### jwst_spectra-Manifestation — Ursache geheilt, Re-Manifest nach Push
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `/commit` (Push) → `gh workflow run kernel-flatten.yml`
- **Lage:** (gemessen 2026-10-02 via `ci_manage log 36946571514 --all`) Der Job `jwst-spectra` (`kernel-flatten 36946571514`, success) druckte die Compiler-Usage-Zeile: `.github/workflows/kernel-flatten.yml:189` reichte `--window-start/--window-end/--decimate-min/--jobs` an `jwst_spectra_compiler`, der sie nicht parst (`_ => usage`, `jwst_spectra_compiler.rs:512-517`) → Abbruch vor `finalize_workdir`, kein Upload. Das Release `exoplanetarchive.ipac.caltech.edu` trägt kein `curated48_spectra.bin` (GH-API-Assets + `archive_search --verdict` = 404); der Alt-Ort `ssd.jpl.nasa.gov` trägt die Bytes weiter. Die vier ungeparsten Argumente aus `kernel-flatten.yml:189` entfernt.
- **Blockade:** uncommittet
- **Braucht:** nach Push `gh workflow run kernel-flatten.yml` dispatchen; nach Lauf-Ende `archive_search --verdict "https://github.com/omegaflow/sources/releases/download/exoplanetarchive.ipac.caltech.edu/curated48_spectra.bin"` — 206 → Punkt schließen.

### `blocked_sources.φ` mycelium-pending-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30) `:59` BepiColombo, `:85` MESSENGER, `:98` DEMETER, `:346` GOSAT-GW, `:374` DAS2 Iowa, `:378` Occultation-DB, `:402` ExoMars TGO, `:406` Akatsuki, `:410` Kaguya, `:414` Chandrayaan-1, `:418` Chang'e MRM, `:422` Tianwen-1 RoPeR, `:426` Phobos 2, `:430` Vega 1/2, `:434` Hayabusa, `:438` Tianwen-1 MoRIC, `:442` Shandong, `:458` Danuri ShadowCam, `:462` CDSE-CCM.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### Register-Träger — `phi/pipeline/index.φ` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Katalog-Port (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-02 via `register_lookup --open`) Katalog-Offenstand 5 (Arbeitsdateien gitignored).
- **Blockade:** Porting offen
- **Braucht:** je Katalog die erreichbaren Kandidaten über `docs/SOURCE_PORT.md` portieren.

### Register-Träger — `phi/pipeline/ledger.φ` SSDC
- **Status:** termin | **Bindung:** termin:2026-10-02
- **Trigger:** neue SSDC-Zugangsprozedur (`https://limadou.ssdc.asi.it/query.php`)
- **Lage:** (gemessen 2026-10-02 via `archive_search --verdict`) `query.php` HTTP 200 / 6894 B — die CAS-Login-Seite, nicht der Datenzugang (Wall bestätigt; `ledger.φ:6` `ausstehend`); `state/zustand/wartend.φ:10` (laic-cses) verlangt nach Termin `archive_search --playwright`, CAS-Login = Gate zu.
- **Blockade:** Prozedur nicht live
- **Braucht:** `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"` sobald SSDC den offenen Zugang meldet (Antwort Sotgiu / neue Prozedur).

### `http_401`-Residuum
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue Mail/Asset-Messung
- **Lage:** (gemessen 2026-09-30) nach der GitHub-PAT-Rotation kein neuer 401.
- **Blockade:** keine
- **Braucht:** weiter beobachten.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383`)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

## LOCK

- **`rr-brustgurt`** (Operator): LOCK — Hardware erst bei Förderung; Live-BLE HR NotSupported → keine RR; FIT `nn=0` (gemessen 2026-09-26); Brustgurt Polar H10/HRM-Dual. (`state/zustand/wartend.φ:22`)

## An river

Origin: mycelium-folge220 (CI-Tafel).

- **tapvizier-TAP-Klasse (`rave-cdn 36944700194` / `sb9-cdn 36944703250` / `first14-cdn 36944707063`, Re-Dispatch @`60ba8dccf`):** alle drei erneut `failure` — dieselbe Ursache wie zuvor: `tap_query http exit status: 22: curl: (22) ... error: 400` (dreimal, gemessen via `ci_manage log 36944700194`); `sb9`/`first14` gleiche Klasse (`uws job phase ERROR`). Die transiente Deutung ist widerlegt: die von `tap_compiler` erzeugte ADQL (`--table III/279/rave_dr5 … --crossmatch I/355/gaiadr3 … --where "t.\"RAJ2000\" >= … "`) wird deterministisch abgelehnt, während die handrekonstruierte rave-ADQL 200 liefert. Feder `tools/harvest/src/bin/tap_compiler.rs` (Query-Aufbau `:1077-1216`); Riss: hand-ADQL 200 ↔ CI-ADQL 400. Bitte die CI-generierte ADQL gegen `tapvizier` messen und den Query-Aufbau heilen.
- **Orphan-Docs (Meta-Klasse):** `register_lookup --orphan-docs` nennt 5 trägerlose Dokumente (gemessen 2026-10-02; folge219 trug sie, der Archiv-Move hat die Trägerschaft gelöst) — je Dokument fehlt der Owner-Träger:
  - `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (1 Marker) — Sektion `## 7. Offene Punkte — historisch, alle geschlossen` nennt alle Punkte geschlossen; der Scanner liest das Wort „offen" im Titel. Bitte Überschrift auf geschlossen umbenennen (oder descopen).
  - `docs/paper/flyby-path-2-addendum-2026-09-29.md` (26) — Trägerzeile oder gemessenes `descoped`.
  - `docs/concepts/kybernetische-astrophysik.md` (1) — dito.
  - `docs/concepts/exzellenz-konzept.md` (2) — dito.

## An mountain

Origin: mycelium-folge220 (Orphan-Zensus).

- **Orphan-Docs:** `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (1 Marker) und `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (5 Marker) tragen offene Marker ohne Live-Träger (gemessen 2026-10-02 via `register_lookup --orphan-docs`). Bitte je Dokument eine Trägerzeile in deiner Übergabe setzen oder ein gemessenes `descoped` — die Marker sind teils Prosa, Marker-Review zuerst.

## An future

Origin: mycelium-folge220.

- **GitHub-Issues:** `gh issue close` ist in `opencode.json` strukturell verweigert (nur `list`/`view`). Gemessen geheilt und schließbar: #113 (dropped-Baseline 1339) + #114 (de44-cdn success). #115 (harvest `36738406375` failure → parser-gap, Mountain) und #81 (clippy, alter SHA; river 78 geheilt) brauchen eine neue Messung. Bitte in die Operator-Queue: `gh issue close 113 114`, dann `#47–#53` gegen die letzten `*-cdn`-Läufe prüfen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
