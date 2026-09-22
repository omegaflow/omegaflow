<!--
  title: Handover — Entscheid-Folge 73 (Nr. 11 Feld-Grammatik geschlossen; smail-Riegel bei bau erledigt) (Stand 2026-09-21)
  session: Entscheid-Folge 73
  class: handover
  date: 2026-09-21
  sha256: db2ae73b9578d84e16641774d02fa6d9f3658ac7e5750179c0c863cd8fdbf71f
  status: live
-->
# Handover — Entscheid-Folge 73 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder offene Punkt trägt seinen nächsten
Schritt in derselben Zeile. Wartestellungen (`wartend`) sind kein Auswahlpunkt,
sondern nennen nur ihren Auslöser. Jeder Punkt trägt seinen Status-Tag (`wartend` |
`operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Entscheid-Folge 73)

- **HEAD** `6aae97da` (ernte folge127) — `origin/main` `84dffc5d`. Arbeitsbaum
  trägt fremde uncommittete Arbeit (`docs/handover/handover-2026-09-21-ernte-folge126.md`
  als gestageter Move `D`) — nicht angefasst.
- **Postfach** — `mail_ledger.φ`: kein neuer Eingang; neuester weiterhin
  Privatpersonen — Hardware-Punkt beidseitig declined). `external-state.md`-Zeile
  nachgemessen.
- **CI** — `ci_manage list` 2026-09-21 ~09:0x: `tools-build` `35570698807`
  in_progress; `ci-check` `35570773976` pending; `te-gate` `35570482875` pending;
  `hyperscanning-te` `35570480672` failure; `ps1-cdn` `35569486280` in_progress;
  `free-model-bench` `35567266519` + `free-model-agent-bench` `35567268692`
  in_progress; `tools-build` `35570476690` success; `ned-cdn` `35568810102`
  success. Kein Poll. `external-state.md`-Zeile fortgeschrieben.
- **`register_lookup --open`** — 2 fremde OPEN (`handover-2026-09-20-operator-entscheidungen.md`);
  3 `[entscheid]`-Registerpunkte `phi/blocked_sources.φ:21/60/65` (SuperDARN /
  solar-system-open-data / Amentum) → Operator-Queue.
- **`git_safety --snapshot`** — `refs/safety/1789973940`.

## Messung dieses Atoms (kein Punkt)

- **`post.md` `An entscheid:` gefaltet** — Nr. 11 „strukturierte Feld-Grammatik":
  Operator-Entscheid gebaut (Grammatik IST `docs/specs/sources-v2-spec.md` §1;
  3-Token-`field`-Form als §10-Gap registriert, Header-Referenz
  `src/main.rs`→`src/archivar/parse.rs` korrigiert, 8 Parser-Arme §10-Pending);
  Zeile aus `post.md` gelöscht.
- **smail-Wahrheits-Riegel erledigt** — bau folge118 hat
  `tools/service/src/bin/smail.rs` gebaut (QUELLEN-Parse, `--send`-Refusal
  `exit(2)`, `--dry-run`-Tabelle, kein Bypass); folge72-Punkt gelöscht.

## Offen (Tafel — parallel abarbeitbar, keine Rangfolge)

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| `open_points_check` im Release | `wartend` | `eigen` | Trigger `tools-build` `35570698807` → `tools-latest` + `~/.local/bin`-Symlink; dann läuft der Baum-Abgleich im Planungs-Pass |
| Free-Model-Bench (P13 + P2–P4) | `wartend` | `eigen` | Artefakte **einmalig** lesen, sobald die Läufe durch sind — `ci_manage view 35567266519`/`35567268692`; Ranking oder pending |
| PINE64-Dokumentationspflicht | `blockiert` | `linie:bau` | `An bau:`-Zeile in `post.md` (Mantis-Shrimp minimal bauen — Spec `docs/specs/mantis-shrimp-bom.md` + BOM liegen —, dann Ox64 dokumentieren) |

## Operator-Queue (Stand folge73; einfache Sprache, je Frage mit Alter)

3. **ISH Chat (GitHub-Dritt-OAuth-App, Scopes `read:user`/`user:email`)** —
   Sicherheitsereignis. **Frage:** App unter
   `github.com/settings/connections/applications` widerrufen? (Alter: seit 2026-09-20)
4. **SuperDARN** (`blocked account`, `phi/blocked_sources.φ:21`) — HF-Radar-
   Ionosphären-Konvektion; Route über Globus + PI-Vereinbarung. **Frage:** Konto +
   PI-Vereinbarung eingehen? (Alter: seit 2026-09-16)
5. **solar-system-open-data REST** (`blocked key`, `phi/blocked_sources.φ:60`) —
   HTTP 401 (Bearer-Token). **Frage:** Konto/Token anlegen? (Alter: seit 2026-09-20)
6. **Amentum Developer** (`blocked account`, `phi/blocked_sources.φ:65`) —
   geomagnetisch/aviation-radiation/gravity (trial). **Frage:** Registrierung
   `developer.amentum.io/register`? (Alter: seit 2026-09-20)
7. **Split-Routing-Verifikation** — `./bin/proton-exit.sh ca` + direct↔tunnel-
   Nachmessung der 8 `000`-Hosts. **Frage:** Operator-Wort/Route (sudo+Netz)?
   (Alter: seit Ernte folge12–17)
8. **Cookie-Transfer** — `operator-gebunden`, Auslöser „Bedarf". (Alter: seit 2026-09-16)

## Benchmark

- **Delegation (Entscheid-Folge 73):** 1 × `research-max` (pro/max) für die
  Programm-Messung — vier Programme, JS-gerenderte Seiten, DNS-tote Domains
  (hartes Recherche-Atom, kein Routine-Job). Kein flash-Doppel-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/post.md` (`An entscheid:` gefaltet + gelöscht; `An bau:`
  PINE64 neu)
- `docs/handover/handover-2026-09-21-entscheid-folge73.md` (neu)
- Move `handover-2026-09-21-entscheid-folge72.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (Postfach- + CI-Zeile)
- **nicht** angefasst: fremde `phi/*`, `tools/harvest`, der gestagete
  ernte-Move, fremde post-Zeilen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Nach dem Push wird
`ci-check.yml` dispatcht; kein Poll — das Ergebnis einmalig lesen
(`ci_manage view <id>`).
