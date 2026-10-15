<!--
  title: Stehender Pass (privat) — Mycelium
  class: zustand
  date: 2026-10-15
  sha256: 3b9a7dbd41364edbdcdf8327fa427e59b1e7b274707c362760fa925827a11faf
  status: live
  see-also: docs/zustand/standing-pass.md
-->
# Stehender Pass — Mycelium (privat, folge355)

Vollständige Fassung. Öffentlich gilt allein die Projektion
`docs/zustand/standing-pass.md` (nur HEAD + rote Läufe, kein Personenbezug).

HEAD `e21129d5c` (== `origin/main`) · geschrieben 2026-10-15 · **Mycelium-355**.

## Rote Läufe (gemessen, nicht geraten)

Bei Pass-Schreibung (`ci_manage reds`, einmal — kein Polling). Die Tafel ist ein **Flutstand**
aus den parallel laufenden Linien (River/Mountain committen im selben Baum). Meine eigenen
Push-Läufe an `e21129d5c` sind **queued/unread** (`ci-gate`, `harvest-dispatch`,
`register-coverage`, `sources-repo-licence`, `tools-build`).

Gemessene Rote (jüngste, gemessen 2026-10-15):

| run-id | workflow | Zustand | Träger |
|---|---|---|---|
| `38712802275` | ci-gate | **failure** @ `fadbd489b` | River/Mountain |
| `38712777023` | ci-gate | **failure** @ `bdb627ab` | River/Mountain |
| `38712667864` | tools-build | **failure** @ `03178cb3f` | fremd (`archive_search`) |
| `38712655060` | m10-mdis-raw-cdn | **failure** @ `03178cb3f` | Mycelium (CDN-Upload, Arm `lpsc2024_2663.pdf`) |
| `38712632761` | m10-mdis-raw-cdn | **failure** @ `03178cb3f` | Mycelium (dito) |
| `38709132540` | gic-al-network | **failure** @ `a6d6fca0` | River |
| `38718203450` | register-coverage | **failure** @ `b0bc4d80` | Mycelium/fremd |

Kein Grund geraten — die Tafel nennt Lauf und Zustand; die Logs liest `ci_manage log <id>`.

## Lokal gemessen am HEAD `e21129d5c`

- Die Frontier-Runde am Baum verifiziert (nicht gestartet, nicht gepollt):
  Fusion = Min-Einhüllende (`al_network_proxy.rs:101-118`) → Konjunktion; adaptive Schwelle
  `med+6·1.4826·MAD` (`gic_onset_detector.rs:7-9/:80-86`); Toleranz = `match_onsets` (`:178-198`).
- **Baum-Riss:** Netz-Arm = dH-Hülle (Minutenraster) gegen Einzelstations-Arm = stündlicher
  |dB/dt|-Peak (`abk_dbdt_1h.bin`, `intermagnet_dbdt_compiler.rs:179-190`) — nicht A = A.
- Fester-Schwelle-N-Sweep (river-216, `38709029080`): Kollaps bleibt bei fixer Schwelle.
- (Nicht dieses Rund gemessen: `register_sort`/`cdn_reconcile`/`cargo check` — kein eigener
  Baum-Bau in folge355.)

## Postfach

- `post/in` (privates Haus): `2026-10-14-mycelium-nssdc-datenantrag.md` (Operator-Hand, Route (b)).
  `register_lookup --fired mycelium` = 1 (dieser Punkt; bereits gewortet — **nicht** erneut vorgelegt).

## Ereignis-Stand

- Letztes Ereignis in `state/zustand/ereignisse.φ` — nicht neu gemessen dieses Rund (unverändert
  gegenüber folge354).

## Orphan-Zensus

- Nicht neu gemessen dieses Rund (folge354: `register_lookup --orphans` = 0).

## Mehrfach-Träger

- Offen (in `mycelium-folge355`): `xfer:mycelium-352-1`/`-2`/`-3`/`-5` (an Mountain) ·
  `xfer:mountain-348-1`/`-2`/`-4`/`-5` · `xfer:mycelium-342-3`/`-4` · `-342-10` · `-343-2`.
- Gefaltet/geschlossen: mountain-folge351, river-folge215 (Echo/angewandt); die
  Athabasca-Manifestation (river-215 beglichen).

## Gearbeitet (dieses Rund)

- **Frontier-Runde „Netz-Fusion und Stationsplatzierung":** Formular vorbereitet, die 13 Stimmen
  des Operators am Baum verifiziert, den dH-vs-|dB/dt|-Riss benannt, das Verdikt getragen
  (`state/stimmen/2026-10-15-netzfusion-verdikt.md`). Commit `e21129d5c`.
- **Nächster Messschritt an River übergeben** (Operator-Wort): Min gegen Union bei fester Größe/
  Schwelle/Fenster.
- folge355 geschrieben; folge354 → `archiv/`.
- Operator-Worte dieses Runds in `state/operator-gespraeche/2026-10-15-mycelium.md` geschnitten.

## Snapshot / Tools

- `git_safety --snapshot` nicht neu gesetzt dieses Rund (kein destruktiver Akt). Push steht
  (HEAD == `origin/main` == `e21129d5c`).

## session_burn

- `opencode.db` fehlt auf dieser Maschine → nicht gemessen (open 0.0 · close pending).
