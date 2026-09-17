<!--
  title: Handover — Forschung-Folge 63 (Stand 2026-09-17)
  session: Forschung-Folge 63
  class: handover
  date: 2026-09-17
  sha256: e170765be9569782a785c40c0adb4dedfb8f371dfc4ae9f78caf7acbcf22ad35
  status: live
-->
# Handover — Forschung-Folge 63 (2026-09-17)

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

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (automatisch, keine Auswahl)

- **Postfach** — `state/mail/mail_ledger.φ` liegt nicht im Baum (gitignored,
  lokal); der Zustand-Eintrag `docs/zustand/external-state.md:20` gilt: letzter
  externer Eingang 2026-09-16 (Sotgiu-Antwort CSES-02, „wait a few weeks"), die
  fünf Sonden-Anfragen 13:47–13:55Z ohne Antwort, NSE/Haug-Sendung 18:50Z. Kein
  neuer Eingang in dieser Session gemessen. `post.md` ohne Zeile an forschung.
- **CI-Status am HEAD `f54d35e2`** — Watchdog-Snapshot 17:05:47Z vor dem Push;
  `paper-check` `35243312945` @`a130d88d` success, `pii-exposure` `35230489744`
  failure (1×), CDN-Welle aktiv (demeter/ps1/physionet/gaia-xp/planetary-odf);
  §4-Lauf `35244044555` `pioneer-cell-census` in_progress (head `bf78b453`), ein
  Duplikat `35244087174` pending. `git_safety --snapshot` = `refs/safety/1789661283`.
- **HEAD `f54d35e2`** — `origin/main` == HEAD (keine unpushed Commits,
  Fast-Forward frei).

## §4 — fsky-Census (härtester undatiert, forschung-eigen)

- Lauf `35244044555` `pioneer-cell-census` **in_progress** (gemessen 2026-09-17;
  head `bf78b453`), Duplikat `35244087174` pending. Schritt: den Lauf **einmal**
  lesen (`ci_manage view 35244044555`; bei Abschluss `gh run view 35244044555
  --log`) und das Ergebnis in `docs/paper/twenty-second-band-ground-chain.md:188–190`
  eintragen: trägt fsky den 44–58-mHz-Komplex → upstream der ODP-Kette; trägt er
  ihn nicht → **§8 light-time** ist der erste benannte Kandidat.

## Docs-Pendings — Sichtung 2026-09-17

- **Mariner `PSPA-00316` — geschlossen (gemessen):** das CDN-Asset
  `mariner_occlt.bin` ist vorhanden; Dispatch `35244811694` (2026-09-17) lief
  idempotent auf „already present". Registriert `phi/sources.φ:9318` (note:
  run `35204897220`, 1 375 496 B). Kein offener Dispatch mehr.
- **Juno post-EFB OCRU — Abgleich geschlossen (gemessen 2026-09-17):** der
  OCRU-Bestand ist `jnogrv_0001` (21 `.ODF`, 2013-284…2016-137) und hat den
  eigenen Ernte-Arm `--volume jnogrv_0001 --out …/juno_ocru_odf.bin`
  (`planetary-odf-cdn.yml`); `juno_odf.bin` trägt die disjunkte JUGR-Serie
  `jnogrv_1001` (2016-185…2017-244). Die Survey-Zeile
  `survey-2026-09-17-sonden-request-only.md:35–38` ist entsprechend korrigiert.
  Die Manifestation + der `sources.φ`-Block liegen bei der ernte-Linie
  (`handover-2026-09-17-ernte-folge66.md:99`).
- **Pioneer ATDF dtype — kein Parser-Gap (gemessen 2026-09-17):** `DATA_TYPE` ist
  das ATDF-Feld #12 (`TKFORM`-Index 11, `atdf.rs:281`); die Gate-Werte sind
  1|2 = one-way/two-way (`atdf.rs:508/644`), `DTYPE_THREEWAY_DOPPLER = 3` ist
  bewusst unbenutzt. Die Survey-Angabe „DTYPE 12/13" ist der ODF/TRK-Code-Raum
  (`odf.rs`: data_type 12 = two-way), nicht der ATDF-Wert; die genannte Datei
  `pioneer10_doppler_tracking_SC_23.asc` existiert am Pfad nicht (die
  Markwardt-Readable- und die To-Slava-Liste tragen nur binäre
  `.DAT/.TDR/.TDF/.FL/.fl`). Dreiweg-Aufnahme wäre eine Physik-Entscheidung, kein
  mechanischer Arm.
- **Offen:** MAVEN-TNF-Shards (fehlt ein TRK-2-34-SFDU-TNF-Record-Parser +
  PDS4-XML-Label-Leser in `extract.rs`; `pds-ppi.igpp.ucla.edu/.../tnf/`); GOES-16
  ABI (`sources.φ`+Workflow, GSICS pending,
  `survey-2026-09-14-kapitulationen-pendings-inventur.md:51`); WWLLN
  (netcdf-Compiler + CDN, `survey-2026-09-13-weberin-quellen.md:188`); Ulysses/
  BepiColombo/LRO (`sources.φ`, anonymen Pfad messen,
  `survey-2026-09-16-sonden-flotte.md:57–59`). (Schritt: je Eintrag der genannten
  Survey-Zeile.)

## CDN-Concurrency Follow-up

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht
  (`grep -q "^mro_odf_"` übersieht ein halbes Set); `cancel-in-progress: false`
  bleibt bis dahin. (Schritt: auf die erwarteten Shard-Namen härten, messbar erst
  nach einem erfolgreichen `mro_odf`-Lauf.)

## NSE/Haug — Route offen, Rohdaten zugesagt

- Thomas Keller (TRISP) sendet die NSE-I(q,t)-Rohdaten „in a few days". (Schritt:
  bei Mail-Eingang `nse_haug_trisp`-Quelle + Compiler + `sources.φ`; 0-Kanon:
  kein Asset ohne Datei.)

## Legacy-Konzepte (hintenangestellt — Operator-Wort)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als 4.;
  Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt: bei
  Wiederaufnahme den Rat-Erster-Atom bauen — `tools/measure/src/bin/
  silence_map_probe.rs`, Null-Kalibrierung als Spiegel der FP/FN/Symmetrie-Gates
  von `te.rs`.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- Der Juno-Abgleich dieser Session lief als `grind-flash`-Dispatch (Routine-
  Messung: Listing + INDEX.TAB gegen die Compiler-Beine) — flash lieferte die
  disjunkte Volumen-Relation und den offenen ernte-Arm; keine Eskalation. Die
  Routine-Klasse bleibt geschlossen (flash-first).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/surveys/survey-2026-09-17-sonden-request-only.md`,
  `docs/handover/handover-2026-09-17-forschung-folge63.md` (+ archiviertes
  `handover-2026-09-17-forschung-folge62.md`). Fremde uncommittete Arbeit (nicht
  anfassen): die ~70 `M .github/workflows/*-cdn.yml`, `docs/concepts/tools-map.md`,
  `docs/zustand/external-state.md`, `phi/sources.φ`,
  `tools/utils/src/bin/archive_search*.rs`, die gestagten Renames
  `handover-2026-09-16-*` → `archiv/`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
