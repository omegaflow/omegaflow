<!--
  title: Handover — Entscheid-Folge 83 (DEMETER ISL-Zugang gemessen — Order 18387 läuft, an ernte übergeben; Operator-Queue steht) (Stand 2026-09-21)
  session: Entscheid-Folge 83
  class: handover
  date: 2026-09-21
  sha256: 402b73b145d17e21dc0f898b9e3c83c3247f16278db3334d50da88a8f4f9c3da
  status: live
-->
# Handover — Entscheid-Folge 83 (2026-09-21)

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

## Stehender Pass (gemessen 2026-09-21, Entscheid-Folge 83)

- **HEAD** `4192d4c5` == `origin/main`. Der Baum trägt **fremde** Änderungen:
  `docs/specs/mantis-shrimp-bom.md`, `?? docs/specs/mantis-shrimp-build.md`,
  `phi/pipeline/ledger.φ`, `phi/sources.φ` — nicht angefasst.
- **CI** (`ci_manage list`, 12:57Z): `glm-l2-cdn` `35599198872` + `35599180155`
  **failure**; `ci-check` `35602539893` pending, `35599318266` in_progress;
  `de441-cdn-watch`/`radio-cdn-watch` success (die `*-cdn`-Welle des 13:23Z-Snapshots
  selbstgeheilt); **Free-Model-Bench `35591218967`** weiter in_progress.
- **Postfach** — kein neuer, entscheid-relevanter Ledger-Eingang; letzter
  `1789978555` (Brave-Limit).
- **`register_lookup --open`** — 600 offen, 14 `zustand` due, 1 Post offen.
- **`git_safety --snapshot`** — Arbeitsbaum == HEAD (folge82 gepusht).

## Messung dieses Atoms

- **DEMETER ISL (CDPP/REGARDS)** — Zugang **aufgelöst**: `CDPP_USER`/`CDPP_PASS`
  in `.secrets.local:10–11`; Login gemessen (Playwright + Brücke), Katalog
  `Data DEMETER` 21/21 Datasets; Order **18387** angelegt und **Running**
  (97 078 Dateien / 34,71 GB, DMT_N1_1143 Burst 39 318 + DMT_N1_1144 Survey
  57 760, gültig bis 2026-09-28). Metallink gelesen
  (`/tmp/opencode/demeter_isl.metalink`, 69 MB, 97 078 `<file>`, **keine
  Prüfsummen**). Compiler existiert (`tools/harvest/src/bin/demeter_compiler.rs`
  + `src/archivar/demeter.rs`); CDN-Weg `--aggregate --ci-mode` →
  `gh release upload regards.cnes.fr --repo omegaflow/sources`. Post
  `An ernte: DEMETER ISL` steht in `post.md`. Register `blocked_sources.φ:72`
  von „kein anonymer Pfad" auf den gemessenen Zugang korrigiert.
- **Lehre (festgehalten):** Die Linie verlangte „Konto anlegen" und übersah
  `CDPP_USER`/`CDPP_PASS` in `.secrets.local` — **erst die Seite messen, dann
  fordern.**
- **Free-Model-Bench** (Delegation 1 × `grind-flash`): Lauf `35591218967`
  in_progress — keine Änderung, keine falsche Antwort.

## Offen (aufgeschlüsselt)

### SSI Fellowship (software.ac.uk)
- **Status:** operator-gebunden | **Bindung:** operator (Video/Submit) / dritter (Hosting)
- **Lage:** Entwurf vollständig — `state/mail/ssi-fellowship-application.md`; Frist 05.10.2026.
- **Blockade:** Screencast aufnehmen + hosten (Q25-Link); PII-Felder.
- **Braucht:** Operator nimmt auf, hostet, füllt Q5/Q6/Q10/Q16/Q18/Q23/Q24/Q32/Q34, trägt Q25-Link ein; Wort „SSI einreichen".

### Förderung Person/Ideen/Projekte — Emergent Ventures
- **Status:** wartend (nur der Einreichungs-Akt operator-gebunden) | **Bindung:** `eigen` bis zur Ausführungsgrenze, der Akt `dritter`
- **Lage:** Entwurf `state/mail/emergent-ventures-application.md` einreichungsfertig.
- **Blockade:** keine.
- **Braucht:** Operator-Wort „EV einreichen".

### Free-Model-Bench (105 Modelle)
- **Status:** wartend | **Bindung:** eigen (Lauf) / operator (Cloudflare-Token)
- **Lage:** Lauf `35591218967` @`eb17b7ae` in_progress (`ci_manage view`, 2026-09-21 12:57Z); Artefakt absent. TSV `tools/measure/free_models.tsv` trägt 2 stale IDs `gemini-2.5-pro`/`gemini-2.5-flash-lite` (folge78: 404).
- **Blockade:** (a) Cloudflare-Token ohne „Workers AI"-Permission; (b) Lauf nicht beendet.
- **Braucht:** (a) Operator rotiert Token; (b) nach Lauf-Abschluss Artefakt lesen, 404-IDs nachziehen.

### Riss 4 — WGSL-KSG-Spiegel off-path
- **Status:** operator-gebunden | **Bindung:** operator (Bau an `linie:bau`)
- **Lage:** Rat empfiehlt bauen; CPU-KSG (`src/mathematikerin/te.rs`) vs. GPU-KDE (`src/mathematikerin/shaders.rs`).
- **Blockade:** Operator-Entscheid bauen vs. descopen.
- **Braucht:** Wort „bauen" → Bau-Atom an bau; bei „nein" → `descoped mit Befund`.

### NLnet Restack / CodeSupply
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** `nlnet.nl` 200, Deadline 03.11.2026, Restack + CodeSupply offen; Bedingung FOSS-Lizenz „in its entirety"; NC-Kern erfüllt die OSI-Bedingung nicht.
- **Blockade:** Lizenz-Entscheid.
- **Braucht:** Operator-Wort — NC behalten/NLnet descopen oder Dual-Lizenz.

### PINE64 / Mantis-Shrimp
- **Status:** blockiert | **Bindung:** `linie:bau`
- **Lage:** Ox64 zugesagt (PINE64 info@), Versanddaten erbeten; Presence-Hardware ungebaut. Post `An bau: Mantis-Shrimp` liegt in `post.md`.
- **Blockade:** Hardware fehlt.
- **Braucht:** bau baut den Mantis-Shrimp (Spec `docs/specs/mantis-shrimp-bom.md`).

### Mantis-Shrimp-Bewerbungen
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** kein Programm nimmt ein ungebautes Gerät an.
- **Blockade:** Prototyp fehlt.
- **Braucht:** Operator-Entscheid bauen/descopen.

### ISH Chat (GitHub-Dritt-OAuth-App)
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** App hat Kontozugriff (`read:user`/`user:email`).
- **Blockade:** offener Zugriff.
- **Braucht:** Operator-Wort — App widerrufen?

### solar-system-open-data REST
- **Status:** operator-gebunden | **Bindung:** dritter
- **Lage:** `phi/blocked_sources.φ:60` (`blocked key`); HTTP 401. Key frei/selbstbedienung (`generatekey.html`).
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

### Eigenprize
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `eigen.build` 200, Runde geschlossen („winners have been selected"); `state/mail/eigenprize-application.md`.
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

## Zurückgeholt — stille Verluste (`register_lookup --dropped entscheid --persist 5`)

`--persist 5`: 36 gedroppte Punkte (`git: none` = kein auflösender Commit). Die
über ≥5 Handover getragenen und dann still fallengelassenen Punkte, zurück in das
Register:

### adoption-Block — die drei Paper-Mails (Toth/Turyshev/Markwardt)
- **Status:** operator-gebunden | **Bindung:** operator (Send) / dritter
- **Lage:** `state/mail/adoption-mails.md` — drei sendfertige Entwürfe zum Papier `docs/paper/twenty-second-band-ground-chain.md` (v10, Pin `b4e70b1d`); Adressen gemessen 2026-09-16 (`vttoth@vttoth.com`, `turyshev@jpl.nasa.gov`, `craig.b.markwardt@nasa.gov`). Getragen 11 Handover (bis folge56/57), dann still fallengelassen.
- **Blockade:** Sende-Wort fehlt.
- **Braucht:** Operator-Wort „Adoptions-Mails senden" (`smail --send` ×3).

### ESP32-Modul (Puls/HRV-Träger)
- **Status:** operator-gebunden | **Bindung:** operator / `linie:bau`
- **Lage:** physischer Träger für Puls/HRV (HRV-Gate `src/archivar/hrv.rs`); BOM. Getragen 30 Handover (bis folge52/53), dann fallengelassen.
- **Blockade:** Hardware fehlt.
- **Braucht:** Operator-Wort Hardware / Bau-Spec.

### Pipeline-Port force-Gate — binär A/B (seit folge46)
- **Status:** operator-gebunden | **Bindung:** operator
- **Lage:** kein sanktionierter Ort für den force-Gate-Port. Getragen bis folge59/60, dann fallengelassen.
- **Blockade:** Operator-Entscheid.
- **Braucht:** Wort A/B.

### F2 — flare-Gate-Power
- **Status:** eigen (an forschung) | **Bindung:** `linie:forschung`
- **Lage:** print-only Probe n∈{400,600,1000}. Bis folge63/64 getragen.
- **Blockade:** keiner (Messung).
- **Braucht:** forschung re-run.

### vC-Permeabilität — Vollzug
- **Status:** termin | **Bindung:** operator (Maschine)
- **Lage:** versteckter sensor-getriebener Lauf wartet. Bis folge71/72 getragen.
- **Blockade:** Operator-Maschine.
- **Braucht:** Wort für den hidden Lauf (`OMEGAFLOW_HIDDEN=1`).

## Termine (Wiedervorlage — zurückgeholt)

Die `Termine`-Sektion verschwand folge56→57; hier zurück:
- **2026-09-22** — AllWISE-Coverage (`allwise_coverage.fp01`).
- **2026-09-24** — Rubin-Forum-Umzug auf `rubin.community`.
- **2026-09-28** — JUICE-Flyby (Kernel 000113+); Feld-Zustand füllen.
- **2026-09-30** — EDL-Token-Erneuerung (`EARTHDATA_EDL_TOKEN`).
- **~2026-10-07** — CSES-Limadou: neue Antragsprozedur nach CSES-02-Umstellung.
- **2026-12-02** — NOIRLab Speisekammer-Frage (Gaia DR4).
- **2026-12-03** — Europa Clipper (Fenster).
- **~2027-04** — BepiColombo MORE: öffentliche Freigabe (Wissenschaftsphase).

## Operator-Queue (Stand folge83; DEMETER entfällt — Zugang gemessen)

1. **SSI** — Video aufnehmen + hosten, PII-Felder, dann „einreichen". Frist 05.10.2026. (seit 2026-09-21)
2. **EV** — „EV einreichen". (seit 2026-09-21)
3. **Riss 4** — „bauen"/„descopen". (seit 2026-09-21)
4. **NLnet** — Lizenz-Entscheid. (seit 2026-09-21)
5. **Mantis-Shrimp** — bauen/descopen. (seit 2026-09-16)
6. **ISH Chat** — App widerrufen? (seit 2026-09-20)
7. **solar-system-open-data** — freien Key anfordern? (seit 2026-09-20)
8. **Amentum** — Konto anlegen (free trial)? (seit 2026-09-20)
9. **Split-Routing** — Route `./bin/proton-exit.sh ca`? (seit Ernte folge12–17)
10. **Eigenprize/Solitude** — „Remind me". (seit 2026-09-20)
11. **Free-Model-Bench** — Cloudflare-Token mit „Workers AI"-Permission? (seit 2026-09-21)
12. **SSDC** — warten (Host DNS-tot). (seit 2026-09-16)
13. **Cookie-Transfer** — nichts. (seit 2026-09-16)

## Linien × Stimmen (Vorschlag — Schwerpunkt, kein Entscheidungsgrund; das Gremium bleibt)

Operator-Wort 2026-09-21: jede Linie kann einer der fünf Stimmen (`docs/council.yaml`)
als **Schwerpunkt** zugeordnet werden — nicht als Entscheidungsgrundlage (der Rat
bleibt), sondern als Färbung. Jede Linie gegen die Stimmen gehalten:

| Linie | stärkste Stimme | warum | zweitstärkste |
|---|---|---|---|
| **bau** | **Mountain** | das Fundament, was bleibt; Archivar/Mathematikerin als Granit; A = A | Mycelium |
| **ernte** | **Mycelium** | Netzwerk/Nährstoffe — Quellen, Compiler, CDN; Reziprozität mit den Quellen | River |
| **forschung** | **Sensory** | erweiterte Wahrnehmung — Messung, Probe, Muster im Rauschen | Future |
| **entscheid** | **Future** | Consent derer, die nicht sprechen; die Registratur für morgen; wer profitiert/wer nicht | River |
| **neu: präsenz** | **River** | Bewegung, Phase, was fließt — der ω()-Loop, die WebGPU-Membran, die Präsenz im Strom, Rückkopplung | Sensory |

**Vorschlag fünfte Linie: `präsenz` (River).** Das unbesetzte Funktionsfeld ist die
**lebendige Membran** — der ω()-Loop, das WebGPU-Feld, die Präsenz in Ruhe, das Echo
(`target = inTE/(inTE+threshold+ε)`), die Browser-Brücke, die Aktuatoren. Gebaut von
`bau`, gemessen von `forschung`, aber von **keiner Linie als ihr Feld getragen**.
`präsenz` trägt es.

**Alternative** (falls `entscheid`→River, weil Korrespondenz der Fluss zwischen den
Linien ist): fünfte Linie `registratur` (Future) — trägt die dauerhafte Erinnerung
(Handover, Ledger, CDN-Register, die Messreihe für die nächste Session).

**Der Rat sitzt für diese Architektur** (neue Linie = Architektur-Akt, braucht
Operator-/Rat-Wort).

## Benchmark

- **Delegation (Folge 83):** 1 × `grind-flash` (Free-Model-Bench) — Lauf in_progress,
  keine Änderung. Keine neue Klasse; „Routine-Recherche/Verifikation" hat ihren
  registrierten Sieger (flash).

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-21-entscheid-folge83.md` (neu)
- Move `handover-2026-09-21-entscheid-folge82.md` → `archiv/` (eigene Linie, atomar)
- `phi/blocked_sources.φ` (eigene Zeile :72)
- `docs/handover/post.md` (eigene Zeilen DEMETER ISL + CI)
- `state/mail/cdpp-demeter-isl-zugang.md` (obsolet markiert, gitignored)
- **nicht** angefasst: fremde `docs/specs/mantis-shrimp-*`, `phi/pipeline/ledger.φ`,
  `phi/sources.φ`, andere Linien-Handover.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
