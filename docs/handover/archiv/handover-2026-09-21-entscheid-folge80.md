<!--
  title: Handover — Entscheid-Folge 80 (SSI-Formular vollständig vorbereitet: alle 35 Felder + 6-Min-Screencast-Skript + schriftliche 800–1000-Wörter-Variante; DEMETER/CDPP-Entwurf + Dry-Run; SuperDARN-Mail-Korrektur) (Stand 2026-09-21)
  session: Entscheid-Folge 80
  class: handover
  date: 2026-09-21
  sha256: 8ecb78eef8b2ff0135b5d8188c6d8cd91e996e8038b79d3e12024c3675f2596a
  status: live
-->
# Handover — Entscheid-Folge 80 (2026-09-21)

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

## Stehender Pass (gemessen 2026-09-21, Entscheid-Folge 80)

- **HEAD** `0c0b30fb` bei Session-Beginn; während des Atoms rückten ernte folge132 und
  forschung folge134 nach → HEAD jetzt `6d256545` == `origin/main`. Der Baum war bei
  Beginn **nicht clean** (fremd: `.github/workflows/*`, forschung folge133/134,
  `KERNEL_INDEX.md`, `external-state.md`, `post.md`, `phi/*`); diese Arbeit wurde von
  ihren Linien committet. `post.md` ist jetzt clean — die DEMETER-Zeile gefaltet +
  gelöscht, die Mantis-Shrimp-Zeile an bau gesetzt (eigene Hunks).
- **CI** — `free-model-bench` `35591218967` @`eb17b7ae` **in_progress** (`updated_at`
  10:54:33, kein Poll); `ci-check` `35592556416` pending; viele `*-cdn` failed/queued
  = ernte-Domäne.
- **Postfach** — kein neuer Ledger-Eingang; letzter `1789978555` (Brave-Limit).
- **`register_lookup --open`** — 613 offen, 14 `zustand` due, 2 Post offen;
  entscheid-Registerpunkte `blocked_sources.φ:21/60/65/70`.
- **`git_safety --snapshot`** — `refs/safety/1789988960`.

## Messung dieses Atoms (kein offener Punkt)

### SSI Live-Formular (`general`)
- Alle **10 Seiten** live abgeschritten; Pflichtfelder **Q1–Q35** vollständig erfasst
  (die frühere Liste war unvollständig — auch Q1–Q6, Q15, Q17, Q19–Q22, Q25, Q28–Q30,
  Q35 sind Pflicht). Q7/Q13/Q14/Q31–Q33 optional.
- **Träger:** 6-Min-Screencast (max. 6:00 + 15 s; Voiceover; 1/1/4-Struktur);
  schriftliche Alternative 800–1000 Wörter + 1–2-Satz-Begründung. **Gehosteter Link
  Pflicht** (Google Drive/Dropbox, **nicht YouTube**, Download bis Ende Dez 2026),
  `.mp4`, **kein Upload, keine Anhänge**. Kein Budgetfeld.
- **Frist** 23:59 GMT+1, **05.10.2026** (Live-Formular S.1 + Apply-Seite); die
  „7 October"-Angabe auf S.10 ist stale (`unverified` welcher Wert gilt).
- Formular `https://forms.cloud.microsoft/e/pJdGh0rRSx` (HTTP 200); Preview-PDF +
  Video-Guide HTTP 200.

### DEMETER/CDPP Route (`general`)
- **Selbst-Registrierung existiert** (SPA-Modal, kein `/register`): `cdpp-archive.cnes.fr/user/cdpp/modules/1778`
  → Login → „New user?" → *Request project access*; Pflichtfelder E-Mail, Passwort,
  Vorname, Nachname, Country, Organization, Registration reason. **Mailweg**
  `cdpp@cnes.fr` (cc `support-Regards@cnes.fr`). Konto → `waiting-accounts` →
  Admin-Akzeptanz.
- **Korrektur:** der 403 ohne Token war der fehlende Header `scope: cdpp` — Metadaten
  sind ohne Konto lesbar (200); nur das **Ordern** braucht Auth. Produkte:
  `DMT_N1_1143` (ISL Burst, 39 318 Dateien) / `DMT_N1_1144` (ISL Survey, 57 760).

### SuperDARN — Korrektur der folge79-Lage
- Die **Mail ist bereits gesendet** (Operator, 2026-09-18, `state/mail/auftrag-superdarn-globus-zugang.md`
  §Ausgang); Globus-ID + Connect-Personal-Endpoint stehen. Der **Formweg ist blockiert**
  (reCAPTCHA „Ungültige Domain für Websiteschlüssel"). Offen ist nur die
  **Gruppeneinladung** im Operator-Postfach. folge79 führte „Operator-Wort Globus-Konto"
  — der ist erledigt; die Zeile war stale.

## Offen (aufgeschlüsselt)

### SSI Fellowship (software.ac.uk)
- **Status:** operator-gebunden | **Bindung:** operator (Video/Submit) / dritter (Hosting)
- **Lage:** Entwurf **vollständig vorbereitet** — `state/mail/ssi-fellowship-application.md`:
  alle 35 Felder beantwortet (offene Operator-Felder Q5/Q6 PII, Q10/Q16/Q18/Q23/Q24/Q32/Q34),
  **6-Min-Screencast-Skript** (1/1/4) + schriftliche 800–1000-Wörter-Variante + Begründung;
  Frist 05.10.2026.
- **Blockade:** Screencast aufnehmen + hosten (Q25-Link); PII-/Zuordnungsfelder.
- **Braucht:** Operator nimmt das Skript als Screencast auf, hostet bei Google
  Drive/Dropbox, füllt Q5/Q6/Q10/Q16/Q18/Q23/Q24/Q32/Q34, trägt Q25-Link ein; dann
  Wort „SSI einreichen".

### Förderung Person/Ideen/Projekte — Emergent Ventures
- **Status:** wartend (Session-Arbeit; nur der Einreichungs-Akt `operator-gebunden`)
  | **Bindung:** `eigen` bis zur Ausführungsgrenze, der Akt `dritter`
- **Lage:** Entwurf `state/mail/emergent-ventures-application.md` gegengelesen,
  repo-wahr, einreichungsfertig; Platzhalter offen (Vollzeit-Dauer, Budget-Ballpark,
  private Formularfelder).
- **Blockade:** keine.
- **Braucht:** Operator-Wort „EV einreichen".

### DEMETER ISL (CDPP) — Zugang
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** Entwurf **fertig** — `state/mail/cdpp-demeter-isl-zugang.md` +
  `…body.txt`, `smail --dry-run` verifiziert (1338 B, an `cdpp@cnes.fr`); Route gemessen
  (Selbst-Registrierung oder Mail; Produkte `DMT_N1_1143/1144`).
- **Blockade:** Dritt-Akt (Konto/Mail) fehlt.
- **Braucht:** Operator-Wort — Mail senden (`smail --send`) **oder** selbst registrieren.

### Free-Model-Bench (105 Modelle)
- **Status:** wartend | **Bindung:** eigen (Lauf) / operator (Cloudflare-Token)
- **Lage:** voller T1–T7-Lauf `35591218967` @`eb17b7ae` **in_progress** (`updated_at`
  10:54:33, kein Poll). TSV `tools/measure/free_models.tsv` trägt **12 `google`-Zeilen**
  (gemessen `awk -F'\t' '$1=="google"'` = 12); die zwei stale IDs `gemini-2.5-pro` /
  `gemini-2.5-flash-lite` (folge78: HTTP 404) stehen weiter drin — lokal kein
  `GOOGLE_API_KEY` (`auth.json` hat keinen `google`-Eintrag), Live-Verifikation nur
  über den CI-Secret.
- **Blockade:** (a) Cloudflare-Zweig tot (Tokens ohne Workers-AI-Permission);
  (b) Run-Ergebnis noch nicht da.
- **Braucht:** (a) Operator rotiert Cloudflare-Token mit „Workers AI"-Permission;
  (b) nach Run-Abschluss Artefakt einmal lesen (`ci_manage view 35591218967` /
  `gh run download`), die 404-google-IDs anhand des Artefakts nachziehen.

### Riss 4 — WGSL-KSG-Spiegel off-path
- **Status:** operator-gebunden | **Bindung:** operator (Bau an `linie:bau`)
- **Lage:** Rat empfiehlt **bauen**; Kategorie „Riss" verworfen. CPU-KSG
  (`src/mathematikerin/te.rs`) vs. GPU-KDE (`src/mathematikerin/shaders.rs`).
- **Blockade:** Operator-Entscheid bauen vs. descopen.
- **Braucht:** Wort „bauen" → Bau-Atom an bau; bei „nein" → `descoped mit Befund`.

### NLnet Restack / CodeSupply
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** 5.000–50.000 €, Einzelperson, DE erfüllt EU-Dimension; Deadline 03.11.2026;
  Bedingung FOSS-Lizenz „in its entirety"; NC-Kern erfüllt die OSI-Bedingung nicht.
- **Blockade:** Lizenz-Entscheid.
- **Braucht:** Operator-Wort — NC behalten/NLnet descopen oder Dual-Lizenz.

### PINE64 / Mantis-Shrimp
- **Status:** blockiert | **Bindung:** `linie:bau`
- **Lage:** Ox64 zugesagt (PINE64 info@), Versanddaten erbeten; Presence-Hardware ungebaut.
- **Blockade:** Hardware fehlt.
- **Braucht:** bau baut den Mantis-Shrimp (Spec `docs/specs/mantis-shrimp-bom.md`).

### Mantis-Shrimp-Bewerbungen
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** kein Programm nimmt ein ungebautes Gerät an.
- **Blockade:** Prototyp fehlt.
- **Braucht:** Operator-Entscheid bauen/descopen.

### SuperDARN
- **Status:** wartend | **Bindung:** dritter (Globus-Gruppe)
- **Lage:** Mail gesendet 2026-09-18; Formweg blockiert (reCAPTCHA-Fehlkonfiguration);
  offen ist die **Gruppeneinladung** im Operator-Postfach.
- **Blockade:** Einladung ausstehend.
- **Braucht:** Wiedervorlage (Trigger: Mail-Ledger-Eingang `superdarn`/Globus).

### ISH Chat (GitHub-Dritt-OAuth-App)
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** App hat Kontozugriff (`read:user`/`user:email`).
- **Blockade:** offener Zugriff.
- **Braucht:** Operator-Wort — App widerrufen?

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

### SSDC Limadou
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Konto existiert, CAS-Login lädt; Sotgiu (2026-09-16): CSES-02-Umstellung,
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

### Cookie-Transfer
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Auslöser „Bedarf".
- **Blockade:** kein Bedarf.
- **Braucht:** nichts — wartend.

### text_review — Lauf auf den Förder-Entwürfen
- **Status:** blockiert | **Bindung:** `linie:bau`
- **Lage:** Bin gebaut + committet (`197bccf1`); `tools-build.yml` trägt keinen
  measure-Bin; Post steht (`post.md`, `An bau: text_review`).
- **Blockade:** measure-Bin-Release-Pfad fehlt.
- **Braucht:** bau trägt `text_review` in `tools-build.yml` ein.

## Operator-Queue (Stand folge80; einfache Sprache, Lage/Blockade/Braucht, mit Alter)

1. **SSI** — Entwurf fertig (alle Felder + Screencast-Skript). **Braucht:** Video
   aufnehmen + hosten, PII-Felder ausfüllen, dann Wort „einreichen". Frist 05.10.2026.
   (folge80 vollständig vorbereitet)
2. **EV** — Entwurf einreichungsfertig. **Braucht:** Wort „EV einreichen". (seit 2026-09-21)
3. **DEMETER/CDPP** — Entwurf + Dry-Run fertig. **Braucht:** Wort „Mail senden" oder
   „selbst registrieren". (neu folge80)
4. **Riss 4** — Rat empfiehlt bauen. **Braucht:** Wort bauen/descopen. (seit 2026-09-21)
5. **NLnet** — **Braucht:** Lizenz-Entscheid. (seit 2026-09-21)
6. **Mantis-Shrimp** — bauen oder descopen. (seit 2026-09-16)
7. **ISH Chat** — App widerrufen? (seit 2026-09-20)
8. **solar-system-open-data** — Konto/Token? (seit 2026-09-20)
9. **Amentum** — registrieren? (seit 2026-09-20)
10. **Split-Routing** — Route `./bin/proton-exit.sh ca`? (seit Ernte folge12–17)
11. **Eigenprize/Solitude** — „Remind me". (seit 2026-09-20)
12. **Free-Model-Bench** — Cloudflare-Token mit „Workers AI"-Permission rotieren?
    (seit 2026-09-21)
13. **SSDC** — warten. (seit 2026-09-16)
14. **SuperDARN** — nur noch Gruppeneinladung annehmen. (folge80 korrigiert)
15. **Cookie-Transfer** — nichts. (seit 2026-09-16)

## Benchmark

- **Delegation (Folge 80):** 2 × `general` (SSI-Live-Formular, DEMETER/CDPP-Route) +
  1 × `general` (Google-Free-Model-IDs) — Routine-Recherche/Extraktion, flash-first.
  Die 2 Routinen lieferten vollständig (SSI: 10 Seiten + alle Pflichtfelder + Träger;
  DEMETER: Selbst-Registrierung + `scope: cdpp`-Korrektur). Die Google-Probe lieferte
  eine Prämissen-Korrektur (TSV trägt 12 `google`-Zeilen; mein `sgrep "google\|gemini"`
  war ein Literal-Pipe-Fehler). Klasse „Routine-Recherche" hat einen registrierten
  Sieger (flash) — kein Doppel nötig.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-entscheid-folge80.md` (neu)
- Move `handover-2026-09-21-entscheid-folge79.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` (eigene Hunks: DEMETER-Zeile gefaltet+gelöscht,
  Mantis-Shrimp-Zeile an bau; Header-sha neu)
- `state/mail/ssi-fellowship-application.md` (überarbeitet; gitignored)
- `state/mail/cdpp-demeter-isl-zugang.md` + `.body.txt` (neu; gitignored)
- **nicht** angefasst: fremde `.github/workflows/*`, `docs/zustand/external-state.md`,
  `docs/reference/KERNEL_INDEX.md`, forschung-handover, `phi/*`, `src/*`, `tools/*`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
