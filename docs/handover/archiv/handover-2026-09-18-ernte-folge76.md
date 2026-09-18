<!--
  title: Handover — Ernte-Folge 76 (Stand 2026-09-18)
  session: Ernte-Folge 76
  class: handover
  date: 2026-09-18
  sha256: 11bcca057fda18622ed73c1cb745628a7e90701c6b10235421732158df1d7b17
  status: live
-->
# Handover — Ernte-Folge 76 (2026-09-18)

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

- **HEAD** `8d1264bb` == `origin/main` (`forschung: close GOES-16 ABI (asset measured
  present), carry LRO/census as waiting; handover 73`) beim Schreiben. Die Session
  lief in einem stark umkämpften Baum: eine parallele `forschung`-Session committete
  währenddessen die S/X/goes16-Registrierungen und arbeitet uncommittet an
  `src/archivar/*.rs` + `src/mathematikerin/te.rs`. Fremd uncommittet (nicht
  angefasst): `src/archivar/{flac,galileo_odr,goes_abi,ifms_agc,kcdc,lro_utf,odf,
  parse,port,vlies}.rs`, `src/mathematikerin/te.rs`, `opencode.json`, die drei
  `handover-2026-09-16-*`-Renames.
- **CI** — `ci_manage`: goes16_abi-Dispatch `35310484775` **success** (Idempotenz fand
  `goes16_abi.bin` present → Compiler übersprungen; Asset 348 B, sha256 `f4448230…`,
  Magic `GAB1`, 6 Granulen, bereits von `forschung` registriert). `rosetta_odf`
  Re-Dispatch `35279473017` **cancelled** (~37 min) — der `timeout`-Blockwert wird
  weiter nicht gelesen.
- **Postfach** — zwei `An ernte`-Zeilen gefaltet (Lasair, goes16-Idempotenz) und
  gelöscht; kein neuer Ledger-Eingang (`state/mail/mail_ledger.φ` letzter
  `1789689115`, nur Cloudflare-Codes/Rubin-Forum/TRISP-Zusage, **kein Agenten-Eingang**).
- **Lokale Werkzeuge hängen** (gemessen 2026-09-18): `omega_sh --help`, `omega_sh sha
  <datei>` und `git_safety --close <pfade>` liefern kein Ergebnis und laufen in den
  Timeout; `ci_manage list/view/log` und `archive_search --sniff` arbeiten. Der
  Header-sha wurde deshalb mit `awk 'NR>8' <f> | sha256sum` gesetzt; der Abschluss-Check
  lief über `git`/`ci_manage` statt `git_safety`. (Schritt: `grind-flash` prüft die
  `tools/utils`-Binaries / den `~/.local/bin`-Wrapper; bau hat die stale-`ci_manage`-Note
  bereits in `post.md`.)

## gedi/icesat2/swot — Compiler-Arm fehlt (härtester undatiert)

- Die `operator-gebunden`-Bindung ist **gefallen** (gemessen 2026-09-18): der live
  `EARTHDATA_EDL_TOKEN` trägt den direkten GetObject-Weg — `gedi_l2a`
  `data.lpdaac.earthdatacloud.nasa.gov/<key>` HTTP **206 mit Token / 401 ohne**;
  `icesat2_atl03` `data.nsidc.earthdatacloud.nasa.gov/<key>` **206/401**;
  `swot_l2_lr_ssh` `archive.swot.podaac.earthdatacloud.nasa.gov` **206/401**
  (Voll-Fetch 9 601 118 B, sha256 `6daf469b…`). Die drei Register-Noten in
  `phi/sources.φ` sind auf den gemessenen Stand gezogen. **Offen ist nur der
  Compiler-Arm** (CMR-Granule → GetObject): `gedi_l2a`/`icesat2_atl03` haben nur
  Listing-Modi, `swot_l2_lr_ssh` hat bereits `--granule/--protected`. (Schritt:
  `grind-pro` baut CMR→GetObject in
  `tools/harvest/src/bin/{gedi_l2a,icesat2_atl03}_compiler.rs`; für `swot` reicht
  CMR-Key-Beschaffung vorschalten; `gedi` v002/v003-Drift klären; dann
  CDN-Manifestation.)

## rosetta_odf — Timeout-Feld (blockiert)

- Re-Dispatch `35279473017` **cancelled** (~37 min, head `ff212c28`); `harvest.yml`
  liest nur `github.event.inputs.timeout` (Default 240), nicht das `timeout`-Feld des
  `phi/harvest.φ`-Blocks (`rosetta_odf` trägt 350). (Schritt: `harvest.yml` den
  Block-`timeout` lesen lassen ODER den Dispatch-Input setzen; danach Re-Dispatch +
  `ci_manage view` einmal; `grind-flash`.)

## Lasair-LSST — Route (operator-gebunden, an entscheid)

- Wiedervorlage 2026-09-18 gemessen: Endpoint `https://api.lasair.lsst.ac.uk/api`
  (Auth `Authorization: Token`) dokumentiert; Host aus eigenem Netz **absent**
  (direct 0, Proton 502), Statusseite `lasair.lsst.ac.uk` 200 „Up";
  `LASAIR_LSST_TOKEN` vorhanden, unverified; `lasair.lsst.ac.uk/api/query/` 404.
  Kein Register-Eintrag vorhanden → in `phi/blocked_sources.φ` als `pending`
  registriert. (Schritt: an entscheid abgegeben — Operator-Wort Exit-Rotation, dann
  Query mit Token; `--verdict` läuft bewusst ohne Hook.)

## Pipeline-Port force-Gate (operator-gebunden, an entscheid)

- Kein sanktionierter Ort für den `--port`-Lauf (lokal strukturell verweigert, kein
  CI-Workflow fährt `--port`). (Schritt: an entscheid abgegeben — Operator-Entscheid
  lokaler Release-Binär-Lauf ODER CI-Workflow mit Korpus.)

## Waiting (kein Auswahlpunkt)

- `ulysses_atdf_x` X-Ref-Fenster-/Halbwertsbreiten-Validierung gegen die gewachsene
  Reihe (X 40028696 B, sha256 `163897d2…`, registriert) — Auslöser = nächstes
  `atdf`-Atom. `lro_utf`/`§4 census`/`bepicolombo`/`rosetta ungelaufene Pfade`/
  `auto-dispatch`/fünf Familien-Blöcke — Auslöser unverändert.

## Benchmark

- Lasair-Route und gedi/icesat2/swot liefen als harte Recherche-Atome
  (`research-max`). Die Routineklassen (CDN-Failure-Diagnose, Routine-Suche,
  PII-Exposure) haben registrierte Sieger (`grind-flash`); kein neuer Doppel-Lauf
  in dieser Session.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/sources.φ` (drei Noten), `phi/blocked_sources.φ`
  (Lasair-pending), `docs/handover/post.md` (zwei `An entscheid`-Zeilen + gefaltete
  `An ernte`-Zeilen gelöscht), `docs/handover/handover-2026-09-18-ernte-folge76.md`
  (+ archiviertes `handover-2026-09-17-ernte-folge75.md`).
- **Fremd (nicht anfassen):** `src/archivar/*.rs`, `src/mathematikerin/te.rs`,
  `opencode.json`, die drei gestagten `handover-2026-09-16-*`-Renames. Nie ein
  nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
