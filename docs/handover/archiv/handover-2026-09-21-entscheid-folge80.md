<!--
  title: Handover — Entscheid-Folge 80 (DEMETER/CDPP-Entwurf + Dry-Run; SuperDARN-Mail-Korrektur) (Stand 2026-09-21)
  session: Entscheid-Folge 80
  class: handover
  date: 2026-09-21
  sha256: 0d092ac8de63df026f1ee958417556018b756cc3dc9094299988967f0d1b2c7a
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

### PINE64 / Mantis-Shrimp
- **Status:** blockiert | **Bindung:** `linie:bau`
- **Lage:** Ox64 zugesagt (PINE64 info@), Versanddaten erbeten; Presence-Hardware ungebaut.
- **Blockade:** Hardware fehlt.
- **Braucht:** bau baut den Mantis-Shrimp (Spec `docs/specs/mantis-shrimp-bom.md`).

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

### Cookie-Transfer
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Auslöser „Bedarf".
- **Blockade:** kein Bedarf.
- **Braucht:** nichts — wartend.

## Operator-Queue (Stand folge80; einfache Sprache, Lage/Blockade/Braucht, mit Alter)

3. **DEMETER/CDPP** — Entwurf + Dry-Run fertig. **Braucht:** Wort „Mail senden" oder
   „selbst registrieren". (neu folge80)
4. **Riss 4** — Rat empfiehlt bauen. **Braucht:** Wort bauen/descopen. (seit 2026-09-21)
6. **Mantis-Shrimp** — bauen oder descopen. (seit 2026-09-16)
7. **ISH Chat** — App widerrufen? (seit 2026-09-20)
8. **solar-system-open-data** — Konto/Token? (seit 2026-09-20)
9. **Amentum** — registrieren? (seit 2026-09-20)
10. **Split-Routing** — Route `./bin/proton-exit.sh ca`? (seit Ernte folge12–17)
12. **Free-Model-Bench** — Cloudflare-Token mit „Workers AI"-Permission rotieren?
    (seit 2026-09-21)
13. **SSDC** — warten. (seit 2026-09-16)
14. **SuperDARN** — nur noch Gruppeneinladung annehmen. (folge80 korrigiert)
15. **Cookie-Transfer** — nichts. (seit 2026-09-16)

## Benchmark

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
