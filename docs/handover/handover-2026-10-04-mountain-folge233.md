<!--
  title: Handover — Mountain-Folge 233 (Stand 2026-10-04)
  session: Mountain-Folge 233
  class: handover
  date: 2026-10-04
  sha256: 203db5ba367e64f649e0f025d62261ebf378859786dbbf23adca96d8ed8ab607
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

## Burn: open 0.005 · close 0.48 · cap 0.50 Grund: Runde flash-first — Line + 6 grind-flash (PDS-Felder, Disposition, witnesses, Medizin, Measure/close, Gaia-RR, Arm-Bau), kein pro/max

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

Audit aller Mountain-eigenen Einträge in `phi/blocked_sources.φ` (2026-10-04/05): die drei `gap-Token`-Risse (pds3-binary/pds4-binary/pds4-fits „Compiler-Bin fehlt") sind geheilt; DAS2-Feldverdikt gesetzt; superdarn → `blocked account`, Cassini titanNotebook / Gaia cluster_ka / TUH → `descoped`, Chang'e-Kadenz (`tau 2`) gesetzt, Viking/ExoMars/Gaia-RR-Feldverdikte stehen (Registrierung Mycelium). Offen bleiben:

### EMM-Reader-Arm — Gap
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein gefetchtes `emm_exi_l2.tar` (Member-Schema)
- **Lage:** (gemessen 2026-10-04 via `glob`/`sgrep`) `emm_sdc_compiler.rs` erkennt nur Container-Magic; kein tar-Reader in `src/archivar`; kein `emm_exi_l2.tar` lokal (`gh release view` → not found).
- **Blockade:** Member-Schema + Einheit ungemessen
- **Braucht:** ein `emm_exi_l2.tar` fetchen (Cognito = Operator-Hand), Member-Schema + Einheit messen, dann `src/archivar/emm_sdc.rs`.

### LEOS-Reader-Arm — Gap
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** eine authentifizierte LEOS-JSON-Antwort
- **Lage:** (gemessen 2026-10-04 via `sgrep`) `sgrep leos src` = 0; API `/api/admin/data/front/list` 40301 (Konto-Gate).
- **Blockade:** Antwortschema ungemessen
- **Braucht:** LEOS_USER/PASS-Session (Operator-Hand), eine Antwort als Schema-Vorlage (`kasi.rs` gibt das Muster).

### konverter-Arm — Gap
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein benanntes Feld mit `_mag`/`_magnitude`, das eine Magnetmessung ist
- **Lage:** (gemessen 2026-10-04 via `sgrep`) alle `_mag` in `sources.φ` sind astronomische Magnituden; der eine B-Feld-Key `psp_imf_bmag_nt` trägt `_nt`/nT.
- **Blockade:** kein Feld/Paar messbar → Arm nicht baubar ohne Fabrikation
- **Braucht:** die Quelle nennen, deren Einheit fälschlich als `mag` gemappt ist.

### iEEG 4D-Anker — BIDS-Sidecar
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `archive_search --verdict https://www.ieeg.org/api` wieder 200 (aktuell 503)
- **Lage:** (gemessen 2026-10-04) emit-Pfad `src/archivar/electrodes.rs` trägt `Position::Electrode`; REST/`/api`+/services 503; OpenNeuro ds003844 x=y=z=0.
- **Blockade:** kein Sidecar-Fetch-Feld in `SourceConfig` + kein Datensatz mit belegten x/y/z
- **Braucht:** `SourceConfig`-Sidecar-URL-Feld + ein Datensatz mit echten MNI-Koordinaten.

### gras-2c `range`-Wire-Slot — Architektur
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-04 via `sread`) `range` parst als `RangeAxis`-Metadaten (`parse.rs:1455`); kein Wire-Slot.
- **Blockade:** Wire-Contract-Entscheid (26×f64)
- **Braucht:** Rat/Operator-Wort, ob ein Slot nötig ist; sonst Metadaten-only mit Befund schließen.

### Geteilter Baum — eigene Code-/Register-Hunks halten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der Index ist frei (Mycelium 231h/231i schließt seinen `git add`-Stand)
- **Lage:** (gemessen 2026-10-05 via `git status`) Mycelium hält `gaia_rrl.rs`/`viking_grav.rs`/Workflows/`harvest.φ` + `sources.φ`/`extract.rs`/`main_flow.rs`/`mod.rs` im Index; meine `electrodes.rs`-Verdrahtung (types/channels/extract/main_flow/mod) und die `sources.φ`-Hunks (DAS2, Chang'e-tau) sind unstaged.
- **Blockade:** ein pfad-begrenzter Commit würde Myceliums Index-Hunks sweepen
- **Braucht:** `git status` erneut lesen; sobald der Index frei ist, `git commit <eigene Pfade>` mit den Code-/`sources.φ`-Hunks.

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

- **KRFM/Hayabusa — erledigt durch Mycelium 231h** (`186c05ac9`: Phobos-KRFM-sha256, 7 Hayabusa-LIDAR, MoRIC). Offen bleiben zwei Feld-Verdikte zur Registrierung:
- **ExoMars TGO ACS:** `format pds4_fixed_width`, `at mars`, `field ROW_DATA acs_nir_solar_occultation_spectrum gaussian-inverse-square em count`; Asset `.../Orbit_4140/acs_raw_sc_nir…EC__4_0.tab` 543800 B sha256 `465f3c07…`.
- **Gaia RR:** `format tap` (generischer Arm, `extract.rs:3854`), `at sun`, `field phot_g_mean_mag gaia_dr3_rr_g_mag inverse-square em mag`; ADQL `vari_classifier_result JOIN gaia_source WHERE best_class_name='RR'` (HTTP 200).

## An future

Origin: mountain folge233.

- **Medizin-Pool:** der Baum führt ~97 `declined` (2026-10-03/04), nicht `pending`/„on hold". Das Wort „rest auf on hold" ist als Oszillator-Gate-Verdikt (`no-physical-force`/`molecular`/`registry`) umgesetzt; ein Rückverlagern nach `pending` wäre ein Verdikt-Widerruf und braucht ein neues Wort. Kein erneutes Vorlegen (bereits gewortet).
- **TUH EEG → `descoped`** (Elektroden-Doku rein symbolisch, keine Koordinaten); **NSRR `descoped`**; iEEG bleibt `pending` (BIDS-Sidecar-Bau, 503).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz. Der private Wort-Laut nur im privaten `state/operator-gespraeche/`.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push. `/consent` ist der session-weite Consent, nie das Commit-Wort.
