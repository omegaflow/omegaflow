<!--
  title: Handover — Forschung-Folge 74 (Stand 2026-09-18)
  session: Forschung-Folge 74
  class: handover
  date: 2026-09-18
  sha256: b9f84b38a0dda81979b115955db7827f5ab76328684bd4e139bfbda04423fb5a
  status: live
-->
# Handover — Forschung-Folge 74 (2026-09-18)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-18)

- **HEAD** `8d1264bb` == `origin/main`; fremd uncommittet (nicht angefasst):
  `opencode.json`, `src/archivar/*.rs`, die drei gestagten
  `handover-2026-09-16-*`-Renames. Safety-Net `refs/safety/1789709906`.
- **CI** (`ci_manage list`): die zwei in diesem Atom dispatchten Läufe —
  `pioneer-cell-census` `35312007651` **queued**, `harvest` `35312009465`
  (format `lro_trk`) **pending**, beide @`8d1264bb`. Davor `harvest` `35311500920`
  pending; `pii-exposure` `35287195140` failure (exit 2 = Exposition bleibt,
  erwartet); `xp-pilot-cdn` `35305640254` failure; sonst fremde Linien.
- **Postfach** — letzter Ledger-Eingang `1789689115` (Rubin-Forum), kein
  Agenten-Eingang (zitiert `external-state.md:20`, nicht kopiert).

## CDN-Concurrency Follow-up (härtester undatiert, Trigger gefeuert)

- **Gemessen 2026-09-18** (Delegation `general`/flash, `ci_manage view`/`log`):
  der Trigger ist **gefeuert** — `planetary-odf-cdn` Lauf `35189038542`
  @`87680961`, mro_odf-Job `105102142213` **success** mit echtem Harvest +
  Manifest (2026-09-17T10:09Z); alle späteren mro_odf-Jobs `success = skip`
  (Shards present). `mro_odf` ist **kein** `phi/harvest.φ`-Key; die 14
  Shard-Blöcke stehen in `phi/sources.φ:7241–7345`.
- Der Prefix-Check (`.github/workflows/planetary-odf-cdn.yml:35–40`) testet nur
  `grep -q "^mro_odf_"` — **keine** Vollständigkeit; ein unvollständiger
  Shard-Satz würde übersprungen.
- (Schritt: den Check auf die erwarteten Shard-Namen härten — Manifest-Vergleich
  statt Prefix-Grep; Design offen, Rat bei Bedarf. Messbar nach dem nächsten
  `planetary-odf-cdn`-Lauf.) · `pending` (nächstes Atom)

## §4 fsky-Census (`wartend`)

- Trigger = neuer Census-Lauf. **Dieses Atom dispatcht:** `pioneer-cell-census`
  `35312007651` queued @`8d1264bb` (2026-09-18). (Schritt: `ci_manage view <id>`;
  bei success `gh run download <id> -n pioneer-cell-census` →
  `pioneer-cell-census.txt`; Kreuz-Rang + r²-Peak gegen die Zwei-Arm-Frage deuten.)

## LRO utF (`wartend`)

- Trigger = Abschluss des `harvest`-Laufs. **Dieses Atom dispatcht:**
  `35312009465` (format `lro_trk`) pending @`8d1264bb`; der alte `35310768153`
  blieb in_progress/stale. (Schritt: nach Abschluss `archive_search --sniff` auf
  `lro_trk.bin`, sha256/Größe, `phi/harvest.φ:25–32` + `phi/sources.φ`-Block auf
  present.)

## BepiColombo (`wartend`)

- **Gemessen 2026-09-18** (`archive_search --playwright`): `bc_mpo_more/` trägt
  `bundle_bc_mpo_more.lblx` (2026-09-17 19:11) + readme + `document/`, **kein
  `data/`**. Trigger = `data/`-Manifestation (ESA-seitig, extern). (Schritt: bei
  Manifestation PDS4-TNF/ODF-Parser nach dem tatsächlichen Datentyp.)

## NSE/Haug — Rohdaten zugesagt (`wartend`)

- Keller-Antwort (17.09.), sendet „in einigen Tagen". Trigger = Dateieingang.
  (Schritt: bei Eingang `nse_haug_trisp`-Quelle + Compiler + `sources.φ`;
  0-Kanon: kein Asset ohne Datei.)

## Legacy-Konzepte (`operator-gebunden`, hintenangestellt)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als 4.,
  Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt: bei
  Wiederaufnahme den Rat-Erster-Atom bauen —
  `tools/measure/src/bin/silence_map_probe.rs`.)

## Paper / Präregistrierung (`termin`)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## ci_manage `dispatch` — Entscheid (kein Bau)

- Operator-Frage 2026-09-18: braucht `ci_manage` einen `dispatch`-Subbefehl?
  **Nein** — `ci_manage` bleibt der Lesende (`list/view/log/cancel/rerun`); der
  Anstoß ist per Konvention `gh workflow run` (AGENTS.md; `gh` im line-Profil).
  Ein `dispatch` würde das Lesetool mit Schreibrecht mischen. (Kein offener Punkt.)

## Benchmark

- Delegation `general` (flash) für die mro_odf-/Prefix-Check-Messung: vollständiges
  Ergebnis (5 Läufe, Job-IDs, exakte Code-Zeilen, Verdikt) in einem Turn. Keine
  Dopplung gegen pro/max — die Routine-Recherche-Klasse ist geschlossen
  (flash-Sieger, 2026-09-16). Kein `max`-Atom in diesem Atom.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/handover/handover-2026-09-18-forschung-folge74.md`
  (+ archiviertes `handover-2026-09-18-forschung-folge73.md`).
- **Fremd (nicht anfassen):** `opencode.json`, `src/archivar/*.rs`, die drei
  gestagten `handover-2026-09-16-*`-Renames.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
