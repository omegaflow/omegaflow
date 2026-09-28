<!--
  title: Handover — Mycelium-Folge 194 (2026-09-28)
  session: Mycelium-Folge 194
  class: handover
  date: 2026-09-28
  sha256: 9b4f9619f5c06119aa3a53bc4f662fa41b4e70b5298f02dde15aca16a6b84449
  status: live
-->
# Handover — Mycelium-Folge 194 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`;
Operator-Akte leben in Futures Operator-Queue, Dritt-Waits in
`state/zustand/wartend.φ`, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-28-mycelium-folge193.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-28 | „Erste Handlung: `sread docs/concepts/tool-forms.md` — die Form-Karte … damit die erlaubte Form am Punkt der Handlung steht." | Quelle: Mycelium-Session 194.
- Wort | 2026-09-28 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher … Eine Session ist ein abgeschlossenes Atom." — session-weiter Consent (Delegation), **nicht** das Commit-Wort | Quelle: Mycelium-Session 194.
- Wort | 2026-09-28 | „hast du alle mails im eingang geprüft?" / „nein du siehst nicht alles siehe nachricht von future" — der Eingang wird vollständig gelesen (absender/getaggt, nicht nur der jüngste), und die **Absender-Handover** der anderen Linien werden gelesen, bevor der Pass schließt | Quelle: Mycelium-Session 194.
- Wort | 2026-09-28 | „weil du head - liest das ist echt problematisch und eine llm seuche" / „ich möchte dass das strukturell gefixt wird … ihr sollt nicht den kompletten kontext einlesen aber nach der suche das ergebnis nicht kastrieren" — `head`/`tail` sind **strukturell verweigert** (opencode.json, form-guard, tool-forms, AGENTS); Lesen ist Fenstern (`--offset/--limit`), kein Anschneiden; gekappter Output wird vollständig aus der Spill-Datei gelesen | Quelle: Mycelium-Session 194.
- Wort | 2026-09-28 | „copilot darf auf keinen fall meine daten in die cloud schicken ich gebe eh schon viel zu viel preis" — private/`state/`-Daten verlassen die Maschine **nie** an eine zweite Cloud-Stimme; die lokale Copilot-CLI ist gestrichen (`opencode.json`-Deny, form-guard, `bin/copilot_ask`/`bin/copilot_review` verweigern) | Quelle: Mycelium-Session 194.
- Wort | 2026-09-28 | „public github würde doch die analyse deutlich vereinfachen" / „einer der größten reibungsflächen ist dass wir uns bei ci jobs blind stellen" — **public** Daten dürfen über den Sandkasten gehen; der rote CI-Lauf wird via `bin/ci_triage` analysiert (public-only, leeres Temp-Dir) | Quelle: Mycelium-Session 194.
- Wort | 2026-09-28 | „prüft copilot auch das was auf die öffentlichen repos geht beim eingang? … ob code und assets und docs unsere hausregeln tragen" — Nachschau auf das **Veröffentlichte**: der public Baum wird gegen die Hausregeln geprüft (`bin/house_audit` mechanisch, `bin/public_audit` semantisch) | Quelle: Mycelium-Session 194.
- Wort | 2026-09-28 | „alles drei" — PII-Funde redigieren, `Archivers`→`Archivars` fixen, `public_audit` re-messen | Quelle: Mycelium-Session 194.
- Wort | 2026-09-28 | „vorher agents riss" — der AGENTS-Widerspruch (Z. 209 vs 215/217) wird dem Rat vorgelegt, **bevor** committet wird | Quelle: Mycelium-Session 194.
- Wort | 2026-09-28 | „ja bitte und dann /commit" — Handover + Wort-Register schreiben, dann Commit | Quelle: Mycelium-Session 194.

## Offen (aufgeschlüsselt)

### register-coverage — der CI-Riss (privates Handover unsichtbar)
- **Status:** wartend | **Bindung:** eigen (Workflow) ← Mountain (Tool-Arm)
- **Trigger:** Mountain liefert den Tool-Arm `UNVERIFIABLE_PRIVATE` in `tools/register`.
- **Lage:** (gemessen 2026-09-28 via `bin/ci_triage 36413456780`) der Job `orphans` exitet 2 mit `4 orphan entries [future 4 committed 0 uncommitted]`; lokal meldet `register_lookup --orphans` **0**, weil Futures privates Handover den Träger hält — der CI-Runner sieht den gitignored Baum nicht. Rat-Verdikt **Form A**: die Abwesenheit als `UNVERIFIABLE_PRIVATE` benennen (aus dem `--fail`-Zähler nehmen); lokaler Vollcheck bleibt Sitzungspflicht.
- **Blockade:** der Tool-Arm fehlt (`tools/register` = Mountain).
- **Braucht:** `tools/register` — `UNVERIFIABLE_PRIVATE`-Arm; danach `.github/workflows/register-coverage.yml:22` verifizieren (Feder Workflow = mycelium).

### public_audit — Ganz-Baum-Nachschau des public Baums
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort — der scoped Lauf `d85db4c4-45e1-4fc7-beb6-c0ed8b418cfe` steht.
- **Lage:** (gemessen 2026-09-28, scoped `docs/concepts/*` via `bin/public_audit`, 1,38 Cr) der Pfad trägt: Sandkasten `git archive HEAD | tar`, Copilot plan-los, `Changes +0 -0`; Befunde auf stdout. Der Ganz-Baum-Lauf ist **ungemessen**.
- **Blockade:** Credit-/Egress-Entscheid.
- **Braucht:** `bin/public_audit` (Operator-Wort).

### tools-map — die vier Werkzeuge verdrahten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Planungs-Pass — `docs/concepts/tools-map.md` trägt die vier noch nicht.
- **Lage:** (gemessen 2026-09-28) `bin/session_check`, `bin/ci_triage`, `bin/house_audit`, `bin/public_audit` stehen, fehlen aber in `docs/concepts/tools-map.md`.
- **Blockade:** keine.
- **Braucht:** `sread docs/concepts/tools-map.md` + Eintrag + `omega_sh sha docs/concepts/tools-map.md`.

### ci-check — clippy geheilt; Bestätigungslauf pending
- **Status:** wartend | **Bindung:** eigen (CI-Aufsicht)
- **Trigger:** Ende des lebenden `ci-check`-Laufs.
- **Lage:** (gemessen 2026-09-28 11:0xZ via `ci_manage status`) `36413456788` pending, `36409581203` (test) in progress; der clippy-Rot `src/archivar/odf.rs:209` ist in `9f8debcc3` geheilt (Baseline 1054).
- **Blockade:** keine.
- **Braucht:** `ci_manage log <id>` — clippy grün + delta 0.

### ci_watchdog — Matcher heilt echte Runner-Shutdowns
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster transienter Rot-Lauf, belegt in `ci_watchdog.log`.
- **Lage:** (gemessen 2026-09-28) `bin/ci_watchdog.sh` prüft transient zuerst; Assertion-Klasse auf echte Rot-Marker begrenzt; `bash -n` 0.
- **Blockade:** keine.
- **Braucht:** der nächste Shutdown-Rot trägt „rerun … measured transient cause".

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster periodischer `hinet-cdn`-Lauf.
- **Lage:** (gemessen 2026-09-27) `36344350143` rot, Job-Log `unread`; Vorlauf 8× `attempt stayed unready`, Auth 200.
- **Blockade:** quellenseitige Readiness (Hinet).
- **Braucht:** `ci_manage jobs 36344350143` beim Trigger.

### D5-Orphan-Residuum — CDN-Manifestations-Weg + P2P
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:288`).
- **Lage:** (gemessen 2026-09-27) kein Producer-Bin, keine Register-Zeile, kein `*-cdn.yml`; der generische Weg (`src/archivar/cdn.rs` `upload_release`) steht.
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante bis der Producer steht.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige Routen) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

### Register-Träger — 9 Ledger-Port-Kandidaten (gemessen, Port steht aus)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Port-Pass — `phi/pipeline/ledger.φ` trägt die neun `ausstehend`.
- **Lage:** (gemessen 2026-09-28 durch drei Taucher; Register steht `phi/pipeline/ledger.φ` `ausstehend`) je Kandidat eine Messung, kein `gap` → kein Klassen-Träger:
  - Akatsuki RS (`data.darts.isas.jaxa.jp/pub/pds4/data/vco/vco_rs/`): F5-WAF (transient 503, `--playwright` passiert), anonym erreichbar; L1-`.img` + `.lblx`; Kraft **em**; **parser-def-Risiko** (PDS4-Binary, kein Parser-Arm) → Compiler nötig.
  - Hayabusa (`sbnarchive.psi.edu/pds4/hayabusa/`): 200 anonym; LIDAR `.tab` (MET/UTC/RANGE); Kraft **em**.
  - Kaguya/SELENE LRS (`ode.rsl.wustl.edu/…/KAGUYA(SELENE)/LRS/`): 200 anonym; L1-Roh, **em**.
  - Chandrayaan-1 (`pds-geosciences.wustl.edu/missions/chandrayaan1/`): 200; Mini-RF `.img` (PDS3) + M3 ENVI-Kubus; **em**; **parser-def** (`pds3-image`, `envi-cube`).
  - Venera 15/16 (`pds-geosciences.wustl.edu/venera/`): Altimetrie → **gravity**, Radiometrie → **thermal**; ASCII, Einheiten belegt.
  - Vega 1/2 (`pds-smallbodies.astro.umd.edu/holdings/vega2-c_sw-mischa-3-rdr-original-v1.0/`): **Ledger-URL 404** (Pfad-Drift; real mit `_sw`); 200; Kraft offen.
  - Phobos 2 KRFM (`pds-smallbodies.astro.umd.edu/holdings/phb2-m-krfm-3-photometry-v1.0/`): 200; `krfm.dat` PDS3-fixed-width, Spalten ohne Unit; **em**; **parser-def** (`unit-auto-detect`, `pds3-fixed-width`).
  - ExoMars TGO ACS (`archives.esac.esa.int/psa/ftp/ExoMars2016/em16_tgo_acs/`): 200; NIR-Okkultationsspektrum; **em** → accept-Kandidat. FREND (Neutronen) → Kraft **undetermined** (kein Kanal der 9) → eigener pending-Punkt.
  - Danuri/KPLO (`pda.kasi.re.kr/` + ShadowCam): 200; Kraft offen.
- **Blockade:** keine.
- **Braucht:** Port über `docs/SOURCE_PORT.md` + Disposition in Register; Vega-Pfad korrigieren.

### Träger (Prosadokumente, eigene)
- `docs/concepts/arxiv-api.md` | offene Marker (2) | `register_lookup --orphan-docs` beim nächsten Pass.
- `docs/concepts/exzellenz-konzept.md` | offene Marker (3) | dito.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | Step 5 Familien-Identität (sha `39da4f17…`) | nächster Zensus-Pass.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | wartend Mail-Eingang | Trigger echte SAMPLE_CONTACT-Mail.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Layout-Wort in Future-Queue | Migration nach Wort.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | §7 Roh-Korpora-Disposition | nach Wort.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending tot | `--verdict` je Host beim Trigger.
- `docs/concepts/tools-map.md` | vier neue Werkzeuge fehlen | Eintrag + sha.
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` | Operator lädt Fork-Build unpacked (Future-Queue) | danach Marker schließen.

## Weitergabe (fremde Feder — Aufenthalt beim Eigentümer)

- **register-coverage-CI-Riss** (gemessen 2026-09-28 via `ci_triage 36413456780`): `4 orphan entries [future 4 committed]` — der CI-Runner sieht das private Handover nicht; lokal 0. Ziel: **Mountain** (`tools/register`, Arm `UNVERIFIABLE_PRIVATE`). Quelle: Future folge147 + Mycelium 194.
- **PII in der Git-Historie** (gemessen 2026-09-28 via `house_audit`): die private Adresse/Mail stand in getrackten Docs; HEAD ist redigiert (4 Docs, sha neu), die **Historie trägt sie weiter** → GitHub-GC-Ticket #4761801. Ziel: **Future** (Operator-Akt History-Rewrite). Quelle: Mycelium 194.
- **Register↔CDN-Riss** + **zwei `format`-Findings** (aus folge193): vier register-`url`-Zeilen 404 auf dem CDN; `vizier.cfa.harvard.edu`/`www.ldeo.columbia.edu` als JSON geparst. Ziel: **Mountain** (Quelle-Identität/`format`). Quelle: Mycelium 192/193.
- **Atomic state write** (`save_state` in-place, `src/mathematikerin/machines/matrix.rs:334`). Ziel: **River** (temp+rename + Test). Quelle: Council 2026-09-28.
- **Copilot-Streichung an die Linien**: `docs/handover/handover-2026-09-28-river-folge55.md` und `…mountain-folge194.md` nennen Copilot als Stimme; die Cloud-Grenze (private Daten nie) + public-CI-Ausnahme (`bin/ci_triage`) gilt für alle. Ziel: **river**, **mountain** (per Absender-Zeile). Quelle: Mycelium 194.

## Absender-Zeilen (eingehend, gefaltet)

- future folge147 → mycelium: der Postfach-Abschnitt des Stehenden Passes trug **2** handlungsrelevante Eingänge; die Vollprüfung der 180 Mails fand **acht** — der Pass liest den Eingang vollständig (absender/getaggt). Zugleich der Register↔CDN-/CI-Riss (privates Handover fürs CI unsichtbar). Quelle: `state/future/handover/handover-2026-09-28-future-folge147.md:129-132`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
