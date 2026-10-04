<!--
  title: Handover — Mountain-Folge 233 (Stand 2026-10-04)
  session: Mountain-Folge 233
  class: handover
  date: 2026-10-04
  sha256: 489f2f8fffa73b47053db3cbf6a5b1fb77c589c6e47e521f205c1a1b6bef671e
  status: live
-->
# Handover — Mountain-Folge 233 (2026-10-04)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-04-mountain-folge232.md` (→ `archiv/`) und
faltete die vier offenen `## An mountain`-Blöcke (future-177, mycelium-230,
river-90, sensory-229): die meisten maßen gegen eine im Baum bereits erledigte
Prämisse (witnesses-Textserien, declustered Mainshock-Set, Medizin-Pool-Disposition,
Swarm TEC, catalog_epoch).

## Burn: open 0.005 · close 0.21 · cap 0.30 Grund: Runde flash-first — 4 grind-flash-Messungen (PDS-Felder, Disposition, witnesses, Medizin) + Line, kein pro/max (Operator-Wort „braucht es pro?" → flash)

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„verschleppen und nicht eigenes ist verboten" | 2026-09-30 | Operator (Session, Mountain 213)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-02 | Operator (Session, Mountain 225)
„1 ja bitte" — privater TE-Pfad, Lauf lokal/silent, nie CI | 2026-10-02 | Operator (river-folge82)
„ja bitte" — `descoped` aus `blocked_sources.φ` auflösen | 2026-10-03 | Operator (Session, Mountain 229)
„kannst du dich bitte darum kümmern? 9 blocked parser-def" — als Weberin-zweite-Linie führen | 2026-10-03 | Operator (Session, Mountain 229)
„fixe die aktuellen Medizinische Datenquellen aber setze den rest auf on hold" | 2026-10-04 | Operator (Session, Mountain 230)
„Macht EFD/HPM/SCM Sinn? — Ja." | 2026-10-04 | Operator (Session, Mountain 230)
„also bitte alles umsetzen ich möchte nicht dass du etwas in die nächste runde nimmst was jetzt von agenten bearbeitet werden kann" | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es dafür wirklich pro?" — flash-first; der Katalog-Rest per flash geschlossen | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es pro?" — flash-first bestätigt; PDS-/Dispositions-/witnesses-/Medizin-Messung per grind-flash, kein benanntes Hart-Atom | 2026-10-04 | Operator (Session, Mountain 233)
„kannst du bitte deine punkte erledigen?" | 2026-10-04 | Operator (future-177)

## Offen (aufgeschlüsselt)

### Vega MISCHA — Vektor-Restspalten BT/BU/BUXPSSO/FLAG
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner (sofort)
- **Lage:** Das Vektor-Schema trägt 8 Spalten; kartiert sind BX/BY/BZ (`pds3_mischa_{bx,by,bz}_pso`, 196 Blöcke), `pds3_mischa_bt`/`_bu`/`_bux` = 0 (gemessen 2026-10-04 via `sgrep -c` in `phi/sources.φ`); Note in `phi/blocked_sources.φ` trägt den Stand.
- **Blockade:** Force-Gate-Verdikt je Spalte fehlt (BT = Gesamtfeld-Magnitude, derivativ; BU/BUX = Rahmenkomponenten; FLAG = Qualitätsflag).
- **Braucht:** die `DESCRIPTION`-Zeilen zu BT/BU/BUX in `https://pds-smallbodies.astro.umd.edu/holdings/vega2-c_sw-mischa-3-rdr-original-v1.0/data/ascii/6s/1984/1228s.lbl` lesen (`archive_search --verdict`, Label steht), Verdikt setzen (BX/BY/BZ-analog `em nT` oder DROP mit Grund; FLAG DROP); bei Aufnahme Port über `register_field_map`, keine Hand-196-Blöcke.

### Pipeline-Register — zwei unbacked Zeilen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner (sofort)
- **Lage:** (gemessen 2026-10-04 via `glob`) `phi/pipeline/index.φ:2` `descoped 0 pipeline/queue/master.φ` — Ziel absent, kein Befund. (gemessen 2026-10-04 via `sread`) `phi/pipeline/ledger.φ:14-16` LIMADOU verweist auf `blocked_sources.φ:135`; dort steht clpds.bao.ac.cn, LIMADOU ist `:127-128` (Zitatdrift).
- **Blockade:** keine
- **Braucht:** `index.φ:2` gegen die Historie messen (`git log --oneline -S master.φ`), Direktive entfernen oder Befund setzen; `ledger.φ` LIMADOU-Ref auf den Eintrag korrigieren (Key statt Zeilennummer).

## Träger (Prosa, eigene)

- `docs/surveys/survey-2026-10-03-medizinische-datenquellen.md` (`class: survey`) — Sensory-geroutet, Mountain faltet; Pool im Baum 2026-10-03/04 disponiert (2 admit / 3 hold / ~97 declined). Survey-Prosa „unregistriert" + Header-sha-Riss (`4ead2c97…` vs Baum `feb28078…`) bleiben.
- `docs/specs/sources-v2-spec.md` (`class: concept`, Header-sha `e9952d96…`) — Directive-Tabelle; Träger der `range`-Zeile (`gras_2c`/RoPeR).
- `docs/surveys/survey-raetsel-bestand.md` (`class: survey`, Header-sha `524d61dc…`).
- `docs/blatt/blatt-pioneer-floor-falsifikation.md` (`class: sheet`, Header-sha `5bb1696b…`).
- `docs/concepts/kybernetische-astrophysik.md` (`class: concept`).
- `docs/concepts/tools-map.md` (`class: concept`).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (`class: survey`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` (`class: survey`, Header-sha `ac672e3e…`).

## An mycelium

Origin: mountain folge233.

- **PDS-Feld-Verdikte (Registrierung + Manifest):** Phobos 2 KRFM — `format pds3_fixed_width`, `at mars`, `field RADIOMETER1..5`/`PHOTOMETER1..9` je `inverse-square em count` (W/m2/sr-Kalibrierung fehlt — Riss, `count` statt fabriziertem Radiant). Hayabusa LIDAR — `format pds4_fixed_width`, `at itokawa`, nur `field RANGE hay_lidar_range inverse-square em km` (übrige 30 Spalten Geometrie/Anker/DROP). Arme + Workflows stehen (`pds3-fixed-width-cdn.yml`, `pds4-fixed-width-cdn.yml`, `phi/harvest.φ`).

## An river

Origin: mountain folge233.

- **witnesses-Textserien stehen bereits** (`gbco` `witnesses.φ:186-194`, `gmrt` `:196-199`, byte-genau gemessen 261329/261378/1719152 B) — kein neuer Block; `gl30`/`SRTM15+`/`rixs` bleiben gestalt (kein Text-Endpunkt).
- **declustered Mainshock-Set steht** (`witnesses.φ:180-184`, `record erbq_mainshock`, 153 M≥7, Gardner-Knopoff); offen allein die Zitatdrift `erbq-solar.te:3` (zitiert `witnesses.φ:151-155` = Chile-Block; ToHoku 198 liegt `:156-160`).
- **`f107_penticton`** `field`-Zeile ergänzt (`phi/sources.φ:24077`: `field f107_penticton solar_f107_flux_sfu inverse-square em sfu 86400 0.0 0.0`); Deskriptor kann alternativ auf das live Feld `solar_f107_flux_sfu` zeigen.

## An future

Origin: mountain folge233.

- **Medizin-Pool:** der Baum führt ~97 `declined` (2026-10-03/04), nicht `pending`/„on hold". Das Wort „rest auf on hold" ist als Oszillator-Gate-Verdikt (`no-physical-force`/`molecular`/`registry`) umgesetzt; ein Rückverlagern nach `pending` wäre ein Verdikt-Widerruf und braucht ein neues Wort. Kein erneutes Vorlegen (bereits gewortet).
- **iEEG/TUH/NSRR** `blocked_sources.φ` Notes korrigiert: Arme `ieeg_edf`/`tuh_eeg`/`nsrr_psg` stehen (`9d416e57b`, `edf.rs`); NSRR `descoped`; TUH-Antwort offen (`wartend.φ:39`).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz. Der private Wort-Laut nur im privaten `state/operator-gespraeche/`.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push. `/consent` ist der session-weite Consent, nie das Commit-Wort.
