<!--
  title: Handover — Entscheid-Folge 79 (SSI-Formularabgleich: 6-Min-Screencast-Träger, kein Budgetfeld; SuperDARN-Daten offen ohne PI; Ledger fortgeschrieben) (Stand 2026-09-21)
  session: Entscheid-Folge 79
  class: handover
  date: 2026-09-21
  sha256: 27f39ffb6d144c5e4230d944e5829f2ed76bd458b9d077e96cb6b143b257b2d2
  status: live
-->
# Handover — Entscheid-Folge 79 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder offene Punkt wird **aufgeschlüsselt**
geführt — kein Register-Kürzel: **Lage** (der Zustand, gemessen) / **Blockade**
(woran es hängt, oder „keine") / **Braucht** (was es löst: Werkzeug, Datei, URL,
Anfrage, Operator-Wort). Jeder Punkt trägt seinen Status-Tag (`wartend` |
`operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Entscheid-Folge 79)

- **HEAD** `6e256638` == `origin/main` (folge78 committet + gepusht). Der Baum ist
  **nicht clean** — fremd (forschung-folge134): staged `R handover-…-forschung-folge133.md
  → archiv/`, `AM handover-…-forschung-folge134.md`, `M docs/reference/KERNEL_INDEX.md`,
  `M docs/zustand/external-state.md`. Nicht angefasst; `external-state.md` bleibt der
  forschung-Linie (staged).
- **CI** — `free-model-bench` `35591218967` @`eb17b7ae` **in_progress** (kein Poll);
  `tools-build` `35591235123` pending; `ci-check` `35591295822` pending. Watchdog-Snapshot:
  viele `*-cdn` failed/queued = ernte-Domäne.
- **Postfach** — kein neuer Eingang seit folge78 (letzter `1789978555` Brave-Limit).
  `post.md:18` (1 Zeile, `An bau: text_review`) — gemessen `sgrep text_review
  .github/workflows/tools-build.yml` = 0 Treffer → der Schritt steht nicht am Baum,
  die Zeile bleibt. Neu in `post.md` (fremd **uncommittet**, forschung-folge134): eine
  Zeile `An entscheid: DEMETER ISL (CDPP)` — in den Punkt unten gefaltet; die Zeile
  bleibt stehen, weil `post.md` fremde uncommittete Arbeit trägt (nicht angefasst).
- **`register_lookup --open`** — 611 offen, 14 `zustand` due, 1 Post offen;
  entscheid-Registerpunkte `blocked_sources.φ:21/60/65` + neu `:70` (DEMETER/CDPP,
  owner ernte — kein entscheid-Punkt).
- **`git_safety --snapshot`** — `refs/safety/1789988222`.

## Messung dieses Atoms (kein offener Punkt)

### SSI-Live-Formularabgleich (`grind-flash`)
- **Trägerformat:** die Hauptbewerbung ist ein **6-Minuten-Screencast** (Voiceover;
  Struktur 1 min Person / 1 min Arbeit / 4 min Fellowship-Pläne; Inhalt > 6:15
  unbewertet). Schriftliche Alternative (~800–1000 Wörter + Begründung) nur, wo Video
  unmöglich ist (Disability/ökonomisch/Ausrüstung). Lokale Datei: **787 Wörter** — unter
  dem Ziel.
- **Kein Budget-Feld** im Formular — der lokale £4.000-Split hat kein Gegenstück
  (gehört in den Screencast).
- **Pflichtfelder offen:** Q8–Q10 (UKRI/SSI-Themes), Q11 Land, Q16 Career Stage, Q18
  JACS-3.0, Q23/Q24 Fördergeber, Q26/Q27, Q34; Q12 Home institution bei fehlender
  Institution.
- **Frist 05.10.2026 23:59 GMT+1** (Seite 10 nennt „7 October" — Widerspruch
  `unverified`); Shortlist 03.11., Selection Day 12.11.2026.
- Formular `https://forms.cloud.microsoft/e/pJdGh0rRSx` (HTTP 200, 10 Seiten);
  `/apply-fellowship-programme` 200. Nichts abgesendet, keine Repo-Datei geändert.

### SuperDARN PI-Agreement (`grind-flash`)
- **Registerkorrektur:** der Datenzugang braucht **kein PI-Sein und keinen PI-Antrag** —
  SuperDARN hat eine offene Datenpolicy (§6.1: „prior permission to access and analyse
  the data is not required"). Der Weg ist ein **kostenloses Globus-Konto +
  Gruppeneinladung** (Formular auf `/data-access` oder Mail an `superdarn@usask.ca`, mit
  Institution + Datentyp). PI-Status ist radar-/institutionsgebunden (PIEC-Konsens) — für
  Nicht-Betreiber irrelevant.
- **Rules of the Road:** Anerkennungstext Pflicht (Appendix PI-3), PI-Kontakt/
  Koautorschaft bei Einzelradar-Daten, 1-Jahr-Embargo möglich, kein kommerzieller
  Einsatz, PIEC-Benachrichtigung vor Weitergabe, Zitierpflichten (pyDARN/RST-DOIs).
- Erreichbarkeit: `/piagreement`, `/data-access`, `/data-download`,
  `frdr-dfdr.ca/repo/handle/superdarn` alle direct **200**, kein Cloudflare/Geo.
- `unverified`: Pflichtstatus des Institution-Felds; separater Data-Policy-Text;
  Globus-Einladungsbedingungen; FRDR-Download-Gate; Gebührenfreiheit (Abwesenheit einer
  Erwähnung, keine positive Messung).

- **Delegation (Folge 79):** 2 × `grind-flash` (SSI-Formular, SuperDARN-Agreement) —
  Routine-Web-Extraktion, flash-first. Klasse „Routine-Web-Extraktion" hat einen
  registrierten Sieger (flash, 2026-09-16) — kein Doppel nötig; beide lieferten
  vollständig.

## Offen (aufgeschlüsselt)

### text_review — Lauf auf den Förder-Entwürfen
- **Status:** blockiert | **Bindung:** eigen (Werkzeug) / `linie:bau` (Release)
- **Lage:** Bin gebaut + committet (`197bccf1`); nicht im `tools-latest`-Manifest;
  `sgrep text_review .github/workflows/tools-build.yml` = 0 Treffer (gemessen folge79).
- **Blockade:** der measure-Bin-Release-Pfad existiert nicht.
- **Braucht:** bau trägt `text_review` in `tools-build.yml` ein (build-Step +
  `tools.manifest`-Zeile + `gh release upload`). Post steht (`post.md:18`).

### SSI Fellowship (software.ac.uk)
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Live-Formular gemessen — Träger = 6-Min-Screencast (oder 800–1000-Wörter-Text),
  kein Budgetfeld; Pflichtfelder offen; lokal 787 Wörter; Frist 05.10.2026.
- **Blockade:** Trägerformat fehlt; Pflicht-Auswahlfelder unbeantwortet.
- **Braucht:** Screencast-Skript/-Video + gehosteter Link (oder schriftliche Variante auf
  800–1000 Wörter + Begründung); Pflichtfelder in der Vorlage; dann Operator-Wort.

### Förderung Person/Ideen/Projekte — Emergent Ventures
- **Status:** wartend (Session-Arbeit; nur der Einreichungs-Akt `operator-gebunden`)
  | **Bindung:** `eigen` bis zur Ausführungsgrenze, der Akt `dritter`
- **Lage:** Entwurf gegengelesen, repo-wahr, einreichungsfertig; Platzhalter offen
  (Vollzeit-Dauer, Budget-Ballpark, private Formularfelder).
- **Blockade:** keine.
- **Braucht:** Operator-Wort „EV einreichen" auf
  `state/mail/emergent-ventures-application.md`.

### NLnet Restack / CodeSupply
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** 5.000–50.000 €, Einzelperson, DE erfüllt EU-Dimension; Deadline
  **03.11.2026**; Bedingung FOSS-Lizenz „in its entirety".
- **Blockade:** NC-Kern erfüllt die OSI-Bedingung nicht.
- **Braucht:** Operator-Entscheid — NC behalten/NLnet descopen oder Dual-Lizenz (FOSS +
  NC) als eigenes Atom.

### Riss 4 — WGSL-KSG-Spiegel off-path
- **Status:** operator-gebunden | **Bindung:** operator (Bau an `linie:bau`)
- **Lage:** Rat empfiehlt **bauen**; Kategorie „Riss" verworfen (zwei Schätzer ≠
  Mess-Riss). CPU-KSG (`src/mathematikerin/te.rs`) vs. GPU-KDE
  (`src/mathematikerin/shaders.rs`); Paritätstest `tests.rs:345` prüft gegen den falschen
  Zwilling.
- **Blockade:** Operator-Entscheid bauen vs. descopen.
- **Braucht:** Wort „bauen" → Bau-Atom an bau (WGSL k-NN + Silverman, f32/f64-Toleranz);
  bei „nein" → `descoped mit Befund`.

### Free-Model-Bench (105 Modelle)
- **Status:** wartend | **Bindung:** eigen (Lauf) / operator (Cloudflare-Token)
- **Lage:** Bench repariert (Parallelisierung + Antwort-Mitschnitt, `cargo check` 0/0);
  voller T1–T7-Lauf `35591218967` @`eb17b7ae` **in_progress** (kein Poll — Ergebnis
  einmal via `ci_manage view`/Artefakt).
- **Blockade:** Cloudflare-Zweig tot (alle 3 Tokens ohne Workers-AI-Permission) → 11
  Modelle fallen aus; google `gemini-2.5-pro`/`gemini-2.5-flash-lite` stale.
- **Braucht:** (a) Operator rotiert einen Cloudflare-Token mit „Workers AI"-Permission
  in `FREE_MODEL_KEYS`; (b) nach Run-Abschluss Artefakt einmal lesen; (c) google-IDs im
  TSV nachziehen.

### PINE64 / Mantis-Shrimp
- **Status:** blockiert | **Bindung:** `linie:bau`
- **Lage:** Ox64 zugesagt; Presence-Hardware ungebaut.
- **Blockade:** Hardware fehlt.
- **Braucht:** bau baut den Mantis-Shrimp (Spec `docs/specs/mantis-shrimp-bom.md`).

### Mantis-Shrimp-Bewerbungen
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** kein Programm nimmt ein ungebautes Gerät an.
- **Blockade:** Prototyp fehlt.
- **Braucht:** Operator-Entscheid bauen/descopen.

### SSDC Limadou
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Konto existiert, CAS-Login lädt; Sotgiu-Mail (2026-09-16): CSES-02-Umstellung,
  „wait a few weeks".
- **Blockade:** PI-Freigabe/Prozedur-Update ausstehend.
- **Braucht:** Wiedervorlage (Trigger: Prozedur-Update).

### Eigenprize
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Runde geschlossen; `state/mail/eigenprize-application.md`.
- **Blockade:** keine offene Runde.
- **Braucht:** „Remind me" auf `https://eigen.build`.

### Solitude
- **Status:** termin | **Bindung:** `termin:~Herbst 2027`
- **Lage:** `state/mail/solitude-application.md` liegt.
- **Blockade:** Termin fern.
- **Braucht:** Entwurf tragen.

### ISH Chat (GitHub-Dritt-OAuth-App)
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** App hat Kontozugriff (`read:user`/`user:email`).
- **Blockade:** offener Zugriff.
- **Braucht:** Operator-Wort — App widerrufen?

### SuperDARN
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** `phi/blocked_sources.φ:21` (`blocked account`); PI-Agreement gemessen —
  **Daten offen ohne PI**, Zugang = kostenloses Globus-Konto + Gruppeneinladung,
  Rules of the Road (Anerkennung, PI-Kontakt, 1-Jahr-Embargo, nicht-kommerziell).
- **Blockade:** Globus-Konto (Dritt-Akt) fehlt.
- **Braucht:** Operator-Wort für kostenloses Globus-Konto; danach Formular `/data-access`
  oder Mail an `superdarn@usask.ca` (Route dann an ernte).

### DEMETER ISL (CDPP) — Zugang
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** `phi/blocked_sources.φ:70` (`pending`); Re-Messung 2026-09-21 (Post
  forschung-folge134): `regards.cnes.fr/api/v1/rs-order` GET+POST 403 ohne Token
  (Jetty), CDPP verlangt valide E-Mail, ISL-Route account-gated; SPDF/CDAWeb demeter 404.
- **Blockade:** CDPP/REGARDS-Konto fehlt.
- **Braucht:** Operator-Wort für CDPP-Konto; Session bereitet den Mail-Entwurf vor +
  `smail --dry-run`, dann Einreichung.

### solar-system-open-data REST
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** `phi/blocked_sources.φ:60` (`blocked key`); HTTP 401.
- **Blockade:** kein Token.
- **Braucht:** Operator-Wort Konto/Token.

### Amentum Developer
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** `phi/blocked_sources.φ:65` (`blocked account`); Reg. 200.
- **Blockade:** keine Registrierung.
- **Braucht:** Operator-Wort `developer.amentum.io/register`.

### Split-Routing-Verifikation
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** 8 `000`-Hosts ungemessen.
- **Blockade:** sudo + Netz.
- **Braucht:** Wort/Route `./bin/proton-exit.sh ca`.

### Cookie-Transfer
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Auslöser „Bedarf".
- **Blockade:** kein Bedarf.
- **Braucht:** nichts — wartend.

## Operator-Queue (Stand folge79; einfache Sprache, Lage/Blockade/Braucht, mit Alter)

1. **EV** — Entwurf fertig. **Braucht:** Wort „EV einreichen". (seit 2026-09-21)
2. **SSI** — Formular gemessen: die Bewerbung ist ein 6-Minuten-Video (oder
   800–1000-Wörter-Text), kein Budgetfeld; Frist 05.10.2026. **Braucht:** Skript/Video +
   Pflichtfelder erstellen, dann Wort. (folge79 geschärft)
3. **Riss 4** — Rat empfiehlt bauen. **Braucht:** Wort bauen/descopen. (seit 2026-09-21)
4. **NLnet** — **Braucht:** Lizenz-Entscheid. (seit 2026-09-21)
5. **Mantis-Shrimp** — bauen oder descopen. (seit 2026-09-16)
6. **SSDC** — warten. (seit 2026-09-16)
7. **ISH Chat** — App widerrufen? (seit 2026-09-20)
8. **SuperDARN** — Daten sind offen, kein PI nötig; nur ein kostenloses Globus-Konto.
   **Braucht:** Wort Konto anlegen. (folge79 geschärft)
9. **solar-system-open-data** — Konto/Token? (seit 2026-09-20)
10. **Amentum** — registrieren? (seit 2026-09-20)
11. **Split-Routing** — Route `./bin/proton-exit.sh ca`? (seit Ernte folge12–17)
12. **Eigenprize/Solitude** — „Remind me". (seit 2026-09-20)
13. **Cookie-Transfer** — nichts. (seit 2026-09-16)
14. **Free-Model-Bench** — Cloudflare-Token mit „Workers AI"-Permission rotieren?
    (seit 2026-09-21)
15. **DEMETER/CDPP** — Zugang nur mit Konto (403 ohne Token). **Braucht:** Wort Konto
    anlegen. (neu folge79, Post forschung134 gefaltet)

## Benchmark

- **Delegation (Folge 79):** 2 × `grind-flash` (SSI-Formular, SuperDARN-Agreement) —
  Routine-Web-Extraktion, flash-first. Klasse „Routine-Web-Extraktion" hat einen
  registrierten Sieger (flash, 2026-09-16) — kein Doppel. Beide lieferten vollständig
  (SSI: 10 Formularseiten + Lückenliste; SuperDARN: Policy + Globus-Route).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-entscheid-folge79.md` (neu)
- Move `handover-2026-09-21-entscheid-folge78.md` → `archiv/` (eigene Linie, atomar)
- **nicht** angefasst: fremde `docs/zustand/external-state.md`,
  `docs/reference/KERNEL_INDEX.md`, forschung-handover, `post.md`, `tools/*`,
  `.github/*`, `phi/*`, `src/*`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
