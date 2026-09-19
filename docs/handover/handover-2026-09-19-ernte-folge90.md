<!--
  title: Handover — Ernte-Folge 90 (Stand 2026-09-19)
  session: Ernte-Folge 90
  class: handover
  date: 2026-09-19
  sha256: a22070a410242cd3654cc48e612bcae02986a6364026d379a00b0fd834533034
  status: live
-->
# Handover — Ernte-Folge 90 (2026-09-19)

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

## Stehender Pass (gemessen 2026-09-19)

- **HEAD** `eb2c5f95` (== origin/main). Safety-Net `refs/safety/1789798570`.
- **Postfach** — neuester `state/mail/mail_ledger.φ`-Eingang `1789795811`
  (Rubin-Forum AGN-Lightcurves, informativ); keine Sonden-Antwort, keine
  Limadou-/SuperDARN-Antwort. `docs/zustand/external-state.md` jetzt clean.
- **CI** — `exofop-cdn` `35426107470` **success**; `glm-l1b-cdn` `35425276060`
  success; `trmm-lis-cdn` `35425276991` success; `fugin-cdn-dispatch` `35425278304`
  success; `planetary-odf-cdn` `35351411938` success; `icesat2` `35425364754`
  **cancelled**; `lro` `35425365015` in_progress. **Offen:** `harvest`
  `35426123577` **failure** — noch nicht gelesen (`ci_manage log 35426123577`).

## Source-Port — offene Arme (härtester undatiert zuerst)

- **Cassini RSS raw closed-loop** `phi/blocked_sources.φ:21-23` (neu) — CORSS_8001
  (pds-rings) ist PROCESSED RING OCCULTATION (τ(r), Marouf 1986), kein raw RSS →
  descoped mit Messung; `cassini_tnf` deckt nur `co-s-rss-1-*`. Raw closed-loop
  liegt am PDS-Atmosphären-Knoten `co-ss-rss-1-scc1..14/sce1/digr/engr` (URL
  2026-09-19 HTTP 200). (Schritt: eine `co-ss-rss-1-scc*`-Volumes messen — Format
  Tracking-Record —, dann Arm/Register.) `pending`.
- **gedi_l2a** `phi/harvest.φ:35-43` — `gather_messages`-Hang besteht fort
  (`c4533fe9` half nicht); Run am timeout 180 ohne Fortschritt. (Schritt:
  GEDI-HDF5-Pfad messen, nicht erneut dispatchen.) `blockiert` (Code).
- **rosetta_odf Shards** `phi/harvest.φ:101-108` — `planetary-odf-cdn` success,
  aber `rosetta_odf.bin` 404: der Release `archives.esac.esa.int` trägt 6 Shards
  (4×1073741816 B, 2×~290 MB), URLs **nicht** in `phi/sources.φ`. (Schritt: Shard-URLs
  kuratieren + in `sources.φ` registrieren, dann sniffen.) `offen`.
- **icesat2_atl03** `phi/harvest.φ:53-61` — Re-Dispatch `35425364754` erneut
  cancelled am timeout 360; Asset 404. Granule 1 parst (8192 Records). (Schritt:
  Harvest-Budget/Scharding für den ATL03-Tag reparieren, dann sniffen.)
  `blockiert` (Budget).
- **FUGIN Bulk** `phi/blocked_sources.φ:39-41` — Pilot `fgn00000000.sky1` present
  (14212953 B, sha256 e509844e…); die 269 Cube-Assets des Bulk-Laufs sind noch
  nicht einzeln verifiziert. (Schritt: `gh release view fugin.nao.ac.jp` / sniff
  der Cube-Assets.) `wartend`.
- **lro_trk** `phi/harvest.φ:63-73` — Run `35425365015` in_progress. `wartend`.

## Wartend (kein Auswahlpunkt)

- **Lasair-LSST** `external-state.md:23` — Re-Messung am Termin 2026-09-19:
  direct + Proton 502, Wayback 200 ohne Snapshot; Eintrag aktualisiert. `wartend`.
- **Voyager/Mariner/Viking/Juno/Cassini-Antworten** `blocked_sources.φ:52-66` —
  Anfragen offen. `wartend`.
- **Limadou PI-Freigabe** `ledger.φ:26-28` — per-act consent (Operator). `operator-gebunden`.
- **Queue-Korpora** `ledger.φ:90-128` — `--port` braucht das Operator-Wort für den
  lokalen Release-Binär-Lauf. `operator-gebunden`.

## Benchmark

- **flash-first:** der Cassini-Arm über `grind-pro` (Urteil: das CORSS-Bundle ist
  ein abgeleitetes Produkt → descoped mit Messung, statt einen falschen Arm zu
  bauen); die mechanischen Registerpflichten (ExoFOP-Close, KCDC, GLM/TRMM/FUGIN/
  icesat2/lro/rosetta-Verifikation) über `grind-flash`. Kein Doppel-Lauf, keine
  pro/max-Konkurrenz nötig.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/blocked_sources.φ` (CORSS descoped + raw-Cassini-pending
  + GLM/TRMM/FUGIN-Notes + KCDC-Block entfernt), `phi/harvest.φ` (exofop present +
  icesat2/lro/rosetta-Notes), `phi/pipeline/ledger.φ` (exofop CDN present),
  `phi/sources.φ` (nur exofop-Note-Hunk), `docs/zustand/external-state.md`
  (Lasair-Zeile), `docs/handover/handover-2026-09-19-ernte-folge90.md` (neu),
  `docs/handover/archiv/handover-2026-09-19-ernte-folge89.md` (verschoben).
- **Fremd (nicht anfassen):** die forschung-Linie-`handover`-Moves
  (`handover-2026-09-19-forschung-folge87.md` neu, die `handover-2026-09-16-*`-
  Moves), `phi/sources.φ`-Fremd-Hunks. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Kein neuer Workflow in
dieser Session gebaut (der Cassini-Arm wurde descoped) → kein Post-Push-Dispatch.
