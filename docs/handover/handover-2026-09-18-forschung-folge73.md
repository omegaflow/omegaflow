<!--
  title: Handover — Forschung-Folge 73 (Stand 2026-09-18)
  session: Forschung-Folge 73
  class: handover
  date: 2026-09-18
  sha256: 098672700b41bebdf03952db0d13742f065760189fbd0076b83dff57957429e9
  status: live
-->
# Handover — Forschung-Folge 73 (2026-09-18)

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

- **HEAD** `72679549` == `origin/main`; der geteilte Zustand ist in
  `docs/zustand/external-state.md` Zeile 22 zitiert, nicht kopiert. Safety-Net
  `refs/safety/1789709329`. Fremd uncommittet (nicht angefasst): `opencode.json`,
  die drei gestagten `handover-2026-09-16-*`-Renames.
- **CI** (`ci_manage list`): `auto-dispatch` `35309750233` success; `harvest-dispatch`
  `35310733180` success (dispatchte `goes16_abi`/`lro_trk`/`rosetta_odf`);
  `harvest` `35310484775` + `35310766701` success (goes16_abi-Skip: Asset present);
  `harvest` `35310768153` (lro_trk) + `35310769705` (rosetta_odf) in_progress;
  `pioneer-odf-cdn` `35309762909` success. `ci-check` `35307448019` failure,
  `xp-pilot-cdn` `35305640254` failure — nicht Forschung.
- **Postfach** — kein neuer Agenten-Eingang (zitiert `external-state.md` Zeile 20,
  measured-at 2026-09-18).

## Kein abarbeitbarer undatierter Punkt

- Die Forschung-Linie trägt nur `wartend`/`operator-gebunden`/`termin`-Punkte.
  Die nächste Session sagt das im Planungs-Pass und arbeitet keinen erfundenen
  Punkt.

## LRO utF (`wartend`)

- Trigger = Abschluss des `harvest`-Laufs `35310768153` (in_progress @`af64a132`).
  (Schritt: nach Abschluss `archive_search --sniff` auf `lro_trk.bin`,
  `phi/harvest.φ` + `phi/sources.φ`-Block auf present.)

## §4 fsky-Census (`wartend`)

- Trigger = neuer Census-Lauf; keiner in `ci_manage list`. (Schritt:
  `ci_manage view <id>` + `gh run download <id>` → `pioneer-cell-census.txt`;
  Kreuz-Rang + r²-Peak gegen die Zwei-Arm-Frage deuten.)

## BepiColombo (`wartend`)

- Radio-Science-Bundle `bc_mpo_more/` lebt, `data/` 404 (Cruise). Trigger =
  `data/`-Manifestation. (Schritt: bei Manifestation PDS4-TNF/ODF-Parser nach dem
  tatsächlichen Datentyp.)

## CDN-Concurrency Follow-up (`wartend`)

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht;
  `cancel-in-progress: false` bleibt. Trigger = erfolgreicher `mro_odf`-Lauf.
  (Schritt: auf die erwarteten Shard-Namen härten, messbar nach dem Lauf.)

## NSE/Haug — Rohdaten zugesagt (`wartend`)

- Keller-Antwort gesendet (17.09.). Trigger = Dateieingang. (Schritt: bei Eingang
  `nse_haug_trisp`-Quelle + Compiler + `sources.φ`; 0-Kanon: kein Asset ohne Datei.)

## Legacy-Konzepte (`operator-gebunden`, hintenangestellt)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als 4.,
  Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt: bei
  Wiederaufnahme den Rat-Erster-Atom bauen — `tools/measure/src/bin/silence_map_probe.rs`.)

## Paper / Präregistrierung (`termin`)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- Routine-CI-Log-Extraktion (harvest-Läufe `goes16_abi`/`lro_trk`): der
  registrierte Sieger `grind-flash` (Klasse geschlossen 2026-09-16) wurde zitiert,
  nicht gedoppelt. Der flash-Lauf lieferte die Skip-Zeilen und die Run-Disposition;
  die Asset-Messung (`--sniff`, sha256) lief in der Hauptsession.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/harvest.φ`, `phi/sources.φ`,
  `docs/handover/post.md` (nur die eigene Zeile),
  `docs/handover/handover-2026-09-18-forschung-folge73.md` (+ archiviertes
  `handover-2026-09-18-forschung-folge72.md`).
- **Fremd (nicht anfassen):** `opencode.json`, die drei gestagten
  `handover-2026-09-16-*`-Renames.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
