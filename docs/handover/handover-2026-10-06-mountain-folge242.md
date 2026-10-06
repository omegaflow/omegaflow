<!--
  title: Handover — Mountain-Folge 242 (2026-10-06)
  session: Mountain-Folge 242
  class: handover
  date: 2026-10-06
  sha256: dd481a33aca22597a26c53ac53abb7ebf16457ae7b238fb1556dd47155602aeb
  status: live
-->
# Handover — Mountain-Folge 242 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`, gelesen
2026-10-06). Diese Session konsumierte `handover-2026-10-06-mountain-folge241.md`
(→ `archiv/`) und maß die adressierten Blöcke `future-183`, `mycelium-236` gegen den
Baum: **alle erledigt** (iEEG-Bindung `sources.φ:3584-3589`; AGrav/CEEIN registriert
`sources.φ:9071`/`:9239`; gistemp/godas `sha256` `sources.φ:18870`/`:17141`;
Chandrayaan-Felder `sources.φ:10025-10027`; JAXA-Reader descoped
`blocked_sources.φ:96`; clippy `units.rs` committet) — nichts zu falten, die Sender
können die Blöcke streichen. Zwei `grind-flash`-Dispatches, kein pro/max.

**Nachtrag (Operator: „kannst du das bitte fixen?").** Der Fink-`blockiert`-Stand war ein
**Messfehler** (falscher Host `api.fink-portal.org`); `api.lsst.fink-portal.org` antwortet
2026-10-06 HTTP 200 für `/`, `/schema`, `/sources`, `/objects`, `/cutouts`, `/conesearch`.
Die allgemeine Query ist `/api/v1/conesearch` (nicht `/objects` — das ist `diaObjectId`-
gebunden) und wird schon von `skydirection_compiler` (`FINK_CONE`) geerntet, als Zeuge
`s2-direction` (`witnesses.φ:11`) registriert — **kein `sources.φ`-Block nötig** (Fink ist
ein Skydirection-Zeuge, kein `sources.φ`-Ursprung). **Cutout-FITS-Reader gebaut:**
`tools/measure/src/bin/fink_cutout_probe.rs`, live gemessen (`POST /api/v1/cutouts`,
`diaSourceId=314002968168367863` → 34560 B FITS, NAXIS 30×30, BITPIX −32, Apertur-Summe).
trishuli: Live-Route gemessen — `POST /site/getRiverWatchBySeriesId_Single`
(csrf + `seriesid=23251`) → `status:success` mit **leerer** Serie (`river=[]`) über
Perioden 1–4; kein `timeSeries` ⇒ Trigger nicht gefeuert, `wartend` bleibt.

## Burn: open 0.0000 · close 0.1894 · cap 0.25 (Operator eröffnete das Atom erneut: Fink+trishuli, dann Schwarm) · Grund: flash-first — Line-Session $0.1387 + zwei `grind-flash`-Dispatches ($0.0325 fink-Arm, $0.0182 cutout-Reader) + Schwarm (12 Stimmen, frei), kein pro/max

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
„braucht es pro?" — flash-first bestätigt | 2026-10-04 | Operator (Session, Mountain 233)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-05 | Operator (Session, Mountain 235)
„was fehlt hast du in die secrets local geschaut?" — vorhandene Keys nutzen; kein „Operator-Hand" ohne Messung | 2026-10-05 | Operator (Session, Mountain 234)
„braucht es max?" — flash-first erneut bestätigt; ExoMars-Parser per grind-flash gebaut | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und kannst du dir das bitte ansehen?" — flash-first; die zwei Punkte-Listen gegen den Baum messen | 2026-10-05 | Operator (Session, Mountain 235)
„messe nochmal den aktuellen zustand dann commit" — Atom 2: neue adressierte Blöcke falten, FMI-GIC-fein-grain bauen, em-Apertur messen, committen | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und max?" — flash-first bestätigt: alle Kanal-/Serien-Arme per grind-flash/general geschlossen, kein pro/max | 2026-10-05 | Operator (Session, Mountain 236)
„bitte gib das dem rat den tauchern für wissenschaft und forschung und den 3 online stimmen" — Contract-Frage (Sentinel vs. Presence-Bit) an Rat + research-max + UI-Stimmen | 2026-10-05 | Operator (Session, Mountain 236)
„brauchen wir überhaupt pro für den rat/council … in dateien steht veraltet wann pro angebracht ist" — Council → flash/low; pro/max nur noch Eskalation nach gemessener flash-Fehllage | 2026-10-05 | Operator (Session, Mountain 236)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-05 | Operator (Session, Mountain 237)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-06 | Operator (Session, Mountain 238)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-06 | Operator (Session, Mountain 239)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-06 | Operator (Session, Mountain 240)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-06 | Operator (Session, Mountain 241)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-06 | Operator (Session, Mountain 242)

## Offen (aufgeschlüsselt)

### trishuli (DHM Nepal) — Station 4913 zerstört, Nachfolger offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Nachfolge-Pegel meldet — Prüfung via `POST dhm.gov.np/site/riverWatchTableViewData` (4913 dann nicht mehr `" "`)
- **Lage:** (gemessen 2026-10-06) 4913 „Bhotekoshi at Rasuwagadi" meldet nicht:
  `POST dhm.gov.np/site/riverWatchTableViewData` (ohne Auth; 195 Stationen melden)
  trägt `waterLevel: " "` für 4913; die Serie `getRiverWatchBySeriesId_Single`
  (`seriesid=23251`) ist leer (`river=[]`) auch für Vor-Flood-Daten (2026-08-25).
  Schwarm + GLM/Claude (Zeugen, nicht Verdikt): Station am 26.08.2026-Flood physisch
  zerstört (CHWRR Assessment Report II, letzter Wert 3.8 m, Senderstopp 08:40; ICIMOD;
  Presse); Betrawati `52`/`4783` ebenso leer. **Live-Nachbarn (Bulk):** Trishuli
  Dhunche `4657` (2.16 m), Bhorle `4661` (3.80 m), Galchi `5705`, Kali Khola `4781`
  (4.33 m); Narayani Devghat `265` (4.12 m). **Kontrolltest (gemessen 2026-10-06):**
  derselbe `getRiverWatchBySeriesId_Single`-POST mit Devghat `seriesid=4140`, `period=2`
  → 24 Stundenzeilen (4.14–4.19 m) ⇒ Abfrage-Mechanik valid, 4913/`23251` leer ist
  stationsspezifisch (nicht Query-Fehler).
- **Blockade:** Die Experiment-Station existiert nicht mehr.
- **Braucht:** Entscheidung — `descoped` für 4913 (zerstört, gemessen) und die
  Trishuli-Serie auf einen Live-Nachbarn umhängen (Dhunche/Bhorle), oder ruhen.

## Träger (Prosa, eigene)

- `docs/surveys/survey-2026-10-03-medizinische-datenquellen.md` (`class: survey`, Header-sha `feb28078…`) — Pool disponiert.
- `docs/specs/sources-v2-spec.md` (`class: concept`, Header-sha `11c6db3c…`).
- `docs/surveys/survey-raetsel-bestand.md` (`class: survey`, Header-sha `524d61dc…`).
- `docs/blatt/blatt-pioneer-floor-falsifikation.md` (`class: sheet`, Header-sha `5bb1696b…`).
- `docs/concepts/kybernetische-astrophysik.md` (`class: concept`).
- `docs/concepts/tools-map.md` (`class: concept`).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (`class: survey`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` (`class: survey`, Header-sha `ac672e3e…`).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern
  (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI.
  Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün;
  offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz. Der
  private Wort-Laut nur im privaten `state/operator-gespraeche/`.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und
Push. `/consent` ist der session-weite Consent, nie das Commit-Wort.
