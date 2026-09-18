<!--
  title: Survey — Verlorene/vergessene Diskussionen über beide Historien (Stand 2026-09-17)
  class: survey
  date: 2026-09-17
  sha256: 830d62253cb5f075b12dbc15f7feade06f137f913e97ee8fc247f93d7e3353c9
  status: live
  see-also: docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md docs/handover/post.md
-->
# Survey — Verlorene/vergessene Diskussionen über beide Historien (Stand 2026-09-17)

Anlass: Post-Zeile an die Entscheid-Linie (2026-09-17) — Archäologie über beide
Historien (aktuell + `archive-root/omegaflow-legacy`). Beleg-Fall TTL/φ-CDN-Gate:
ein Prinzip, dreifach verhandelt, lokal gebaut, CI-Instanz nie gebaut. Frage:
welche Diskussionen wurden verhandelt und nie gebaut, wo behauptet die Doku den
Baum, wo drifteten Namen/Ebenen — und weitere solcher lost-and-forgotten-Fälle.

Provenienz der Messung: `git -C /home/johannes/backup/archive-root/omegaflow-legacy
log --all -S/--grep`, `git grep`, `archive_search --root`, `sgrep`, `sread` über
beide Repos. Alle Belege Hash/Datei:Zeile. Wo unklar: als offen benannt, nie
spekuliert.

## Struktur-Befund

Das aktuelle Repo ist ein Fresh Start — `8399b401`/`52eca216` („no branch
legacy"). `d9d2c720` existiert dort nicht (`git log -1 d9d2c720` → unbekannter
Commit); die Entscheidungs-Historie lebt ausschließlich im Legacy-Repo
`/home/johannes/backup/archive-root/omegaflow-legacy`. Jede Archäologie über
Diskussionen läuft zwingend über das Legacy-Repo. Das Survey
`survey-2026-09-17-omegaflow-legacy-konzepte.md` deckt „verlorene Ideen" ab —
diese Karte ergänzt die Diskussions-/Entscheidungs-Nähte, ohne zu duplizieren.

## Korrekturen am Beleg-Fall (gemessen)

1. **Zeilenbereich Pfeiler:** §12 = 142–162, §13 = 164–181 („TTL < Datei" =
   174–177, „5-Minuten-Takt" = 155–158). Datei in beiden Repos identisch
   (181 Zeilen, §12/§13 wortgleich).
2. **„cache_fresh_cdn ehem. cdn_fresh" ist unzutreffend:** beide Funktionen
   koexistieren — `src/archivar/fetch.rs:899` `cdn_fresh` (CDN-Last-Modified-
   Schicht, `CI_REFRESH_S = 300` :900) und `:1059` `cache_fresh_cdn`
   (Cache-mtime + CDN-Stempel-Schicht, Tests :1247–1261). `cdn_fresh` lebte schon
   im Legacy (Einführung `001c08cc`, Operator-Wort „cdn_fresh bleibt Ernte-Uhr"
   `74f12f14`/`d91df8dc`). `cache_fresh_cdn` hat keine Legacy-Historie
   (`git log -S` → 0 Treffer), erster Register-Eintrag
   `docs/handover/archiv/handover-2026-09-13-ernte-folge12.md:71` — eine Funktion
   der Fresh-Start-Ära, keine Umbenennung.
3. **`d9d2c720` nur im Legacy** (2026-08-21, Fetch-Sturm-Reparatur,
   `src/archivar.rs` +774/−142, In-Flight-Guard + 2ⁿ-Void-Backoff; Bestätigung
   `legacy docs/surveys/survey-2026-08-28-nadel3-corona.md:701`).
4. **Survey-Pfad:** `survey-2026-08-19-landschaft.md` liegt nur im Legacy
   (`docs/surveys/`; Kopie in `vanilla-dateidocs/`); Zeilen 81–83 korrekt
   („healthcheck 3-stündlich … 5-min-Takt müsste im sources-Repo leben —
   unverifiziert, AGENTS unangetastet").
5. **„Wortgleich eingewandert" bestätigt:** Legacy `AGENTS.md:122` == aktuell
   `docs/concepts/archivar-mathematikerin.md:22` (identischer Satz „The CI
   Archivar runs every 5 minutes…"); im aktuellen AGENTS.md steht der Satz nicht
   mehr, im Concept-Doc lebt er unverändert.
6. **„Nie gebaut" heute gemessen:** kein 5-min-Cron in `.github/workflows/` —
   `health-check.yml:10` `'0 */3 * * *'`, `kernel-flatten.yml:10`
   `'17 5 1 * *'` (monatlich), sonst stündlich/täglich. Der lokale Client
   unterstellt den 5-min-Takt hart (`fetch.rs:910` `ttl.max(CI_REFRESH_S)`):
   Quellen mit ttl < 300 s fallen immer auf die Live-API zurück. Die eine offene
   Messung: das sources-Repo (refresh.yml/I02) ist lokal nicht geklont.

## (a) entschieden und gebaut

| Eintrag | Beleg | Nächste Entscheidung |
|---|---|---|
| TTL/φ-Gate lokal | `src/archivar/fetch.rs:313` `origin_stale`, Φ-Backoff :322, Tests `src/archivar/tests.rs:4550–4597`, Nutzung `main_flow.rs:1001` | keine — geschlossen; die CI-Seite fehlt (→ b1) |
| Fetch-Sturm-Reparatur | Legacy `d9d2c720`; Befund `legacy docs/TODO.md:3672–3678` | keine — erledigt |
| CDN-Frische via Last-Modified (kein API-Quota) | Legacy `001c08cc` → heute `fetch.rs:899–911` | keine |
| „cdn_fresh bleibt Ernte-Uhr" (Operator-Wort) | Legacy `74f12f14`, `d91df8dc`; heute unverändert `fetch.rs:899` | keine |
| Selbsterkennung CI/Lokal (dual mode) | Legacy `1916fbf1`→archiviert `b37d88ce`; heute `--ci-mode`-Compiler-Flotte + `health-check.yml:39` `--verify phi` | keine |
| Renames MEMBRANE_WGSL→FIELD_WGSL, Doppelname archivar, radiate→membrane, port→probe | Legacy `8b1c38b5`, `3d565949` | keine (abgeschlossen; letzte Doku-Spur → d5) |

## (b) verhandelt und nie gebaut

| Eintrag | Wo verhandelt | Warum nie gebaut | Lebt der Anspruch? |
|---|---|---|---|
| **CI-Instanz des TTL/φ-CDN (5-min-Takt)** | dreifach: `pfeiler-der-architektur.md:155–158`+`174–177` (beide Repos identisch); `archivar-mathematikerin.md:22` (= Legacy AGENTS:122 wortgleich); `sources-v2-spec.md:57` | als Lücke 2026-08-19 gemessen (`legacy survey-…-landschaft.md:81–83`), nie geschlossen, wortgleich migriert; kein CI-Bau | ja — in 3 Live-Doku-Stellen + hart im Client (`fetch.rs:900`) |
| Voyager Decimation>1 | 7× unverifiziert durchgereicht: `bau-folge49:81–82`, `51:93`, `52:112`, `53:112`, `54:93`, `55:73`, `56:50` | Schritt benannt, nie gewählt („Granulat wählen + Workflow erweitern") | **geschlossen mit Beleg (2026-09-18)** — im Baum verifiziert: `src/archivar/voyager_odr.rs:256` `decimation_ratio` + Tests `:517`/`:582`, `galileo_odr.rs:171` `year_full`; Beleg `bau-folge57:77` |
| Earthdata-Token-Hook live | `bau-folge25:35` „live unverifiziert"; `bau-folge26:36–40`; `SECURITY.md:13–14`; `tools/utils/…/token.rs:43–44` | gebaut, nie gegen den Dienst gefahren (Musterklasse: `ernte-folge3:41`) | **geschlossen mit Beleg (2026-09-18)** — gebaut UND gefahren: `token.rs:24` 401/403/307-Auslöser, `:40` `EARTHDATA_EDL_TOKEN`-Direktpfad; live 2026-09-18 `archive_search --sniff` der geschützten PODAAC-GRACE-URL → 401 ohne Token, 200 mit Token, 84 701 036 B, sha256 `8bd14764105360e06b35c0ab35312def4869d402fca8a834249fa5b92f1cfec0` |
| gpu-feature-gate | Legacy `8b1c38b5` „als eigenes Atom registriert"; heute 0 Treffer im Baum | nie unter dem Namen gebaut; ersetzt durch „GPU is the membrane" (`archivar-mathematikerin.md:36`) | nein — als descoped mit diesem Beleg zu schließen |
| Permeability-Radiation-Bindung | `AGENTS.md:116` „pending (the TE machine lives; the binding awaits its own atom)"; `survey-…-legacy-konzepte.md:44` (Channel Apertures) | wartet auf eigenes Atom (Atom 9 strahlt Rohfeld) | ja — pending |
| HRV/ESP32-Puls-Bindung | `AGENTS.md:79` (RMSSD/tone-Gate steht in `src/archivar/hrv.rs`; Bindung pending) | — | ja — pending |
| Atom D (phase/presence-Konsum) | `docs/specs/spectral-oscillator.md:221–228` „Atom D is unbuilt"; `binary-protocol.md:173` | Slots fahren seit v9 mit, nichts liest sie | ja — benannt ungebaut |
| field absorption | `docs/specs/force-system.md:96` „field absorption is pending" | — | ja — pending |
| row-parallel TE-Re-Shape / WGSL-FFT | `AGENTS.md:116` „Open: … the named alternative" | — | ja — benannte Alternative |

## (c) Doku-Behauptung ≠ Baum

| Doku sagt | Baum zeigt | Beleg | Nächste Entscheidung |
|---|---|---|---|
| „CI Archivar runs every 5 minutes" | kein 5-min-Cron; health-check 3 h, kernel-flatten monatlich | `archivar-mathematikerin.md:22` + `pfeiler:155–158` vs `health-check.yml:10`, `kernel-flatten.yml:10` | bauen oder die 3 Doku-Stellen + `CI_REFRESH_S` auf die gemessene Wahrheit korrigieren; vorher: sources-Repo messen |
| „The biotic force went from Council deliberation to compiled code in one pass" (Präsens-Erzählung) | `force_id_of("biotic") == None` (`force.rs:243`, `tests.rs:6302`); 9. Kraft = electric | `docs/concepts/kybernaut-native-methodology.md:23` vs `force.rs:13` | Zeile als Legacy-Ära-Story kennzeichnen oder korrigieren |
| remove-bias.md-Plan referenziert `warm_cache` | Symbol existiert nicht mehr (im Legacy gebaut: `4e54f1a6`; heute `cache_fresh_at`/`cache_path_for`) | `remove-bias.md:951,960`; Header ohne date/status (:1–5) | Plan archivieren oder offene Punkte extrahieren |
| Kernel-Plan „after the v6 protocol and the Trommelfell (packets 2/3)" | Protokoll heute v9; die Automation selbst ist gebaut (`kernel-flatten.yml`, `KERNEL_INDEX.md:75`) | `kernel-curation-ci-automation-plan.md:8,48` | Versionszitat als historisch markieren oder archivieren |
| „The only Python left in CI is the sources-repo catalog mirror (I02)" | von diesem Baum aus nicht messbar; Legacy-TODO:3934 „Abschaltung nach Verifikation" — Verifikation unbelegt | `archivar-mathematikerin.md:22` (= Legacy AGENTS:122 wortgleich) vs `legacy TODO.md:3932–3939` | offen — sources-Repo prüfen |
| *(Musterbeleg, repariert)* README/BINARY_PROTOCOL v6 vs Code v7, Geisterreferenz ALIGNMENT_PROTOCOL | 2026-08-19 gemessen und berichtigt | `legacy survey-…-landschaft.md:64–83` | Beleg: die Klasse ist bekannt und reparierbar |

## (d) Namens-/Ebenen-Drift

| Eintrag | Beleg |
|---|---|
| `cdn_fresh`/`cache_fresh_cdn` sind zwei Schichten, keine Umbenennung (Beleg-Fall-Korrektur) | `fetch.rs:899` vs `:1059`; `-S cache_fresh_cdn` in Legacy: 0 Treffer |
| `biotic` → `electric` (Umbenennung mit Doku-Rest) | Legacy `d0e9b4cc`, `86e451e8`; Rest: `methodology.md:23` |
| `sources_biosphere.φ` → `sources_live.φ` → `sources.φ` (drei Registernamen, eine Quelle; biosphere-Felder heute `biosphere_fire_*` unter `thermal`) | Legacy `31e2b536`, `e208d6bb`; aktuell `phi/sources.φ:985–1006` |
| **Atom-Nummerierung ohne Index:** parallele Serien — Atom 7/8/9/10/11/53 (AGENTS), Atom A–D (spektral), Atom A/B/C (tiefenphasen-flotte `handover-…-tiefenphasen-flotte.md:18–30`), Atom B (nobel-dag, pcmci), TE-Atome 1–4, K01/K03/K05 (Legacy-Atome + Flatten-Policy `ephemeris_compiler.rs:1510`), Q01 (`eraen.md:46,88`); Legacy „Code-Einheiten A–C" (`TODO.md:3927`) ≠ heutige Atom A/B/C | gleiche Symbole, verschiedene Referenten |
| Protokoll-Ebenen v6 → v7 → v8 → v9 nebeneinander ohne Zeitmarken | `kernel-plan.md:8` (v6), `spectral-oscillator.md:119` (v8), `:133` (v9), `:249` („v7 holdings"), aktuell v9 (`archivar-mathematikerin.md:20`) |
| Abgeschlossene Renames (historisches Muster): warm_cache→cache_fresh_at, radiate→membrane, port→probe, MEMBRANE_WGSL→FIELD_WGSL | Legacy `7d33f83c`, `3d565949`, `8b1c38b5`; letzte Doku-Spur: `remove-bias.md:960` |

## Weitere Kandidaten

- **Musterklasse benannt:** `ernte-folge3:41` — „unverifiziert — gebaut nach
  Vertrag, nie gegen den Dienst gefahren". Der Beleg-Fall ist kein Einzelfall,
  sondern eine bekannte Klasse (b3, b6, c4/c5 sind Instanzen desselben Musters).
- **Gegenprobe (nicht verloren):** Ein-Blatt + Nadeln I–XIII aus dem Legacy-TODO
  („offen") sind heute gebaute Probes (`tools/measure/…`, `docs/blatt/`) — die
  Migration war an dieser Front gründlich (bestätigt vom Konzepte-Survey: 54/55
  Konzeptdateien haben Gegenstücke).
- **Korrekt geschlossen (kein Eintrag nötig):** cone mode + browser-texture-Pfad —
  descoped mit Befund (`AGENTS.md:54`, `archivar-mathematikerin.md:20,35`) —
  Musterbeleg dafür, dass „descoped mit Messung" nicht verloren heißt.
- **Überschneidung mit dem Konzepte-Survey**
  (`survey-2026-09-17-omegaflow-legacy-konzepte.md:44`): Channel
  Apertures/Radiation-Bindung — dort unter „entblockbar", hier unter (b); die
  Karte verlinkt, statt zu duplizieren.

## Die eine offene Messung vor den Entscheidungen

Das sources-Repo (`omegaflow/sources`, I02/refresh.yml) ist lokal nicht geklont —
dort entscheidet sich, ob der 5-min-Takt irgendwo lebt und ob die I02-Python-
Behauptung noch stimmt. Erst diese Messung macht (b1) und (c5) entscheidbar:
bauen oder Doku auf die Wahrheit korrigieren.
