<!--
  title: Handover — Ernte-Folge 135 (Stand 2026-09-21)
  session: Ernte-Folge 135
  class: handover
  date: 2026-09-21
  sha256: 784dec81da67ad147412dff63c1f4f46df9742747e39c6210f9d1ebca471cb38
  status: live
-->
# Handover — Ernte-Folge 135 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet. Jeder offene Punkt trägt seinen nächsten Schritt in derselben Zeile;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Folge 135)

- **HEAD** beim Start `690dd2e3` (ernte folge134); während der Session zogen drei
  fremde Linien nach: `048b1bf8` future folge85, `0b1a7d03` forschung folge140,
  `3e319087` river folge3 — HEAD jetzt `3e319087` == `origin/main`. Die eigenen
  fünf `phi/*.φ` wurden von den Fremd-Commits nicht berührt.
- **Postfach** — letzter Ledger-Eingang `1789978555` (Brave-Limit, unverändert);
  kein neuer Eingang ⇒ nicht fällig (Zitat `external-state.md:20`).
- **CI** — `ci_manage list` 2026-09-21: `ci-check` `35627171626` pending @`690dd2e3`;
  `glm-l2-cdn` `35623731418` **success** (Parser-Fix verifiziert, Asset sha256
  `1c4d89f0…` == `sources.φ:8046`); `harvest-dispatch` `35627171708` success.
- **git_safety** — Snapshot `refs/safety/1790008894` (Start).

## Offen (aufgeschlüsselt)

### PS1 Order-10-Final — Wurzel gemessen, Asset weiter absent
- **Status:** offen | **Bindung:** eigen
- **Lage:** Ziel-Tag ist `ssd.jpl.nasa.gov-ps1` (Workflow-Upload, Log-Z.583), nicht
  der Familien-Tag `ssd.jpl.nasa.gov`; Asset `ps1_dr2_coverage.fp01` dort absent
  (Release 392860039, assets []). Lauf `35595711594` success, aber „final combine
  not reached: not every band part stands yet" — Bänder 651–2643 offen, Band 650
  unvollständig (`650_90_99` fehlt). Kein Defekt, Durchsatz/Stunden-Schedule.
  `footprints.φ:19` auf den korrekten Tag nachgezogen.
- **Blockade:** Bänder-Ernte (Zeitplan).
- **Braucht:** Stunden-Schedule setzt fort; bei final combine sha256/Größe →
  `footprints.φ:19`.

### DEMETER ISL — Download blockiert, Parser-Gap (mountain)
- **Status:** wartend | **Bindung:** termin (Order) + linie:mountain
- **Lage:** `ledger.φ:78` parser-gap; `demeter.rs:60` verlangt `ISL SURVEY`
  @[204..214], `DMT_N1_1143` trägt `ISL BURST`; Order 18387 läuft,
  `availableFilesCount=0`.
- **Blockade:** Order-Download + Marker-Prüfung.
- **Braucht:** mountain: Marker auf `ISL SURVEY|ISL BURST` erweitern; dann
  Download → `demeter_compiler` → CDN (`blocked_sources.φ:70-72`).

### EPA RadNet — FRS-Weg geschlossen, AGOL-Route eingetragen
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `blocked_sources.φ:74-76`; FRS `get_program_list` 302→503/500 (ORDS),
  `RADNET`/`ERM` nicht in `FRS_PROGRAM_FACILITY`; `get_facilities` verlangt
  `state_abbr|registry_id|pgm_sys_id|zip|spatial`. Koordinaten liegen bei AGOL
  (`EPA_Radiation_Air_Monitors`, Item `caed82e8e13f47b183e2c7a1412949bc`,
  Web-Mercator); `radnet_compiler --agol`.
- **Blockade:** CI-Verifikation des Compilers.
- **Braucht:** `radnet_compiler` im CI dispatchen, `unjoined`-Zähler lesen.

### Babamul — Zugang gelöst, CDN-Manifestation offen
- **Status:** offen | **Bindung:** eigen
- **Lage:** `ledger.φ` ausstehend; `/api/babamul/objects` Bearer 200 (anonym 401);
  `/surveys/{survey}/alerts` verlangt Pflichtparameter (400 ohne); Workflow
  `babamul-cdn.yml`, CDN `babamul_alerts.bin` 404.
- **Blockade:** Manifestation.
- **Braucht:** `babamul-cdn.yml` dispatch → sha256 → `sources.φ:14016`.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin (Dienst)
- **Lage:** `ledger.φ:14`; `/tap/tables` HTTP 500 (PostgreSQL refused).
- **Blockade:** Pithia-DB.
- **Braucht:** Re-Messung bei 200.

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (PI)
- **Lage:** `ledger.φ:26`; Portal 200, keine neue Anleitung.
- **Blockade:** PI-Portal.
- **Braucht:** neue Anleitung auf dem Limadou-Portal.

### SuperDARN RAWACF
- **Status:** wartend | **Bindung:** dritter (Globus/PI)
- **Lage:** FITACF gebaut + registriert; nur full-res wide-beam RAWACF bleibt
  Globus+PI (`blocked_sources.φ:21-24`).
- **Blockade:** Globus-Gruppe/PI-Freigabe.
- **Braucht:** Globus-Invite.

### NRS02-10/12/13 SHAPE
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `nrs_stations.φ:17`; Bucket `noaa-passive-bioacoustic` (GCS) trägt
  `nrs/products/sound_level_metrics/` nur für NRS01+NRS11; `nrs/audio/` 01–12
  (kein 13). Quelle-seitig keine SHAPE-Verankerung für 02-10/12/13.
- **Blockade:** Quelle.
- **Braucht:** keine — pending hält.

### index.φ Queue/Catalog
- **Status:** ausstehend | **Bindung:** eigen
- **Lage:** `index.φ:34-121` (41 offen; queue/master, pre-cdn-Pools, tap_index- und
  catalog-Pools teils 0/leer). Die Katalog-Pools sind gitignored lokale Holdings;
  die 34 `candidate`-Zeilen wurden lokal auf `decline`/`dead` disponiert (URL-
  Varianten bereits disponierter Quellen), der getrackte Beleg steht im Ledger
  (`bestand phi/pipeline/catalog/candidates-pools`).
- **Blockade:** teils am Datenträger (gitignored).
- **Braucht:** Disposition je Pool.

### Wartend (kein Auswahlpunkt)
- Lasair-LSST (502), Sonden-Antworten, BepiColombo, EMODnet HFRADAR NADR
  (Re-Messung fällig 2026-10-19).

## Benchmark

- Kein Doppel-Lauf: die Routine-Agenten-Klasse ist geschlossen (flash-Sieger,
  `tools-map.md`). Candidates lief `grind-pro` (Force-Gate, ~$0.16), die Messungen
  `grind-flash` — keine flash/pro-Paarung auf derselben Aufgabe, kein neuer Sieger.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Commit:** `phi/sources.φ`, `phi/blocked_sources.φ`, `phi/pipeline/ledger.φ`,
  `phi/witnesses.φ`, `phi/footprints.φ`, neues Handover
  `handover-2026-09-21-ernte-folge135.md`, Move
  `handover-2026-09-21-ernte-folge134.md` → `archiv/`.
- **Fremd (nicht angetastet):** die drei Fremd-Commits future folge85 / forschung
  folge140 / river folge3 (bereits committet); die gitignored Katalog-Holdings
  `phi/pipeline/catalog/*.φ` (lokal disponiert, nicht getrackt).
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
