<!--
  title: Auftrag — Vorfilter Tür 2 + Tür 4: Stabilisierung + Rauschgeometrie je Sonde
  class: auftrag
  date: 2026-09-04
  sha256: a63b442194b693cb0c2f1489535662e4c09c5328d90e4d0144e6a8f807076d7c
  status: geschlossen
  see-also: docs/auftrag/auftrag-quiet-zone-uebertragung.md docs/befund/befund-voyager-roh-doppler-zugang.md docs/TODO.md
-->

# Auftrag: Vorfilter Tür 2 + Tür 4 — Stabilisierung + Rauschgeometrie je Sonde

## Zweck

Tür 1 (Voyager) ist geschlossen (2026-09-04): kein offener Cruise-Doppler,
und die kausale Grenze ist das dreiachsen-stabilisierte Selbst-Rauschen
(~10⁻⁶ cm/s², ~10× über a_P). Das Quiet-Zone-Rezept trennt Selbst-Rauschen
nicht — es braucht eine **Distanz-Geometrie** der Störung (medium-getrieben).
Bevor Tür 2 (New Horizons) und Tür 4 (Mariner/Galileo/Cassini) geerntet
werden, läuft der Vorfilter: Stabilisierungs-Schema und dominante
Rauschquelle je Sonde messen. Eine Tür, deren Rauschen selbst-getrieben und
über der Anomalie liegt, fällt ohne Harvest — das Voyager-Urteil, nicht
wiederholen.

## Vorfilter-Kriterien (aus der Tür-1-Lehre, je Sonde zu messen)

1. **Stabilisierungs-Schema:** spinstabilisiert (passiv, Pioneer-artig)
   oder dreiachsenstabilisiert (aktive Düsen → Selbst-Rauschen-Kandidat)?
2. **Dominante Rauschquelle** des Radio-Trackings im relevanten
   Distanz-Bereich: medium-getrieben (Plasma → Distanz-Geometrie, die
   Zonen-Teilung greift) oder selbst-getrieben (Raumschiff-Dynamik,
   Zonen-Teilung blind)?
3. **Größenordnung:** liegt das Selbst-Rauschen über der Anomalie
   (a_P ≈ 10⁻⁷ cm/s²) → die Tür fällt.

## Zu prüfende Sonden (Fragen offen, Antworten zu messen)

- **Tür 2 — New Horizons** (~60 AU): Welches Stabilisierungs-Schema? Wie
  ist die aktive Kette beschaffen — dominiert nach der Plasmazone
  medium- oder selbst-getriebenes Rauschen? Existiert ein öffentlicher
  Doppler-/Tracking-Datensatz (PDS-RSS, REX, DSN-Tracking; vgl. das
  Tür-1-NAVIO-Vorbild)?
- **Tür 4 — Mariner/Galileo/Cassini** (retroaktiv): je Sonde das
  Stabilisierungs-Schema (welche spinstabilisiert, welche dreiachsen?)
  und die Band-Charakteristik (S-Band plasma-getrieben?). Wo liegen die
  Roh-Doppler-Bestände (PDS-RSS, SPDF, Turyshev-Depot-Analoga)? Cassini
  (Ka-Band sauber) als Kontrast einordnen, nicht als Hauptkandidat.
- Welche dieser Sonden hat überhaupt **das Distanz-Muster** geflogen
  (jahrelang weit draußen, Rauschen mit Abstandsgang)?

## Disziplin

Messen, nicht spekulieren: je Sonde die Antwort mit Beleg (Spezifikation,
Literatur-/Archiv-URL, PDS/SPDF-Bestandsliste mit HTTP-Status). Was nicht
gemessen ist, trägt `pending` — keine Vermutung. Ausgang je Sonde:
`bestehen` (Rezept anwendbar, Datenzugang lohnt) / `fallen`
(Selbst-Rauschen über der Anomalie, Voyager-artig) / `pending`.

## Rückgabe

Tabelle je Sonde: Stabilisierungs-Schema, dominante Rauschquelle,
öffentlicher Datenbestand (`open`/`request-only`/`unavailable`),
Vorfilter-Verdikt. Plus die eine Entscheidung: welche Tür (wenn überhaupt)
trägt den nächsten Harvest.

## Befund (2026-09-04, grind-flash agent, gemessen)

**Verdikt-Tabelle:**

| Sonde | Stabilisierung | Rauschquelle | Datenbestand | Vorfilter |
|---|---|---|---|---|
| New Horizons (~60 AU) | kombiniert: Cruise/Hibernation **spinstabilisiert** (passiv, keine Reaktionsräder, ~100 d/a ohne Lage-Manöver), Encounter dreiachsen | selbst-getrieben **episodisch**, nicht kontinuierlich — Voyager-Regel greift nicht; Größe vs a_P nicht publiziert (`pending`) | REX `open` (Okkultation + TNF, kein Empfangs-Doppler, kein NAVIO-Analogon); SPDF 404; Nav-Doppler `request-only` | **besteht** (Schranke), Harvest nicht offen |
| Galileo | **Dual-Spin** (Spun-Section 3 rpm, passiv) | kein kontinuierlicher 3-Achsen-Betrieb; Größe `pending` | **`open`** (PDS3 PPI): `GO-…-RSS-…-V1.0` (TRK-2-25 TDF + TRK-2-18 ODF, TRK-2-34 absent) + GWE `GO-X-RSS-1-ODR-V1.0` (open-loop ODR) — kein PDS4 `gll.rss` (korrigiert 2026-09-05, `befund-galileo-gwe-bestand.md`) | **bestehen** — passive Stabilisierung, medium-getriebenes S-Band-Rauschen; aber ≤5 AU (keine Quiet-Zone) |
| Cassini | dreiachsen (Mariner-Mark-II-Bus) | Voyager-artige Selbst-Rauschen-Klasse | `open`: PDS4 `cassini-rss-raw-*` (gwe/sce/sagr/sroc/ssa/Titan, je DOI) | **fallen** als Hauptkandidat (3-Achsen → Voyager-Urteil); bleibt Kontrast (Ka ohnehin sauber) |
| Mariner (10 gemessen) | M10 dreiachsen; 4/5/6/7/9 einzeln `pending` | — | kein PDS-RSS-Bestand, `unavailable` | **fallen** — kein Distanz-Muster (Ziele alle ≤1,7 AU) |

**Entscheidung:** New Horizons trägt den nächsten Harvest — die einzige Tür,
die die Stabilisierungs-Schranke besteht, das **>50-AU-Muster jetzt bewohnt**
(~60 AU, seit ~2021) und eine moderne Kette hat. Der Harvest ist aber
**kein offener Download** (gemessen: SPDF 404, REX nur Okkultation/TNF):
das nächste Atom ist die `request-only`-Doppler-Anfrage (JPL/DSN-ODF für
NH-Tracking-Pässe). Reserve: Galileo-GWE (`open`, `bestehen`) — aber andere
Geometrie (Plasma-Zone ≤5 AU, kein Quiet-Zone-Nachbau). Cassini und Mariner
fallen als Harvest-Türen.

**`pending`-Reste:** NH-Selbst-Rauschen-Größe vs a_P nicht publiziert;
NH-JPL/DSN-Doppler-Anfrage benannt, nicht ausgeführt; NH-REX-Tiefenprüfung
(geschlossene Doppler-Spur in raw/tnf?) offen; Galileo-Bestandsaufnahme +
Vorfilter der eigenen Rausch-Kurve gemessen 2026-09-05
(`befund-galileo-gwe-bestand.md` — `gll.rss` existiert nicht, real =
PDS3 `GO-…-RSS-…-V1.0`); Galileo-empirische-Rausch-Kurve offen;
Mariner-Einzelschemata tür-irrelevant.

## Register-Satz

*Tür 1 hat den Preis des Rezepts gemessen: Selbst-Rauschen hat keine
Geometrie. Bevor die nächste Sonde geerntet wird, wird ihr Rauschen
verortet — die Diagnose geht dem Harvest voraus, nicht nach.*

## Status

`geschlossen`. Vorfilter ausgeführt (2026-09-04). New Horizons `besteht`
(Schranke + >50-AU-Muster), Harvest-Weg = `request-only`-Doppler-Anfrage;
Galileo-GWE = offene Reserve; Cassini + Mariner `fallen`. Drei
`pending`-Reste (oben benannt) sind nächste Schritte, keine Vermutungen.
**Nachtrag (2026-09-05):** die Galileo-Zelle trug eine falsche Adresse — ein
PDS4-Bündel `gll.rss` existiert nicht; der reale Bestand (PDS3
`GO-…-RSS-…-V1.0`, TRK-2-25/2-18, GWE open-loop ODR) und der Vorfilter der
eigenen Rausch-Kurve sind in `docs/befund/befund-galileo-gwe-bestand.md`
gemessen (Tabelle oben korrigiert).
