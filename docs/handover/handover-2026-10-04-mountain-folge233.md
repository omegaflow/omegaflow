<!--
  title: Handover — Mountain-Folge 233 (Stand 2026-10-04)
  session: Mountain-Folge 233
  class: handover
  date: 2026-10-04
  sha256: 3d137b3c0128b41d7ab9dd7e9f928b1e9d3059a1a80804e84a1664c710296720
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

Audit aller Mountain-eigenen Einträge in `phi/blocked_sources.φ` (2026-10-04): die drei `gap-Token`-Risse (pds3-binary/pds4-binary/pds4-fits „Compiler-Bin fehlt") sind geheilt — die Bins stehen (`c0af97075`/`30d93daf7`). Der DAS2-Feldverdikt ist gesetzt (`hapi_csv_{magnitude,x,y,z}_nt`, `em nT`). Offen bleiben die folgenden Arme/Messungen:

### Hope/Al-Amal EMM — Reader-Arm
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-04 via `glob`) `emm_sdc_compiler.rs` + `emm-sdc-cdn.yml` stehen; `src/archivar/emm*` = 0.
- **Blockade:** keine
- **Braucht:** Reader-Arm `src/archivar/emm_sdc.rs` nach `pds4`-Vorbild, `field` aus dem `emm_exi_l2.tar`-Schema.

### superdarn MAP-Grid — RST-Reader
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-04 via `glob`) kein `*rst*.rs`; Globus-only, RST-Byte-Offsets ungemessen.
- **Blockade:** Offsets fehlen
- **Braucht:** RST-Header-Offsets messen (1 Datensatz), Reader-Arm bauen.

### ExoMars TGO ACS — Occ-Spektrum
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-04 via `glob`) `pds4.rs`/`pds4_binary_compiler.rs` stehen; Occ-Spektrum ungemessen, Sample offen.
- **Blockade:** keine
- **Braucht:** ein Occ-Spektrum-Asset laden (`archive_search --verdict` auf `archives.esac.esa.int/psa/ftp/ExoMars2016/`), Feld-Verdikt + Sample (Mycelium).

### Chang'e-1/-2 MRM — Kadenz
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-04 via `sgrep`) Block `sources.φ` trägt `pds4_fits_ce1/ce2_mrm` + 8 Felder, Assets present; cadence unread.
- **Blockade:** keine
- **Braucht:** `cadence` aus dem registrierten Asset messen und der Zeile beisetzen.

### Viking Mars gravity (WUSTL) — Endpunkt
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-04 via `archive_search --verdict`) HTTP 206, 1 B — kein Daten-Endpoint/Arm.
- **Blockade:** Endpoint ungemessen
- **Braucht:** `pds-geosciences.wustl.edu/missions/viking/gravity.html` Baum mit `sfetch --links` öffnen, Endpunkt/Arm benennen oder `descoped`.

### Cassini titanNotebook — Reader
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-04 via `sgrep`) `cassini_odf_compiler.rs` erntet `*.ODF`; der titanNotebook-Log-Baum hat keinen Reader.
- **Blockade:** keine
- **Braucht:** Log-Baum-Format messen (`sfetch`), Reader oder `descoped` mit Befund.

### Juno Gravity Raw Orbits CSV — Reader
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-04 via `glob`) kein `juno*.rs` CSV-Reader; `juno_odf_compiler.rs` erntet ODF.
- **Blockade:** keine
- **Braucht:** CSV-Spalten messen, Reader-Arm oder `descoped`.

### Gaia — cluster_ka ADQL + RR-Reader
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-04 via `sgrep`) `cluster_ka` HTTP 400 (kein Table); korrigierte ADQL offen. RR-JOIN HTTP 200, Reader-Arm fehlt.
- **Blockade:** externe Katalog-Quelle für cluster
- **Braucht:** korrigierte ADQL gegen `TAP_SCHEMA` messen; RR-Reader via `astrometry_series`-Vorbild.

### iEEG/TUH — 4D-Anker
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-04 via `sgrep`) Arme `ieeg_edf`/`tuh_eeg` stehen (`edf.rs`); Elektroden-ICRS fehlt.
- **Blockade:** MNI/Patient-Koordinaten fehlen
- **Braucht:** Elektroden-Koordinaten + Kanal-Matrix je Datensatz registrieren (Arm steht).

### konverter — Arm für falsche Feld-Einheit
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-04 via `sgrep`) `sgrep -i konverter src/archivar` = 0.
- **Blockade:** Arm fehlt
- **Braucht:** Konverter-Arm (mag → nT) nach `unit_from_name_suffix`-Vorbild bauen.

### gras-2c — Wire-Slot für `range`
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-04 via `sread`) `range`-Direktive parst als Metadaten (`parse.rs:1455`); Wire-Slot fehlt.
- **Blockade:** Wire-Contract-Entscheid
- **Braucht:** `range`-Slot im 26×f64-Record definieren (Architektur) oder als Metadaten-only mit Befund schließen.

### LEOS — Reader-Arm
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Konto-Session (`LEOS_USER/PASS` in `.secrets.local`)
- **Lage:** (gemessen 2026-10-04 via `sgrep`) `sgrep leos src` = 0; API 40301 Login-Gate.
- **Blockade:** Konto-Session + Arm
- **Braucht:** Reader-Arm `leos.rs`; Konto-Session ist Operator-Hand.

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
