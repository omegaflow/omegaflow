<!--
  title: Daten-Holdings-Inventur (Teil B) — was existiert, wo, was gehört wohin
  class: survey
  date: 2026-09-03
  sha256: f9c1787cdb717d37a9b86d6ce14689b4cb3e294254ea89a2276634d0b4432471
  status: live
  see-also: AGENTS.md (The Cache Ablage), docs/specs/ref-phi-register.md docs/specs/ref-auth-apis.md 
-->

# Daten-Holdings-Inventur (Teil B)

Gemessen am 2026-09-03 auf dem lokalen Run-Host.
Zweck: vollständige Landschaft der großen Ablagen erfassen, jede nach Herkunft
und Disposition einordnen, bevor irgendetwas bewegt wird. Regel: nie blind
Gigabytes verschieben; Mess-/Sitzungs-/Backup-Daten erst nach Freigabe je
Holding.

## Run-Pfad (autoritativ)

- `OMEGAFLOW_STATE`-Default: `~/.local/state/omegaflow` (systemd-Dienste setzen es explizit auf genau diesen Pfad).
- Cache-Root (`cache_root()`): `~/.local/state/omegaflow/archivar_cache`.
- Mess-/Probe-Datensätze gehören als **flache Dateien** direkt in diesen Cache-Root (siehe AGENTS „The Cache Ablage"). Füllung ist lazy: erst ein Lauf/Compiler oder manuelles Staging legt sie an.
- Dauerhafte Heimat jedes Datensatzes ist das CDN (`omegaflow/sources`, netloc-Releases). Abwesenheit lokal = `pending` (0 honored), nie „verloren".

## Gesamtland (gemessen, vor Aufräum)

| Ablage | Größe | Natur | Disposition |
|---|---|---|---|
| `projects/omegaflow-legacy` | 14 G | alter git-repo-Snapshot; 13 G `target/` | `target/` = regenerierbar → **entfernt 2026-09-03 (13 G)**; Code liegt in Git-Historie |
| `projects/omegaflow` | 4,2 G | Live-Repo; 3,8 G `target/` | `target/` regenerierbar → **entfernt 2026-09-03 (3,8 G)** |
| `knowledge` | 32 G | Mess-/Sitzungs-/Herkunfts-Sicherung | Archivgut |
| `knowledge/archive` | 16 G | davon `archive/data` 15 G (abk, opencode-tmp, jwst-harvest …) | Archivgut — weiter zu inventarisieren |
| `knowledge/sessions` | 5,1 G | LLM-Sitzungs-Logs | Archivgut |
| `knowledge/provenance` | 2,4 G | Herkunfts-/Nachweis-Daten | Archivgut |
| `knowledge/data` | 1,5 G | Ephemeriden (`ephemeris_{body}.bin`), `omni2_serie.bin` | Staging-Kandidaten → Cache-Root |
| `backups` | 23 G | Sicherungskopien | Dedup gegen CDN/Repo |
| `backups/omegaflow` | 16 G | Repo-Schnappschuss (`data/` = gitignored Messdaten: cmb/cosmicflows, pioneer, ephemeris) | Dedup |
| `backups/omegaflow-worktrees` | 6,5 G | Worktree-Schnappschüsse (gitignored cwd-Daten wie `omni2_serie_1h.bin`) | Dedup |
| `backups/state/omegaflow` | 194 M | Kopie des Live-OMEGAFLOW_STATE (gate/mail/reports/phi) | Dedup gegen Live-State |

## Konkrete Probe-Datensätze und ihr jetziger (Backup-)Ort

| Datensatz | aktueller Ort |
|---|---|
| `omni2_serie.bin` | `knowledge/data/omni2_serie.bin`, `backups/omegaflow/data/` |
| `omni2_serie_1h.bin` | `backups/omegaflow-worktrees/bz-messen/omni2_serie_1h.bin` |
| `abk_dbdt_daily.tsv` | `knowledge/archive/data/abk_dbdt_daily.tsv` |
| dark_flow (`cmb_planck_smica_n64.json`, `cosmicflows_cf4.json`) | `backups/omegaflow/data/` |
| Ephemeriden (`ephemeris_{body}.bin`) | `knowledge/data/` (12 aktive) + `backups/omegaflow/data/` |
| Pioneer (`pioneer10_*.bin` u. a.) | `backups/omegaflow/data/` |
| `abk_dbdt_1h_*`, kegel-, GIC-/Corona-Serien | **descoped** (gemessen 2026-09-26: keine eigenen Quellen — lokale Probe-Logs `signal_cone_audit_probe`, `corona_{lag,event,ladder,conditional,confound_matrix}_probe`; `abk_dbdt_1h` → phi/sources.φ:1567, GIC → phi/sources.φ:8642/8762) |

## Aktive Ephemeriden (gehören als `omegaflow_eph_{body}.bin` in den Cache-Root)

earth, juno, jupiter, mars, mercury, neptune, new_horizons, saturn, uranus,
venus, voyager1, voyager2. (`new_horizons`/`voyager1`/`voyager2` sind 976-B-Placeholder, pending.)
Nicht aktive (nur Archiv): cassini, europa_clipper, galileo_e1/e2, juice,
messenger, near, rosetta.

## Erledigt (2026-09-03)

- `omegaflow-legacy/target` + `omegaflow/target` entfernt (~17 G freigegeben, regenerierbar).
- Staging in den Cache-Root (`~/.local/state/omegaflow/archivar_cache/`), als Kopien, Backups bleiben:
  `omni2_serie.bin`, `omni2_serie_1h.bin` (aus bz-messen), `abk_dbdt_daily.tsv`,
  `cmb_planck_smica_n64.json`, `cosmicflows_cf4.json`, und die 12 aktiven
  Ephemeriden als `omegaflow_eph_{body}.bin` (earth/juno/jupiter/mars/mercury/
  neptune/new_horizons/saturn/uranus/venus/voyager1/voyager2).
- Dedup (byte-identische Duplikate entfernt; je Datensatz bleibt Live-Cache +
  mindestens eine Sicherung + CDN):
  - regenerierbarer Build-Schrott in Backups (`gate-tragen/target`, `tools-betten/target`, ~1,8 G);
  - Ephemeriden-Duplikate in `backups/omegaflow/data`, die identisch zu
    `knowledge/data` waren (~1,5 G; 2 dort verbleibende sind nicht-identisch → behalten).

## Offen / Befunde (Schritt-für-Schritt, je Freigabe)

1. `omegaflow-legacy` (ohne `target/`) nach `projects/archive/omegaflow-legacy` verschoben (Code in Git-Historie). `knowledge/` und `backups/` bleiben als **Sicherungs-Archive in situ** — nichts im Repo referenziert sie; eine Umlagerung dieser ~50 G irreplacebarer Sicherungsdaten bedarf einer eigenen, definierten Ziel-Layout-Entscheidung, kein Blindwurf. Vorlage: „Ziel-Layout — Migrationsplan" unten.
2. **Lokalisierung der Serien `abk_dbdt_1h_*`, kegel-Log, GIC/corona: `descoped`** — als Datei nirgends unter allen Holdings vorhanden (nur ein Verdict-Report `knowledge/archive/reports/report-09-signalkegel…`). Gemessen 2026-09-26: keine eigenen Quellen, sondern lokale Probe-Logs (`signal_cone_audit_probe`, `corona_{lag,event,ladder,conditional,confound_matrix}_probe`); `abk_dbdt_1h` ist registriert → phi/sources.φ:1567, GIC ebenso → phi/sources.φ:8642/8762.

## Ziel-Layout — Migrationsplan (Wort steht, 2026-09-30)

Operator-Wort 2026-09-30 (Future-Folge 158, `state/operator-gespraeche/`):
**der Empfehlung folgen** — Ziel-Layout = **CDN-/`data/<netloc>/`-Schema** (der
Compiler-Standard). Es wird noch nichts angelegt oder bewegt; der nächste gemessene
Schritt ist die Byte-Messung je Holding (Schritt 2). Die Holdings liegen außerhalb
des Repos (`~/knowledge`, `~/backups`) und bleiben ungetrackt; getrackt ist nur diese
Vorlage.

### Drei Ziele, ein Satz je Datensatz

- **cache-root** (`~/.local/state/omegaflow/archivar_cache/`): Live-Staging,
  flache Dateien (AGENTS „The Cache Ablage"). Füllung lazy.
- **CDN** (`omegaflow/sources`, Netloc-Releases): dauerhaftes Heim; dorthin
  nur über die Registry — `phi/sources.φ`-`url`-Zeile + CI-Manifestation
  (AGENTS „CDN-Manifestation"). Lokale Kopie = Sicherung.
- **archive-root** (`$HOME/backup/archive-root/`): unbewegliches Altgut
  (Sitzungs-/Nachweis-/Rohdaten ohne CDN-Heim).

### Zielstruktur (Vorschlag)

- `knowledge/` bleibt das lokale Sicherungs-Archiv in situ:
  - `knowledge/data/` — Staging-Kandidaten (Ephemeriden, `omni2_serie.bin`):
    zuerst cache-root + CDN-Registrierung; der lokale Rest ist Backup.
  - `knowledge/archive/data/` — Roh-Ernte (`abk`, `opencode-tmp`,
    `jwst-harvest`): je Datensatz prüfen — registriert → CDN; sonst
    archive-root.
  - `knowledge/sessions/`, `knowledge/provenance/` — Archivgut ohne CDN-Heim:
    bleiben `knowledge/` (oder archive-root).
- `backups/` bleibt der lokale Sicherungs-Root, nach Dedup auf die
  Unique-Bytes geschrumpft:
  - `backups/omegaflow/` — Repo-Snapshot: dedup gegen Repo + CDN; nur die
    gitignorierten Messdaten bleiben, die keinen Live-/CDN-Zwilling haben.
  - `backups/omegaflow-worktrees/` — Worktree-Snapshots: dedup gegen Repo.
  - `backups/state/omegaflow/` — Live-State-Kopie: dedup gegen Live-State.

### Was wohin — Mapping (heute → Ziel)

| Holding (heute) | Ziel | gitignored | CDN |
|---|---|---|---|
| `knowledge/data` (1,5 G: Ephemeriden, `omni2_serie.bin`) | cache-root (gestaged) + CDN-Registrierung | ja | die registrierten |
| `knowledge/archive/data` (15 G: abk, opencode-tmp, jwst-harvest) | registrierte → CDN; Rest archive-root | ja | teilweise |
| `knowledge/sessions` (5,1 G) | `knowledge/` / archive-root (kein CDN) | ja | nein |
| `knowledge/provenance` (2,4 G) | `knowledge/` / archive-root (kein CDN) | ja | nein |
| `backups/omegaflow` (16 G) | dedup gegen Repo+CDN; Unique-Bytes bleiben Backup | ja | die registrierten |
| `backups/omegaflow-worktrees` (6,5 G) | dedup gegen Repo | ja | nein |
| `backups/state/omegaflow` (194 M) | dedup gegen Live-State | ja | nein |

### Schrittfolge (jeder Schritt endet an der Kante)

1. ~~**Operator-Wort** zum Ziel-Layout~~ — **erledigt 2026-09-30** (Future: „der Empfehlung folgen", CDN-/`data/<netloc>/`).
2. **Byte-Messung** je Holding gegen Live-Cache / `phi/sources.φ`+CDN / Repo
   (vor jedem Move): was ist ein byte-identisches Duplikat, was unique.
3. **Registry-first** für jeden Mess-Datensatz: `url`-Zeile in
   `phi/sources.φ` → CI-Manifestation → CDN; erst danach ist die lokale Kopie
   „Sicherung".
4. **Move nur der Unique-Bytes** in die Zielwurzel; Dedup-Duplikate erst
   löschen, wenn Schritt 2/3 sie als ersetzt gemessen haben (`0 honored`:
   ohne Nachbau-Quelle nichts löschen).
5. Sitzungs-/Nachweis-Daten (kein CDN-Heim) folgen dem Operator-Wort ins
   Zielgewölbe; kein Blindwurf über ~50 G.

**Kante:** Der Plan legt nichts an und verschiebt nichts. Das Operator-Wort zum
Ziel-Layout steht (2026-09-30, CDN-/`data/<netloc>/`); jeder der obigen Schritte ist
ein eigener, gemessener Move. Nächster Schritt: die Byte-Messung je Holding
(Schritt 2).
