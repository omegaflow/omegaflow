<!--
  title: Standpass — öffentliche Projektion
  class: zustand
  date: 2026-10-15
  sha256: b9e7d62bfe73f523352095fe954447c3209ea1b254129ed3c2de3c571afeaab1
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
| HEAD | `5923562d2` | 2026-10-15 |
| rote CI-Läufe | **283** non-success — Träger: `ci-gate` (173) · `register-coverage` (72) · `tools-build` (23) · `gic-al-network` (4) · `quake-feeds-cdn` (3) · `m10-mdis-raw-cdn` (2) · `themis_asi-fodg-cdn` · `ssd-jpl-raw-cdn` · `ci-check` · `zenodo-raw-cdn` · `health-check` · `venus-radar-cdn` | 2026-10-15 |

**Der Tauchauftrag der acht Fäden ist gelaufen.** Acht `taucher` (ein Faden je Dispatch, harte
Bandagen) haben die acht offenen Fäden gemessen; die vollständigen Befunde stehen in
`docs/surveys/survey-2026-10-15-tauchbefunde-acht-faeden.md`. Die Kern-Korrektur ist bestätigt:
**Block ≠ Kanal** — für **HMI**, **XUV/log R'HK**, **Shear**, **Neutrino** und **PTA** steht der
Quellen-Block schon, es fehlt die **Kanal-Zeile** (`field`); für **SILSO**, **EDGES/21 cm**, **TGF**
und **Wolkenbedeckung** fehlt die Quelle. Nur **WGLC** fehlt als ganzer Block (die `field`-Form ist
die Rat-Schemafrage).

**Zwei Risse stehen ungeglättet.** (a) **SILSO ist keine Lücke, sondern ein Verwurf** — das
Git-Verdikt `decline unphysical` (`e988e464d`), `docs/specs/sunspots.md` und `src/archivar/port.rs:2048`
verwerfen die Sonnenfleckenzahl; die Weltmessung findet die Quelle dagegen öffentlich. (b) **Der
Neutrino-Kanal ist doppelt leer** — vier Record-Compiler (`auger`/`icecat`/`antares_vo`/`amon`)
verlangen die Zeugen-Art `S2Direction`, `src/archivar/witness.rs` bildet dieselben MAGICs aber auf
`PointEvent` → sie brechen in Zeile 1 ab; das erklärt die 404 der Neutrino-/CR-Assets zwanglos.

**Venus-Bildträger + externe Quellen unverändert.** Drei COG-Träger sind registriert und laufen über
die `zenodo-raw`-Matrix auf das CDN (Venera-Polar 194 MB, Provenienz-Raster 89 MB, Arecibo 1988
965 MB). GEOFON steht im Register; SEANOE ist mit `4f2fe9ff7` als Duplikat von `argo_gc` überholt.

**Noch unread:** die tiefen Log-Gründe der fremden roten Läufe (kein Polling); die eigenen
Push-Läufe am neuen HEAD sind queued. Die Logs liest `ci_manage log <id>`.
