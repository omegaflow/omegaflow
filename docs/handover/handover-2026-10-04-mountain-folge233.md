<!--
  title: Handover — Mountain-Folge 233 (Stand 2026-10-04)
  session: Mountain-Folge 233
  class: handover
  date: 2026-10-04
  sha256: d55b4ddb121437900e0d3bbf3ad6026fd7d6e3dae6348c72d4858030f3b574ae
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

## Burn: open 0.005 · close 0.25 · cap 0.30 Grund: Runde flash-first — 4 grind-flash-Messungen (PDS-Felder, Disposition, witnesses, Medizin) + Line, kein pro/max (Operator-Wort „braucht es pro?" → flash)

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

(keine — alle eigenen Punkte dieses Atoms gearbeitet.)

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

## An future

Origin: mountain folge233.

- **Medizin-Pool:** der Baum führt ~97 `declined` (2026-10-03/04), nicht `pending`/„on hold". Das Wort „rest auf on hold" ist als Oszillator-Gate-Verdikt (`no-physical-force`/`molecular`/`registry`) umgesetzt; ein Rückverlagern nach `pending` wäre ein Verdikt-Widerruf und braucht ein neues Wort. Kein erneutes Vorlegen (bereits gewortet).
- **iEEG/TUH/NSRR** `blocked_sources.φ` Notes korrigiert: Arme `ieeg_edf`/`tuh_eeg`/`nsrr_psg` stehen (`9d416e57b`, `edf.rs`); NSRR `descoped`; TUH-Antwort offen (`wartend.φ:39`).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz. Der private Wort-Laut nur im privaten `state/operator-gespraeche/`.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push. `/consent` ist der session-weite Consent, nie das Commit-Wort.
