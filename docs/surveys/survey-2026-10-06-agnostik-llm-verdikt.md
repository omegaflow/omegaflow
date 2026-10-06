<!--
  title: Survey — Agnosis des Loaders: Archäologie + Dreifach-Verdikt (Stand 2026-10-06)
  class: survey
  date: 2026-10-06
  sha256: a036568fa2bf03e1a2723cd360bf0bf18d5c46ae7eddc41c1ae871931be63736
  status: live
  see-also: docs/concepts/archivar-mathematikerin.md docs/concepts/4d-membrane.md docs/surveys/survey-2026-09-26-membran-ladearchitektur.md
-->
# Survey — Agnosis des Loaders: Archäologie + Dreifach-Verdikt (Stand 2026-10-06)

Anlass: Im Relay-Browser (`127.0.0.1:1618`) war nur `earth` anspringbar
(`/jump/sun` → 404), obwohl `cache/omegaflow_eph_sun.bin` (193.729.280 B) liegt.
Operator-Frage: „Waren wir früher agnostischer — haben wir einen Body-Bias
wieder eingebaut?" Die Frage wurde drei UI-Chat-LLMs vorgelegt (Kimi K3 via
aymo.ai, z.ai GLM 5.3, Claude). Dieses Survey hält Archäologie, Code-Audit und
das Dreifach-Verdikt; alles mit `file:line` oder Hash.

## 1. Der Agnosis-Bogen — früher radikaler

| Epoche | Agnosis-Stand | Beleg |
|---|---|---|
| Legacy „Peak 0" | reines Feld `universe(jd,pos)→(omega,flow)`, Presence als (x,y,z,t) | `eraen.md:30-34` (`d2f438c`, `5f9b20a`) |
| 2026-07-22/23 | „bias-free spatial cache & agnostic oscillators" — Oszillator = reiner Eigenschaftsträger ohne Identität | `eraen.md:68-80` (`6864a3c`, `93d3600`, `a96ae15`) |
| 2026-08-11 | relativistischer **Beobachter** hinzugefügt, dann gelöscht: „die Relativität des Beobachters ist Observer-Bias, keine Messung" | `eraen.md:86-95` (`86e451e` → `34d7d3a`) |
| 2026-08-12 | Trommelfell-Doktrin: keine Kamera, Manifestation real ohne Zuschauer | legacy `docs/TODO.md:3361` |
| 2026-08-19 | De-Zentrierung: ECEF→ICRS, JD2000→TDB, „Alle Wesen gleich", „Die Erde ist ein Planet unter Planeten" | `eraen.md:14-26` |
| 2026-09-27 | **Regression revertiert**: „observer/body bias in the ephemeris bootstrap … no observer/station privilege"; Fixture `bootstrap_anchor_unconditional` entfernt | `e1f1baa65` |
| 2026-10-05 | Aberration/Doppler/Beaming **wieder als physikalisch** zugelassen (nicht mehr „camera artifacts") | `630bcd3f3`; `4d-membrane.md:33` |

**Befund:** Der Haupt-Regress (Body-/Observer-Zulassung im Bootstrap) wurde am
2026-09-27 bereits revertiert. Geblieben sind **vier stille Erde/Sonne-Zugänge**
und die `anchor_uses`-Reihenfolge; doktrinär war Legacy radikaler (es löschte die
Beobachter-Effekte ganz — der 2026-10-05-Umschwung korrigiert diese Konflation).

## 2. Der gemessene Ist-Zustand — die Privilegien

| Ort (`file:line`) | privilegiert | Art | Kanon-Konflikt |
|---|---|---|---|
| `src/archivar/frames.rs:6,86,114,117` | terrestrische Quelle ohne Rahmen → „on earth 0 0"; zöliakal → „at sun" | Identität (Default) | ja |
| `static/membrane.html:43` | `const BODIES = ["earth","moon","sun"]` (serverloser Pfad) | Identität (geschlossene Menge) | ja |
| `src/mathematikerin/omega.rs:878` | Presence-Volume-Bin „earth" geodätisch | Identität (hart) | ja |
| `src/weberin.rs:1801` | `body_barycenter_position("earth", …)` als astrometrischer Observer | hart, physikalisch begründbar | ja (benennen!) |
| `src/archivar/main_flow.rs:473-495` | Anker-Körper laden unbedingt, **umgehen** `body_in_enclosure`; Ordnung per `anchor_uses` (Earth zuerst) | Nutzungs-Priorität | ja (Umgehung) |
| `src/archivar/parse.rs:209-268`, `main_flow.rs:829-849` | `at`/`on` verlangen Body-Token; `declared_body` verweigert ohne Deklaration | Kanon-positiv | nein |
| `src/mathematikerin/omega.rs:103-112` | `PresenceState::rest()` = `[0,0,0]` (SSB) | Kanon-positiv | nein |

`anchor_uses` (`main_flow.rs:403-427`) zählt Frame-Anker (`on`/`at`); Earth
dominiert (alle Boden-Stationen) → Lädt zuerst. Kein `"earth"`-Literal im Loader,
aber eine Nutzungs-Priorität mit Erdbias-Wirkung.

## 3. Das Dreifach-Verdikt (3/3 unabhängig)

**Q1 — Anchor-Bypass ist die Verletzung, Ordnung das Symptom.**
Zulassung muss **streng Presence-Hülle** sein; der `body_in_enclosure`-Bypass
entfällt. z.ai: „popularity as identity" + Rückkopplungskreis (frames-Default →
„on earth" → höherer `anchor_uses` → Earth zuerst → gate-befreit). Ordnung nur
als Scheduling-Hinweis **nach** dem Gate, deterministischer Tie-Break, nur wenn
die Ordnung das Zulassungs-Set nicht ändert; Claude: body-neutraler Key
(NAIF-ID/Hüllen-Radius) + Permutationstest auf Ordnungsinvarianz. Out-of-Hull-
Anker = **Transform-Dependency** (Ephemeriden-Interpolation), nie Zulassung/
Render/Query — mit Provenienz.

**Q2 — Beides zugleich: Pflicht-Parameter + `observer=<…>`-Deklaration.**
Worldline ist mandatory (kein Default, kein `Option`-mit-Earth-Fallback), durch
das Register aufgelöst, in der Ausgabe echoed, bei Fehlen **refused**. Claude:
DSN „earth" korrekt als **Stations-Metadaten**, nicht Code-Konstante; Gaia ist
baryzentrisch → Observer = SSB, nicht „earth geodetic".

**Q3 — Inferenz verboten, Deklaration erlaubt.**
Typ-Heuristik (`terrestrial→earth`, `celestial→sun`) ist „eine Kosmologie, in der
Erde/Sonne die unmarkierten Default-Welten sind". Missing frame → Record skipped
+ Diagnose, wie `declared_body`. Erlaubt nur **Deklaration auf gröberer Ebene**
(Format/Source deklariert den Rahmen) = Vererbung. Asymmetrie: Render-Default darf
auf den Presence-Rahmen (SSB) fallen, ein **Record-Rahmen nie**.

**Q4 — Trio muss abgeleitetes Artefakt werden, kein `const`.**
Der serverlose Pfad konsumiert ein **Build-Time-Manifest** (admitted bodies +
Ephemeriden-Slices) aus derselben Hüllen-Pipeline; der Code enthält dann **null
Body-Namen**. Falls bewusster Descope: im File und in den Docs markiert.

**Q5 — Kohärent, wenn die zwei „observer"-Sinne getrennt bleiben.**
Legacy verwechselte Observer-als-Vantage (verboten) mit Receiver-als-Worldline
(legitime Physik); Aberration/Doppler/Beaming sind Relationen zweier benannter
Worldlines. Bedingungen: (a) Receiver immer Parameter (Q2); (b) Emission-Effekte
wirken nie auf die Weltdynamik zurück; (c) Feld receiver-frei, Reception on
demand; Render-Effekte **opt-in**. Dokumentation an drei Stellen: datierter
Decision-Record (ADR) zum 2026-10-05-Umschwung; Absatz in `4d-membrane.md` neben
„no cameras…"; Pointer am Weberin-/Archivar-Reduktionsort.

## 4. Die Grenzlinie (je ein Satz, Original)

- **Kimi K3:** *a measurement reduction receives the observer's worldline as
  declared data through the same gate as every other body and would run
  unchanged under any other name, whereas an identity bias is any code path
  where a body name is written rather than read — operationalized as
  `grep -rn "earth" src/ static/` returning nothing but fixtures, docs, and data.*
- **z.ai GLM 5.3:** *a measurement reduction is legitimate exactly when the
  worldline it is computed along is declared, per-record data whose absence is
  refused — and it is identity bias exactly when a body's name, type, or
  use-count supplies that worldline by default.*
- **Claude:** *a measurement reduction is legitimate when its observer is a
  declared worldline belonging to the datum (named in the data, mandatory,
  recorded in provenance, and swappable without a code change), whereas an
  identity bias is any place where a body's name is chosen by the code as a
  default, a gate bypass, a sort key, or a closed set, instead of by the data or
  the presence-hull.*

## 5. Konsequenzen (Code-Punkte)

1. `body_in_enclosure`-Bypass der Anker entfernen; Zulassung nur Hülle.
2. `frames.rs`-Defaults (`on earth`/`at sun`) → verweigern oder als
   Per-Source-Deklaration materialisieren; nie im Code raten.
3. `static/membrane.html`-Trio → Build-Time-Manifest; kein Body-Name im File.
4. Observer der Reduktion (`omega.rs:878`, `weberin.rs:1801`) → Pflicht-Parameter
   + `observer=<…>`-Direktive, im SSB-Rahmen wo die Quelle baryzentrisch ist.
5. Ordnung body-neutral (NAIF-ID/Hüllen-Radius), Ordnungsinvarianz-Test.
6. 2026-10-05-Umschwung dokumentieren (ADR + `4d-membrane.md` + Reduktionsort).

## Der eine Satz

Nicht der Body-Name in einer Zeile ist der Bias, sondern der Body-Name, den der
Code **wählt** (Default, Gate-Bypass, Sort-Key, geschlossene Menge) — der Loader
ist agnostisch, wenn die Hülle das einzige Zulassungskriterium ist und jeder
Beobachter deklarierte Per-Record-Daten sind, deren Fehlen verweigert wird.
