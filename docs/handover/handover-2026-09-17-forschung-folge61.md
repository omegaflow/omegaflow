<!--
  title: Handover — Forschung-Folge 61 (Stand 2026-09-17)
  session: Forschung-Folge 61
  class: handover
  date: 2026-09-17
  sha256: e50b156293cd49539ba8b4352eef54cc7e1d5b03ca2cde64e296b89a4e519e73
  status: live
-->
# Handover — Forschung-Folge 61 (2026-09-17)

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

Die fälligen Einträge aus `docs/zustand/external-state.md` werden zu
Session-Beginn gemessen, bevor die Auswahl steht — ihr Ausgang verändert die
Auswahl, sie sind kein Auswahlpunkt. Ergebnis direkt in dieses Handover + den
Zustand-Ledger. Karte: `docs/concepts/tools-map.md` — bei Widerspruch gilt `--help`.

- **Postfach** — `state/mail/mail_ledger.φ` (224 Zeilen) gemessen: **kein neuer
  externer Eingang** seit Folge60. Letzte Eingänge: Rubin-Forum-Summary (17.09.,
  keine Antwort), NSE/Haug-Keller „I will send you the data … a few days" (17.09.
  09:29Z), CSES/Sotgiu „wait a few weeks" (CSES-02-Umstellung). Neu sichtbar:
  GitHub-Support-Ticket #4761801 **declined** (Ledger Z. 215 — neue Anfragen nur
  über support.github.com) → gehört der entscheid-Linie, nicht forschung.
  `docs/zustand/external-state.md` trägt fremde uncommittete Änderungen → zitiert,
  nicht überschrieben.
- **CI-Status am HEAD `84489abb`** — der Watchdog-Snapshot (17:05:47Z) liegt vor dem
  bau-Folge64-Push; `ci_manage list` am HEAD: ci-check `35241725336` pending, die
  CDN-Welle (quake-feeds/swpc-mirror/vires-hapi/allwise) aktiv. **`paper-check`
  `35230433910` @`5dcdda0d` = success** (14:10:52Z) — Reference-Gate (Schritt 2+3)
  erstmals grün. `docs/zustand/external-state.md` fremd → zitiert.

## §4-Messung — tragende Reduktionsstufe (härtester undatiert, forschung-eigen)

- Der `research-max`-Taucher hat §4 (`docs/paper/twenty-second-band-ground-chain.md:
  188–190`) gemessen: das Repo trägt **keine** Zuordnung. Ausgeschlossen sind die
  Stufen, die den Komplex erzeugen — er sitzt schon in **resid0** (rank 6 in
  resid0/resid_c, Papier `:45–47`); die Ablation zeigt nur einen Re-Rank in
  Ded. 7 (45,75 → 57,11 mHz). Die einzige in resid0 eingerechnete benannte
  Moyer-Stufe ist **§8 (light-time)**; §2/§7 (Uhr/Zeitskalen), §10.2.1
  (Troposphäre), §10.5 (Antenne) sind in der Kette absent.
- **Konkrete erste Messung (pending, kein neuer Harvest):** Band-Census auf dem
  fsky-Feld `r[1]` der Serie `data/spdf.gsfc.nasa.gov/pioneer10_skyfreq.bin` —
  zweiter Zweig in `tools/measure/src/bin/pioneer10_cell_census_probe.rs`
  (dieselben Zellen + `peak_of_cell`-Regel `:152–170`, seq aus `(r.t, r[1])`
  statt `(r.t, r.resid)`), Vergleich gegen den gemessenen r[8]-Census
  (Station 63: 45,00 mHz; Station 43: 46,45 mHz; Papier `:81–82`). Trägt fsky den
  Komplex → keine Reduktionsstufe trägt ihn (upstream der ODP-Kette); trägt er ihn
  nicht → **§8 light-time** ist der erste benannte Kandidat.
- (Schritt: den fsky-Zweig bauen, `cargo check`, die measure-CI dispatchen;
  Ergebnis in §4 eintragen. Dies ist das nächste Atom.)

## Paper-Gate — Extraktor-Coverage geschlossen

- `extract_arxiv_ids` (`tools/register/src/bin/reference_verify.rs`) um alte
  `subject-class/NNNNNNN`-IDs gehärtet (Helfer `subject_class_end`/`strip_version`/
  `is_arxiv_id`) + `#[cfg(test)]`-Gate-Test (fünf Fälle). `cargo check -p
  omegaflow-register` grün (null Fehler, null Warnungen). Beide alten IDs lösen auf
  (gemessen `archive_search --verdict`: `arxiv.org/abs/astro-ph/0012376` HTTP 200;
  `arxiv.org/abs/gr-qc/0208046` HTTP 200).
- **Offen:** `paper-check` am neuen SHA nach dem Push. (Schritt: `ci_manage list` →
  `ci_manage view <paper-check-id>`.)

## NSE/Haug — Route offen, Rohdaten zugesagt

- Thomas Keller (TRISP) sendet die NSE-I(q,t)-Rohdaten „in a few days" (Ledger
  Z. 221/223). (Schritt: bei Mail-Eingang `nse_haug_trisp`-Quelle + Compiler +
  `sources.φ`; 0-Kanon: kein Asset ohne Datei.)

## Docs-Pendings — Sichtung 2026-09-17

- Machbare Einzelne (je Survey-Zeile belegt): Mariner `PSPA-00316` CDN-Dispatch
  (`survey-2026-09-17-sonden-request-only.md:66–67`); Juno post-EFB OCRU ↔
  `juno_odf.bin` (`:38`); Pioneer ATDF dtype-12/13 (`src/archivar/atdf.rs:508`
  gated nur 1|2); MAVEN-TNF-Shards; GOES-16 ABI `sources.φ`+Workflow
  (`survey-2026-09-14-kapitulationen-pendings-inventur.md:51`); WWLLN
  (`survey-2026-09-13-weberin-quellen.md:188`); Ulysses/BepiColombo/LRO
  `sources.φ` (`survey-2026-09-16-sonden-flotte.md:57–59`). (Schritt: je Eintrag
  der genannten Survey-Zeile.)

## CDN-Concurrency Follow-up

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht
  (`grep -q "^mro_odf_"` übersieht ein halbes Set); `cancel-in-progress: false`
  bleibt bis dahin. (Schritt: auf die erwarteten Shard-Namen härten, messbar erst
  nach einem erfolgreichen `mro_odf`-Lauf.)

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

- Extraktor-Härtung als `grind-flash`-Dispatch (Routine, flash-first), §4-Diagnose
  als `research-max` (hartes Atom: mehrstufige Register-/Reduktions-Recherche).
  Kein Doppellauf; die gemessene Routine-Klasse ist geschlossen (flash-first).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `tools/register/src/bin/reference_verify.rs`,
  `docs/handover/handover-2026-09-17-forschung-folge61.md` (+ archiviertes
  `handover-2026-09-17-forschung-folge60.md`), `docs/handover/post.md`. Fremde
  uncommittete Arbeit (nicht anfassen): `docs/zustand/external-state.md`, die
  gestagten Renames `handover-2026-09-16-{entscheid-folge24,forschung-folge44,
  forschung-folge51}` → `archiv/`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
