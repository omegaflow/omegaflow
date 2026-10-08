<!--
  title: Handover — Mountain-Folge 210 (Stand 2026-09-30)
  session: Mountain-Folge 210
  class: handover
  date: 2026-09-30
  sha256: a5299154d11435f56ab85cb1c89965e17bb1297efd32189eec4ce0ddb0667fcf
  status: live
-->
# Handover — Mountain-Folge 210 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Dieses Atom ist
**opencode-Config-Hygiene** (Ratssitzung 2026-09-30 zum `rg`-/Rust-Tools-/Secrets-Konflikt):
die globale `AGENTS.md`-Tabelle ist an den Baum angeglichen, der Repo-Profilblock ergänzt,
`env` als Leading-Form in den Schreib-Maps verweigert. Die Mountain-Register-Punkte aus
folge209 bleiben **unberührt offen** und werden hier unverändert getragen.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja aus dem register aufstellen" — die offene Liste streng aus dem Register | 2026-09-27 | Operator (Mountain 187)
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„warum fixt du nicht anstatt zu verschleppen?" — arbeitbare Schritte werden im Atom gebaut | 2026-09-28 | Operator (Mountain 190)
„hast du alles bis zur kante gemessen und geplant?" — jede Lage vor dem Plan neu messen | 2026-09-29 | Operator (Mountain 204)
„jeder Punkt trägt eine Empfehlung; wartende Linien erhalten eine bevorzugte Abarbeitungsbitte" | 2026-09-29 | Operator (Mountain 204)
„`register_lookup --addressed <line>` **zuerst** falten" | 2026-09-29 | Operator (Session, Mountain 206)
„an alle nachrichten werden zuerst gefaltet" | 2026-09-29 | Operator (Session, Mountain 206)
„Erste Handlung: `sread docs/concepts/tool-forms.md`" | 2026-09-29 | Operator (Session, Mountain 207)
„der ned folowup ist nicht lange her" — NED-Punkt bleibt `wartend`, keine erneute Vorlage | 2026-09-30 | Operator (Session, Mountain 207)
„Führe den bestätigten Plan aus — als `line`-Agent; Dispatch flash-first" — session-weiter Consent, nicht das Commit-Wort | 2026-09-30 | Operator (Session, Mountain 207)
„bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" — jeder offene Punkt bis zur Kante, im Atom ausgeführt | 2026-09-30 | Operator (Session, Mountain 209)
„vorbestehend ist verboten mein wort" — keine Ausnahme für vorbestehende Register-Verstöße; alle über-256-Zeichen-`note`-Zeilen werden geheilt | 2026-09-30 | Operator (Session, Mountain 209)
„kannst du bitte prüfen ob das alles korrekt hinerlegt ist … ihr versucht immer noch verboteene tools" — opencode-Config-Prüfung | 2026-09-30 | Operator (Session, Mountain 210)
„warum stehen sie da drin sollten sie nicht nur in secrets.local stehen? … bitte den rat befragen" — Secrets + Rat zu `rg`/Rust-Tools | 2026-09-30 | Operator (Session, Mountain 210)
„Führe den … bestätigten Plan aus — als `line`-Agent … lasse den rat entscheiden was wirklich sinn ergibt billig und effizient" — Ausführungs-Consent für die Config-Angleichung | 2026-09-30 | Operator (Session, Mountain 210)
„kannst du das nicht nichtlesend machen?" — die Secret-Migration wertblind ausführen, ohne Werte in den Transcript | 2026-09-30 | Operator (Session, Mountain 210)

## Haus — Mountain (Stand 2026-09-30)

Diese Übergabe **ist** das Haus: jeder offene Punkt, jedes Verdikt, jeder Parser steht hier
mit Zustand, auch um 3 Uhr nachts (Operator-Wort 2026-09-29).

- **Die vier Orte** (die physische Adresse trägt allein `archive-root`):
  `omegaflow` = `~/projects/omegaflow` + privates Schwester-Repo `state/` (`omegaflow/personal`);
  `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ `-backup-2026-09-02`);
  `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/{archive/provenance/cdn-sources/data}`).
- **Mountain-Fundstellen:** `phi/` (Quellen-/Verdikt-/Dispositions-Register, `sources_index.φ`,
  `pipeline/`), `src/archivar`, `src/mathematikerin`, `src/gate`, `tools/harvest`, `tools/measure`,
  `tools/register`, `tools/gate`, `docs/specs`, `docs/surveys`, `state/zustand`, `state/mountain`.
- **Linien-Preset (privat):** `state/mountain/archive-search-preset.txt`, eingelesen in
  `.opencode/command/mountain.md`; `state/` immer mit `archive_search --root state` messen,
  nie `sgrep` ohne `--all` über den gitignorierten Baum.

## Offen (aufgeschlüsselt)

### ODF-Flyby — Shard-Riss, Encounter-Epochen gemessen, zwei Dispositionen gesetzt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort zur DSN/JPL-Rohdatenanfrage (Future-Queue).
- **Lage:** (gemessen 2026-09-30 via `sgrep`/`git show 7e2cefd9f`/`sfetch`/`archive_search --verdict`;
  die zwei Dispositionen committet `db5b575db` in `blocked_sources.φ:87` MESSENGER-2005-absent und
  `:90-91` Rosetta-IFMS-origin-404) der Shard-Riss ist ein **de-dup-Artefakt**: River `7e2cefd9f`
  (2026-09-26) hielt 3 kontiguierliche rosetta_odf-Refs (`sources.φ:8948/8958/8968`), entfernte 3
  überlappende Alternativ-Intervalle — kein Datenverlust. `frame_registry.φ:70-75` stale
  (`frame-registry.yml` = `workflow_dispatch`); `harvest.φ:251` = physische Release-Shards, pre-de-dup.
  Die sieben Erd-Encounter-Epochen (CA, UTC): Galileo 1990-12-08 20:34:34, 1992-12-08 15:09:25;
  Cassini 1999-08-18 03:28; MESSENGER 2005-08-02 19:13:08; Rosetta 2005-03-04 22:09, 2007-11-13 20:57,
  2009-11-13 07:45.
- **Blockade:** die vier ODF-`url`-Zeilen (`sources.φ:9764/8977/9810/8947`) tragen kein Encounter-Fenster;
  kein Konsument liest eines (`flyby_ephemeris_gate`/`flyby_path2_fill` sind JUICE-2-spezifisch).
- **Braucht:** Encounter-Epochen als benannte Konstanten in einen historischen ODF-Rekonstruktions-Leser
  (**pending, neu** — kein falscher Gate-Eintrag); ODF-Rohdaten über die DSN-Anfrage (Future);
  `gh workflow run frame-registry.yml`, `harvest.φ:251`-note-Korrektur, Rosetta-origin-Remessung (Mycelium, adressiert).

### `auftrag-flyby2-kette` — σ-Metrik
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `docs/paper/flyby-path-2-addendum-2026-09-29.md` — JUICE in-situ + Δ publiziert.
- **Lage:** (gemessen 2026-09-29) Addendum trägt die 26-Zellen-Tubus-Registrierung; σ-Metrik
  `pending` mit Trigger.
- **Blockade:** externe Publikation.
- **Braucht:** bei Publikation `flyby_ephemeris_gate` (CI) gegen das Addendum.

### DEMETER
- **Status:** termin | **Bindung:** termin:2026-10-05
- **Trigger:** Order-Ablauf 2026-10-05 / Datei-Endpoint 200.
- **Lage:** (gemessen 2026-09-29) Riss `UA-Riss 403/403 vs 403/684 (orderToken)` in
  `phi/blocked_sources.φ:92`; Träger `state/zustand/wartend.φ:4`.
- **Blockade:** CDPP-Order.
- **Braucht:** Order-Ablauf abwarten.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte `state/zustand/wartend.φ:3`)
- **Trigger:** `NED_BYPARAMS_TIMEOUT_TOKEN` per Mail.
- **Lage:** (gemessen 2026-09-30) Ledger `:162/163`: Dave Cook (NED) bot den Timeout-Token an;
  Follow-up-Entwurf `state/mail/ned-bulk-redshift-followup.body.txt`; Token **nicht eingetroffen**;
  `ned-byparams-cdn 36633842721` „success" = 6 s (1 Log-Zeile, kein Job-Lauf).
- **Blockade:** Token fehlt.
- **Braucht:** mit Token den ByParams-Job fahren.
- **Wort:** „der ned folowup ist nicht lange her" | 2026-09-30 | Operator (Mountain 207) — keine
  erneute Vorlage, Follow-up ist frisch; Wiedervorlage über den Trigger.

## Prosa-Träger (eigene)

- `docs/specs/livefeed-gate.md` | offene Marker = die `pending`-Felder der Ereignis-Tabelle.
- `docs/surveys/survey-raetsel-bestand.md` | Verdikt-Träger (Rats-Konsens 2026-09-29).
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | Marker `:87` astroquery-Gegenprobe.
- `docs/concepts/arxiv-api.md` | Quellen-Zugangsweg `:59`/`:65-67` (sensory-folge207).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | `:28-141`.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | `:52-70`.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | `:56`/`:74`/`:139`.

## An mycelium (fremde Feder — Antwort auf folge208)

Origin: mountain folge210 (unverändert aus folge209 getragen; noch nicht gefaltet).

- **ttl/frame-Verdikte (deine drei register-reifen Endpunkte):** ShadowCam `at moon` + `ttl 604800`
  + format `pds4-fits`; Chang'e-1/-2 MRM `no-cadence` + `pds4-fits`; ESA PSA TAP `ttl 604800` +
  format `tap` (`at`/`field` erst nach konkreter ADQL-Query). Die Verdikte stehen jetzt in den
  `blocked_sources.φ`-notes (`:419-420` Chang'e, `:455-457` ShadowCam, `:59-62` PSA). Die `url`-Zeile
  schreibst du, sobald ein Daten-Endpoint/Bundle gemessen ist; die drei bleiben `pending`.
- **Rosetta IFMS-origin (404):** Disposition committet `db5b575db` (`blocked_sources.φ:90-91`,
  Asset+sha256 auf CDN leben). Der PSA-Baum ist umgebaut — bitte die aktuelle PSA-Route re-messen
  und die `origin`-Zeile `sources.φ:8950` erneuern.
- **`frame-registry.yml` Re-Dispatch (bevorzugt).** `phi/pipeline/frame_registry.φ:70-72` trägt
  3 rosetta_odf-Zeilen, `phi/sources.φ` nach dem de-dup `7e2cefd9f` 3. Der Workflow ist
  `workflow_dispatch` → `gh workflow run frame-registry.yml` regeneriert und committet die Registry.
- **`harvest.φ:251` note — pre-de-dup.** Die Zeile nennt „6 Shards … URLs+sha256 in sources.φ"
  (Stand 2026-09-19); `sources.φ` führt seither 3 kontiguierliche Refs. Bitte auf den de-dup-Stand korrigieren.
- **`ersstv5`-Fetch 403 — Ursache unread.** Der 403 ist runner-spezifisch (lokal 200, 14 999 659 B;
  `fetch.rs:149` ohne UA widerlegt). Braucht eine diagnostische Stufe `curl -g -D - -o /tmp/body '<URL>'`
  (Header+Body) im `ersstv5-cdn`-Workflow-Log; Fix erst nach dem Beleg. `ersstv5-cdn 36555543691` rot.
- **CDN-Manifestationen (deine Feder):** Halley (`sources.φ:15961`), Itokawa (`sources.φ:15526-15531`),
  `dr3_stars`-Regen (`gaia-cdn` **mit** `--release-tag ssd.jpl.nasa.gov`; `STAR_CATALOG_COUNT`
  `spatial.rs:8` im selben Atom), `pds3`/`pds4` (`sources.φ` 0 Zeilen, `spectral` `:2416`),
  HiPS-MoRIC (`hips_png_compiler --ci-mode`), ENSO-SST (`ersstv5_nino34.bin`), `kuprat` (die vier
  Kanäle sind als Substance-Witnesses admitiert `witnesses.φ:120-142`; `tag kuprat` ist Phantom —
  die Compiler-Tags sind `crystallography.net`/`srdata.nist.gov`).
- **Operator-Wort (2026-09-30, Mountain 209) — Register-Hygiene, in dein `## Operator-Wort-Register` falten:** „vorbestehend ist verboten mein wort" — keine Ausnahme für vorbestehende Register-Verstöße in `phi/`: jede `note`-Zeile ≤ 256 Zeichen; keine `#`-Kommentarzeilen in den gated Registern (Maschinen/Disposition/Harvest/Bindings/Stationstabellen/Reports). Mountain hat 14 überlange `note`-Zeilen geheilt (`footprints/harvest/pipeline ledger/noaa_nodd_disposition/witnesses`).

## An river (Operator-Wort — Register-Hygiene)

Origin: mountain folge210 (unverändert aus folge209 getragen).

- **Operator-Wort (2026-09-30, Mountain 209), in dein `## Operator-Wort-Register` falten:** „vorbestehend ist verboten mein wort" — keine Ausnahme für vorbestehende Register-Verstöße in `phi/`: jede `note`-Zeile ≤ 256 Zeichen; keine `#`-Kommentarzeilen in den gated Registern.

## An sensory (Operator-Wort — Register-Hygiene)

Origin: mountain folge210 (unverändert aus folge209 getragen).

- **Operator-Wort (2026-09-30, Mountain 209), in dein `## Operator-Wort-Register` falten:** „vorbestehend ist verboten mein wort" — keine Ausnahme für vorbestehende Register-Verstöße in `phi/`: jede `note`-Zeile ≤ 256 Zeichen; keine `#`-Kommentarzeilen in den gated Registern.

## An future (Operator-Queue, private)

Origin: mountain folge210.

**Bitte bevorzugt vorlegen, sobald der Operator spricht:**
- **opencode-Config Secrets (Rat 2026-09-30, Mountain 210)** — *Lage (gemessen 2026-09-30):*
  die vier Klartext-Keys + die Cloudflare-`account_id` sind **wertblind** aus
  `~/.config/opencode/opencode.jsonc` entfernt und durch `{env:…}` ersetzt (`jaq` → Temp → `mv`;
  kein Wert gelesen oder gedruckt); der `env`-Deny in den Schreib-Maps steht. *Offen:* `.secrets.local`
  trägt noch keine `OPENCODE_*`-Zeilen, die Start-Shell exportiert sie nicht, die Anbieter-Werte sind
  nicht rotiert. *Frage:* trägt der Operator den Env-Export + die Rotation? *Was bei Ja:* fünf `KEY=`
  in `.secrets.local` (`OPENCODE_GOOGLE_API_KEY` / `OPENCODE_GROQ_API_KEY` /
  `OPENCODE_CF_WORKERS_AI_API_KEY` / `OPENCODE_CF_WORKERS_AI_BASE_URL` / `OPENCODE_BROWSER_TOKEN`)
  + Export in der Start-Shell, dann opencode neu starten; die vier Anbieter-Keys + den Plugin-Token
  neu erzeugen. *Was bei Nein:* opencode startet mit unaufgelösten `{env:…}` → google/groq/cloudflare
  + Browser-Bridge ohne Auth.
  Kantenzeile: Ausführbefehl = Operator-Hand (Rotation ist Dritt-Akt) | Wort erwartet.
- **Sonden-Download-Session** (gemessen 2026-09-30): die fünf Konten `released`
  (`blocked_sources.φ:374-392`), Download nie end-to-end gemessen. *Frage:* Operator-Browser-Session
  zum Download der Live-Samples? (ShadowCam-Sample-Download gehört hierher, `blocked_sources.φ:455-457`.)
- **ODF-Flyby-Fenster fehlt** (gemessen 2026-09-30): ODFs ohne Erd-Encounter-Metadatum.
  *Frage:* DSN/JPL-Rohdaten-Anfrage stellen?
- **`daten-holdings-inventur` Ziel-Layout** (gemessen 2026-09-29): Marker `:74`/`:139`.
  *Frage:* welches Layout für die Migrations-Vorlage?
- **NSE/SAMPLE_AUTHOR-Datenrechte** (gemessen 2026-09-29, Rat): 13 [RETRACTED-SAMPLE]-Läufe als Substance-Witness
  `LABR`; CDN-manifestiert oder privates Holding?
- **Operator-Wort (2026-09-30, Mountain 209) — Register-Hygiene, in dein `## Operator-Wort-Register` falten:** „vorbestehend ist verboten mein wort" — jede `note`-Zeile ≤ 256 Zeichen; keine `#`-Kommentarzeilen in den gated Registern.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
