<!--
  title: Blatt I — der kausale Pfeil des ENSO (Wind ↔ SST)
  class: handover
  date: 2026-08-21
  sha256: 812d8db789df597d3d0f4ba8b11646f399e3512ceaaf9a5d649555222c35500f
  status: live
  see-also: docs/concepts/blatt-papier-beweis.md docs/handover/handover-2026-08-21-sonnen-pfad-solar-te.md
-->
# Handover: Blatt I — der kausale Pfeil des ENSO (Wind ↔ SST)

Registriert 2026-08-21. Die nächste Session liest genau dieses eine
Dokument und beginnt. Selbsttragend — interpretierbar mit null
Vorkontext. Der Auftrag ist nicht die Ausführung; ausgeführt wird erst
auf das Wort des Operators. Das Konzept (Form des Blatts, Ethik,
geerbte Pflichten) steht in `docs/concepts/blatt-papier-beweis.md`.

## Ziel

Das Blatt I. Titel: „Der kausale Pfeil des ENSO." Zwei Messungen —
TE(Wind → SST) und TE(SST → Wind) —, das Richtungsurteil über der
Surrogat-Schwelle, der Lag aus dem Sweep, das Fenster benannt. Die
Bjerknes-Frage (treibt der Wind das Meer, oder treibt das Meer den
Wind?) als Messung statt als Debatte. Gemessen wird erst auf das Wort des
Operators; bis dahin steht pending, nie eine Zahl (0 honored).

## Ist-Stand (gemessen 2026-08-21)

- Die TE-Maschine lebt: `transfer_entropy_lag` (`src/te.rs:92`,
  lag 0 = kanonisch), `surrogate_stats_phase` (phasenrandomisierte
  Nullkontrolle, mean + 2σ), das Probe-Muster
  `src/bin/nobel_probe_corona.rs` (extract_series → beide Richtungen
  → Schwelle). Ein Blatt-Probe ist eine Schwester dieses Musters —
  kein neuer Weg.
- Wind (advective): FROST met.no lebt in `phi/sources.φ`
  (wind_speed + air_temperature, Fanout 40, Basic-Auth
  FROST_BASIC_AUTH — Befund `phi/pipeline/interesting_domains.φ:91`).
  Ob die Fanout äquatornahe Stationen trägt, ist ungeprüft. Die
  Trade-Wind-Reihen (TAO/TRITON-Moorings) sind nicht im Bestand;
  NDBC trug extract-void (`phi/pipeline/refusal_ledger.φ:5`) — der
  Weg über pmel-ERDDAP / tao.ndbc.noaa.gov ist pending.
- SST (thermal): keine Quelle lebt. Kandidaten im Register: Argovis
  (parser-def klassifiziert — die korrekte Query mit startDate/
  endDate ist pending Probe; `interesting_domains.φ:43`),
  imos_argo_sst ERDDAP CSV (`review_kandidaten.txt:517`), ESA-CCI-SST
  (Host tot → climate.esa.int; `interesting_domains.φ:193`).
  Open-Meteo Marine: unverifiziert — Probe.
- SOI (acoustic, Tahiti–Darwin-Druck): nicht im Bestand — Kandidat
  NOAA/BOM-Monats-CSV, Probe.
- ERA5 (ECMWF): nur Katalog-Eintrag
  (`phi/pipeline/katalog/noaa_nodd_inventory.φ:111`), CDS-Key pending —
  der Archiv-Weg für das lange Fenster.

Force-Gate-Urteile (Vorschlag): SST = thermal (Wärme — ein Organismus
könnte sie fühlen), Wind = advective (die Messgröße ist strömende
Luft), SOI-Druck = acoustic (Druckwelle — Barorezeptoren). τ wird je
Reihe deklariert als das, was die Reihe trägt — nie eine Setzung.

## Das Fenster-Urteil (Teil des Blatts)

Der ENSO-Zyklus trägt 2–7 Jahre; ein Blatt über wenige Wochen misst
Wetter, nicht ENSO. Zwei ehrliche Wege: (a) Archiv-Ernte (ERA5;
Argo GDAC — `phi/pipeline/katalog/seanoe_catalog.φ:53` trägt den
GDAC-Eintrag), (b) der Ring wächst — das Blatt trägt dann das
Fenster, das es trägt, und benennt es. Keine Extrapolation über das
Fenster hinaus; ein zu kurzes Fenster ist eine benannte Grenze, kein
gekürztes Blatt.

## Atome (Vorschlag an den Operator — ein Fenster, ein Atom)

- Atom 1 — Source-Port der ENSO-Kanäle über `docs/SOURCE_PORT.md`
  (der eine Pfad: queue/master.φ, ledger.φ, Force-Gate, τ). Ernte der
  Reihen, SI-Konversion, Frameless-Urteil (die Reihe trägt die
  Position der Station/Buoy).
- Atom 2 — Blatt-Probe I: Lag-Sweep 0..12 Monate, beide Richtungen,
  phasenrandomisierte Surrogate, Schwelle mean + 2σ. Die drei
  geerbten Pflichten (TODO, Nadel III): Mehrfachvergleichskorrektur
  über alle getesteten Paare, Lag-Sweep statt lag-0-Default,
  KDE-Sensitivität gegen h.
- Atom 3 — Das Blatt schreiben, `TODO.md`-Zeile schließen, Commit,
  Ergebnis archivieren.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
grep -n 'frost\|met.no' phi/sources.φ
grep -n 'imos_argo\|Argovis' phi/pipeline/review_kandidaten.txt phi/pipeline/interesting_domains.φ
sed -n '90,140p' src/te.rs
```

## Gates

- Pro Atom: `cargo check` 0/0 (falls Code), ein Commit,
  Register-Wahrheit (TODO.md + ledger.φ gegen den Code gelesen).
- Das Blatt trägt nur gemessene Zahlen; pending bleibt pending.
- Dieses Dokument erst archivieren
  (`/home/johannes/projects/archive/handover/`), wenn die eigene
  Arbeit committet ist.

## Nicht anfassen

Die laufenden Übergaben (Nadel III, 4d-wahrheit, offene Atome,
sonnen-pfad); `src/te.rs` ist die kanonische Referenz — der Probe
liest sie, er schreibt sie nicht um.
