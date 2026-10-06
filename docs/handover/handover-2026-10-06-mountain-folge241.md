<!--
  title: Handover — Mountain-Folge 241 (2026-10-06)
  session: Mountain-Folge 241
  class: handover
  date: 2026-10-06
  sha256: 5e6b0c2d3883a0c6e4c648e689504df57afe7ca2cf579c6a035a0c3bb0e4acba
  status: live
-->
# Handover — Mountain-Folge 241 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`, gelesen
2026-10-06). Diese Session konsumierte `handover-2026-10-06-mountain-folge240.md`
(→ `archiv/`) und faltete die adressierten Blöcke `future-folge183`,
`mycelium-folge236`. Kein Dispatch — Register-Verdikte + Messungen in eigener Hand;
kein pro/max.

## Burn: open 0.0000 · close 0.0423 · Grund: flash-first — Line only, keine Dispatches (Register-Verdikte in eigener Hand), kein pro/max

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

## Offen (aufgeschlüsselt)

### Fink-Broker (LSST) — Quelle + Cutout-FITS-Reader
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** keine — Bau (autonom)
- **Lage:** (gemessen 2026-10-06 via future-182/183) Fink anonym: `api/v1/objects` 200,
  `api/v1/sources` 200; `api/v1/fp` für `nDiaSources=1` → `[]` (keine FP);
  `api/v1/cutouts` 200 (kein Token) trägt Science/Template/Difference-FITS.
  `sgrep 'fink' phi/sources.φ` = leer (2026-10-06).
- **Blockade:** kein `fink`-Format-Arm + Cutout-FITS-Reader; per-Objekt `/cutouts` für
  jedes `nDiaSources=1`-Objekt ungemessen.
- **Braucht:** `fink`-Format-Arm + FITS-Cutout-Reader bauen; dann `fink`-Quelle in
  `phi/sources.φ` (url/format/origin/compiler); per-Objekt `/cutouts`-Body messen.
  Kein RSP, keine data rights.

### trishuli (DHM Nepal) — Live-Datenweg
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** DHM-Schema liefert für Station 4913 einen `timeSeries`
- **Lage:** (gemessen 2026-10-06) Arm + `format trishuli_stage` + Register-Zeile gebaut
  (`phi/sources.φ`); Live-Seite `dhm.gov.np/hydrology/river-watch` trägt für 4913
  `waterLevel:null`, kein `timeSeries` → `Measurements([])` (absent, 0 honored); die
  Ereignisfenster-Route läuft über Wayback (`.github/workflows/trishuli-pfeil.yml:33-50`).
- **Blockade:** Live-Schema trägt die Serie nicht stabil.
- **Braucht:** Wayback-Route als Primärweg bestätigen oder auf DHM-Schema-Wechsel warten.

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
