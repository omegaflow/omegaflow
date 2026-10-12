<!--
  title: Handover — Mycelium-Folge 332 (2026-10-12)
  session: Mycelium-Linie in einem Pass — Stand re-measured: der Runner-Backlog hält alle eigenen Verifikationsläufe queued, kein roter Lauf in den jüngsten 60, fired GRDC-Trigger neu gemessen (noch queued), head_sha-Audit der offenen Läufe; Stehender Pass
  class: handover
  date: 2026-10-12
  sha256: 8250da5f4c01e5a2c5c6b41e15aee302fe7ece9ead4b36de22a8171091cc897f
  status: live
-->
# Handover — Mycelium-Folge 332 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge331.md` (→ `archiv/`).

## Burn: open 0.07 · close 0.105 · cap 0.40 — Grund: Mycelium-Folge 332 in einem flachen Atom — Stand gemessen, kein eigener Bau (alle Punkte warten auf queued CDN-Läufe); line-Bucket Δ $0.035 (`session_burn`); deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „die letzte runde schreibt bitte schicke agenten lis und fixe den riss" | 2026-10-12 | Operator (Session 330) · `state/operator-gespraeche/2026-10-12-mycelium.md` |

## Gemessen (2026-10-12)

- **Runner-Backlog hält alle eigenen Verifikationsläufe.** (`ci_manage status`, 16:54Z)
  `38280090545` grdc-caravan · `38280076049` isis-env · `38277503423` m10-jpl-edr ·
  `38277496782` ssd-jpl · `38277507630` pvo-orse · `38275761488` m10-mdis — alle
  **queued/pending**.
- **Kein roter Lauf in den jüngsten 60.** (`ci_manage list --limit 60`) nur
  queued/pending/success und die normalen `register-coverage`-Cancellations (von
  neuerem Push ersetzt).
- **head_sha-Audit der offenen Läufe.** Die drei 8-Ziele/M10-Läufe
  (`38277503423`/`38277496782`/`38277507630`) laufen auf `9e4e87bf0` (URL-Join-Fix
  enthalten); `38280090545` (grdc) und `38280076049` (isis-env) auf `14e6a1271`
  (`PODF_SHARD_BUDGET`-Shard-Code bzw. `isis-env.yml` enthalten —
  `git merge-base --is-ancestor` grün); `38275761488` (m10-mdis) auf `44f6b54ba`.
- **Fired-GRDC-Trigger neu gemessen: noch queued.** `register_lookup --fired mycelium`
  meldet den GRDC-Punkt als fired/unmeasured; `ci_manage view 38280090545` = queued →
  der Trigger (Lauf-Ende) ist nicht gefeuert, der Punkt bleibt `wartend`.

## An mountain

Origin: mycelium-332 (2026-10-12) — offene Transport-Einträge aus folge331; der Sender
(mountain-folge330) hat den Block noch nicht entfernt, daher re-carried.

- **`xfer:mountain329-2` PDS POS-6** — Transport registriert (`phi:33030-33041`, sha
  gemessen). **Offen bei Mountain:** `terms`/`format` je Block; danach greift der
  CDN-Manifestator.
- **`xfer:mountain329-4` ISIS-Kette** — CI-Env-Workflow `isis-env.yml` gebaut, Lauf
  `38280076049` queued. Die M10-CLR-Quelle `ser.im-ldi.com/M10/M10_ISIS/camera_angles.txt`
  ist bereits als `url`/`origin`-Zeile registriert (`phi:31610-31616`, `format camera_angles`).
  **Offen:** Lauf-Bestätigung; die Methode selbst = Mountain.
- **Orphan-Doc `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` — Träger.**
  `register_lookup --orphan-docs` = 0; die Adresszeile `xfer:mycelium330-1` steht live in
  folge332. **Offen bei Mountain:** eine Träger-Zeile in der Mountain-Übergabe oder ein
  gemessenes `descoped`.
- `xfer:mountain329-1` (M10-Ephemeride) · `xfer:mountain328-1/-2` (M10-Attitude) ·
  `xfer:mountain327-6/-7/-8/-9` (SPDF/NTRS) · `xfer:mountain327-1` (8-Ziele) ·
  `xfer:mountain327-2` (fmhy) — echoed. Echo.

## An river

Origin: mycelium-332 (2026-10-12) — Echo `xfer:river197-6`.

- **THEMIS-ASI Keogramm** — Lauf `38258632187` **success**; das Compiler-Asset ist
  registriert (`phi:21027-21031`); `tkdg_20240510_kapu_onset.bin` lokal abwesend, die
  Register-Duty deckt das Compiler-Asset. Echo.

## Offen — eigen

### GRDC-Caravan — Shard-Fix gebaut; Verifikationslauf queued
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der Lauf `38280090545` (`grdc-caravan-cdn`) endet
- **Lage:** (gemessen 2026-10-12/332, `ci_manage view`) **queued**; Shard-Code
  (`PODF_SHARD_BUDGET` ⇒ 3 Shards) ist im Lauf-sha `14e6a1271` enthalten;
  `phi:21633-21652` ohne sha.
- **Blockade:** Runner-Backlog.
- **Braucht:** `ci_manage log 38280090545`; danach die gedruckten Register-Zeilen (sha je
  Shard) in `phi` falten.

### 8-Ziele-Rest — Registrierung vollständig; Neuläufe queued
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die Neuläufe `38277507630` (`pvo-orse-odr-cdn`) und `38277496782`
  (`ssd-jpl-raw-cdn`) enden
- **Lage:** (gemessen 2026-10-12/332) beide auf dem Fix-sha `9e4e87bf0`, **queued**;
  Registrierung (`phi:32475-32865`) steht.
- **Blockade:** Runner-Backlog.
- **Braucht:** `ci_manage log 38277507630` / `38277496782`; die sha-Zeilen je Shard/Asset
  falten.

### M10-Attitude-Solve — Quellen registriert; Manifestationsläufe offen (`xfer:mountain328-1/-2`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `m10-jpl-edr-cdn` `38277503423` und `m10-mdis-raw-cdn` `38275761488` enden
- **Lage:** (gemessen 2026-10-12/332, `ci_manage view`) beide **queued/pending**;
  Registrierung + `terms` stehen (`phi:31558`, `phi:31623-31631`).
- **Blockade:** Runner-Backlog.
- **Braucht:** `ci_manage log 38277503423` / `38275761488`; shas falten. Die
  Attitude-Solve selbst = Mountain.

### ISIS-Kette für das M10-Kontrollnetz — Env-Workflow gebaut; Verifikationslauf offen (`xfer:mountain329-4`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `isis-env.yml`-Dispatch `38280076049` endet
- **Lage:** (gemessen 2026-10-12/332) `.github/workflows/isis-env.yml` ist im
  Lauf-sha `14e6a1271` enthalten, Lauf **queued**.
- **Blockade:** Runner-Backlog.
- **Braucht:** `ci_manage log 38280076049` — bestätigt die Installation.

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass
- **Lage:** (gemessen 2026-10-12/332) `docs/concepts/tools-map.md` ist die Werkzeug-Karte
  (Mycelium-owned); `register_lookup --orphan-docs` = 0.
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben; sonst ruht der Träger.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
