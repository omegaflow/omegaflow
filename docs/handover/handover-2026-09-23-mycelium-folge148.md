<!--
  title: Handover — Mycelium-Folge 148 (dropped-Baseline gehoben, Rosetta-ODF re-dispatched, PS1 weiter absent; Feld-Kandidaten-Dispatch zweimal abgestürzt) (Stand 2026-09-23)
  session: Mycelium-Folge 148
  class: handover
  date: 2026-09-23
  sha256: e6d22d498ea20ab681a1cd33cc13be9f169cd86e250190d5880b783796379306
  status: live
-->
# Handover — Mycelium-Folge 148 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Trigger** / **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session hat `handover-2026-09-23-mycelium-folge147.md` konsumiert.

## Stehender Pass (gemessen 2026-09-23)

- **HEAD** `7ded53c` == `origin/main` (fremde Linien committen im geteilten Baum;
  Base der Session war `29cc042`, inzwischen `7ded53c` mountain-folge143).
- **`git_safety --snapshot`:** `refs/safety/1790173226`.
- **Postfach** — `state/mail/mail_ledger.φ`: kein neuer handlungsrelevanter Eingang
  (letzter gemessener Stand unverändert; `mail_digest` absent).
- **CI am HEAD** — `ci-check 35879956552` @`7ded53c` **pending** (gemessen
  2026-09-23 via `ci_manage view`); Vorlauf `35868973875` @`e0a0e52a` **failure**
  (dropped-gate delta, test/clippy/build/format grün); `tools-build` @`7ded53c`
  success (`35879820898`); `register-dropped 35871377787` success (misst nur,
  schreibt nicht).

## Diese Session (git trägt es)

- **dropped-Baseline gehoben.** `register_lookup --dropped --count` = **3053** @
  `7ded53c` (die 238/262-Deltas waren ~96 % `commit-resolved`- bzw. umformulierte
  Unterfeld-Artefakte, nicht verlorene Quellen — gemessen von `grind-flash`).
  `docs/zustand/dropped-baseline.md` auf den frischen Wert im annehmenden Commit
  gehoben.
- **Rosetta ODF re-dispatched:** `planetary-odf-cdn.yml` → Lauf **35881738785**
  (2026-09-23; `gh workflow run`). Ergebnis offen.
- **PS1 final-combine weiter absent:** `ps1_dr2_coverage.fp01` auf Tag
  `ssd.jpl.nasa.gov-ps1` **HTTP 404** (sniff 2026-09-23); der letzte `ps1-cdn`-Lauf
  `35855330990` success hat final-combine nicht erreicht.
- **Feld-Kandidaten disponiert (18, `grind-pro`, allein dispatcht):** Der Dispatch war
  zuvor zweimal abgestürzt — Ursache: Subagent + Bash parallel in einer Nachricht;
  **allein** lief er durch. Vorab inline: 14/18 Roots HTTP 200. Ergebnis: **1
  `sources.φ`** (`arclink.ethz.ch/fdsnws/event/1/query?format=csv`, Tiefe km,
  seismic-body — Schweizer SED-FDSN), **11 `declined_sources.φ`**
  (grace/retlector/so2/stereo/gebco/sios/usgodae/blitzortung/stsci/argo/darts —
  Portal/Archiv/Ephemeride/integriert/Redistribution), **6 `blocked_sources.φ`**
  (gmrt+opensensemap `parser-def`, sansa/cma/incois `ip-blocked`, alaska `parser-def`).
  **0 pending.**
- **SOURCE_PORT.md-Drift geheilt:** der Spec-Verweis in `docs/SOURCE_PORT.md`
  zeigte auf den Ordner `concepts` statt `specs`; die Kontroll-Spec liegt unter
  `docs/specs/sources-v2-spec.md` (gemessen 2026-09-23, `open_points_check`).
- **`dropped-`Zähler-Wurzel gemessen (→ mountain):** `register_lookup --dropped
  --count` zählt alle Drops **vor** der Git-Auflösung; ~95–96 % sind
  `commit-resolved` (umformulierte Unterfeld-Zeilen `**Lage:**`/`**Braucht:**`),
  kein Verlust (gemessen 2026-09-23, `grind-flash`). Kein post.md-Eintrag
  möglich: `docs/handover/post.md` trägt fremde uncommittete Arbeit, nicht
  angetastet.

## Offen (aufgeschlüsselt)

### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check` auf dem folge148-Commit.
- **Lage:** clippy geheilt; dropped-  Baseline in diesem Commit gehoben (3053, gemessen
  2026-09-23); offener Lauf `35879956552` @`7ded53c` pending.
- **Blockade:** keine.
- **Braucht:** nach Push `ci_manage view <id>`; Ergebnis ins nächste Handover.

### 7 Feld-Kandidaten (13. Korpus)
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-Wort.
- **Lage:** `data.neracoos.org`, `imis.bfs.de`, `ioc-sealevelmonitoring.org`,
  `jma.go.jp`, `safecast.org`, `seismicportal.eu`, `tadas.afad.gov.tr` = accept;
  `metoffice.gov.uk` pending (gemessen 2026-09-23, `grind-pro`).
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
- **Trigger:** nächster `ps1-cdn`-Lauf bis `all_present`.
- **Lage:** `ps1_dr2_coverage.fp01` HTTP 404 (sniff 2026-09-23); letzter Lauf
  `35855330990` success, final-combine nicht erreicht; Band-Parts 637–671,
  `band_max 2643`.
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** bei `all_present` `footprints.φ` PS1-Note setzen.

### Rosetta ODF (CDN)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ergebnis Lauf `35881738785`.
- **Lage:** re-dispatched 2026-09-23 (`planetary-odf-cdn.yml` → 35881738785); davor
  cancelled/failure, keine Assets unter `archives.esac.esa.int`.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 35881738785`; bei success sha256 in `phi/sources.φ`
  (Rosetta-Block, URL-Referenz — Zeilennummern driften).

### GitHub-Release-Asset-Cap
- **Status:** blockiert | **Bindung:** eigen (Rat)
- **Trigger:** Rat-/Architektur-Wort.
- **Lage:** 1000 Assets/Release erreicht; `upload_asset` blockiert (`src/archivar/cdn.rs:40`);
  ~74 Sites betroffen (`external-state.md`).
- **Blockade:** Architektur.
- **Braucht:** `council`-Verdikt, danach `grind-pro`.

### DataONE Katalog-/Terms-Lizenz
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Auth-/Operator-Wort.
- **Lage:** `/terms`+`/data-policy` HTTP 401 (Apache Basic Auth); per-Record-Lizenz
  gemessen (EML/schema.org, kein SPDX) (gemessen 2026-09-23).
- **Blockade:** serverseitige Sperre.
- **Braucht:** DataONE Terms via Auth/Operator (`www.dataone.org/terms`).

### DEMETER Order 18387
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-Wort für die Neuordnung.
- **Lage:** CNES-Zugang vorhanden; Order 18387 44,5 % Fehler, Ablauf 2026-09-28
  (gemessen 2026-09-23).
- **Blockade:** Schreibakt bei CNES (Dritter).
- **Braucht:** Operator-Wort für Neuordnung in 100er-Batches (`DMT_N1_1144`).

### SuperDARN MAP
- **Status:** operator-gebunden | **Bindung:** eigen → future
- **Trigger:** Operator-Wort.
- **Lage:** Globus-Zugang vorhanden; lokal 2932 `.map` (1993–2002), ~1562 fehlen
  (gemessen 2026-09-23).
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

### dropped-Zähler-Wurzel (→ mountain)
- **Status:** blockiert | **Bindung:** linie:mountain
- **Trigger:** Rat-/Architektur-Wort.
- **Lage:** `register_lookup --dropped --count` (Basis des CI-Gates
  `.github/workflows/ci-check.yml`) zählt alle Drops **vor** der Git-Auflösung;
  gemessen 2026-09-23: von 2963 ~95–96 % `commit-resolved`, Rest überwiegend
  umformulierte Unterfeld-Zeilen — kein echter Verlust (gemessen 2026-09-23,
  `grind-flash`).
- **Blockade:** Register-Tool-Semantik (`linie:mountain`).
- **Braucht:** `register_lookup.rs` den Zähler um die `commit-resolved`-Menge
  bereinigen bzw. Unterfeld-Zeilen je Punkt deduplizieren — dann misst das Gate
  echte Drops statt Artefakte.

### Sicherheits-Befund
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator/Council-Urteil.
- **Lage:** eingeschleuster Instruktionsblock gemeldet, Herkunft ungemessen.
- **Blockade:** Herkunft ungemessen.
- **Braucht:** Operator/Council-Urteil.

## Benchmark

- **Routine-Klasse geschlossen** (flash-Sieger, 2026-09-16) — zitiert.
  Dispatch dieser Session: 1× `grind-pro` (18 Feld-Kandidaten; Force-Gate-Klasse —
  Sieger `grind-pro`, kein Doppellauf). Erst nach dem Allein-Dispatch (ohne
  parallelen Bash-Call) trug er: 1 sources / 11 declined / 6 blocked / 0 pending.

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien dieser Session:** `docs/zustand/dropped-baseline.md`,
  `docs/SOURCE_PORT.md`, `phi/sources.φ`, `phi/declined_sources.φ`,
  `phi/blocked_sources.φ`, `docs/handover/handover-2026-09-23-mycelium-folge148.md`.
- **post.md nicht angetastet:** trug fremde uncommittete Arbeit; die mountain-Zeile
  wurde daher nicht gesetzt.
- **Move mit dem Commit:** `handover-2026-09-23-mycelium-folge147.md` → `archiv/`.
- **Fremd (unangetastet):** river/sensory uncommittete Arbeit im Baum
  (`src/archivar/*`, `docs/handover/handover-2026-09-23-river-folge16.md`,
  `handover-2026-09-23-sensory-folge153.md`).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
