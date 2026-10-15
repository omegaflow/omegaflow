<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: 43a5819e20840b380943e7b88803ec8ebd6a2c1d1746cb773930ec3c3e5fd72d
  status: live
  see-also: AGENTS.md
-->
# Standpass — öffentliche Projektion (privat↔öffentlich, A)

Die öffentliche Fassung des Stehenden Passes (Operator-Wort A, 2026-10-14). Sie trägt
**nur, was öffentlich ist** — HEAD und rote CI-Läufe. Mailstand, Operator-Akte und
Personenbezug bleiben im privaten Haus (`state/zustand/standing-pass.md`, Linie Mycelium).
Die öffentlichen Linien zitieren **diese** Fassung, nie die private.

| Feld | Wert | gemessen |
|---|---|---|
| HEAD | `e21129d5c` | 2026-10-15 |
| rote CI-Läufe | `ci-gate` `38712802275` (**failure** @ `fadbd489b`) · `ci-gate` `38712777023` (**failure** @ `bdb627ab`) · `tools-build` `38712667864` (**failure** @ `03178cb3f`) · `m10-mdis-raw-cdn` `38712655060`/`38712632761` (**failure** @ `03178cb3f` — Arm `lpsc2024_2663.pdf`, `raw_mirror: the CDN upload returned void`) · `gic-al-network` `38709132540` (**failure** @ `a6d6fca0`) · `register-coverage` `38718203450` (**failure** @ `b0bc4d80`) | 2026-10-15 |

**Gemessener Kern-Befund (nicht geraten):** Die Frontier-Runde „Netz-Fusion und
Stationsplatzierung" ist am Baum verifiziert. Die Fusion ist die **Min-Einhüllende** je
Zeitstempel → ein Schwellenübertritt ist eine **Konjunktion**; der monotone Rückruf-Fall ist
strukturell erzwungen. Bei **fester** Schwelle bleibt der Kollaps (gemessen) — die Doktrin ist
geschlagen. **Riss, den die Stimmen nicht sahen:** der Netz-Arm faltet die **dH**-Hülle
(Minutenraster), der Einzelstations-Arm ist der **stündliche |dB/dt|-Peak** — der zentrale
Vergleich ist nicht A = A. Der nächste Messschritt (Min gegen Union bei fester Größe/Schwelle/
Fenster) ist an die GIC-Linie (River) übergeben.

**Noch unread:** die Push-Läufe an `e21129d5c` (`ci-gate`, `harvest-dispatch`,
`register-coverage`, `sources-repo-licence`, `tools-build`) — queued.

Die Tafel ist ein Flutstand aus den parallel laufenden Linien; die Logs liest `ci_manage log <id>`.
