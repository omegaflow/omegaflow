<!--
  title: Handover — Entscheid-Folge 81 (Stehender Pass; Free-Model-Bench wartet auf Lauf-Abschluss; SuperDARN-Trigger nicht gefeuert; text_review an bau übertragen) (Stand 2026-09-21)
  session: Entscheid-Folge 81
  class: handover
  date: 2026-09-21
  sha256: 523ad7f414ba2cf2f1d13e9285b168c44eec649378488dfa775200c5886242be
  status: live
-->
# Handover — Entscheid-Folge 81 (2026-09-21)

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

## Stehender Pass (gemessen 2026-09-21, Entscheid-Folge 81)

- **HEAD** `44e77da0` == `origin/main`. Der Baum trägt eine **fremde** Änderung
  `.github/workflows/tools-build.yml` (bau-Domäne — der text_review-Release-Pfad
  entsteht dort; nicht angefasst).
- **CI** (Watchdog-Snapshot 13:23:20): aktiv `ci-check` `35592556416`,
  `kernel-flatten` `35591255532`, `free-model-bench` `35591218967` (in_progress,
  `updated_at` 10:54:33, kein Poll), `signal-cone-audit-cdn` `35587808407`; viele
  `*-cdn` failed/queued = **ernte-Domäne**.
- **Postfach** — kein neuer, entscheid-relevanter Ledger-Eingang; letzter gesehener
  Eintrag Brave-Limit `1789930248` (folge80 nannte `1789978555`). Kein
  SuperDARN-/Globus-Eingang (`sgrep superdarn|globus state/mail/mail_ledger.φ` →
  nur der Ausgang `1789718159`).
- **`register_lookup --open`** — 605 offen, 14 `zustand` due, 1 Post offen;
  entscheid-Registerpunkte `blocked_sources.φ:21/60/65`.
- **`git_safety --snapshot`** — `refs/safety/1789990731`.

## Messung dieses Atoms (kein offener Punkt)

- **Free-Model-Bench `35591218967`** (`grind-flash`, flash-first): Lauf
  **in_progress**, Artefakt `absent` (`ci_manage view` listet keins; Job-Log
  HTTP 404 während des Laufs). Die zwei google-404-Kandidaten
  (`gemini-2.5-pro`, `gemini-2.5-flash-lite`) bleiben `absent` — keine Messung
  möglich, bis der Lauf endet.
- **Zuständigkeits-Korrektur:** `tools/measure/free_models.tsv` ist **entscheid-eigen**
  (folge64/66/75/76/77) — kein Post an ernte; die ID-Korrektur bleibt bei entscheid.
- **text_review** ist von bau gefaltet (`bau-folge122.md:74` „Post an bau,
  gefaltet") und aus diesem Register entfernt; der Post `post.md:18` trägt nur
  noch `An bau: Mantis-Shrimp`.

## Offen (aufgeschlüsselt)

### DEMETER ISL (CDPP) — Zugang
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** Entwurf fertig — `state/mail/cdpp-demeter-isl-zugang.md` + `…body.txt`,
  `smail --dry-run` verifiziert (1338 B, an `cdpp@cnes.fr`); Route gemessen
  (Selbst-Registrierung oder Mail; Produkte `DMT_N1_1143/1144`).
- **Blockade:** Dritt-Akt (Konto/Mail) fehlt.
- **Braucht:** Operator-Wort — Mail senden (`smail --send`) oder selbst registrieren.

### Free-Model-Bench (105 Modelle)
- **Status:** wartend | **Bindung:** eigen (Lauf) / operator (Cloudflare-Token)
- **Lage:** Lauf `35591218967` @`eb17b7ae` **in_progress**, Artefakt `absent`
  (gemessen 2026-09-21). TSV `tools/measure/free_models.tsv` trägt 12 `google`-Zeilen;
  die zwei stale IDs `gemini-2.5-pro` / `gemini-2.5-flash-lite` (folge78: HTTP 404)
  stehen weiter drin; lokal kein `GOOGLE_API_KEY`.
- **Blockade:** (a) Cloudflare-Zweig tot (Token ohne Workers-AI-Permission);
  (b) Lauf noch nicht beendet.
- **Braucht:** (a) Operator rotiert Cloudflare-Token mit „Workers AI"-Permission;
  (b) **nach Lauf-Abschluss** Artefakt einmal lesen (`ci_manage log 35591218967 --all`,
  sonst `gh run download 35591218967 -n <artifact>`), die 404-IDs nachziehen.

### Riss 4 — WGSL-KSG-Spiegel off-path
- **Status:** operator-gebunden | **Bindung:** operator (Bau an `linie:bau`)
- **Lage:** Rat empfiehlt bauen; CPU-KSG (`src/mathematikerin/te.rs`) vs. GPU-KDE
  (`src/mathematikerin/shaders.rs`).
- **Blockade:** Operator-Entscheid bauen vs. descopen.
- **Braucht:** Wort „bauen" → Bau-Atom an bau; bei „nein" → `descoped mit Befund`.

### PINE64 / Mantis-Shrimp
- **Status:** blockiert | **Bindung:** `linie:bau`
- **Lage:** Ox64 zugesagt (PINE64 info@), Versanddaten erbeten; Presence-Hardware ungebaut.
- **Blockade:** Hardware fehlt.
- **Braucht:** bau baut den Mantis-Shrimp (Spec `docs/specs/mantis-shrimp-bom.md`).

### SuperDARN
- **Status:** wartend | **Bindung:** dritter (Globus-Gruppe)
- **Lage:** Mail gesendet 2026-09-18 (`mail_ledger.φ` `1789718159`); Formweg blockiert
  (reCAPTCHA); **kein** Globus-Gruppen-Eingang im Ledger (gemessen 2026-09-21).
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

## Operator-Queue (Stand folge81; unverändert seit folge80, nicht neu vorgelegt)

Kein neuer Messwert seit folge80 → die Queue wird **nicht** erneut vorgelegt
(Reibungs-Regel). Sie steht wie in folge80, mit Alter:

3. **DEMETER/CDPP** — „Mail senden" oder „selbst registrieren". (seit 2026-09-21)
4. **Riss 4** — „bauen"/„descopen". (seit 2026-09-21)
6. **Mantis-Shrimp** — bauen/descopen. (seit 2026-09-16)
7. **ISH Chat** — App widerrufen? (seit 2026-09-20)
8. **solar-system-open-data** — Konto/Token? (seit 2026-09-20)
9. **Amentum** — registrieren? (seit 2026-09-20)
10. **Split-Routing** — Route `./bin/proton-exit.sh ca`? (seit Ernte folge12–17)
12. **Free-Model-Bench** — Cloudflare-Token mit „Workers AI"-Permission? (seit 2026-09-21)
13. **SSDC** — warten. (seit 2026-09-16)
14. **SuperDARN** — nur noch Gruppeneinladung annehmen. (seit 2026-09-18)
15. **Cookie-Transfer** — nichts. (seit 2026-09-16)

## Benchmark

- **Delegation (Folge 81):** 1 × `grind-flash` (Free-Model-Bench-Artefakt) —
  flash-first, Routine. Ergebnis: Lauf `in_progress`, Artefakt `absent`; kein
  pro/max-Doppel nötig (keine falsche Antwort, nur ein nicht messbarer Zustand).
  Keine neue Klasse — „Routine-Extraktion" hat ihren registrierten Sieger (flash).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-entscheid-folge81.md` (neu)
- Move `handover-2026-09-21-entscheid-folge80.md` → `archiv/` (eigene Linie, atomar)
- **nicht** angefasst: fremde `.github/workflows/tools-build.yml` (bau),
  `post.md`, `phi/*`, `src/*`, `tools/*`, andere Linien-Handover.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
