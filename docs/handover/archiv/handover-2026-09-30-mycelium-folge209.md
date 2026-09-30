<!--
  title: Handover — Mycelium-Folge 209 (2026-09-30)
  session: Mycelium-Folge 209
  class: handover
  date: 2026-09-30
  sha256: 11f3a9f6c3b6a3e563b880d8ca30ebea794588cddbf0a88ddbf6679f21ae5bbe
  status: live
-->
# Handover — Mycelium-Folge 209 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-09-30-mycelium-folge208.md` (→ `archiv/`).

## Operator-Wort-Register

- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209 (Consent Phase 2 + Kante-Direktive).
- Wort | 2026-09-30 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)." | Quelle: Mycelium-Session 208 (session-weiter Consent, Delegation).
- Wort | 2026-09-29 | „Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)." | Quelle: Mycelium-Session 207 (session-weiter Consent, Delegation).
- Wort | 2026-09-29 | „bitte fixen Rest — 18 offen, echte Ursachen" | Quelle: Mycelium-Session 206.
- Wort | 2026-09-29 | „ja beides" — Issues-Zensus+Triage autonom; Issues-Stand in den Stehenden Pass | Quelle: Mycelium-Session 206.
- Wort | 2026-09-28 | „ja bitte commit erst wenn alle anderen sessions committed sind" | Quelle: Mycelium-Session 196 (geteilter Baum; pfad-begrenzt committen).

## Haus (die vier Orte) — gemessen 2026-09-30

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`,
  Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ `omegaflow-legacy-backup-2026-09-02`).
- `temp` = `/tmp/opencode`.
- `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` —
  `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep`
  ohne `--all` über den gitignorierten Baum.

## An-Blöcke — gefaltet (2026-09-30)

- **mountain-208** (frame-registry Re-Dispatch, `harvest.φ:251`, Rosetta-origin,
  `ersstv5`-403, CDN-Manifestationen) und **sensory-210** (trägerloser
  `survey-2026-09-07-tmp-opencode-scan`, Orphan-Zensus-Riss) sind in diesem Atom
  gefaltet. Gemessen und im selben Atom abgearbeitet:
  - `frame_registry.φ:72-74` trägt **3** rosetta_odf-Zeilen (== `sources.φ`):
    der Re-Dispatch `frame-registry 36676501603` (Commit `aa894a859`) hat den
    Schedule erfüllt; kein weiterer Dispatch nötig.
  - `harvest.φ:251`-note auf **3 Shards** korrigiert (de-dup `7e2cefd9f`).
  - Rosetta-IFMS-origin re-messen: alte Route 404 (direct+Proton+Wayback);
    PSA-Baum umgebaut → RSI-Root live (200). `origin` auf
    `…/INTERNATIONAL-ROSETTA-MISSION/RSI/` erneuert (`sources.φ:8950/8960/8970`);
    `blocked_sources.φ:89` auf `descoped` disponiert (Asset+sha auf CDN).
  - `survey-2026-09-07-tmp-opencode-scan.md` → `status: done` (Marker war Prosa
    im entschiedenen §7-Nachsatz).
  - Der Pass-Orphan-Riss (survey-messpunkt-verteilung = River) ist im Stehenden
    Pass bereits korrigiert (Zeile 47-48).

## Offen (aufgeschlüsselt)

### ci-gate / ci-check nach Baseline-Bump
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-gate`/`ci-check`-Lauf am neuen HEAD
- **Lage:** (gemessen 2026-09-30 via `ci_manage jobs 36676970377`) `ci-gate`
  36676970377: clippy/build/format **success**, nur `dropped-gate` failure —
  baseline 1148 | current 1157 | delta 9 (die aufgelaufenen Plan-Pass-Drops).
  Die Baseline ist in diesem Atom auf **1157** gebumpt
  (`docs/zustand/dropped-baseline.md`). `ci-check 36676970291` pending;
  `register-coverage 36676970316` success; `paper-check 36676970282`/`36676975491`
  success → **#98 geschlossen**.
- **Blockade:** keine
- **Braucht:** neuen `ci-gate`/`ci-check`-Lauf lesen (`ci_manage jobs <id>`);
  bei grün ist der ci-Rot-Stand leer.

### te-gate fpr-membrane — River-Entscheid
- **Status:** wartend | **Bindung:** eigen ← river
- **Trigger:** `te-gate.yml`-Lauf
- **Lage:** (gemessen 2026-09-30 via `ci_manage view 36639308643`) Lauf failure,
  hat **#112** erzeugt (te-gate-Finding rot bestätigt); `#13` (n=1000 FPR) +
  `#43` (measure-gates) bleiben.
- **Blockade:** keine
- **Braucht:** `ci_manage log 36639308643`; River-Entscheid (Phase vs. Arx-Switch).

### HiPS-Tree-Arm — erster Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `hips-png-cdn`-Lauf
- **Lage:** (gemessen 2026-09-30 via `ci_manage status`) `hips-png-cdn 36675582975`
  **in_progress** (Schritt `hips-png-shard (4,0) → Harvest one HiPS Dir shard`);
  älterer Lauf 36639304016 in_progress (262 140 Kacheln, Norder 0–7).
- **Blockade:** keine
- **Braucht:** `ci_manage log 36675582975` nach Lauf-Ende — der erste Lauf misst
  den echten Kachel-/Sha-Stand (0 honored: keine Kachel fabriziert).

### Legacy-CDN Re-Manifest — Produzenten-Route unerreichbar
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `nvss-cdn`/`first14-cdn`/`kernel-flatten`-Lauf
- **Lage:** (gemessen 2026-09-30 via `ci_manage log`) Die Umschreibung auf
  Produzenten-Tags ist **nicht ausführbar**: `nvss-cdn 36639771707` +
  `first14-cdn 36639775480` scheitern, weil `tapvizier.cds.unistra.fr` async-Jobs
  nach 9000 s auf `EXECUTING` stehen → `async returned void` (Issue „flatten: nvss
  returned void" offen). `kernel-flatten 36639779027` scheitert separat: `bodies`
  `spk_split: target 2174567: segment data begins before address 1828776577 — the
  segment's summary arrived after its data` → `curl: (23)`, exit 1. Die Legacy-Assets
  bleiben unter `ssd.jpl.nasa.gov` 200/206 (`sources.φ:10606/:10476/:9208`).
- **Blockade:** tapvizier async down (Produzent); spk_split-Ordnung (Kern)
- **Braucht:** alternativer TAP-Endpunkt (`archive_search --verdict
  "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync"`) oder descopen; `spk_split`
  als An mountain (Kern-Bug); Legacy-URLs **nicht** umschreiben, solange die
  Produzenten route nicht trägt.

### kuprat Family-Tag — Phantom-Name
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission der vier Kuprat-Kanäle (`witnesses.φ`)
- **Lage:** (gemessen 2026-09-30) kein Compiler setzt `tag kuprat` (`sgrep kuprat
  tools` = nur Prosa/Bin-Namen); `…/releases/tag/kuprat` = 404. Die Assets liegen
  real unter gemessenen Tags: `crystallography.net/rixs_spin.bin`,
  `…/rixs_charge.bin`, `…/eels_acoustic.bin` (`crystal_compiler.rs:405/434/477/517`),
  `srdata.nist.gov/srd62_suprastrom.bin` (`srd62_compiler.rs:259`).
- **Blockade:** kein `kuprat`-Tag-Host (verstößt gegen `<host>-<family>`-Form)
- **Braucht:** An-Block an Mountain; kein Mycelium-Tag-Akt.

### future-155/156 Sources-Zeilen — Aufnahme-Regel
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-`ttl`/`frame`-Zeilen (`phi/sources.φ`)
- **Lage:** (gemessen 2026-09-30) Keiner der neun Endpunkte aufnehmbar:
  `parse.rs:80-85` verlangt `ttl>0`/`no-cadence` **und** `frame`/`extract`.
  Register-reif (nur Mycelium-Direktiven fehlen): ShadowCam (`url`+`format
  pds4-fits`), ESA PSA TAP (`url` ADQL+`format tap`), Chang'e-MRM (`url`+`format
  pds4-fits`).
- **Blockade:** Mountain-Feder (ttl/frame)
- **Braucht:** An-Block an Mountain; ShadowCam-Download = Operator-Hand.

### KARI KPDS (Danuri/KPLO) — pending
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** KARI-Portal-Antwort / neuer Daten-Endpunkt (`phi/blocked_sources.φ:454`)
- **Lage:** (gemessen 2026-09-30) `phi/blocked_sources.φ:454` pending:
  https://www.kari.re.kr/kpds/ HTML-Portal HTTP 200 (3969525 B); kein
  maschinenlesbarer Daten-Endpunkt gemessen; `extract.rs:2689` HTML-Arm steht —
  kein parser-def, kein gap.
- **Blockade:** kein Daten-Endpoint
- **Braucht:** `archive_search --verdict "https://www.kari.re.kr/kpds/"` bei neuer
  Portal-Antwort oder descopen.

### GitHub-Issues — Zensus
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Issues-Zensus (`gh issue list`; `#98` geschlossen)
- **Lage:** (gemessen 2026-09-30) **#98 geschlossen** (paper-check grün,
  `gh issue close 98`). Offen u. a. `#81` (clippy), `#80` (Anomaly-Report,
  unread), `#58/#15` (cargo-test-GPU-Rot), `#60/#17` (recheck-live), `#13/#112`
  (te-gate), `#43` (measure-gates).
- **Blockade:** keine
- **Braucht:** Rest nach Heilung/freiem Lauf schließen.

### Gate-Umbau träger-bewusst
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --dropped`-Lauf
- **Lage:** (gemessen 2026-09-30) `register_lookup --dropped [<line>]` bricht
  lokal nach **>10 min** ab (zweimal, je 300 s/600 s) — der selbst-messende,
  träger-bewusste Umbau steht; die Gate-Zahl ist CI-only. Baseline in diesem Atom
  auf 1157 gebumpt.
- **Blockade:** keine
- **Braucht:** Design-Akt: `--dropped` selbst-messend/träger-bewusst (Laufzeit +
  Träger-Auflösung), damit getragene Punkte nicht als Drop zählen.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `bepicolombo-more`
- **Lage:** (gemessen 2026-09-30 via `state/zustand/wartend.φ`) Wiedervorlage,
  Aufnehmer mycelium.
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

### Quellenseitige Waits
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** hinet-CDN-Readiness / Antworten
- **Lage:** (gemessen 2026-09-30 via `ci_manage log 36607421243`) `hinet-cdn`
  failure: „cont status never read Available — the request stays unfetched".
  NSSDCA/JPL-Antworten (`mariner10`/`viking`/`cassini-trk`/`juno-jplnav`) offen
  (`mail_ledger`).
- **Blockade:** Quellen-Readiness/Antwort
- **Braucht:** Re-Dispatch `hinet-cdn` bei Readiness; Antworten aus dem Postfach.

### Trägerlose Docs
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs`-Pass
- **Lage:** (gemessen 2026-09-30 `register_lookup --orphan-docs`) **3** (der
  Scan-Survey ist descoped): `survey-2026-09-03-orphan-verdicts.md` (1, echter
  offener Akt: Klassen-Zensus über die 315 Workflows / „Step 5 (CDN-kanonisch)
  bleibt offen"), `docs/concepts/exzellenz-konzept.md` (2, Prosa-Vokabular
  `pending`/`offen` im 0-Kanon — kein offener Akt), `docs/concepts/
  kybernetische-astrophysik.md` (1, Prosa-Vokabular).
- **Blockade:** die Marker ohne echten offenen Akt
- **Braucht:** die zwei Konzept-Docs als Träger benannt lassen (Prosa-Marker);
  `survey-2026-09-03-orphan-verdicts.md` Step 5 umsetzen oder descopen.

### ersstv5 403 — Diagnose
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** `ersstv5-cdn`-Lauf mit Diagnose-Step
- **Lage:** (gemessen 2026-09-30, mountain-208) 403 runner-spezifisch; lokal 200
  (14 999 659 B); `fetch.rs:149` ohne UA widerlegt. Braucht Header+Body-Beleg.
- **Blockade:** Diagnose-Step fehlt
- **Braucht:** `curl -g -D - -o /tmp/body '<URL>'` im `ersstv5-cdn`-Workflow-Log;
  Fix erst nach dem Beleg.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383` §14.4)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

## An mountain

Origin: mycelium-folge209.

- **spk_split-Ordnungsfehler** (Kern, blockiert `kernel-flatten`):
  `target 2174567: segment data begins before address 1828776577 — the segment's
  summary arrived after its data` → `curl: (23)` (`ci_manage log 36639779027`,
  2026-09-30). Träger: mountain.
- **Drei register-reife Endpunkte** brauchen `ttl`/`frame` (sonst fällt jeder
  Block durch `parse.rs:80-85`): ShadowCam (`at moon`+`ttl 604800`), ESA PSA TAP
  (`ttl 604800`+`field`/`at`), PDS Chang'e-MRM (`no-cadence`). Mycelium setzt
  dann `url`+`format` (`blocked_sources.φ:447-449`/`:59-62`/`:410-412`).
- **Vier Kuprat-Kanäle** (`witnesses.φ:124/130/136`) sind auf der CDN (206),
  aber 0 Register-Zeilen; `tag kuprat` = Phantom/404 — die Admission nutzt die
  **gemessenen** Tags.
- **Risse:** `blocked_sources.φ:443` (`gap html-parser-arm`) widerspricht
  `extract.rs:2689`; `mountain-207` nennt KASI-MOC-Heimat `phi/footprints.φ` FP01 —
  der Baum trägt dort keine KASI-Zeile.

## An river

Origin: mycelium-folge209.

- **`te-gate` #112** (gemessen 2026-09-30, Lauf 36639308643): rot bestätigt;
  `#13`/`#43` bleiben. Braucht River-Entscheid Phase vs. Arx-Switch.
- `ci-gate` clippy/format sind am HEAD **grün** (Mountain `spatial.rs`,
  River `te.rs`/`fam_calibration.rs` geheilt) — nur `dropped-gate` war rot;
  Baseline in diesem Atom gebumpt.

## An future

Origin: mycelium-folge209.

- **Secret-Rotation (dringend):** ein `grind-flash`-Taucher führte
  `sgrep -i TOKEN .secrets.local` aus; Secret-Werte landeten im Modell-Transcript.
  Operator-Wort zur Rotation der berührten Keys erbeten; Taucher-Prompts müssen
  `.secrets.local`-Werte ausdrücklich verbieten.
- **NSE-Redistribution + Dank** — Reply `state/mail/[redacted].md`
  wartet; Empfehlung: um die Lizenzfrage erweitern (gemessen 2026-09-28).
- **ENSO-Zuschnitt** — `ersstv5-cdn` success; offen nur der Blatt-Zuschnitt
  (Empfehlung NINO3.4).
- **Kuprat-Zeugenart** — Rat trägt „substance"; `witnesses.φ` bleibt bei vier Arten.
- **`gic-causal-driver.md` DOI-Minting** — DOIs `pending`; erst nach Einfrieren.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
