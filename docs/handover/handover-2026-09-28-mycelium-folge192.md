<!--
  title: Handover — Mycelium-Folge 192 (2026-09-28)
  session: Mycelium-Folge 192
  class: handover
  date: 2026-09-28
  sha256: a858cb430ae1c99f5bf18204ba3567c08f27eccfa348a595dd0a16155f2d35b3
  status: live
-->
# Handover — Mycelium-Folge 192 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** /
**Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` | `termin`;
Operator-Akte leben in Futures Operator-Queue, Dritt-Waits in
`state/zustand/wartend.φ`, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-28-mycelium-folge191.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-28 | „warum broweranbindung store review haben wir das nicht schon längt geforkt und gepinnt a - d /consent" — der Store-Review-Pfad ist überholt (Fork `tools/browser-extension/` + MCP-Pin `opencode.json:158-163` stehen); Consent zur Ausführung des Plans A–D | Quelle: Mycelium-Session 190.
- Wort | 2026-09-28 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | Quelle: Mycelium-Session 191.
- Wort | 2026-09-28 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher (alle Sub-Agenten), höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Eine Session ist ein abgeschlossenes Atom." — session-weiter Consent (Delegation), nicht das Commit-Wort | Quelle: Mycelium-Session 192.
- Wort | 2026-09-28 | „fixen statt verschleppen, mein wort" — ein arbeitbarer gemessener Befund wird im Atom gebaut, nicht als Handover-Punkt getragen | Quelle: Mycelium-Session 192.
- Wort | 2026-09-28 | „beides, mein wort" — den `cdn-health`-Workflow (Ersatz für `cds_watchdog`) und die Copilot-CLI als read-only Recherche-Stimme bauen | Quelle: Mycelium-Session 192.

## Offen (aufgeschlüsselt)

### ci-check — clippy geheilt; Bestätigungslauf pending
- **Status:** wartend | **Bindung:** eigen (CI-Aufsicht)
- **Trigger:** Ende des ci-check-Laufs `36385567226`.
- **Lage:** (gemessen 2026-09-28T06:20Z via `ci_manage view 36385567226`) `pending`, head_sha `302063d36`; der Vorgängerlauf `36385522146` wurde `cancelled` (superseded, kein Code-Rot). Der clippy-Rot `src/archivar/odf.rs:209` ist im Atom `9f8debcc3` geheilt; Baseline 1054.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36385567226` — clippy grün + delta 0 gegen Baseline 1054.

### gosat-cdn — Leer-Monat-Skip gebaut; Bestätigungslauf pending
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ende des `gosat-cdn`-Laufs `36385537670`.
- **Lage:** (gemessen 2026-09-28T06:20Z via `ci_manage view 36385537670`) `pending`, head_sha `87b200c02`; 0 Granules endet mit `exit(0)` + benannter Skip-Meldung an `tools/harvest/src/bin/gosat_tanso3_compiler.rs:1180`; `cargo check` 0/0.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <neu>` — alle Monate grün.

### modis-cdn — Split fertig, Lauf dispatcht; Familien-Bindung gebaut
- **Status:** wartend | **Bindung:** eigen (CI-Aufsicht)
- **Trigger:** Ende des dispatchten `modis-cdn`-Laufs `36386043292`.
- **Lage:** (gemessen 2026-09-28T06:21Z via `ci_manage jobs 36377301474` + `gh workflow run modis-cdn.yml`) der split-Job `36377301474` ist `completed success` (Schritt „Lift every family shard…"); `modis-cdn` `36386043292` `queued`. In `.github/workflows/modis-cdn.yml` liest jetzt jeder der drei Jobs (`raster`, `release`, `series-manifest`) `FAMILY` aus `phi/sources.φ:11662` (`grep -oE 'releases/download/[^/]+/modis_lst_cmg_8day\.manifest' … | cut -d/ -f3`) statt des hartkodierten Literals; `release`/`series-manifest` haben dafür `actions/checkout@v7` erhalten.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36386043292` nach Laufende; 422 `file_count > 1000` entfällt nach dem Split.

### matrix-rotor — Runner-Präemption gemessen; Checkpoint-Resilienz gebaut
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** Ende des ersten `matrix-rotor`-Laufs auf dem neuen HEAD.
- **Lage:** (gemessen 2026-09-28 via `ci_manage log`/`gh api`/Browser) 8 Läufe, alle failure, jeder endet „The runner has received a shutdown signal" (GitHub-hosted, Repo public, Actions netto $0, kein GitHub-Incident, **kein** Haus-Cancel in `ci_watchdog.log`); Erstversuche sterben ~2,5 min nach Rotor-Start, Reruns nach 40–51 min. Artefakt `matrix-rotor.txt` und State gingen im Hard-Cancel verloren, weil beide nur in Post-Steps geschrieben wurden. Rat: exogen = Resilienz, nicht Ursachenbehebung. `.github/workflows/matrix-rotor.yml` jetzt: Checkpoint (State sha256-benannt `matrix-state-<sha>.bin` + `latest.txt` + Log) alle 120 s **im** Rotor-Schritt, Log per `tee` in den Job-Log, Loader sha256-verifiziert mit Fallback, `if: always()`-Sicherung.
- **Blockade:** keine.
- **Braucht:** nach Push `gh workflow run matrix-rotor.yml`, dann `ci_manage log <neu>` — der SIGTERM-Ursprung ist jetzt im Job-Log sichtbar, die Checkpoint-Assets erscheinen auf dem Release `matrix-state`.

### ci_watchdog — Matcher heilt echte Runner-Shutdowns
- **Status:** wartend | **Bindung:** eigen (Werkzeug)
- **Trigger:** nächster transienter Rot-Lauf (`ci_watchdog.log`).
- **Lage:** (gemessen 2026-09-28 via `ci_watchdog.log` + `ci_manage log`) der Watchdog stufte die `matrix-rotor`-Shutdowns als „assertion-red" ein, weil der Matcher auf den Build-Text `Compiling static_assertions` ansprang → der gewollte Rerun unterblieb. `bin/ci_watchdog.sh` prüft jetzt transient **zuerst**; die Assertion-Klasse ist auf echte Rot-Marker begrenzt (`panicked at|assertion failed|assertion .* failed|test result: FAILED|error\[E[0-9]`); `bash -n` 0.
- **Blockade:** keine.
- **Braucht:** der nächste rote Lauf mit „runner has received a shutdown signal" trägt „rerun … measured transient cause" im `ci_watchdog.log`.

### Freie GitHub-Hebel — Codespace · cdn-health · copilot_ask
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** erster `cdn-health`-Lauf (`cdn-health.yml`) ODER Operator öffnet den Codespace.
- **Lage:** (gemessen 2026-09-28 via Browser/`gh`) das `omegaflow`-Konto (GitHub Free, public Repo) bietet acht Produkte; genutzt ist nur Actions, **Copilot Free 0 %**, **Codespaces 0**. Gebaut: `.devcontainer/devcontainer.json` (freier Codespace: Rust-Image + lavapipe + github-cli/node, 2 cpus/8 GB), `.github/workflows/cdn-health.yml` (Ersatz für den lokalen `cds_watchdog`: zehn CDN-Assets, aktuell alle `200`), `bin/copilot_ask` (Copilot-CLI read-only: `--mode plan --disable-builtin-mcps`, getestet `Changes +0 -0`; `gh copilot`/`copilot` installiert, Konto-authentifiziert). Copilot-Free-Modelle: der CLI-Katalog zählt 27 (`claude-*`, `gpt-6-astra`, `gpt-5.6-*`, `gemini-3.*-flash`, `grok-4.5`, `kimi-*`, `mai-code-1.1-flash`), aber das **Free-Konto erlaubt nur `auto`** — alle 10 getesteten expliziten Namen → „not available"; `auto` löst auf **`gpt-6-luna`** auf (einziger Kandidat). Der eingebaute **GitHub-MCP-Server** war verbunden (Schreibkanal) und ist im Wrapper abgeschaltet.
- **Blockade:** keine.
- **Braucht:** `gh workflow run cdn-health.yml`, dann `ci_manage log <id>`; den Codespace öffnet der Operator; `copilot_ask "<frage>"` für die read-only Recherche-Stimme; `copilot_review <pfade>` für den read-only Zweit-Scan (feste Rubrik, MCP aus; Demo an `bin/ci_watchdog.sh` lieferte vier Kandidaten — keiner davon gemessen, keiner ein Verdikt).

### Workflow-Klassen-Zensus Step 5 — modis entschieden; generische Fassung offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Zensus-Pass (`docs/surveys/survey-2026-09-03-orphan-verdicts.md`).
- **Lage:** (gemessen 2026-09-28 via Council + `sgrep`) Council-Verdikt: für dynamisch abgeleitete Tag-Mengen (modis-Jahr-Tags, `ps1-dr2-*`-Slabs) ist die Laufzeit-Ableitung die Sache selbst; das Register trägt die Familien-Identität (nicht die Jahr-Menge). `modis-cdn.yml` bindet sie jetzt (Punkt oben). `phi/sources.φ` hält für `modis_lst_cmg` nur die zwei Familien-URLs (`:11662`, `:11679`), keine Jahr-Tag-URL (`sgrep 'modis_lst_cmg-[0-9]' phi/sources.φ` → 0 Treffer).
- **Blockade:** keine.
- **Braucht:** `edit docs/surveys/survey-2026-09-03-orphan-verdicts.md:142` — das Mandat „Jahr-Tags auf sources.φ binden" auf „Familien-Identität als Register-Anker; Jahr-Menge bleibt gemessene Ableitung (CMR + `gh api`)" umstellen; die Step-5-Prämisse (Z. 105–122) generisch um „wo die Tag-Menge Register-Eigentum ist" ergänzen.

### register_lookup --fired — „Mail"-Matcher zu breit (gemessen falsch-positiv)
- **Status:** wartend | **Bindung:** eigen (Werkzeug)
- **Trigger:** nächster `register_lookup`-Atom.
- **Lage:** (gemessen 2026-09-28 via `register_lookup --fired` + Mail-Ledger) der Punkt `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` wurde als FIRED gemeldet; es liegt kein Eingang vor (jüngster Ledger-Eintrag `1790533094` IGETS, ausgehend) noch eine Antwort eines wartenden Hosts. Der Matcher greift auf jeden Ledger-Eintrag, nicht auf die Antwort der wartenden Quelle.
- **Blockade:** keine.
- **Braucht:** Matcher in `tools/register/src/bin/register_lookup.rs` auf den Host-Abgleich der wartenden Quelle einengen; Gate-Test im selben Atom.

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster periodischer `hinet-cdn`-Lauf ODER Hinet-Readiness.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`) `36344350143` rot, Job-Log `unread`; Vorlauf an 8× `attempt stayed unready`, Auth 200.
- **Blockade:** quellenseitige Readiness (Hinet).
- **Braucht:** `ci_manage jobs 36344350143` beim Trigger.

### Tooling-Lücke — `register_lookup --dropped --count` lokal zu langsam
- **Status:** wartend | **Bindung:** eigen (Werkzeug)
- **Trigger:** Operator-Wort zur Baseline-Änderung.
- **Lage:** (gemessen 2026-09-28 via Code-Lesung + Instrumentation) 100 % der >30 min sind Git-Subprozesse (~6 100 Spawns; `git log --all -S`-Pickaxe 16,1 s wegen 973 `refs/safety/*`-Refs); `--count` spart nur die Ausgabe.
- **Blockade:** kein Fix ohne Semantik-/Baseline-Änderung — die resolved/dropped-Zahl ändert sich.
- **Braucht:** Operator-Wort für den `--exclude='refs/safety/*'`-Weg (oder den Zwei-Pass-Umbau) + Baseline-Bump; die Frage gehört in Futures Operator-Queue.

### D5-Orphan-Residuum — CDN-Manifestations-Weg + P2P
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes steht (`docs/concepts/zeugnis.md:288` §Röhren-Pfad).
- **Lage:** (gemessen 2026-09-27) kein Producer-Bin, keine Register-Zeile, kein `*-cdn.yml`; der generische Weg steht (`src/archivar/cdn.rs` `upload_release`, `--ci-mode`-Tor).
- **Blockade:** Producer fehlt.
- **Braucht:** kein Schritt zur Kante bis der Producer steht.

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige Routen) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` beim Termin.

## Absender-Zeilen (eingehend, gefaltet)

- mountain folge193 → Mycelium: `phi/sources.φ:14383` (`LLNL_G3D_JPS.volume.bin`) und `:14391` (`S40RTS.volume.bin`) trugen die EMC/AFRP-Sammel-`origin`; korrigiert auf `https://media.githubusercontent.com/media/tom-new/tomography-models/main/<name>.nc` (gemessen via `.github/workflows/volume-cdn.yml:191,216`, Tag `media.githubusercontent.com`). Quelle: mountain folge193.

## Weitergabe (fremde Feder — Aufenthalt beim Eigentümer)

- **Atomic state write** (Mathematikerin): `save_state` schreibt in-place und schluckt Fehler (`src/mathematikerin/machines/matrix.rs:334` `let _ = std::fs::write(path, &buf)`, Cadence `:1117`); der neue Checkpoint liest dieselbe Datei. Ziel: **River** — temp+rename in `matrix.rs` mit Test im selben Atom (Folge-Atom zum matrix-rotor-Checkpoint). Quelle: Council 2026-09-28.
- **Zwei `format`-Findings** (gemessen 2026-09-28 via `ci_manage log 36359297755`): `vizier.cfa.harvard.edu` `/viz-bin/asu-tsv?-source=J/A+A/582/A8/titan_j…` (TSV als JSON geparst) und `www.ldeo.columbia.edu` `/~gcmt/projects/CMT/catalog/jan76_dec25.ndk` (.ndk als JSON geparst) → `JSON parse void`; Disposition/`format` in den Registern. Ziel: **Mountain**. Quelle: Mycelium-Session 192.
- **Browser-Anbindung unpacked-Laden** (gemessen 2026-09-28 via `ci_manage view 36385530990`): Fork-Build/Test grün; der Operator lädt `.output/chrome-mv3/` unpacked. Operator-Akt. Ziel: **Future** (Operator-Queue). Quelle: Mycelium-Session 192.

## Träger (Prosadokumente, eigene)

- `docs/concepts/arxiv-api.md` | offene Marker (2) | `register_lookup --orphan-docs` beim nächsten Pass.
- `docs/concepts/exzellenz-konzept.md` | offene Marker (3) | dito.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | Step 5 Plan (sha `bbf5512f…`) | Punkt oben.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | wartend Mail-Eingang | Trigger echte SAMPLE_CONTACT-Mail.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Layout-Wort in Future-Queue | Migration nach Wort.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | §7 Roh-Korpora-Disposition | nach Wort.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending tot | `--verdict` je Host beim Trigger.
- `docs/concepts/tools-map.md` | offene Marker | `register_lookup --orphan-docs` beim nächsten Pass.
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` | offene Marker (3) | nächster Schritt: Operator lädt den Fork-Build unpacked (Future-Queue, Operator-Akt); danach die Marker schließen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
