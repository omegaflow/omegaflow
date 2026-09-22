<!--
  title: Handover — Entscheid-Folge 70 (Stehender Pass fortgeschrieben; fmt-Rot free_model_bench.rs geheilt; Feld-Grammatik + Ksg off-path in die Operator-Queue gefaltet) (Stand 2026-09-20)
  session: Entscheid-Folge 70
  class: handover
  date: 2026-09-20
  sha256: cfb77acd84c0762a7049b5f8112ecd63a109749f140ab7fc3da2c98eb2f70672
  status: live
-->
# Handover — Entscheid-Folge 70 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen nächsten
Schritt in derselben Zeile. Wartestellungen (`wartend`) sind kein Auswahlpunkt,
sondern nennen nur ihren Auslöser. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-20, Entscheid-Folge 70)

- **HEAD** `ff433f9d` == `origin/main` (bau folge114, fremd). Arbeitsbaum:
  **fremd** — forschung folge125 in Arbeit (`.github/workflows/hyperscanning-te.yml`,
  `src/mathematikerin/te.rs`, `tools/measure/src/bin/hyperscanning_group_te.rs`,
  `docs/handover/handover-2026-09-20-forschung-folge124.md` → `archiv/`,
  `handover-2026-09-20-forschung-folge125.md` neu) — nicht angefasst.
- **Postfach** — `mail_ledger.φ`: letzter Eingang `1789930255` (`info@pine64.org`:
  Ox64 zugesagt, Versanddaten/Telefon erbeten) — **unverändert**; Antwort gesendet
  (`sent_ledger` `1789931195`). Eintrag `external-state.md` (Postfach) zitiert.
- **CI** — `ci_manage list` 2026-09-20 ~22:4x: `rpw-cdn` `35536016815` in_progress,
  `35536005969` failure; `ci-check` `35536012394` pending, `35534657914` in_progress;
  `health-check` `35535074295` in_progress; `te-gate` `35534898200` in_progress;
  `free-model-agent-bench` `35534406541` **pending** (P13), `35532892161` in_progress;
  `free-model-bench` `35532890353` pending; `hyperscanning-te` `35533691654` failure.
  Kein Poll. CI-Zeile in `external-state.md` von forschung folge125 fortgeschrieben
  (nicht angefasst — fremder Hunk auf derselben Zeile).
- **`register_lookup --open`** — 585 offene Zeilen, 3 `[entscheid]`
  (`phi/blocked_sources.φ:21/60/65`, in der Queue); `post.md` nach der Faltung 4
  Zeilen (An ernte ×3, An forschung ×1) — keine mehr an entscheid.
- **`git_safety --snapshot`** — `refs/safety/1789936228`.

## Operator-Queue (Stand folge70; einfache Sprache, je Frage mit Alter)

3. **ISH Chat (GitHub-Dritt-OAuth-App, Scopes `read:user`/`user:email`)** —
   Sicherheitsereignis (Mail 2026-09-20). **Frage:** App unter
   `github.com/settings/connections/applications` widerrufen? (Alter: seit
   2026-09-20)
4. **SuperDARN** (`blocked account`) — HF-Radar-Ionosphären-Konvektion; Route
   über Globus + PI-Vereinbarung (`superdarn.ca/piagreement`). **Frage:** Konto +
   PI-Vereinbarung eingehen? (Alter: seit 2026-09-16)
5. **solar-system-open-data REST** (`blocked key`) — `api.le-systeme-solaire.net`
   HTTP 401 (Bearer-Token); Körperdaten ohne Konto nicht abrufbar. **Frage:**
   Konto/Token anlegen? (Alter: seit 2026-09-20)
6. **Amentum Developer** (`blocked account`) — geomagnetisch/aviation-radiation/
   gravity (trial); Registrierung HTTP 200, Zugang nur mit Konto. **Frage:**
   Registrierung `developer.amentum.io/register`? (Alter: seit 2026-09-20)
7. **Split-Routing-Verifikation** — `./bin/proton-exit.sh ca` +
   direct↔tunnel-Nachmessung der 8 `000`-Hosts. **Frage:** Operator-Wort/Route
   (sudo+Netz, nicht in der Ernte-Session)? (Alter: seit Ernte folge12–17)
8. **Cookie-Transfer** — `operator-gebunden`, Auslöser „Bedarf" (Ziel-Site aktiv
   → Cookie-Editor-Export → `state/cookies/<host>.json`). (Alter: seit 2026-09-16)
10. **Strukturierte Feld-Grammatik** (bau, aus `post.md` gefaltet) — Lage: am
    2026-09-18 als Kanon-Akt vertagt, seither in keiner Übergabe; kein Spec, kein
    Code, keine Messung im Baum. **Frage:** soll bau einen Spec-Entwurf für eine
    strukturierte Grammatik der φ-Feld-Direktiven vorlegen (Kanon-Akt, mit Rat) —
    oder den Strang „nie gebaut, nicht gebraucht" schließen (descoped mit
    Messung)? (Alter: seit 2026-09-18)
11. **Ksg off-path** (Forschung-Linie, aus `post.md` gefaltet) — der
    Familien-Screen ruft nur `topological_te_estimate`, nie Ksg. **Frage:** Ksg
    verdrahten oder mit gemessenem „nicht auf dem Pfad" descopen? (Alter: seit
    2026-09-20)

## Offen

- **P2/P3/P4 Bench-Läufe** — `wartend` (Auslöser: Run-Abschluss `35532890353` /
  `35532892161`; `ci_manage view` einmalig, Artefakt `free-model-bench.tsv` lesen,
  Ranking eintragen).
- `termin` — vC-Permeabilität (Smartwatch + Mantis-Shrimp), Lasair-LSST (API 502),
  BepiColombo MORE (~April 2027), Rubin-Review (Umzug 2026-09-24). `blockiert` —
  TAP-Backends dachs/pithia (extern). `wartend` — adoption-Block, SuperDARN-Globus,
  GitHub-PII/GC (#4761801), Sonden-Antworten, `ci-check`.

## Benchmark

- **Delegationen (Entscheid-Folge 70):** keine Sub-Agenten — die fmt-Heilung des
  eigenen Werkzeugs (`free_model_bench.rs`, eine Datei) lief inline über `rustfmt`;
  die Routine-Klasse ist geschlossen (flash-first, kein pro/max). Kein
  Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-20-entscheid-folge70.md` (neu)
- Move `handover-2026-09-20-entscheid-folge69.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` (drei entscheid-Zeilen gefaltet + gelöscht)
- `tools/measure/src/bin/free_model_bench.rs` (`rustfmt`-Heilung)
- **nicht** angefasst: `docs/zustand/external-state.md` (fremder Hunk, forschung
  folge125, CI-Zeile) — Eintrag zitiert, nicht kopiert.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der P13-Lauf
`35534406541` ist weiter `pending`; kein Dispatch, kein Poll — das Artefakt wird
einmalig gelesen, sobald der Lauf abgeschlossen ist. Der Push löst `ci-check` aus;
kein manueller Dispatch nötig.
