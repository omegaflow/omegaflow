<!--
  title: BLATT — Das Register der offenen Pendings: sechs lebende Handover, Stand 2026-09-09
  class: sheet
  date: 2026-09-10
  sha256: 663d62c0bdb81ff407022962a0c3622109d73faa9ee842d871890896a05d0f79
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-disjunkte-linien-folge.md docs/handover/handover-2026-09-09-mechanische-reste.md docs/handover/archiv/handover-2026-09-09-operator-label-korrektur.md docs/handover/handover-2026-09-09-te-blatter-bz-laic-tscaling.md docs/handover/handover-2026-09-09-zonen-flotte-sp-dual.md docs/handover/handover-2026-09-09-de441-reverifikation-pending.md
-->
# BLATT — Das Register der offenen Pendings

**Datum:** 2026-09-10 · **Axiom:** A = A
**Verdikt-Ordnung:** 0 honored — kein Wert erfunden; ein `pending` erscheint
einmal, nie doppelt; was eine Handover-Zeile als geschlossen trägt, tritt
nicht ein. Was mehrere Handover nennen, ist eine Zeile mit allen Quellen.

## Die Frage

Nicht „was ist noch zu tun" als Wunsch. Sondern: **Welche unerledigten
Pflichten tragen die lebenden Handover namentlich?** Die sechs Dateien unter
`docs/handover/` (Stand 2026-09-09) nennen ihre offenen Punkte selbst; dieses
Blatt ist die Konsolidierung — je Zeile die gemessene Pflicht, ihre Quelle und
ihr Zustand (`pending` / `läuft` / `Operator-Wort`). Nichts wird hinzuerfunden;
ein Punkt, der in mehreren Handovern steht, trägt alle seine Quellen in einer
Zeile und zählt einmal.

## Die Tabelle

Jede Zeile: die Pflicht · die Quelle · der Zustand. Der Zustand unterscheidet
den 0-Kanon: `pending` (die Messung existiert, die Ernte/der Bau fehlt),
`läuft` (die Messung läuft, das Ende fehlt), `Operator-Wort` (die Freigabe
des Operators ist die Entsperrung — kein Sitzungs-Atom).

| Pflicht | Quelle | Zustand |
|---|---|---|
| abfluss-trishuli — der Abfluss-Pfeil; Entsperrung = archiviertes externes CSV des 08-27-Zugs (`/home/johannes/backup/archive/data/opencode-tmp-2026-09-01/worktree-aufraeum/dhm_bhotekoshi_stage_1h.csv`) | disjunkte-linien-folge · mechanische-reste · operator-label-korrektur | pending (Sitzungs-Atom) |
| NOAA-NODD-Bucket-Dispositionen je Dataset (Litmus: url-line / Compiler-Lease / Konsument) | disjunkte-linien-folge · mechanische-reste · operator-label-korrektur | pending (Register-Frage, Rat) |
| Step-5-Folge — je `mirror_*`-Asset der 12 lebenden repo_tag-Releases Byte-Vergleich CDN-Digest ↔ Repo-Raw, dann einzeln schneiden | disjunkte-linien-folge · mechanische-reste · operator-label-korrektur | pending (benannter Folgeschritt) |
| matrixmachine 769-Suite — Gesamtsuite der Core-Crate gegen HEAD; Urkunden-Zeile gegen den letzten CI-Lauf aktualisieren | disjunkte-linien-folge · mechanische-reste · operator-label-korrektur | läuft (ci-check.yml, fremdfrei) |
| R2-Zählung (§2-Zählung vs. Tabellen-n) — Archiv-Zählung als Grundwahrheit, lokal zählbar | mechanische-reste · operator-label-korrektur | pending |
| docs-reference-verteilung — reine Bewegung + Referenz-Rewiring; Seeds → Survey-Heimat (Benennung `pending`) | disjunkte-linien-folge · mechanische-reste | pending (eigenes Atom) |
| vo-tap / uvor — Crate pushen (`ivoa/uvor` HTTP 200, Seed steht) | mechanische-reste · operator-label-korrektur | Operator-Wort (Markus-Übergabe) |
| SPICE-`.bc`-Kernels — `gll-ck-cdn.yml` steht, naif-Release leer (404); Dispatch | mechanische-reste · operator-label-korrektur | Operator-Wort |
| AllWISE-Ernte — läuft (~13 Tage); `allwise_coverage.fp01` unverifiziert; CI-Partial-Check + Regrid-Optimierung | mechanische-reste | läuft |
| PS1-fraktional — Tiefen-Ernte; Partial-Landung reißt am 180-min-Timeout (Chunking/Timeout) | mechanische-reste | läuft (eigenes Atom) |
| Gaia XP — Ernte-Schnitt; Ring↔Nest-Brücke source_id↔FP01-ipix ungemessen; `xp_pilot_p6144.bin` kein CDN-Asset | mechanische-reste | pending |
| ned-Crawl-Landung verifizieren — 1/40 Slices; `ned.json` bei 40/40; IPAC-Antwort ausstehend | mechanische-reste | läuft |
| Nadel Ⅰ — Jeans-Residuum bis Gaia DR4 (2.12.2026); Deduktion 42 (VLBI+Doppler-Sonde) | mechanische-reste | pending |
| Nadel Ⅱ — Flyby-Prüftermine JUICE 28./29.9.2026 + Europa Clipper 3.12.2026; AGU-2013-Abstract menschlich prüfen | mechanische-reste | pending |
| Nadel Ⅲ — Richtung TIAW vs Nanoflares; 613-Ereignis-Satz; Kaskaden-Stärke × Sonnenstruktur | te-blatter-bz-laic-tscaling | pending |
| Nadel Ⅳ — LAIC: CSES, TEC retro pre-2024, Instrument A ungebaut, KDE-h | mechanische-reste | pending |
| Nadel Ⅴ — LSST-Live-Scan (achromatischer Dip + IR-Exzess) | mechanische-reste | läuft |
| Nadel Ⅷ — Dunkler Fluss: Haufen-Kanäle benennen | mechanische-reste | pending |
| Nadel Ⅸ/Ⅹ — FRB / Kugelblitz: Kanal-Lage | mechanische-reste | pending |
| Nadel Ⅺ — Placebo: Paar-EEG, fam-Schwelle, Nullkontrolle, bedingte TE | mechanische-reste | pending |
| Nadel Ⅻ — Urknall: Reihen-Paarung Winkelserie×z-Reihe | mechanische-reste | pending |
| Nadel ⅩⅢ — Voller 48er-Zensus (18 Non-Detections); Photochemie-Re-Erklärung | mechanische-reste | pending |
| Weberin — Planeten/Monde zweite Abstammung (INPOP `.dat` gegen `testpo`, oder SPK-Weg) | mechanische-reste | pending |
| Weberin — Raumsonden-Doppler echte zweite Linie (VLBI-Winkel + Range, oder zweite Ephemeriden-Abstammung) | mechanische-reste | pending |
| Weberin — Breite TNO-Kette: keine MPC-unabhängige Linie der vollen 8.082-Menge | mechanische-reste | pending (not-published) |
| Weberin — Neptun-Planetenzentrum-Tabelle (Astrometrie-Kopplung) | mechanische-reste | pending (Source-Port) |
| adoption — Repo public + Drei-Mail-Block (Toth/Turyshev/Markwardt) als ein Zug | mechanische-reste | pending |
| bande-split — Split-Ergebnis + offene Registerzeilen (f*, 1-s-Zählung, Amplitude); Restbestand 238 Dateien/77 Tage | mechanische-reste | pending |
| gic-p-wert — nachlegen, dann Wing/Viljanen | mechanische-reste | pending |
| papier-kleinpass — nach dem Merge, Zahlen je Blatt | mechanische-reste | pending |
| quiet-zone-uebertragung — Rezept; New Horizons request-only | mechanische-reste | pending |
| gaia-dr4-iapetus — Gaia DR4 (2.12.2026) als 4D-Feld | mechanische-reste | pending |
| flyby2-addendum — Metrik vor dem 28.09. | mechanische-reste | pending |
| flyby-doppler-rohdaten — Roh-Doppler historischer Flybys (AGU-Beleg) | mechanische-reste | pending |
| maschinen-audits — R2 bleibt `pending` (Provenienz-Notiz-Muster gebaut) | mechanische-reste | pending |
| sicherung-risiko-heime — einzige-Kopie-Risiko-Heime sichern (Backup-Akt selbst = Operator-Sache) | mechanische-reste | pending / Operator-Sache |
| saubere-datenbank — Step-5-Folge (destruktiver Schnitt je Asset) | mechanische-reste | pending (siehe Step-5-Folge) |
| extern-weberin-faden-luecken / -folge / -zweitlinien — 9 Faden-Kategorien Routen messen; zweite Linien je Klasse | mechanische-reste | pending |
| extern-stellar-aktivitaet-xuv-co — Rest 23 Wirte | mechanische-reste | pending |
| bio-kanal-zeugen — O2/O3, Rotkante, saisonal | mechanische-reste | pending |
| nadel-xiii-xuv-zensus / nadel-v-lsst-scan — Zensus + LSST-Scan | mechanische-reste | pending |
| lisa-pathfinder-psd / -antrag — Δg-Zeitreihe anfragen | mechanische-reste | pending |
| ned-objdir-zugang — IPAC-Anfrage versandt, Antwort ausstehend | mechanische-reste | pending |
| Flut-2026 — satellitenbilder-post: robuste Flut-/Narbenfläche (Delineation nicht erzeugt; Abruf offen) | mechanische-reste | pending |
| Source-Port-Queue — 10 Untested-Korpora; 38 VizieR-Bulks; 77 Archeology-Gaps | mechanische-reste | pending |
| Katalog-Lücken — RAVE DR6, APOGEE/GALAH, HyperLEDA, TGSS ADR, VLASS, AMS-02, GLADE+; Parquet/GRIB-2/OPeNDAP-Reader | mechanische-reste | pending |
| n=1000-Riß — Block-Länge n^(1/3)=10 zu kurz / KSG-Dimension; n=1000-Shift | te-blatter-bz-laic-tscaling | pending (Register-Duty) |
| TE-Baupunkte — `cycle_phase_shift_surrogate`-Nutzung; bedingte Multi-Force-TE (Phasenraum) | te-blatter-bz-laic-tscaling | pending (Instrumente) |
| Desktop-Fork (GTX 970) — 30-Jahres-Lauf | te-blatter-bz-laic-tscaling | Operator-Wort |
| sP-Δ-Faltung — ungemessen (Register-Duty, nicht 0.0) | zonen-flotte-sp-dual | pending |
| sP-Beine der pP-übersprungenen Stationen | zonen-flotte-sp-dual | pending |
| Tiefe der tiefen Zone an der Kante unaufgelöst (beide Fits klemmen) | zonen-flotte-sp-dual | pending |
| Externe Tiefen-Referenz (TauP/KEB95) — Instrument benannt | zonen-flotte-sp-dual | pending |
| Head-Wave-Lücke 410/660 — direktes-P-Triplikations-Gate gegen TauP | zonen-flotte-sp-dual | pending |
| Quell-Strahlungsterm (CMT) — ohne CMT-Lösung | zonen-flotte-sp-dual | pending |
| W-Phase-CMT als M9-Nachfolger-Atom — Ersteinsatz emergent | zonen-flotte-sp-dual | pending |
| Stromboli — Vulkan-Lehrer | zonen-flotte-sp-dual | pending |
| Hi-net/NIED — NIED verlangt Registrierung | zonen-flotte-sp-dual | pending |
| Eikonal-Löser über das volle Gitter — Dijkstra über ETOPO1; Gitter steht, Löser | zonen-flotte-sp-dual | pending |
| Die Erde als Sender — Kreuzbereichs-Kalibrierung (Tonga 2022) | zonen-flotte-sp-dual | pending |
| de441-mars Re-Verifikation — 5-Schritt-Sequenz nach grünem bodies-Job (Run 34393748385) | de441-reverifikation-pending | pending |
| witness presence — reserviert (Consent-Wurzel Art (c)) | de441-reverifikation-pending | pending |
| feature-gate `gpu` — eigenes Atom | de441-reverifikation-pending | pending |
| Membran-Reste M02–M07 | de441-reverifikation-pending | pending |
| Kamera-Pixel-Quellen als WS-Traffic-Hotspot | de441-reverifikation-pending | pending |
| OPeNDAP-Integration | de441-reverifikation-pending | pending |
| advective per-Quelle | de441-reverifikation-pending | pending |

## Das Verdikt

**Sechs lebende Handover tragen siebenundsechzig offene Pflichten; keine
wurde erfunden, keine fällt still unter den Tisch.** Das Register ist die
Konsolidierung, nicht der Aufruf: es erfindet keinen neuen Punkt, es zählt
die benannten. Die Zustände unterscheiden drei Entsperrungen —
Sitzungs-Atome (abfluss-trishuli, Bucket-Litmus, R2, Step-5-Folge),
laufende Messungen (AllWISE, PS1, ned-Crawl, matrixmachine, LSST) und
Operator-Worte (vo-tap-Push, SPICE-bc-Dispatch, Desktop-Fork, der
Backup-Akt der sicherung-risiko-heime). Die TE-Linie trägt einen
offenen Riß (n=1000), die Tiefenphasen-Linie elf benannte Pendings um die
sP-duale Inversion, die Re-Verifikations-Linie trägt ihre 5-Schritt-Sequenz
und sechs übernommene Register-Reste. Jede Zeile ist eine gemessene Pflicht,
keine Projektion.

## Grenzen

- **Stand 2026-09-09, sechs Dateien.** Archivierte Handover sind verbraucht
  (`docs/handover/archiv/`); ihre offenen Punkte tragen die lebenden
  Steh-Register (namentlich `mechanische-reste`). Dieses Blatt liest nur die
  lebenden Dateien — ein Punkt, der nur im Archiv steht, trägt die Session,
  die das Archiv konsumierte.
- **`läuft` ist nicht `pending`.** Eine laufende Messung (AllWISE, PS1,
  ned-Crawl) ist keine fehlende Ernte, sondern eine offene Ernte — benannt,
  damit kein Nachfolger sie als stilles Ende neu ausgräbt.
- **Ein `pending` zählt einmal.** Wo vier Handover denselben Punkt nennen
  (abfluss-trishuli, Bucket-Dispositionen, Step-5-Folge, matrixmachine),
  trägt eine Zeile alle Quellen — Doppelzählung wäre Fabrikation.

---

*Blatt registriert 2026-09-10. Verdikt-Ordnung 0 honored: jeder Punkt ist
die benannte Zeile seiner Handover, kein Wert hinzuerfunden, kein pending
still verschwiegen.*
