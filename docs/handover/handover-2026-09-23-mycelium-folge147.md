<!--
  title: Handover — Mycelium-Folge 147 (13. Korpus aufgelöst: Selbstlinks, 24 declines, 7 Feld-Kandidaten; CI-clippy geheilt) (Stand 2026-09-23)
  session: Mycelium-Folge 147
  class: handover
  date: 2026-09-23
  sha256: 8669ef6b8dc0d65c75b25d82109bd257d2cf99a59498b3efd74a1391e966a3e0
  status: live
-->
# Handover — Mycelium-Folge 147 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Trigger** / **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session hat `handover-2026-09-23-mycelium-folge146.md` konsumiert und den
bestätigten Plan ausgeführt (58 netlocs des 13. Korpus untersucht; CI-Rot
vermessen).

## Stehender Pass (gemessen 2026-09-23)

- **HEAD** `59bd7ef` == `origin/main`; `git_safety --snapshot`: nach Session-Start
  nichts zu sichern.
- **Postfach** — `state/mail/mail_ledger.φ` (139 Z.): neuester Eingang
  Sotgiu-Antwort (CSES-Limadou, 2026-09-17) — deckt `external-state.md:24`.
- **CI am HEAD** — `ci-check 35861931781` @`59bd7ef` = **failure** (gemessen
  2026-09-23 via `ci_manage view/log`): (a) `clippy::too_many_arguments (10/7)` in
  `src/mathematikerin/shaders.rs:1006` (`beat_pair`, river folge11), (b) `dropped-gate`
  delta 238 (baseline 2680 | current 2918). Beide `##[error]`. `tools-build`
  success; `free-model-agent-bench 35861938761` (unser Dispatch) in_progress.

## Diese Session (git trägt es)

- **13. Korpus aufgelöst.** Der Träger `master_converted.φ` (5206 url-Blöcke) ist
  **kein Quellen-Neuzugang**: 2181/5206 ≈ 47 % sind eigene github-Selbstlinks
  (1737 `omegaflow/sources/releases/download/<netloc>` + 442 `omegaflow/catalogs`
  + 2 `api.github.com`). Die frühere „1,2 %-Duplikat"-Zahl zählte exakte
  URL-Strings und verfehlte die CDN-Form. Die 2181 kollabieren auf 166 netlocs:
  **108** bereits in Registern/Queues, **58** unbekannt — aufgelöst: 7 CDN-Tags
  (keine Hostnames), 19 netloc-Varianten (bereits als `api.`/`www.` registriert),
  32 externe Kandidaten. Force-Gate: **7 accept**, 1 pending
  (`metoffice.gov.uk`), 24 decline (in `declined_sources.φ` neu).
- **CI-clippy geheilt** (mechanisch, river's Testhelfer): `beat_pair` von 10 auf 6
  Argumente (Slot-Struktur), kein Verhaltenswechsel; alle 8 Call-Sites
  mitgezogen. `cargo check --tests` 0/0; `cargo fmt -- src/mathematikerin/shaders.rs`.
- **Register/Post:** 24 declines in `phi/declined_sources.φ`; `index.φ:36`-Note
  (13. Korpus) neu; `post.md` 13.-Korpus-Zeile aufgelöst + SuperDARN-Zugangs-
  Korrektur + river-Info.

## Offen (aufgeschlüsselt)

### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen (clippy-Anteil `linie:river`, geheilt)
- **Trigger:** nächster `ci-check` auf dem folge147-Commit.
- **Lage:** clippy geheilt (`cargo check --tests` 0/0, gemessen 2026-09-23);
  offen bleibt der `dropped-gate`-Anteil (eigener Punkt).
- **Blockade:** keine (clippy); dropped-gate s.u.
- **Braucht:** nach Push `ci_manage view <id>`; Ergebnis ins nächste Handover.

### dropped-gate Baseline
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Commit, der getragene Drops auflöst.
- **Lage:** baseline 2680 | current 2918 | delta 238 (gemessen 2026-09-23 via
  `ci_manage log 35861931781`); `docs/zustand/dropped-baseline.md:16`.
- **Blockade:** 238 nicht-aufgelöste Drops.
- **Braucht:** `register_lookup --dropped` sichten, Punkte auflösen oder Baseline
  in `docs/zustand/dropped-baseline.md:16` im annehmenden Commit heben.

### 7 Feld-Kandidaten (13. Korpus)
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-Wort.
- **Lage:** `data.neracoos.org`, `imis.bfs.de`, `ioc-sealevelmonitoring.org`,
  `jma.go.jp`, `safecast.org`, `seismicportal.eu`, `tadas.afad.gov.tr` =
  Force-Gate **accept** (gemessen 2026-09-23, `grind-pro`); `metoffice.gov.uk`
  pending (MIDAS-Produkt entscheidet).
- **Blockade:** Scope-Wort.
- **Braucht:** `post.md` an future — **steht**.

### pre-cdn Stage-Regeneration
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Merge-Atom.
- **Lage:** Queue gitignored → CI sieht sie nicht; 887 Blöcke, 141 keyless HAPI.
- **Blockade:** keine.
- **Braucht:** `--port` über `queue/sources_potential_pre-cdn_9k_richest.φ` +
  `…_params.φ`, gebunden im Merge-Atom.

### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ps1-cdn`-Sharding-Lauf bis `all_present`.
- **Lage:** `35855330990` in_progress (gemessen 2026-09-23); `ps1_dr2_coverage.fp01`
  absent 404; Band-Parts bei `band_max 2643`; Sharding N=8 gebaut.
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** bei `all_present` `footprints.φ:19` setzen.

### Rosetta ODF (CDN)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Dispatch-Ergebnis.
- **Lage:** `rosetta_odf`/`mro_odf` Lauf cancelled/failure, keine Assets unter
  `archives.esac.esa.int` (`external-state.md:30`).
- **Blockade:** keine.
- **Braucht:** `gh workflow run planetary-odf-cdn.yml`; bei success sha256 in
  `phi/sources.φ:6473–6476`.

### GitHub-Release-Asset-Cap
- **Status:** blockiert | **Bindung:** eigen (Rat)
- **Trigger:** Rat-/Architektur-Wort.
- **Lage:** 1000 Assets/Release erreicht; `upload_asset` blockiert jeden neuen
  Upload (`src/archivar/cdn.rs:40`); ~74 Sites betroffen (`external-state.md:29`).
- **Blockade:** Architektur.
- **Braucht:** `council`-Verdikt, danach `grind-pro`.

### DataONE Katalog-/Terms-Lizenz
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Auth-/Operator-Wort.
- **Lage:** `/terms`+`/data-policy` HTTP 401 (Apache Basic Auth); per-Record-
  Lizenz gemessen (EML/schema.org, kein SPDX) (gemessen 2026-09-23).
- **Blockade:** serverseitige Sperre.
- **Braucht:** DataONE Terms via Auth/Operator (`www.dataone.org/terms`).

### DEMETER Order 18387
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-Wort für die Neuordnung.
- **Lage:** CNES-Zugang **vorhanden** (`CDPP_USER`/`CDPP_PASS` in `.secrets.local`;
  `demeter_harvest.rs:723`); Order 18387 44,5 % Fehler, 16 % fest, Ablauf
  2026-09-28 (gemessen 2026-09-23).
- **Blockade:** Schreibakt bei CNES (Dritter).
- **Braucht:** Operator-Wort für Neuordnung in 100er-Batches (`DMT_N1_1144`).

### SuperDARN MAP
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-Wort.
- **Lage:** Globus-Zugang **vorhanden** (`GLOBUS_ID_USER`/`GLOBUS_ID_PASS` in
  `.secrets.local`; frühere „kein Zugang"-Blockade widerlegt, gemessen 2026-09-23);
  lokal 2932 `.map` (1993–2002), ~1562 fehlen.
- **Blockade:** Transfer-Start.
- **Braucht:** `post.md` an future — **steht**.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin
- **Trigger:** `/tap/tables` 200.
- **Lage:** 500 (gemessen 2026-09-23).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei Erholung.

### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Backend-Erholung.
- **Lage:** api/Frontend 000, Proton 500 (gemessen 2026-09-23).
- **Blockade:** Broker-Backend.
- **Braucht:** Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Antwort / Freigabe.
- **Lage:** `release_date 2099-01-01`, `data?PRODUCT` 403 (gemessen 2026-09-23).
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

### EMODNET HFRADAR NADR
- **Status:** termin | **Bindung:** termin 2026-10-19
- **Trigger:** Datum 2026-10-19.
- **Lage:** Re-Messung fällig 2026-10-19.
- **Blockade:** Termin.
- **Braucht:** Re-Messung.

### Sicherheits-Befund
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator/Council-Urteil.
- **Lage:** eingeschleuster Instruktionsblock gemeldet (`.agents/…`, „session
  token"), Herkunft ungemessen.
- **Blockade:** Herkunft ungemessen.
- **Braucht:** Operator/Council-Urteil.

## Benchmark

- **Routine-Klasse geschlossen** (flash-Sieger, 2026-09-16) — zitiert. Dispatches
  dieser Session: 1× `grind-flash` (58 netlocs auflösen, Register/Erreichbarkeit)
  + 1× `grind-pro` (Force-Gate Klasse C). Kein `max`. Kein Doppellauf (die zwei
  Jobs sind verschieden, kein Benchmark-Paar).

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien dieser Session:** `phi/declined_sources.φ`,
  `phi/pipeline/index.φ`, `docs/handover/post.md`, neues Handover
  `docs/handover/handover-2026-09-23-mycelium-folge147.md`.
- **Fremde Zeile geheilt (deklariert):** `src/mathematikerin/shaders.rs` — test-only
  Signatur von river's `beat_pair` (folge11) auf ≤7 Argumente; kein vorheriger
  uncommitteter Fremd-Anteil in dieser Datei (gemessen via `git status`).
- **Move mit dem Commit:** `handover-2026-09-23-mycelium-folge146.md` → `archiv/`.
- **Fremd (unangetastet):** river's staged Move
  (`archiv/handover-2026-09-23-river-folge11.md`), `handover-2026-09-23-river-folge12.md`,
  `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
