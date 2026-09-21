<!--
  title: Handover — Entscheid-Folge 78 (Riss 4: Rat empfiehlt Bau; text_review-Release-Pfad als Riss; Förder-Entwürfe gegengelesen) (Stand 2026-09-21)
  session: Entscheid-Folge 78
  class: handover
  date: 2026-09-21
  sha256: 1e151793226da63dcc0545f74d87d3d2b11bfb0246adc6d91d035ee7dab7a377
  status: live
-->
# Handover — Entscheid-Folge 78 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder offene Punkt wird **aufgeschlüsselt** geführt — kein
Register-Kürzel: **Lage** / **Blockade** / **Braucht**. Jeder Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Entscheid-Folge 78)

- **HEAD** `7695ae23`; `origin/main` `61d3049f` ist Nachfahre (lokaler Branch hinter
  origin; der Baum ist **clean** — die in folge77 genannten fremden uncommitteten
  Pfade sind inzwischen von ihren Linien committet: `5c077e9f` ernte folge130,
  `61d3049f` bau folge121). Kein fremder Pfad angefasst.
- **CI** — `tools-build` `35586902978` **success** (10:06→10:10Z) @`3764b5c0`;
  `text_review.rs` wurde erst mit `197bccf1` (12:07Z) committet → dieser Build trägt
  es **nicht**. Neuer `tools-build` `35587065363` in_progress. Kein Poll.
- **Postfach** — `state/mail/mail_ledger.φ` gemessen: kein neuer Eingang seit
  folge77; letzter relevanter Eingang Sotgiu (SSDC, „warten"), bereits getragen.
- **`register_lookup --open`** — 609 offene Zeilen, 14 `zustand` due, 1 Post offen;
  entscheid-eigene Registerpunkte `phi/blocked_sources.φ:21/60/65` (SuperDARN /
  solar-system-open-data / Amentum), `post.md:18` (Riss 4).
- **`git_safety --snapshot`** — `refs/safety/1789985404`.

## Messung dieses Atoms (kein offener Punkt)

- **Riss 4 — Rat gehört** (`council`, pro/max): Empfehlung **bauen** (WGSL-KSG-Spiegel
  als Bau-Atom). Die Kategorie „Riss" wird verworfen: KDE ≠ KSG sind **zwei Schätzer
  derselben Reihe**, kein Mess-Riss im Sinne des 0-Kanons (der Riss verlangt zwei
  unabhängige Zeugenlinien desselben Werts). Zwischenlage ehrlich `pending` tragen;
  GPU-KDE bleibt als KDE benannt, keine Mittlung. Der Paritätstest `tests.rs:345`
  prüft heute GPU-KDE gegen CPU-KDE — gegen den falschen Zwilling.
- **text_review-Release-Pfad — Riss gemessen:** `tools-build.yml` (gelesen @`61d3049f`)
  baut/publiziert nur `register_lookup/session_burn/open_points_check` (register),
  `archive_search/sgrep/sfetch/sread/git_safety/omega_sh/ci_manage` (utils),
  `commit_check` (gate), `omegaflow`. **Kein measure-Bin.** `bin/.tools_ensure
  text_review` → `pending — absent from the tools-latest manifest`. Die folge77-Annahme
  „Release kommt mit `tools-build` → `tools-latest`" ist **falsifiziert**; der Schritt
  ist so nicht ausführbar.
- **Förder-Entwürfe gegengelesen** (`grind-flash`): EV (`emergent-ventures-application.md`)
  — Zustands-Behauptung korrigiert (16-Pfeil-Kaskade war Null-Artefakt, `fam` = 0
  Zellen), „no defaults" abgeschwächt, QUELLEN erweitert → **einreichungsfertig**
  (Operator-Platzhalter offen). SSI (`ssi-fellowship-application.md`) — von deutschem
  Meta-Text zu englischer Application neu geschrieben, UK-Nutzen vierachsig konkret,
  Budget £4.000 aufgeschlüsselt → **bedingt einreichungsfertig** (Live-Formularabgleich
  + Budget-Bestätigung offen). Beide gitignored.
- **Free-Model-Bench — Artefakt gelesen + Riss diagnostiziert** (`gh run download
  35567266519`, Report `free-model-bench.tsv`, 399 Zeilen): **nur die T7-Klasse lief**
  (294 Zeilen = 98 Modelle × 3; T2–T6 = 0/0) — der Dispatch war auf `task=T7`
  gescoped, der Coding-Teil nie gemessen. Von 294: **1 pass** (`groq
  qwen/qwen3.8-27b` T7 1/3), 170 `wrong_answer`, **50 `pending_rate_limited`**,
  **33 `http_401`** (= alle 11 Cloudflare-Modelle × 3), 21 `timeout`, 9 `http_403`,
  6 `http_404` (google `gemini-2.5-pro`/`gemini-2.5-flash-lite` stale), 4 `http_5xx`
  → **123/294 (42 %) ohne Antwort**, davon Cloudflare/Quota/Netz, nicht Modell.
  Cloudflare gemessen: **alle drei Tokens** (`.secrets.local`: `CLOUDFLARE_API_KEY`,
  `CLOUDFLARE_API_TOKEN`, `CLOUDFLARE_WORKERS_TOKEN`) → **401** am Workers-AI-Endpoint;
  der API_TOKEN ist „active", hat aber **keine Account-/Workers-AI-Permission**
  (`9109`); der WORKERS_TOKEN listet das Konto, ruft Workers AI aber nicht auf.
- **Bench repariert** (`tools/measure/src/bin/free_model_bench.rs`, `cargo check
  -p omegaflow-measure --bin free_model_bench` 0/0): (a) **Parallelisierung** via
  `std::thread::scope` + Round-Robin-Gruppen (`BENCH_WORKERS`, Default 6) — der
  sequenzielle Lauf wäre ~18 h und liefe in den 360-min-Runner-Timeout; (b)
  **Antwort-Mitschnitt** (Spalte `note`, 160 Zeichen, newline/tab-frei) — sonst
  bleibt `wrong_answer` undiagnostizierbar. Der nächste Lauf (alle Tasks, leerer
  `--task`) trägt die volle T1–T7-Matrix.

- **Delegation (Folge 78):** 1 × `council` (Riss 4, pro/max — hartes Architektur-Atom),
  1 × `grind-flash` (Entwürfe, flash-first). Klasse „Architektur-Beratung" pro/max
  gerechtfertigt; „Text-Gegenlesung" flash-first.

## Offen (aufgeschlüsselt)

### text_review — Lauf auf den Förder-Entwürfen
- **Status:** blockiert | **Bindung:** eigen (Werkzeug) / `linie:bau` (Release)
- **Lage:** Bin gebaut + committet (`197bccf1`), `cargo check` 0/0; **nicht** im
  `tools-latest`-Manifest, weil `tools-build.yml` keinen measure-Bin trägt.
- **Blockade:** der Release-Pfad für measure-Bins existiert nicht — `tools-build`
  ist falsifiziert als Trigger.
- **Braucht:** bau trägt `text_review` in `tools-build.yml` ein (build-Step +
  `tools.manifest`-Zeile + `gh release upload`); danach `bin/.tools_ensure text_review`
  und der Lauf. Post an bau ist gesetzt.

### Förderung Person/Ideen/Projekte — Emergent Ventures
- **Status:** wartend (Session-Arbeit; nur der Einreichungs-Akt `operator-gebunden`)
  | **Bindung:** `eigen` bis zur Ausführungsgrenze, der Akt `dritter`
- **Lage:** Entwurf gegengelesen, repo-wahr, einreichungsfertig; Platzhalter offen
  (Vollzeit-Dauer, Budget-Ballpark, private Formularfelder).
- **Blockade:** keine.
- **Braucht:** Operator-Wort „EV einreichen" auf dem vorgelegten Entwurf
  (`state/mail/emergent-ventures-application.md`).

### SSI Fellowship (software.ac.uk)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** englische Application neu geschrieben, UK-Nutzen vierachsig, Budget
  £4.000 aufgeschlüsselt; offen bis **05.10.2026**, DE nur über max. 3 internationale
  Plätze.
- **Blockade:** Live-Formularwortlaut/Wortlimits + Budget-Split unbestätigt.
- **Braucht:** Abgleich mit dem echten SSI-Formular; dann Operator-Wort.

### NLnet Restack / CodeSupply
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** 5.000–50.000 €, Einzelperson, DE erfüllt EU-Dimension; Deadline
  **03.11.2026**; Bedingung FOSS-Lizenz „in its entirety" (`survey-funding-quellen.md`).
- **Blockade:** NC-Kern erfüllt die OSI-Bedingung nicht.
- **Braucht:** Operator-Entscheid — NC behalten und NLnet descopen, **oder**
  Dual-Lizenz (FOSS + NC) als eigenes Atom.

### Riss 4 — WGSL-KSG-Spiegel off-path
- **Status:** operator-gebunden | **Bindung:** operator (Bau an `linie:bau`)
- **Lage:** Rat empfiehlt **bauen**; „Riss"-Kategorie verworfen (zwei Schätzer ≠
  Mess-Riss). CPU-KSG (`src/mathematikerin/te.rs`) vs. GPU-KDE
  (`src/mathematikerin/shaders.rs`); Paritätstest prüft gegen den falschen Zwilling.
  `post.md`-Zeile in dieses Handover gefaltet.
- **Blockade:** Operator-Entscheid bauen vs. descopen.
- **Braucht:** Operator-Wort „bauen" → Bau-Atom an bau (WGSL k-NN + Silverman,
  f32/f64-Paritätstoleranz); bei „nein" → `descoped mit Befund` (KDE benannt, pending).

### Free-Model-Bench (105 Modelle)
- **Status:** wartend | **Bindung:** eigen (Lauf) / operator (Cloudflare-Token)
- **Lage:** Artefakt gelesen, Riss diagnostiziert, Bench repariert (parallel +
  Antwort-Mitschnitt, `cargo check` 0/0). Der volle T1–T7-Lauf ist nach Commit/Push
  dispatchbar (`gh workflow run free-model-bench.yml`, leerer `task`).
- **Blockade:** Cloudflare-Zweig tot (alle 3 Tokens ohne Workers-AI-Permission) →
  11 Modelle fallen aus; google `gemini-2.5-pro`/`gemini-2.5-flash-lite` stale.
- **Braucht:** (a) Operator rotiert einen Cloudflare-Token mit „Workers AI"-Permission
  in `FREE_MODEL_KEYS`; (b) nach `/commit` den vollen Lauf dispatchen; (c) google-IDs
  im TSV nachziehen.

### PINE64 / Mantis-Shrimp
- **Status:** blockiert | **Bindung:** linie:bau
- **Lage:** Ox64 zugesagt; Presence-Hardware ungebaut.
- **Blockade:** Hardware fehlt; Bau gehört zur bau-Linie.
- **Braucht:** bau baut den Mantis-Shrimp (Spec `docs/specs/mantis-shrimp-bom.md`).

### Mantis-Shrimp-Bewerbungen
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** kein Programm nimmt ein ungebautes Gerät an.
- **Blockade:** Prototyp fehlt.
- **Braucht:** Operator-Entscheid bauen/descopen.

### SSDC Limadou
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Konto existiert, CAS-Login lädt; Sotgiu-Mail (2026-09-16): CSES-02-Umstellung,
  „wait a few weeks"; Prozedur wird neu.
- **Blockade:** PI-Freigabe/Prozedur-Update ausstehend.
- **Braucht:** Wiedervorlage (Trigger: Prozedur-Update).

### Eigenprize
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Runde geschlossen; `state/mail/eigenprize-application.md`.
- **Blockade:** keine offene Runde.
- **Braucht:** „Remind me" auf `https://eigen.build`.

### Solitude
- **Status:** termin | **Bindung:** termin (~Herbst 2027)
- **Lage:** `state/mail/solitude-application.md` liegt.
- **Blockade:** Termin fern.
- **Braucht:** Entwurf tragen.

### ISH Chat (GitHub-Dritt-OAuth-App)
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** App hat Kontozugriff (`read:user`/`user:email`).
- **Blockade:** offener Zugriff.
- **Braucht:** Operator-Wort — App widerrufen?

### SuperDARN
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** `phi/blocked_sources.φ:21` (`blocked account`); Route über Globus +
  PI-Vereinbarung.
- **Blockade:** kein Konto.
- **Braucht:** Operator-Wort Konto; Session liest die PI-Vereinbarung vorab.

### solar-system-open-data REST
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** `phi/blocked_sources.φ:60` (`blocked key`); HTTP 401.
- **Blockade:** kein Token.
- **Braucht:** Operator-Wort Konto/Token.

### Amentum Developer
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** `phi/blocked_sources.φ:65` (`blocked account`); Reg. 200.
- **Blockade:** keine Registrierung.
- **Braucht:** Operator-Wort `developer.amentum.io/register`.

### Split-Routing-Verifikation
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** 8 `000`-Hosts ungemessen.
- **Blockade:** sudo + Netz.
- **Braucht:** Wort/Route `./bin/proton-exit.sh ca`.

### Cookie-Transfer
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Auslöser „Bedarf".
- **Blockade:** kein Bedarf.
- **Braucht:** nichts — wartend.

## Operator-Queue (Stand folge78; einfache Sprache, Lage/Blockade/Braucht, mit Alter)

1. **EV** — Entwurf gegengelesen, einreichungsfertig. **Braucht:** Wort „EV
   einreichen". (seit 2026-09-21)
2. **SSI** — Entwurf steht, Frist 05.10.2026. **Braucht:** Formularabgleich +
   Wort. (neu geschärft)
3. **Riss 4** — Rat empfiehlt **bauen**. **Braucht:** Wort bauen/descopen.
   (neu, `post.md` gefaltet)
4. **NLnet** — **Braucht:** Lizenz-Entscheid (NC behalten/descopen vs.
   Dual-Lizenz). (seit 2026-09-21)
5. **Mantis-Shrimp** — bauen oder descopen. (seit 2026-09-16)
6. **SSDC** — warten, kein Follow-up. (seit 2026-09-16)
7. **ISH Chat** — App widerrufen? (seit 2026-09-20)
8. **SuperDARN** — Konto + PI-Vereinbarung? (seit 2026-09-16)
9. **solar-system-open-data** — Konto/Token? (seit 2026-09-20)
10. **Amentum** — registrieren? (seit 2026-09-20)
11. **Split-Routing** — Route `./bin/proton-exit.sh ca`? (seit Ernte folge12–17)
12. **Eigenprize/Solitude** — „Remind me". (seit 2026-09-20)
13. **Cookie-Transfer** — nichts. (seit 2026-09-16)

## Benchmark

- **Delegation (Folge 78):** 1 × `council` (Riss 4) — pro/max, hartes
  Architektur-Atom; 1 × `grind-flash` (Entwürfe) — flash-first. Kein Doppel nötig:
  beide lieferten vollständig (Rat: klare Empfehlung + Begründung; flash: beide
  Entwürfe überarbeitet). Die gemessene Klasse „Architektur-Beratung pro/max" und
  „Text-Gegenlesung flash" bleiben getrennt.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-entscheid-folge78.md` (neu)
- Move `handover-2026-09-21-entscheid-folge77.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` (Riss-4-Zeile entfernt; Post an bau gesetzt)
- `tools/measure/src/bin/free_model_bench.rs` (Parallelisierung + Antwort-Mitschnitt)
- `state/mail/emergent-ventures-application.md`, `state/mail/ssi-fellowship-application.md`
  (gitignored — nicht committet)
- **nicht** angefasst: fremde `tools/*`, `.github/workflows/*`, `phi/*`, `src/*`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
