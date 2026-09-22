<!--
  title: Handover — Entscheid-Folge 82 (Stehender Pass; SuperDARN-Register von blocked auf pending korrigiert — anonyme Routen gemessen; solar-system/Amentum freie Zugänge; Free-Model-Bench wartet) (Stand 2026-09-21)
  session: Entscheid-Folge 82
  class: handover
  date: 2026-09-21
  sha256: 24dd4090b6e3b1a6bb9976bb02222ffa3d750829e128863fbda1beb8b93b6919
  status: live
-->
# Handover — Entscheid-Folge 82 (2026-09-21)

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

## Stehender Pass (gemessen 2026-09-21, Entscheid-Folge 82)

- **HEAD** `6e4326bf` == `origin/main`. Der Baum trägt **fremde** Änderungen:
  `.github/workflows/tools-build.yml` (bau), `src/archivar/geo.rs`,
  `src/archivar/mod.rs`, `tools/harvest/src/bin/glm_l2_compiler.rs`,
  `?? src/archivar/emodnet_hfr.rs` (ernte) — nicht angefasst.
- **CI** (`ci_manage list`, 12:19Z): `ci-check` `35598902913` pending,
  `emodnet-hfr-cdn` `35598877845` success, `tools-build` `35598862427`
  in_progress, `hyperscanning-te` `35598791059` pending / `35596009980`
  in_progress, `te-gate` `35595896140` in_progress; viele `ci-check` cancelled =
  ernte/CI-Domäne. **Free-Model-Bench `35591218967`** weiter in_progress.
- **Postfach** — kein neuer, entscheid-relevanter Ledger-Eingang; letzter
  `1789978555` (Brave-Limit). Kein SuperDARN-/Globus-Eingang.
- **`register_lookup --open`** — 592 offen, 14 `zustand` due, 1 Post offen.
- **`git_safety --snapshot`** — `refs/safety/1789991847`.

## Messung dieses Atoms

- **SuperDARN** (Delegation: 1 × `general`, flash-first): `blocked_sources.φ:21`
  von `blocked account` auf `pending` korrigiert. Anonyme Routen gemessen
  2026-09-21: FITACF-3.0-POST `superdarn.ca/data-download` (`radar=sas&date=…`,
  kein Token) → direkte Datei `sdc-serv.usask.ca/data/YYYY/MM/YYYYMMDD.hhmm.ss.<radar>.<ch>.fitacf.bz2`
  (200, 32859 B, sha256 `48637f08…`, Range 206); Inventar-POST
  `/db-fitacf-files-bounce` (JSON, size30/hash30); FRDR **31 Datasets 1993–2023**
  (DAT+RAWACF, Einzeldateien anonym via 302→globus); Zenodo **822 offene Records**
  (cc-zero/cc-by-4.0, netCDF/FITACF, z. B. `10.5281/zenodo.12996103`). Nur
  full-res wide-beam RAWACF bleibt Globus+PI-gated. Post `An ernte` liegt in
  `post.md`.
- **solar-system-open-data** (`blocked_sources.φ:60`): `blocked key` bleibt,
  Note korrigiert — Key **frei/selbstbedienung** (`generatekey.html`, 200,
  „Enter your email to get your free API Key").
- **Amentum** (`blocked_sources.φ:65`): `blocked account` bleibt, Note korrigiert
  — `register` 200, „14-day free trial", Selbstbedienung.
- **SSDC Limadou** — `ssdc.nssdc.ac.cn` → `net::ERR_NAME_NOT_RESOLVED` (DNS tot);
  der Stand „CAS-Login lädt" ist überholt.
- **Brave** — HTTP 402 (Quota); keyless Ersatz `--mwmbl`.
- **Free-Model-Bench** — Lauf `35591218967` in_progress, Artefakt absent.

## Offen (aufgeschlüsselt)

### DEMETER ISL (CDPP) — Zugang
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** Entwurf fertig — `state/mail/cdpp-demeter-isl-zugang.md` + `…body.txt`, `smail --dry-run` verifiziert (1338 B, an `cdpp@cnes.fr`).
- **Blockade:** Dritt-Akt (Konto/Mail) fehlt.
- **Braucht:** Operator-Wort — Mail senden (`smail --send`) oder selbst registrieren.

### Free-Model-Bench (105 Modelle)
- **Status:** wartend | **Bindung:** eigen (Lauf) / operator (Cloudflare-Token)
- **Lage:** Lauf `35591218967` @`eb17b7ae` in_progress, Artefakt absent (2026-09-21). TSV `tools/measure/free_models.tsv` trägt 12 `google`-Zeilen; zwei stale IDs `gemini-2.5-pro` / `gemini-2.5-flash-lite` (folge78: HTTP 404) stehen weiter drin; lokal kein `GOOGLE_API_KEY`.
- **Blockade:** (a) Cloudflare-Zweig tot (Token ohne Workers-AI-Permission); (b) Lauf noch nicht beendet.
- **Braucht:** (a) Operator rotiert Cloudflare-Token mit „Workers AI"-Permission; (b) nach Lauf-Abschluss Artefakt einmal lesen (`ci_manage log 35591218967 --all`, sonst `gh run download 35591218967 -n <artifact>`), die 404-IDs nachziehen.

### Riss 4 — WGSL-KSG-Spiegel off-path
- **Status:** operator-gebunden | **Bindung:** operator (Bau an `linie:bau`)
- **Lage:** Rat empfiehlt bauen; CPU-KSG (`src/mathematikerin/te.rs`) vs. GPU-KDE (`src/mathematikerin/shaders.rs`).
- **Blockade:** Operator-Entscheid bauen vs. descopen.
- **Braucht:** Wort „bauen" → Bau-Atom an bau; bei „nein" → `descoped mit Befund`.

### PINE64 / Mantis-Shrimp
- **Status:** blockiert | **Bindung:** `linie:bau`
- **Lage:** Ox64 zugesagt (PINE64 info@), Versanddaten erbeten; Presence-Hardware ungebaut. Post `An bau: Mantis-Shrimp` liegt in `post.md`.
- **Blockade:** Hardware fehlt.
- **Braucht:** bau baut den Mantis-Shrimp (Spec `docs/specs/mantis-shrimp-bom.md`).

### ISH Chat (GitHub-Dritt-OAuth-App)
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** App hat Kontozugriff (`read:user`/`user:email`).
- **Blockade:** offener Zugriff.
- **Braucht:** Operator-Wort — App widerrufen?

### solar-system-open-data REST
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** `phi/blocked_sources.φ:60` (`blocked key`); HTTP 401. Key **frei/selbstbedienung** (`generatekey.html`).
- **Blockade:** E-Mail-Registrierung (Dritt-Akt).
- **Braucht:** Operator-Wort — freien Key anfordern.

### Amentum Developer
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** `phi/blocked_sources.φ:65` (`blocked account`); `register` 200, „14-day free trial".
- **Blockade:** Konto fehlt.
- **Braucht:** Operator-Wort — Konto anlegen.

### Split-Routing-Verifikation
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** 8 `000`-Hosts ungemessen.
- **Blockade:** sudo + Netz.
- **Braucht:** Wort/Route `./bin/proton-exit.sh ca`.

### SSDC Limadou
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `ssdc.nssdc.ac.cn` DNS tot (`ERR_NAME_NOT_RESOLVED`, 2026-09-21); Sotgiu (2026-09-16): CSES-02-Umstellung, „wait a few weeks".
- **Blockade:** Host nicht auflösbar / PI-Freigabe ausstehend.
- **Braucht:** Wiedervorlage (Trigger: Host auflösbar / Prozedur-Update).

### Cookie-Transfer
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Auslöser „Bedarf".
- **Blockade:** kein Bedarf.
- **Braucht:** nichts — wartend.

## Operator-Queue (Stand folge82; neue Messwerte seit folge81)

Neu gemessen seit folge80/81: SuperDARN (jetzt `pending`/ernte — kein Operator-Akt),
solar-system-open-data (freier Key), Amentum (freier Trial), SSDC (DNS tot). Damit
entfällt Punkt „SuperDARN" aus der Queue.

3. **DEMETER/CDPP** — „Mail senden" oder „selbst registrieren". (seit 2026-09-21)
4. **Riss 4** — „bauen"/„descopen". (seit 2026-09-21)
6. **Mantis-Shrimp** — bauen/descopen. (seit 2026-09-16)
7. **ISH Chat** — App widerrufen? (seit 2026-09-20)
8. **solar-system-open-data** — freien Key anfordern? (seit 2026-09-20)
9. **Amentum** — Konto anlegen (free trial)? (seit 2026-09-20)
10. **Split-Routing** — Route `./bin/proton-exit.sh ca`? (seit Ernte folge12–17)
12. **Free-Model-Bench** — Cloudflare-Token mit „Workers AI"-Permission? (seit 2026-09-21)
13. **SSDC** — warten (Host DNS-tot). (seit 2026-09-16)
14. **Cookie-Transfer** — nichts. (seit 2026-09-16)

## Benchmark

- **Delegation (Folge 82):** 1 × `general` (flash) — SuperDARN-Routen-Verifikation.
  Ergebnis: FITACF-Tool liefert echte Dateien anonym (sha256 `48637f08…`), FRDR
  31 Datasets, Zenodo 822 Records; keine falsche Antwort, kein pro/max-Doppel
  nötig. Keine neue Klasse — „Routine-Recherche/Verifikation" hat ihren
  registrierten Sieger (flash).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-entscheid-folge82.md` (neu)
- Move `handover-2026-09-21-entscheid-folge81.md` → `archiv/` (eigene Linie, atomar)
- `phi/blocked_sources.φ` (eigene Zeilen :21, :60, :65)
- `docs/handover/post.md` (eigene Zeile `An ernte: SuperDARN`)
- **nicht** angefasst: fremde `.github/workflows/tools-build.yml`,
  `src/archivar/*`, `tools/harvest/*` (bau/ernte), andere Linien-Handover.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
