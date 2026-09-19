<!--
  title: Handover — Ernte-Folge 96 (Stand 2026-09-19)
  session: Ernte-Folge 96
  class: handover
  date: 2026-09-19
  sha256: eeb32564ebb7fad3e01640dbc30629f13979c9f74fe2b71504536d5a32e6bf9e
  status: live
-->
# Handover — Ernte-Folge 96 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19, Folge 96)

- **HEAD** `3d2e6adb` (== origin/main). Safety-Net `refs/safety/1789844625` (Start).
- **Postfach** — `smail`/`post.md`: keine ernte-Zeile, keine Nachricht an ernte.
  `external-state.md:20`: jüngster Ledger-Eingang `1789795811` (Rubin-Forum, informativ).
- **CI** — Watchdog-Snapshot 20:21Z: active `ps1-cdn` `35459788459`, `ci-check`
  `35457844175`, `hyperscanning-te` `35457694014`, `measure-gates` `35456056999`,
  `cassini-odf-cdn` `35455835693`, `te-gate` `35451283398`. `mars-dust-cdn`
  `35461031672` **success** (18:23→18:25Z) — Asset present.

## In Folge 96 geschlossen

- **FUGIN Bulk** — alle 270 `fgn*.sky1` auf CDN present (Release `jvo.nao.ac.jp`,
  270 Assets); 269 `url`-Blöcke + Pilot-sha256 in `sources.φ` registriert (270 Blöcke,
  je `origin`/`compiler`/`sha256`); `blocked_sources.φ`-Eintrag entfernt. Der seit
  2026-09-18 (bau folge78) über fünf Ernte-Runden als `wartend` geführte Punkt war
  undatiert-eigen und ist abgearbeitet.
- **mars_dust CDN-Manifestation** — `mars_dust_MY25_1FpSol.bin` 500947208 B
  sha256 `95cb85094cf4e120452b148977aa7666791ea391ccc6214b1b1f3a222e2d5402`
  present (run `35461031672` success); `sources.φ:8831` sha256 + note,
  `harvest.φ:97` `asset present`, `ledger.φ:14` `verifiziert`. Die stale Run-ID
  (`35461011833`, cancelled) war die Ursache des Dauer-`wartend`.

## Source-Port — offene Arme (kein abarbeitbarer undatierter Punkt)

Nach Schließung von FUGIN und mars_dust bleibt **kein undatierter eigener
Auswahlpunkt** — die verbleibenden Arme sind blockiert/declined oder wartend.

- **TAP-Backends dachs.fai.kz + pithia.cbk.waw.pl** `phi/pipeline/ledger.φ:10-12,18-20`.
  Re-Messung 2026-09-19: Root 200, sync-QUERY VOTable-Error `localhost:5432 connection
  refused`. **Schritt:** Re-Check (Backend-Erholung) via `archive_search --verdict` +
  sync-QUERY. `blockiert` (extern).
- **lmd.jussieu vcd + mcd** `phi/pipeline/ledger.φ:14` — declined (GCM-Modell);
  nur `mars_dust` akzeptiert (em). Kein Schritt.
- **gedi_l2a** `phi/harvest.φ` — `blockiert` (Code, hdf5.rs fremd) → an bau. Kein Schritt.
- **icesat2_atl03** `phi/harvest.φ` — `blockiert` (Budget, bin fremd) → an bau. Kein Schritt.

## Wartend (kein Auswahlpunkt)

- **Cassini ODF+RSR** `phi/harvest.φ:10-29`, `phi/sources.φ` — `cassini-odf-cdn`
  `35455835693` **in_progress** (Re-Dispatch). **Schritt:** Ergebnis aus
  Watchdog-Snapshot / `ci_manage view 35455835693`; grün → Größe/sha256 messen und
  `sources.φ`-`sha256`-Zeile + `harvest.φ` (`asset present`) fortschreiben. `wartend`.
- **Lasair-LSST** `external-state.md:23` — direct+Proton 502, Wayback 200 ohne Snapshot.
- **Voyager/Mariner/Viking/Juno/Cassini-Antworten** `blocked_sources.φ` — Anfragen offen.
- **BepiColombo bc_mpo_more** `blocked_sources.φ` — Freigabe ~April (PSA/Iess).
- **Limadou PI-Freigabe** `ledger.φ` — per-act consent. `operator-gebunden`.
- **Queue-Korpora** `ledger.φ:82-120` — `--port` braucht das Operator-Wort.
  `operator-gebunden`.

## Termin

- **EMODnet HFRADAR NADR** `phi/pipeline/ledger.φ:22-24` — maxTime unverändert
  `2026-07-30T23:30:00Z`. Nächste Re-Messung **2026-10-19**. `termin`.

## Benchmark

- **flash-first, Klasse geschlossen:** die Routine-Messungen (FUGIN-Bulk-Verifikation,
  mars_dust-Asset-Messung) sind reine `gh release view`/`ci_manage`-Reads; der
  Bulk-Diff (269 Blöcke) wurde in-Session generiert (ein `gh --jq`-Stream). Kein
  pro/max-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/sources.φ`, `phi/harvest.φ`, `phi/blocked_sources.φ`,
  `phi/pipeline/ledger.φ`, die neue `handover-2026-09-19-ernte-folge96.md`,
  `docs/handover/archiv/handover-2026-09-19-ernte-folge95.md` (verschoben), die
  Löschung `docs/handover/handover-2026-09-19-ernte-folge94.md` (Move-Rest aus
  Folge 95).
- **Fremd (nicht anfassen):** `src/archivar/fugin.rs`, `src/archivar/tests.rs`,
  `src/archivar/zarr.rs`, `src/mathematikerin/te.rs`, `src/mathematikerin/omega.rs`,
  `tools/measure/src/bin/*`, `tools/utils/src/bin/hdf5_reader.rs`,
  `docs/zustand/external-state.md`, die entscheid/forschung/bau-Handover-Moves.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
