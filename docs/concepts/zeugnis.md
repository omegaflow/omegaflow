<!--
  title: ZEUGNIS — die Identitäts-Röhre: die Zeugin und ihre Zeugen
  class: concept
  date: 2026-09-06
  sha256: 6724d76986aeaec490859ba83bf871a9b9a2c6ba0fe641d108fb4f5699c9ad88
  status: live
  see-also: docs/concepts/kybernetische-astrophysik.md docs/concepts/remove-bias.md docs/concepts/archivar-mathematikerin.md phi/pipeline/catalog/vizier_gold_catalogs.φ phi/pipeline/ledger.φ docs/handover/archiv/handover-2026-09-06-s2-scanner-nadel.md
-->
# ZEUGNIS — die Identitäts-Röhre: die Zeugin und ihre Zeugen

Selbsttragend. Dieses Konzept trägt die Mess-Anordnung des Crossmatch-
Zeugnisses — vom Rat gehalten (2026-09-06, fünf Stimmen + Synthese; der
Name wurde zweimal vor den Rat getragen und endete beim Wort des
Operators), vom Operator beauftragt mit "lege alles rein, keine Grenzen,
wir schreiben Geschichte" und mit der Namens-Haltung: wie Archivar und
Mathematikerin — der Name ist das Handwerk, kein Marketing, kein Schrei.
Die Anordnung heißt **die Zeugin**: sie hört die unabhängigen Zeugen und
legt das eine Zeugnis ab — Placed, Absent, DirectionOnly — die mechanische
Ausführung von A = A über die Sinne. "Absent" ist ein Zeugnis, keine
Vision. Sie ist keine Biologie und kein Jäger; sie ist eine Kybernautin,
ein Geschenk an das Universum. Keine Register-Schreibarbeit. Der Handover
`docs/handover/archiv/handover-2026-09-06-s2-scanner-nadel.md` (Commits `40ed8bb`,
`0ea40ab`, `d830734`) ist in diesen Entwurf eingeflossen: ein großer Teil
dessen, was dieses Konzept skizziert, ist **gebaut** — der S²-Richtungssinn,
der TE-Screen, der Distanz-Anker, die Deredden-Baseline. Die Tabelle in
Abschnitt 12 benennt die gebauten Entsprechungen. Die Disziplin-Änderung
des Handovers (Abschnitt F) ist bindend: Pendings und Deferrals sind
verboten; ein fehlender Wert ist `absent` — ein vollständiger Befund.
Abschnitt 14 folgt ihr.

## 1. Das Material — das Katalog-Universum

`phi/pipeline/catalog/vizier_gold_catalogs.φ` inventarisiert ~731
Stichproben-Tafeln aus ~20 000 VizieR-Tafeln, klassiert nach der
physikalischen Gliederung:

| Klasse | trägt | Familien im Bestand |
|---|---|---|
| **B/** | Doppelsterne, Pulsare, Kataklysmische, kühle Zwerge | `B/cb/cbdata`, `B/psr/psr`, `B/sb9/main`, `B/wd/catalog`, `B/vsx/vsx`, `B/gcvs`, `B/sn/sncat` |
| **I/** | Astrometrie, Parallax, Eigenbewegung | `I/239/hip_main`, `I/345/gaia2`, `I/350/gaiaedr3`, `I/355/paramp`, `I/197A/tic` |
| **II/** | Photometrie optisch→IR, Durchmusterungen | `II/328/allwise`, `II/365/catwise`, `II/281/2mass6x`, `II/349/ps1`, `II/357/des_dr1` |
| **III/** | Spektralklassen, Abundanzen | MK-Kataloge, kühle Sterne |

Distanz-Anker: Gaia-DR3-Parallax (`I/355/paramp`, CDN-Asset
`dr3_stars.bin`); der `tap_compiler --crossmatch`-Mechanismus hat 12
distanzlose Kataloge gegen die Parallaxe nachkompiliert (gaia_dist-Welle
2026-08-16). CDN-Zeit-Assets liegen bereit: `tess_lightcurves.bin`,
`ztf_lightcurves.bin`, `spectra.bin`, `cosmicflows_cf4.json` (vpec).

Vier lebende TAP-Legs, alle anonym HTTP 200 gemessen (2026-09-06, direkt +
Proton-Tunnel): **ALeRCE** (`tap.alerce.online/tap/sync`, Schema `alerce_tap`,
14,39 Mio Objekte — ein Transienten-Broker, kein Positionskatalog),
**SIMBAD** (`simbad/sim-tap/sync`, `public.basic` — Identität, otype,
Parallax, Redshift in einer Zeile), **Gaia-ARI** (`gaiadr3.gaia_source`),
**NED** (`NEDTAP.objdir` — extragalaktischer Redshift).

Darüber hinaus trägt der Pipeline-Ordner das **TAP-Universum als Inventar**
(`phi/pipeline/catalog/tap_index_*.φ`): HEASARC-Xamin (~600 Tafeln), MAST-CAOM,
ARI, CADC, Chandra, ESO, ExoArchive, GAVO, IRSA (~995 Tafeln) — jede mit
ihrem Schema dokumentiert. Die Zeugin fragt nicht vier Dienste; sie hat die
Adress-Karte des gesamten TAP-Universums im Bestand.

Und die pre-CDN-Queue (master.φ, ~700 Astro-Blöcke über 28 Hosts) ist nicht
verloren, sondern verstreut — die Bestandsaufnahme 2026-09-06
(`phi/pipeline/research/agent_output/pre_cdn_astro_recovery_2026-09-06.φ`)
hat drei Funde eingebracht, die die Zeugin direkt tragen:

- **Euclid Q1 lebt** unter `eas.esac.esa.int/tap-server/tap/sync` (gemessen
  200): Schema `q1` mit `q1.phz_catalogue` (photometrischer Redshift),
  `q1.mer_final_catalogue`, `q1.combined_spectra`, `q1.frame_catalog`. Der
  alte Block (easd.esac.esa.int, euclid.ero_source_catalog) war dns-tot —
  der Dienst war nie tot, nur der Pfad. Der photometrische-Redshift-Leg ist
  damit zurückgewonnen (als Distanz-Sinn dem §6-Zirkel gegenüber als
  benannte Abhängigkeit zu führen, nie als Einzel-Wert).
- **Gaia-ARI** wurde im Register als "decline kompilat" geführt — gemessen
  ist der ARI-TAP kein Kompilat, sondern der lebende Gaia-Spiegel
  (gaiadr3.gaia_source liefert source_id/ra/dec/parallax). Ein
  Re-Review-Kandidat: der Distanz-Anker der Zeugin läuft über genau diesen
  Pfad.
- **HEASARC-Xamin lebt** (91 Blöcke, xamin/vo/tap/sync 200), ist aber
  VOTable-only (json und csv werden abgelehnt) — er hängt am Parser-Gap,
  nicht am Tod. NOIRLab lebt über DataLab, MAST-Archiv lebt, OGLE
  (Mikrolinsen-Leg) und ExoFOP leben.

Die CDN-/Force-Gate-Verluste, die der Auftrag benennt, sind genau zwei:
der Exoplanet-Archive-Katalog (bewusst redundant zum integrierten Bestand)
und die IRSA-Feld-Redundanz (bewusst, AllWISE-CDN abgedeckt). Alles andere
war Pfad-Drift, nie Tod — alle vier Nachprüf-Posten sind verifiziert
(Befund 3 der Bestandsaufnahme: PLA lebt unter pla.esac.esa.int, CDMS-Host
lebt, SNC-Nachfolger astrocatalogs gemessen, SKAO lebt als SRCNet), und der
größte Posten (Euclid phz) ist zurückgewonnen.

Und der Vordergrund ist gebunden: die **Planck-Staubmaske**
(`phi/bindings/dust-maske.φ`, AV_RQ, R_V 3.1, Rats-Verdikt 2026-09-05) trägt
die gemessene Rötung des Vordergrunds. Jede Farb-Zeile der Zeugin wird gegen
diese Maske gehalten — die intrinsische Farbe am Punkt ist die gemessene
Farbe abzüglich der Vordergrund-Rötung; ein Objekt hinter Staub fällt erst
nach dem Abzug aus der Verteilung, wenn es wirklich fällt.

Zwei Befunde aus dem Staging-Bestand schärfen die Rolle der Offline-Schicht:
**SPHEREx `splices`** (irsa_gavo_b.φ) ist bereits eine fertige
Crossmatch-Kompilation (2MASS/WISE/IRAC/Gaia in einer Tafel) — ein Beispiel
dafür, dass die Zeugin fertige Kompilate als eigene Linie führen muss, nicht
als sechs Quellen; und die **VizieR-Bulk-Stage** (`vizier_bulk_a.φ`, 25
TAPVizieR-Queries: atnf-Pulsare, apogee, allwise, bzcat5-Blazare, carmenes,
cornish, cosmograil) zeigt den Kompilations-Pfad, auf dem die statischen
Tafeln in die position-indizierte Offline-Schicht wandern.

Und zwei Korrekturen gegen `docs/specs/domain-coverage.md` (gemessen
2026-09-06): **SIMBAD** steht dort als "not reachable" — er ist erreichbar
(HTTP 200, `sim-tap/sync`, anonym); **tess.mit.edu** bleibt not reachable,
aber die TESS-Lichtkurven liegen als CDN-Asset (`tess_lightcurves.bin`) vor.

## 2. Das Prinzip — nicht die Sphäre, die Röhre

Die klassische Crossmatch-Suche fragt: "welches Objekt liegt in diesem
Konus?" — und antwortet mit dem nächsten Nachbarn einer Tafel. Der erste
Erste-Entwurf fragte: "was messen alle unabhängigen Kataloge an diesem
Punkt?" — und schichtete eine Identitäts-Sphäre. Der Rat hat beide Formen
überholt: **Die Zeugin ist keine Sphäre, sie ist eine Röhre.** Ein Objekt ist
eine Kurve durch Zeit und Raum, keine Koordinate. Die Identität eines
Punktes ist seine Weltlinie — Position, Eigenbewegung, Photometrie-Serie,
Spektraltyp — und die Zeugin schichtet nicht ein Stillbild übereinander,
sondern zieht die Röhre durch die Zeit.

Das Messobjekt ist nicht "der Punkt ra/dec", sondern "die Weltlinie, die
durch den Punkt läuft". Ein Punkt, den nur eine Tafel sieht, ist nicht
identifiziert — er bleibt Richtungs-Atom, bis ein zweiter unabhängiger Sinn
antwortet. Ein Punkt, dessen zwei Zeugen dieselbe Weltlinie gemeinsam
treiben, ist bewiesen — nicht aus der Koordinate, sondern aus der Dynamik.

## 3. Die Zeit-Achse — die Weltlinie als Messobjekt

Der Bestand trägt die Zeit bereits, der erste Entwurf hat sie ausgesperrt:
`tess_lightcurves.bin`, `ztf_lightcurves.bin` sind Serien; ALeRCE
`lsst_forced_photometry` trägt `visit`/`detector`/`psfflux` je Besuch;
`magstat` trägt mean/median/sigma pro Band; SIMBAD `basic` trägt
`pmra`/`pmdec`. Die Zeugin liest:

- **Die Richtung UND die Bewegung der Richtung.** `pmra`/`pmdec` ist die
  Trajektorie, nicht die Position. Wer morgen misst, erbt nicht eine
  Koordinate, sondern wohin der Punkt heute wandert — sonst verliert er ihn
  zwischen zwei Katalogen. Die Eigenbewegung ist die Erbschaft der
  ungeborenen Session.
- **Die Lichtkurve als Identität.** Die Farb-Zeile (Bewegung C des ersten
  Entwurfs) ist schon eine bestehende Datentyp-Form: `SkyDirection.bands` /
  `SkyBandSeries` mit `tdb`- und `mag`-Samples — sie wartet nur darauf,
  gefüllt zu werden. Jede Tafel, die eine Serie trägt, füllt die Röhre mit
  Zeit, nicht nur mit Farbe.
- **Der Lichtkegel-Schnitt.** Die Architektur kennt Enclosure-Lemma und
  Signal-Kegel-Gate. Die Röhre wird im Lichtkegel geschnitten: zwei Zeugen
  treiben dieselbe Weltlinie nur dann, wenn ihre Serien innerhalb der
  Lichtlaufzeit-Wand übereinstimmen — die Rømer-Toleranz entscheidet, nicht
  die Euklid-Distanz.

Die Zeit-Routen liegen im Bestand: die anonyme IRSA-ZTF-
Lichtkurven-Schnittstelle (`nph_light_curves`, gemessen — grind_ztf_lichtkurven:
"die Register-Urteil IRSA-Auth declined galt dem Alert-Stream, NICHT der
öffentlichen DR-Lichtkurven-Schnittstelle"), die ALeRCE-magstats-Datenform
(pro Band magmean/median/max/min/sigma, gemessen 2026-08-18 an der
Vorgänger-REST; die TAP trägt dieselbe Form als `magstat`), der
2MASS-PSC-Testbestand (`phi/pipeline/catalog/twomass/test_psc`).

## 4. Die Transfer-Entropie als Identitäts-Beweis

Das ist die Stelle, an der die Anordnung über das bloße Mehr an Bändern
hinausgeht — die Einzigartigkeit, die kein anderes System auf der Erde
besitzt:

Zwei Katalog-Zeugen am selben Punkt, deren Lichtkurven Transfer-Entropie
zueinander tragen, treiben dieselbe Weltlinie. Die Maschine lebt bereits:
`topological_te_phase` (Takens-Embedding, dim 3, order 3), MI-Verzögerung,
Silverman-Bandbreite, Surrogat-Null (mean+2σ, 10 Phasen-randomisierte),
PE-Gate, und die Permeabilitäts-Parabel `target = inTE/(inTE + threshold + ε)`.
Die Parabel wird das Crossmatch-Instrument: **die Übereinstimmung der
Position ist Behauptung, die gemeinsame TE ist Beweis.** Keine
astrophysikalische Crossmatch-Maschine misst Identität über die Dynamik der
Zeugen. Die statische Positions-Übereinstimmung bleibt der Vor-Filter; die
TE entscheidet, ob zwei Antworten an einem Punkt dieselbe Weltlinie sind —
oder zwei verschiedene Objekte, die nur dieselbe Koordinate teilen (ein
Doppelstern-Paar, ein Vordergrund-Hintergrund-Zufall).

Die TE-Röhre ist zugleich der Multi-Quellen-Sinn: wo viele Zeugen am selben
Punkt TE zueinander tragen, ist die Identität nicht nur wahrscheinlich,
sondern kausal verbunden — gemessen, nicht behauptet.

## 5. Das Abstammungs-Feld — Redundanz wird gemessen, nicht angenommen

"Je mehr Tafeln übereinstimmen, desto stärker der Beweis" ist falsch, wo die
Tafeln verwandt sind. SIMBAD `plx_value` kommt aus Gaia. ALeRCE `xmatch`
und `gaiadr3_source` sind Gaia-Kopien. SDSS, PS1 und DES sind auf
gemeinsame Standards kalibriert. Fünf Tafeln, die dieselbe Parallaxe
zitieren, sind eine Messung mit fünf Wurzeln an demselben Baum — Redundanz,
nicht Unabhängigkeit.

Die Zeugin trägt ein **Abstammungs-Feld**: jede Tafel führt ihre Quelle.
Der Identitäts-Beweis zählt unabhängige Linien; Kopien zählen als eine.
Die Genealogie der Daten ist ein Feld erster Klasse, kein Kommentar. Ein
Artefakt, das alle fünf Gaia-Kopien gleichzeitig belügt, belügt nur eine
Linie — und die Zeugin weiß das, weil sie die Wurzeln gezählt hat.

Zur Genealogie gehört die **Semantik der Spalten** — gemessen, nicht
theoretisiert: SDSS `photoObj.z` ist keine Rotverschiebung, sie ist die
z-Band-Magnitude (`modelMag_z`; gemessen 2026-08-18:
`z == modelMag_z` je Zeile, `z = 14.45` physikalisch unmöglich als
Redshift). Eine Tafel, die ihre z-Spalte als Distanz liest, wäre
Fabrication. Das Abstammungs-Feld trägt daher nicht nur, welche Tafel aus
welcher Quelle schöpft, sondern was jede Spalte misst — die Genealogie
reicht bis in die Semantik der einzelnen Spalte. Der ehrliche SDSS-
Distanz-Pfad ist der specObj-Join (`p.objID = s.bestObjID`), nicht die
photoObj-z-Spalte.

## 6. Der Redshift-Zirkel — zwei Domänen, eine benannte Abhängigkeit

Redshift ist kein Distanz-Sinn in der Ein-Zahl-Form. Die Architektur
verrechnet heute `z * C_LIGHT / HUBBLE_H0` — eine lineare Näherung, die bei
kleinem z die Pekuliargeschwindigkeit verschluckt und bei großem z die
Kosmologie leugnet. Das Asset `cosmicflows_cf4.json` (vpec) liegt ungenutzt
vor. Die Zeugin führt den Fluss-Sinn in **zwei Domänen**:

- **Naher Fluss** (`z` klein): `vpec` aus Cosmicflows dominiert über den
  Hubble-Fluss. Die Distanz ist `vpec / H0` plus Hubble-Anteil — nie der
  lineare Zirkel.
- **Ferner Fluss** (`z` groß): kosmologische Distanz, nicht linear.

Und der dritte Distanz-Sinn des ersten Entwurfs — photometrische Distanz —
ist ein **Zirkel**: sie braucht die absolute Helligkeit, die braucht den
Typ, der braucht die Distanz. Die Zeugin löst den Zirkel nicht auf, sie
weist ihn aus: die Abhängigkeit bleibt als benannte Abhängigkeit im Befund,
`absent` als Wert — nie ein erfundener Einzel-Wert, nie ein `pending`
(Handover Abschnitt F). Eine Sphäre, die ehrlich sagt "dieser Punkt hat nur
einen Sinn", ist ein vollständiger Befund, keine Lücke.

## 7. Der Survey-Footprint — die zwei Arten des Schweigens

Ein Katalog, der den Punkt **nie beobachtet** hat (Survey-Footprint),
schweigt anders als einer, der ihn beobachtet hat und **nichts über der
Schwelle** fand. Der erste Entwurf verschmolz beides zu "ehrliche Leere" —
das ist eine Fabrication in Wartestellung.

Die Zeugin trägt den **Footprint als Feld jeder Tafel**: bevor gefragt wird,
was gemessen wurde, wird gefragt, was beobachtet wurde. `absent` (nie
beobachtet) und die ehrliche Leere (beobachtet, nichts über der Schwelle)
sind getrennte Worte. Nur das zweite Schweigen darf gegen eine Identität
zählen; das erste ist ein weißes Feld, das seine Session trägt.

## 8. Die probability bleibt ein Posterior

ALeRCE `probability` (class_id, probability, ranking) ist ein
ML-Posterior; SIMBAD `otype_txt` ist eine kuratierte ontologische Klasse.
Sie in einen "Identitäts-Kegel" zu schichten heißt, eine Verteilung als
Label zu führen. Die Zeugin trägt **getrennte Ebenen**: kuratierte Klasse und
ML-Verteilung. Die probability bleibt eine probability, nie ein Typ — A = A.
Der Typ entsteht nur aus kuratierter Ontologie (SIMBAD, III/-Spektralklassen,
B/-Kataloge); der Posterior ist der Begleiter, nicht das Urteil.

## 9. Die neun Sinne — der Multi-Messenger-Kegel

Der erste Entwurf blieb in einem Kanal gefangen: optisch×IR sind zwei
Farben desselben Boten. Die Architektur trägt neun `force_type`-Kanäle mit
`signal_reach` und `propagation_speed`. Die Zeugin öffnet die
anderen Boten:

- **Gravitationswellen-Skymaps** (der gravity-Kanal): ein Punkt, an dem das
  Licht schweigt und die GW-Skymap antwortet, ist ein Punkt, den nur die
  neun Sinne gemeinsam sehen.
- **Neutrino-Richtungen** (Eis-Detektoren): die Richtung als Zeuge.
- **Kosmische Strahlung**: die Richtung als Zeuge.
- **Der gravity-Katalog** — nicht nur Ereignis-Skymaps, sondern gemessene
  Gravitations-Werte im Katalog-Bestand: die INPOP25c-Asteroidenmassen
  (`phi/pipeline/catalog/asteroid_gm_inpop25c.φ`, Mariani+2025 — GM aus
  Bahndynamik realer Begleiter, Force-Gate: gravity) und das solare ΩG
  (`solar_omega_g.φ`). Der gravity-Sinn trägt also bereits eine
  Katalog-Harvest-Route durch die VizieR-Tafeln — derselbe Weg, den die
  Parallaxe als Distanz-Anker trägt.

Jeder Bote hat seine eigene Lichtkegel-Reichweite; die Röhre befragt jeden
Boten in seiner eigenen `signal_reach`. Wo der erste Entwurf optisch über IR
legte, liegt hier der Sprung zu getrennten Boten — der Force-Gate-Litmus:
eine Anordnung mit neun Sinnen, die nur einen öffnet, bleibt halb. Die
Zeugin öffnet alle neun.

## 10. Die volle S²-Kugel als EIN Bild

Statt punktweise Röhren zu ziehen, werden alle Tafeln als **überlagerte
Dichtefelder auf die ganze Himmelskugel** manifestiert. Die ω()-Schleife
trägt die Kugel als ein Feld; jede Richtung ist eine Abtastung des
Gesamtbilds. Die Röhre ist dann nicht eine Einzel-Abfrage, sondern ein
lokaler Schnitt durch ein globales Feld — und die Abweichung ist nicht
"dieser Punkt hat keinen Gegenpart", sondern "dieser Punkt weicht vom Feld
ab". Die Feld-Permeabilität als exponentielles Relaxieren
(`naturalLatencyTicks` als τ) entscheidet, wie schnell sich das Feld an
eine neue Tafel angleicht. Das Feld altert mit der Tafel, nie gegen sie.

**Dieser Abschnitt ist kein Skizzen-Versprechen — der S²-Sinn ist gebaut**
(Handover Abschnitt C, Commit `d830734`): `src/mathematikerin/s2.rs` +
`S2_WGSL` + `omega.rs` tragen Richtungs-Oszillatoren als zweiten Sinn,
bandbegrenzter Kugelflächen-Winkelkern Y_lm (`S2_LMAX 64`), eigener
WGSL-Pass (GPU↔CPU-Parität <2 %), Atem aus eigener Messreihe (tanh, eigenes
τ), Manifestation auf der Einheitskugel, S²→ℝ³-Rückkopplung
(`spatial_position`) — der 26×f64-Wire bleibt unberührt. Die Katalog-
Dichtefelder dieses Konzepts sind die nächste Füllung dieses gebauten
Sinns: jede Tafel wird als Feld auf die Kugel manifestiert, und die
Röhre wird ein lokaler Schnitt durch das Feld — dieselbe Anordnung, die
hier skizziert ist, auf dem Sinn, der sie bereits trägt.

## 11. Die Schwellen kommen aus dem Bestand, nicht aus dem Wunsch

- **Winkel-Toleranz** aus `sigma_arcsec` der Richtung und dem Fehler jeder
  Tafel — nicht eine feste Sphäre. Die Masche weitet sich dort, wo die
  Messung ungenau ist, und schließt sich dort, wo sie präzise ist.
  `sigma_arcsec` ist bereits Teil der Entität (`SkyDirection`, Handover
  Abschnitt C) — kein Zukunftsfeld, ein bestehendes Feld, das gefüllt wird.
- **Entfernungs-Bereiche** aus der Parallax-Reichweite (die c/π-Grenze der
  Tafel) und der Redshift-Domäne (§6) — nicht aus einem Rundungs-Wert.
- Ein unbekannter Schwellenwert ist nie ein erfundener Schnitt — und nach
  der Disziplin-Änderung (Handover Abschnitt F) auch kein `pending`: er
  ist `absent`, bis seine Messung gebaut ist. Abschnitt 14 folgt dem.

## 12. Name = Implementation — die Rollen und ihre gebauten Entsprechungen

Die drei Verdicts bleiben die bestehenden — `Placed`, `Absent`,
`DirectionOnly` — und wachsen um die **benannte Abhängigkeit**, nicht um
neue Namen für Altes. Die Trennung der Rollen bleibt:

- **Archivar** trägt Fetch und Cache der Tafeln und des Footprints — die
  statischen VizieR-Kompilate als CDN-Assets (die Offline-Schicht) und die
  TAP-Legs als Live-Schicht (die Abfragen). Gebaut: `SkyDirection`
  (`src/archivar/skydirection.rs` — ra/dec, `bands`, `distance`/`redshift`,
  `sigma_arcsec`, `unit_direction()`).
- **Mathematikerin** trägt die Weltlinien-Evaluation und das TE-Feld in der
  ω()-Schleife — die Röhre, die Parabel, die Kugel. Gebaut:
  `src/mathematikerin/s2.rs` + `S2_WGSL` + `omega.rs` (der S²-Sinn) und
  `te.rs` (die TE-Maschine).

Die gebauten Entsprechungen (Handover Abschnitt E):

| ZEUGNIS | gebaut als |
|---|---|
| §3 Lichtkurve als Identität | `SkyDirection.bands` (tdb/mag) |
| §4 TE als Beweis | `pair_te_screen` + `te.rs` |
| §10 S²-Kugel als ein Bild | `s2.rs` + `S2_WGSL` + `omega.rs` |
| §2/§12 Distanz-Anker | `direction_distance_join` |
| §3 Rømer-Toleranz | `tdb_coincidence_probe` |
| §1 Vordergrund-Rötung | `deredden_baseline_probe` + `bayestar2019.be19` |
| §9 IR-Zeuge | AllWISE-Witness (`nadel_gate.rs`) |

Der Punkt erhält eine Identität nur dort, wo ≥2 unabhängige Linien (§5) am
selben Ort in der Röhre konvergieren; die TE (§4) entscheidet, ob sie
dieselbe Weltlinie treiben. Die Abwesenheit eines Gegenparts ist eine
vollwertige Messung — in beiden Formen des Schweigens (§7) getrennt benannt.

## 13. Die Ethik — 0 honored, ohne Grenze

- Nie eine erfundene Distanz: `distance` nur, wo ein Sinn einen Wert
  liefert; `Option::None`, nie ein 0.0-Sentinel. Der Redshift-Zirkel bleibt
  benannt, nie aufgelöst.
- Kein Über-Greifen: der nächste identifizierte Gegenstand definiert den
  Ort, nie ein fernerer, der an ihm vorbei-reicht.
- Keine ID-Fehlkopplung: position-abgeleitet über die Winkel-Toleranz,
  nie Namens-Zusammenführung ohne Winkel-Beweis.
- Keine probability als Label: der Posterior bleibt Posterior.
- Kein Schweigen verschmolzen: `absent` und ehrliche Leere sind zwei Worte.
- Kein `pending`, kein Deferral (Handover Abschnitt F, bindend): ein
  fehlender Wert ist `absent` — ein vollständiger Befund. Der Mechanismus
  wird komplett gebaut, nie als "größere Stufe" vertagt.

## 14. Die Bau-Linie — alles wird gebaut, nichts wird vertagt

Die Disziplin-Änderung des Handovers (Abschnitt F) verbietet Pendings und
Deferrals. Was folgt, ist keine Liste offener Fragen — es ist die
Bau-Linie, jede Stufe benannt, jede komplett zu bauen:

1. Die vollständige Abbildung des ~20k-Tafel-Universums in den
   position-indizierten CDN-Bestand — der Gold-Katalog ist die Stichprobe,
   die Abbildung wird komplett gebaut.
2. Die Fußabdrücke (Survey-Footprints) der großen Durchmusterungen (PS1,
   DES, SDSS, 2MASS, AllWISE) — als eigene Assets, gebaut wie die
   Staubmaske gebaut ist.
3. Die GW-/Neutrino-/CR-Skymap-Routen als Zeugen der neun Sinne — die
   Routen werden gebaut; wo der Teilchen-Kanal in der Force-Registry noch
   keinen Namen trägt, ist der Wert `absent` bis der Name gebaut ist.
4. Der CDN-Manifestations-Weg des Röhren-Assets.

Zwischen den Stufen gibt es keine "benannt offen"-Wartezone. Wo eine Stufe
noch nicht gebaut ist, ist ihr Wert `absent` — und ein `absent` ist ein
vollständiger Befund, keine Lücke, keine Schuld.

## 15. Was die ZEUGNIS trägt

Nicht die Anzahl der Kataloge. Die Zeugin ist die Anordnung: eine
Röhre durch die Zeit statt einer Sphäre um den Punkt, neun Sinne statt
einem, Transfer-Entropie als Beweis statt Übereinstimmung als Behauptung,
die benannte Abhängigkeit statt der erfundenen Distanz, das Feld auf der
Kugel statt der Einzel-Abfrage, der Footprint vor der Messung. Kein
anderes System auf der Erde misst Identität über die Dynamik der Zeugen —
dieses kann es, weil die Maschine dafür schon lebt: der S²-Sinn atmet, der
TE-Screen misst, der Distanz-Anker steht. Was hier steht, ist keine
Vergrößerung des Bestehenden und kein Versprechen auf eine spätere Stufe.
Es ist die erste Mess-Anordnung, in der der Himmel nicht als Katalog
befragt wird, sondern als ein Feld mit neun Sinnen, das durch die Zeit
fließt — und jede Richtung, jede Weltlinie, jede ehrliche Leere ist eine
Antwort dieses Feldes.

So hat der Rat gehalten. So steht es hier — und was davon gebaut ist, ist
gebaut; was noch nicht gebaut ist, wird komplett gebaut, nie vertagt.
