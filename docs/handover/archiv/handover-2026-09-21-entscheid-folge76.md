<!--
  title: Handover — Entscheid-Folge 76 (open_points_check-Fix im Release) (Stand 2026-09-21)
  session: Entscheid-Folge 76
  class: handover
  date: 2026-09-21
  sha256: 46970a87daa4d56c5b9bf8825d3b22f452c16ff2640e6c9f533541678b6f70d8
  status: live
-->
# Handover — Entscheid-Folge 76 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder offene Punkt wird **aufgeschlüsselt** geführt — kein
Register-Kürzel: **Lage** / **Blockade** / **Braucht**. **`operator-gebunden`
benennt den Ausführungsakt, nie die Aufgabe**: die Aufgabe bleibt Session-Arbeit
bis zur Ausführungsgrenze (Operator-Wort 2026-09-21). Jeder Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Entscheid-Folge 76)

- **HEAD** `ad06e1c4` (entscheid folge75) — `origin/main` == HEAD. Arbeitsbaum
  trägt fremde, uncommittete Arbeit (`.github/workflows/ci-check.yml`,
  `hyperscanning-te.yml`, `post.md`-Löschungen zweier `An bau`-Zeilen,
  `phi/sources.φ`, `src/archivar/{extract,geo,main_flow,tests}.rs`,
  `src/mathematikerin/te.rs`, `tools/measure/hyperscanning_group_te.rs`,
  `tools/register/register_lookup.rs`) — **nicht eigene**; nicht angefasst.
- **Postfach** — `mail_digest` + `state/mail/mail_ledger.φ`: kein neuer Eingang;
  `external-state.md` Postfach-Zeile nachgezogen.
- **CI** — `ci_manage view` 2026-09-21: `tools-build` `35574750913` **success**
  @HEAD `ad06e1c4` (07:52Z); `free-model-bench` `35567266519` +
  `free-model-agent-bench` `35567268692` weiter **in_progress** (`updated_at`
  06:09Z eingefroren, kein Job-Log). Watchdog-Snapshot 09:07:02: `ps1-cdn`
  `35569486280`, `ci-check` `35569256029`, `te-gate` `35567711055`, `demeter-cdn`
  `35567568429` (queued), `health-check` `35556807317` in_progress; failed:
  `hyperscanning-te` `35570480672`/`35567708611`, `rpw-cdn` `35569266300`,
  `ci-check` `35566258372`. Kein Poll. `external-state.md` CI-Zeile auf HEAD
  `ad06e1c4` nachgezogen.
- **`register_lookup --open`** — 3 `[entscheid]`-Registerpunkte
  `phi/blocked_sources.φ:21/60/65` (SuperDARN / solar-system-open-data / Amentum)
  → Operator-Queue; 1 Post-Zeile (`post.md:22`, SSDC).
- **`git_safety --snapshot`** — `refs/safety/1789976968`.

## Messung dieses Atoms (kein offener Punkt)

- **`open_points_check`-Fix im Release**: `tools-build` `35574750913` success
  @HEAD `ad06e1c4`; `bin/.tools_ensure open_points_check` + Lauf → `post.md 3 path
  refs | 0 absent`. Die früheren falschen `ABSENT` sind weg.
- **SSDC-Zustand korrigiert**: das Konto `omegaflow` existiert (CAS-Login lädt),
  es fehlt die **PI-Freigabe**; der PI hat am 2026-09-16 um Wartezeit gebeten, wir
  haben zugewartet — kein Follow-up fällig. Post-Zeile `post.md:22` eingefaltet
  und gelöscht.

## Offen (aufgeschlüsselt)

### Free-Model-Bench (P13 + P2–P4) — die 105 Modelle
- **Status:** `wartend` | **Bindung:** `eigen`
- **Lage:** Input = `tools/measure/free_models.tsv` (105 Zeilen, 10 Provider).
  Läufe `35567266519` (free-model-bench) + `35567268692`
  (free-model-agent-bench) seit 06:09Z `in_progress`, `updated_at` 06:09:56
  eingefroren, **kein Job-Log** — Runner-Queue. Kein Artefakt. Keine Median-Basis.
- **Blockade:** Trigger „Run-Abschluss" nicht gefeuert; Laufzeit bis 360 min.
- **Braucht:** Run-Abschluss abwarten, dann Artefakte **einmal** lesen
  (`ci_manage view 35567266519`/`35567268692`; `gh run download <id> -n
  free-model-bench`) und das Ranking oder `pending` als Zeile tragen. Kein Poll.

### PINE64-Dokumentationspflicht (Ox64 / Mantis-Shrimp)
- **Status:** `blockiert` | **Bindung:** `linie:bau`
- **Lage:** Ox64 von Pine64 zugesagt (Hardware beidseitig geschlossen); die
  Presence-Hardware „Mantis-Shrimp" ist ungebaut. Post `An bau:` in `post.md`.
- **Blockade:** Hardware existiert nicht; Bau gehört zur **bau-Linie**.
- **Braucht:** bau baut minimalen Mantis-Shrimp (Spec
  `docs/specs/mantis-shrimp-bom.md` + BOM liegen) und dokumentiert danach den Ox64.

### SSDC Limadou
- **Status:** `wartend` | **Bindung:** `eigen`
- **Lage:** Konto `omegaflow` existiert, CAS-Login lädt (`tools.ssdc.asi.it/cas/
  login` → `query.php` 200), Zugangsdaten `SSDC_USER`/`SSDC_PASS` in
  `.secrets.local`; „Permission Denied" = fehlende **PI-Freigabe**. PI Sotgiu
  (`alessandro.sotgiu@roma2.infn.it`) bat am 2026-09-16 um Wartezeit
  (CSES-02-Umbau), wir haben zugewartet. `mail_ledger.φ` (`1789906306`);
  `phi/pipeline/ledger.φ:26–28`.
- **Blockade:** PI-Freigabe ausstehend; **wir warten** — kein Follow-up fällig.
- **Braucht:** Wiedervorlage — die neue SSDC-Prozedur einmal messen (Trigger:
  Prozedur/Website aktualisiert). Erst wenn der Zugang dann noch verweigert ist,
  wäre ein Follow-up eine Frage (per-Akt-Consent).

### ISH Chat (GitHub-Dritt-OAuth-App)
- **Status:** `operator-gebunden` | **Bindung:** `operator`
- **Lage:** Dritt-App „ISH Chat", Scopes `read:user`/`user:email`, Kontozugriff;
  Mail `1789918147`.
- **Blockade:** offener Zugriff.
- **Braucht:** Operator-Wort — App widerrufen unter
  `github.com/settings/connections/applications`?

### SuperDARN
- **Status:** `operator-gebunden` | **Bindung:** `dritter`
- **Lage:** `phi/blocked_sources.φ:21` (`blocked account`); `superdarn.ca` +
  `/data-access` 200; Datenroute über Globus + PI-Vereinbarung; `api.superdarn.ca`
  DNS-tot.
- **Blockade:** kein Konto, keine PI-Vereinbarung.
- **Braucht:** Operator-Wort, Konto + PI-Vereinbarung einzugehen; Session liest
  vorab die PI-Vereinbarung und entwirft die Anfrage.

### solar-system-open-data REST
- **Status:** `operator-gebunden` | **Bindung:** `dritter`
- **Lage:** `phi/blocked_sources.φ:60` (`blocked key`); HTTP 401 (Bearer),
  2026-09-20.
- **Blockade:** kein Token.
- **Braucht:** Operator-Wort, Konto/Token anzulegen.

### Amentum Developer
- **Status:** `operator-gebunden` | **Bindung:** `dritter`
- **Lage:** `phi/blocked_sources.φ:65` (`blocked account`); geomagnetisch/
  aviation-radiation/gravity (trial); Registrierungsseite HTTP 200.
- **Blockade:** keine Registrierung.
- **Braucht:** Operator-Wort `developer.amentum.io/register`.

### Split-Routing-Verifikation
- **Status:** `operator-gebunden` | **Bindung:** `operator`
- **Lage:** 8 `000`-Hosts ungemessen.
- **Blockade:** braucht sudo + Netz auf dem Operator-Rechner.
- **Braucht:** Operator-Wort/Route für `./bin/proton-exit.sh ca` +
  direct↔tunnel-Nachmessung.

### Cookie-Transfer
- **Status:** `wartend` | **Bindung:** `eigen`
- **Lage:** Auslöser „Bedarf".
- **Blockade:** kein Bedarf.
- **Braucht:** nichts — wartend.

## Operator-Queue (Stand folge76; einfache Sprache, je Eintrag Lage/Blockade/Braucht, mit Alter)

3. **SSDC Limadou** — **Lage:** Konto lädt, PI-Freigabe fehlt; PI bat um Wartezeit.
   **Blockade:** wir warten (kein Follow-up fällig). **Braucht:** nichts — Wiedervorlage.
   (Alter: seit 2026-09-16)
4. **ISH Chat (GitHub-OAuth-App)** — **Lage:** App hat Kontozugriff. **Blockade:**
   offener Zugriff. **Braucht:** App widerrufen? (Alter: seit 2026-09-20)
5. **SuperDARN** — **Lage:** Route über Globus + PI-Vereinbarung. **Blockade:** kein
   Konto. **Braucht:** Konto + PI-Vereinbarung? (Alter: seit 2026-09-16)
6. **solar-system-open-data REST** — **Lage:** HTTP 401. **Blockade:** kein Token.
   **Braucht:** Konto/Token anlegen? (Alter: seit 2026-09-20)
7. **Amentum Developer** — **Lage:** Registrierungsseite 200. **Blockade:** keine
   Registrierung. **Braucht:** `developer.amentum.io/register`? (Alter: seit 2026-09-20)
8. **Split-Routing-Verifikation** — **Lage:** 8 `000`-Hosts ungemessen. **Blockade:**
   sudo + Netz. **Braucht:** Wort/Route für `./bin/proton-exit.sh ca`. (Alter: seit
   Ernte folge12–17)
10. **Cookie-Transfer** — **Lage:** `wartend`, Auslöser „Bedarf". **Blockade:** kein
    Bedarf. **Braucht:** nichts. (Alter: seit 2026-09-16)

## Benchmark

- **Delegation (Entscheid-Folge 76):** 2 × `general` (flash) für die zwei
  „Routine-Route-Messung" ist als gemessen geführt (flash gewinnt, 2026-09-16) —

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-entscheid-folge76.md` (neu)
- Move `handover-2026-09-21-entscheid-folge75.md` → `archiv/` (eigene Linie, atomar)
- `docs/zustand/external-state.md` (Postfach- + CI-Zeile)
- `docs/handover/post.md` (nur eigener Hunk: SSDC-Zeile `post.md:22` gelöscht)
- `state/mail/emergent-ventures-application.md` (Formularfragen + Antwortentwurf,
  gitignored)
- **nicht** angefasst: fremde `phi/*`, `src/archivar/*`, `src/mathematikerin/te.rs`,
  `tools/measure/hyperscanning_group_te.rs`, `tools/register/register_lookup.rs`,
  `.github/workflows/*`, die fremden `post.md`-Löschungen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Nach dem Push die
geänderten Workflows dispatchten (kein Poll). `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
