<!--
  title: Survey — Die Weberin: offene Quellen-Routen, Treffer (Stand 2026-09-13)
  class: survey
  date: 2026-09-13
  sha256: 2803fce3d12d3c97d03aca15fb47253cd068ae364706390e62110bc7dc87ae97
  status: live
  see-also: docs/surveys/survey-2026-09-13-weberin-quellen.md docs/surveys/survey-2026-09-13-weberin-quellen-folge.md
-->
# Die Weberin — offene Quellen-Routen, Treffer (Stand 2026-09-13)

Dritter Durchgang. Die Taucher haben die im ersten Snapshot
(`survey-2026-09-13-weberin-quellen.md`) und der Folge (`-folge.md`) offenen
Punkte mit der erweiterten Kaskade nachgemessen: die `--verdict`-Leiter probt in
Stage 2 jetzt auch den userspace-Proton-Exit (`socks5h://127.0.0.1:25344`,
wireproxy) und kennt `--cacert`; der Archivar honoriert `OMEGAFLOW_CA_BUNDLE`
(Commit `868f604`). Alle Messungen datieren 2026-09-13, lokaler Exit plus
Proton-Exit.

Status-Vokabular (bindend, wie im ersten Snapshot): `live` / `blocked` /
`declined` / `pending` / `not-published`. Absent heißt `not-published` — nie
eine Null.

## Fink — Broker (em, S²-Zeuge)

**Korrektur:** der aktive ZTF-Host ist `api.ztf.fink-portal.org`, nicht
`api.fink-portal.org` (alt, tot) und nicht `fink-portal.org`.

| Route | GET | POST | HTTP | Verdikt |
|---|---|---|---|---|
| `api.ztf.fink-portal.org/swagger.json` | 200 | — | live (Swagger 2.0, alle Pfade) | live |
| `api.ztf.fink-portal.org/api/v1/objects` | 200 | 200 | JSON-Alert-Array | live |
| `api.lsst.fink-portal.org/api/v1/{objects,sources}` | 504 | — | nginx/1.20.1, Upstream tot | pending |
| `api.fink-portal.org` (alt) | 000 | — | — | not-published |

**note:** `lsst.fink-portal.org` (Portal) lebt (200); nur der API-Host
`api.lsst…` antwortet `504`. Doku `doc.lsst.fink-broker.org` benennt
`https://api.{survey}.fink-portal.org` als Muster. Re-messen.

## TNS — Broker (em, S²-Zeuge)

**Korrektur:** der 403 ist die Fronting-Schicht, nicht die App — mit POST +
gültigem Bot-Key ist die Quelle `live`.

| Route | Status | Server | Verdikt |
|---|---|---|---|
| GET `…/tns_public_objects.csv.zip` | 403 | `awselb/2.0` (WAF) | blocked (GET) |
| POST + `tns_marker`-UA + `api_key` | 200 | Apache/Drupal | live (ZIP 14,3 MB) |
| `POST /api/get/search` | 200 | Apache | live (JSON) |

**note:** `api_key` und `data` als getrennte Form-Felder; Rate-Limit
`x-rate-limit-limit: 30`. Als Quelle registriert (`post_body
api_key={TNS_API_KEY}`, `header user-agent {TNS_UA}`).

## HAWC / LHAASO (em, S²-Zeuge)

- **HAWC** — TLS-Lücke geschlossen: Bundle = System-Store + Let's-Encrypt `YR1`
  + `ISRG Root YR`; `curl --cacert <bundle> …/3HWC.yaml` → 200 (ohne `--cacert`
  curl 60). `live`, aber nicht als Quelle registriert.
- **LHAASO** — kein TLS-Gap (Kette vollständig: Leaf + RapidSSL + DigiCert G2);
  `casdc.china-vo.org/…/table.csv` 200. `live`.

## Proton-Exit

- `proton-wg` (wireproxy) läuft: `socks5h://127.0.0.1:25344` → Exit
  `135.136.39.36` (direct `185.51.184.2`). Stage 2 der `--verdict`-Leiter probt
  ihn jetzt zusätzlich zu Kernel-`proton*`-Interfaces.
- Der Exit öffnet **keine** der blockierten Türen zusätzlich (Fink/HAWC/TNS
  scheitern beidseitig aus TLS-/Netz-/Server-Grund, nicht aus Geo-Block).

## Gesamtbild

Zwei `blocked`-Fälle sind in dieser Messung in `live` überführt (TNS via
POST+Key; HAWC via `--cacert`), einer korrigiert (Fink-Host `api.ztf.…`). Offen
bleiben LSST-Fink (`504`, re-messen) und die Quellen-Registrierung von
HAWC/LHAASO. Kein Wert ist fabriziert; jeder ungemessene Punkt ist `pending`.
