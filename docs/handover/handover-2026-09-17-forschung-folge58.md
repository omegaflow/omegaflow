<!--
  title: Handover — Forschung-Folge 58 (Stand 2026-09-17)
  session: Forschung-Folge 58
  class: handover
  date: 2026-09-17
  sha256: ed447b6fb8885ad85811df690e9afa9f7d78b43121886d5ea0aaa01dfa21eb9d
  status: live
-->
# Handover — Forschung-Folge 58 (2026-09-17)

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

- **Postfach** — `state/mail/mail_ledger.φ` gemessen (91 Zeilen): kein Eingang zu
  den fünf Sonden-Anfragen (NSSDC/JPL-NAV), keine Antwort auf GitHub-GC #4761801,
  Privacy, NSE/Haug, Rubin. Der Ledger trägt fremde uncommittete Änderungen →
  nur gelesen, nicht überschrieben. (Schritt: bei Ledger-Eingang lesen; öffnet
  eine Route → `sources.φ`-Eintrag + Ernte-Draft.)
- **CI-Status am HEAD `e3dcc1d2`** — Watchdog-Snapshot `/tmp/opencode/ci_status.md`
  (2026-09-17T14:57Z): aktiv planetary-odf-cdn 35218760142, health-check
  35212981417, allwise-cdn 35210315065, maven-tnf-cdn 35205267703, demeter-cdn
  35202413563, ned-cdn 35196346181, ps1-cdn 35195526277; queued physionet-cdn
  35223822751/35222570485, gaia-xp-full-cdn 35223753234/35222567901; failed
  paper-check 35197130927 (07:57Z). `ci_manage list` am HEAD bestätigt dieselben
  Läufe. `docs/zustand/external-state.md` trägt fremde uncommittete Änderungen →
  zitiert, nicht überschrieben.

## Werkzeuge (gebaut — nutzt sie)

- `archive_search` — Inhalt (`--root`)/Pfade (`--index`)/NTFS/16 Netz-Modi/`--playwright`/`--verdict`/`--sniff`/`--all`; ersetzt bash-`grep`, `curl`, webfetch.
- `sgrep [-i]` — Zeilensuche über `git ls-files`.
- `sfetch` — fetch; ersetzt `curl -s`.
- `omega_sh` — `reports|status|search|fetch|jwst`.
- `smail` — Mail (Resend), `--dry-run`; Inhalte nie getrackt.
- `register_lookup` — `--live`/`--open`/`--history`.
- `git_safety` — `--snapshot`/`--restore`/`--list`.
- `ci_manage` — `list`/`view`/`cancel`/`rerun`; statt `gh run list`/`gh run view`.
- `sread [--offset --limit]` — Datei lesen.
- `session_burn` — Burn je Session.
- `gh` — nur `--log`/`--log-failed`/`workflow run`/`run download`.

## Paper-Gate — sechs Papiere repariert, Push-Verifikation offen

- `paper-check` war am SHA 54d1b7e5 rot (`Export gate … exit 1`): tonga-lamb-crosscheck
  (title 105 > 75, abstract 267 > 200), broken-null-control (abstract 214 > 200),
  depth-phase-echo-fleet (abstract 274 > 200), corona-heating-ladder /
  h0-lines-register / sturzflut-tibet-pfeil (Kopf-`sha256` veraltet — Body-Hash
  ≠ Kopf). Alle sechs in dieser Session repariert: Kopf-sha über den Body ohne
  Header neu berechnet; die zwei langen Abstracts auf ≤200 Wörter gekürzt ohne
  Wertverlust; tonga-Titel auf 56 Zeichen. Der Reference-Gate-Schritt (arXiv/DOI)
  lief nie, weil der Export-Gate-Schritt zuerst rot war. (Schritt: nach dem Push
  `paper-check` am neuen SHA messen — `ci_manage list`, dann `ci_manage view <id>`;
  bei Rot im Reference-Gate den Befund als Handover-Zeile tragen.)

## Docs-Pendings — Sichtung 2026-09-17

- Machbare Einzelne (je Survey-Zeile belegt): Mariner `PSPA-00316` CDN-Dispatch
  (`survey-2026-09-17-sonden-request-only.md:66–67`); Juno post-EFB OCRU ↔
  `juno_odf.bin` (`:38`); Pioneer ATDF dtype-12/13 (`src/archivar/atdf.rs:508`
  gated nur 1|2); MAVEN-TNF-Shards; GOES-16 ABI `sources.φ`+Workflow
  (`survey-2026-09-14-kapitulationen-pendings-inventur.md:51`); WWLLN
  (`survey-2026-09-13-weberin-quellen.md:188`); Ulysses/BepiColombo/LRO
  `sources.φ` (`survey-2026-09-16-sonden-flotte.md:57–59`). (Schritt: je Eintrag
  der genannten Survey-Zeile.)

## Format-Gate

- Fremde Dateien aus CI-format-Lauf 35185912267 offen: `src/archivar/kcdc.rs:425`
  (ernte), `src/archivar/odf.rs:425` (bau),
  `tools/harvest/src/bin/{cassini,dart}_tnf_compiler.rs`,
  `tools/harvest/src/bin/las_compiler.rs`,
  `tools/register/src/bin/cdn_reconcile.rs:310`,
  `tools/utils/src/bin/archive_search.rs:437`. Die Kyoto-Dateien sind noch nicht
  durch einen format-Lauf gemessen. (Schritt: nächster ci-check-Lauf am
  gepushten SHA, `ci_manage view`; fremde Pfade gehören als Post an ernte/bau,
  nicht in diese Linie.)

## CDN-Concurrency Follow-up

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht
  (`grep -q "^mro_odf_"` übersieht ein halbes Set); `cancel-in-progress: false`
  bleibt bis dahin. (Schritt: auf die erwarteten Shard-Namen härten, messbar
  erst nach einem erfolgreichen `mro_odf`-Lauf.)

## Legacy-Konzepte (hintenangestellt — Operator-Wort)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als
  4.; Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt:
  bei Wiederaufnahme den Rat-Erster-Atom bauen — `tools/measure/src/bin/
  silence_map_probe.rs`, Null-Kalibrierung als Spiegel der FP/FN/Symmetrie-Gates
  von `te.rs`.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
