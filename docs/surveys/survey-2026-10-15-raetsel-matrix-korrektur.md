<!--
  title: Rätsel-Matrix — Korrektur: Block vs Kanal (die acht Fäden)
  class: survey
  date: 2026-10-15
  sha256: 7085d450e1ad4daccc30f4eae0ca2758ec4e65850a86ff34a3515086d0984982
  status: live
  see-also: docs/surveys/survey-2026-10-13-raetsel-master-matrix.md docs/surveys/survey-2026-10-14-gewebe-faeden.md
-->
# Rätsel-Matrix — Korrektur: Block vs Kanal (die acht Fäden)

Operator-Auftrag 2026-10-15: ein fremder Text (GLM) trug die acht fehlenden Fäden als „nicht
registriert" vor. Nachgemessen am Baum — `phi/sources.φ` + `git log -S` — **nicht** am Text.
Der Befund: der Text verwechselt **Feld-/Kanal-Lücken** der Matrix mit **fehlenden Quellen-Blöcken**.
Die meisten Quellen stehen längst; es fehlen die **Kanal-Zeilen**.

## Methode

- `sgrep` je Token gegen `phi/sources.φ` (kase-insensitiv), plus `git log -S <asset> -- phi/sources.φ`.
- Gemessen am HEAD dieser Session; jede Zeile mit File:Zeile oder Commit belegt.

## Die Korrektur je Faden

| # | Faden | Fremdtext sagt | Baum sagt (Beleg) |
|---|---|---|---|
| 1 | HMI (Photosphären-Magnetogramm) | JSOC gefunden, **terms unklar**, kein Block | **JSOC ist registriert** (`phi:2081` AIA), `terms CC0-1.0` (JSOC_Data_Policy) — terms **nicht** unklar. Nur der `hmi`-**Kanal** fehlt. |
| 2 | Sonnenfleckenzahl | F10.7-Proxy da, SILSO/SSN fehlt | **stimmt**: F10.7 registriert (`phi:433`, `phi:12896` `f107_penticton`); `sunspot`/`silso`/`ssn` = 0. |
| 3 | Stellar-Aktivität (XUV, log R'HK) | RAVE/GALAH gefunden, **nicht registriert** | **falsch**: **RAVE** (`phi:9815` gavo, `phi:9920` dc.g-vo) **und GALAH** (`phi:9898` datacentral) registriert. `xuv`/`rhk`/`ca_hk` = 0. |
| 4 | Schwache Linsen / Shear | KiDS gefunden, **nicht registriert** | **falsch**: **KiDS DR4 bright + DR4.1 WL** (`phi:31167`, `phi:31178`) + Conicus3D WL (`phi:30583`) registriert. `lensing`/`shear` = 0. |
| 5 | Neutrino/IceCube/Auger | LHAASO registriert; Auger nur „gefunden"; IceCube offen | **falsch**: **LHAASO** (`phi:28488`), **Auger** (`phi:18990`, CC-BY-SA-4.0), **IceCube IceTracks DR2** (`phi:14659` dataverse.harvard.edu) alle registriert. `neutrino` = 0. |
| 6 | 21 cm / EDGES | WALLABY gefunden, **nicht registriert** | **halb falsch**: **WALLABY Pilot DR2** HiPS3D (`phi:14519`, ODbL-1.0) registriert; `21cm`/EDGES = 0. |
| 7 | PTA / Pulsar-Timing | EPTA DR2 gefunden, **nicht registriert** | **falsch**: **EPTA DR2** (`phi:30615`, Zenodo `8300645`) registriert. Pulsar-Felder da; PTA-Residuum = 0. |
| 8 | TGF / Blitz / VLF / Wolken | **WGLC registriert**, Athabasca VLF manifestiert | **WGLC falsch**: `wglc` = **0** in phi (nur der Compiler `wglc_compiler.rs` steht). Athabasca VLF **registriert** (`phi:420`; `cfb06a9c5`/`e6c90e38d`, 2026-10-14). `tgf` = 0; Wolken-Kanal = 0. |

## Commit-Belege

- **RAVE / GALAH / KiDS / EPTA / LHAASO** — `a727df6ff` (**mountain-341, 2026-10-13**): „the
  delegated register work (EPTA/RAVE/GALAH/1LHAASO/KiDS …)".
- **Auger** — `b10170c23` (mountain-341, 2026-10-13). **IceCube** — `5be149090` (mycelium, 2026-10-13).
- **Athabasca VLF** — `cfb06a9c5` / `e6c90e38d` (mountain-344, 2026-10-14).
- **Nicht „seit gestern":** die Registrierungen dieser Fäden sind vom **13.10.**, nicht vom 14.10.
  Die 14.10.-Commits betreffen DE-2, GEOFON-Event, γ-Cat/Fermi-4LAC, Athabasca-VLF-Spektren.

## Risse (nicht geglättet)

- **Block ≠ Kanal.** Die Matrix C (2026-10-13) misst **Feld-/Kanal-Lücken** (`hmi`/`sunspot`/`xuv`/
  `lensing`/`neutrino`/`21cm`/PTA/TGF = je **0 Feldzeilen**). Der Fremdtext liest daraus „Quelle
  nicht registriert" — für **6 von 8** Zeilen falsch. Die Quelle ist da, die **Kanal-Zeile** fehlt.
- **SEANOE ist überholt.** Commit `4f2fe9ff7` (mountain, 2026-10-14) hat den SEANOE-Block
  **verworfen** — „measured duplicate of the `argo_gc` asset (same url)". Die (2026-10-15 gelieferte)
  Adresse `seanoe.org/data/00311/42182/` ist genau dieser Argo-Datensatz; die Frage war am 14.10.
  schon beantwortet (der Block wurde am selben Tag erst angelegt, `a3dbdcea0`, dann verworfen).

## Korrigierte Zählung

Acht **fehlende Kanäle**, nicht acht fehlende Quellen. **Nur WGLC fehlt als ganzer Block**; die
übrigen sieben sind Kanal-Lücken über vorhandenen Quellen. Echte Quellen-Lücken daneben: **SILSO**,
**EDGES/21 cm**, **TGF** und **Wolkenbedeckung**. Kanal-Lücken: **HMI**, **XUV/log R'HK**,
**Shear**, **Neutrino**, **PTA**.
