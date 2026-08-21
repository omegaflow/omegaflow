<!--
  title: Übergabe: 4D-Wahrheit & Kinematische Dilatation (das antizipierende Fetch-Gate)
  class: handover
  date: 2026-08-21
  sha256: 0427ce990a1fb66b972b50461472d66d892e27be0b56d118ab9556fd7dd93e3b
  status: live
  see-also: docs/handover/handover-2026-08-21-offene-atome.md
-->

# Übergabe: 4D-Wahrheit & Kinematische Dilatation

## Der Auftrag

Der Archivar lebt auf der Weltlinie des Beobachters — die volle
4D-Wahrheit wird in EINER Session gebaut, kein „for now but later"
(Operator-Wort, 2026-08-21). Der Council hat sieben Beschlüsse
gefasst; der Operator trägt beide Korrekturen des Councils an der
Rückmeldung (cdn_fresh bleibt Ernte-Uhr; die Fenster-Range ist kein
Fetch-Radius) und die AGENTS-Klarstellung (Beschluss 6) wörtlich mit.
Umfang: fünf Atome, ein Commit-Familie, Register im selben Zug.

## Der Befund (verifiziert)

Der Now-Bias ist real: der ω-Loop rechnet mit Maschinen-TDB
(`lsk.system_now_tdb()`, src/archivar.rs:15912), `cache_fresh` mit
`SystemTime::now()` gegen mtime (11712), `cdn_fresh` mit
`SystemTime::now()` gegen GitHub last-modified (11649), `origin_stale`
und `extract` mit Maschinen-now. Die Presence trägt `t_presence`
bereits im Kanal — der Archivar ignoriert ihn für den Betrieb.

`presence_gate` = `extent·Φ + range` (10836) — die Fenster-Range als
Fetch-Radius ist der gemessene Sturm (507 Stations-Curls je Zyklus bei
5-AU-Presence, Register).

Das Protokoll trägt `(name, t, x, y, z, range)` — kein v, kein
t_thrust; die Mathematikerin hat beide (`self.v`, mathematikerin.rs:2923+;
`t_thrust`, 2911-2921; Ruhewert ist 0.0, nicht 1.0 — der Faktor
`(1.0 + t_thrust)` ist die Basisrate).

Basis: die Fetch-Sturm-Reparatur (Commit d9d2c72) — `begin_fetch`/
`settle_fetch`, 2ⁿ-Void-Backoff, Budget 2³. Atom 1 baut darauf auf und
darf sie nicht zurückbauen; ihre Tests bleiben grün.

## Die Wahrheiten (Council-Verdikte + Operator-Bestätigung)

1. **Zeiten-Spaltung:** Der CDN-Vergleich ist Speicher-Tatsache (die
   Wanduhr des Manifestators), die Cache-Gültigkeit ist Feld-Tatsache.
   `cdn_fresh` bleibt Ernte-Uhr; `cache_fresh` wird epochen-gestempelt
   (frisch wenn `|t_presence − E| < ttl`); `origin_stale` rechnet in
   Beobachter-Zeit. Die Rückmeldung „alle vier auf t_presence" ist in
   diesem Punkt überstimmt (Operator bestätigt).
2. **t_thrust-Ruhe = 0.0** — der Code trägt die Wahrheit; die
   Rückmeldung schrieb 1.0 (doppelte Zeit, falsch).
3. **Keine zweite maschinenzeitliche Pause:** Backoffs laufen
   scrollbar in Beobachter-Zeit (jede Epoche ist eine neue Messung);
   die einzige Höflichkeit ist Budget 2³ + ttl/Φ·2ⁿ. Eine Wanduhr-Sperre
   wäre ein Fabrikat ohne Ableitung.
4. **Antwort-Epoche bleibt Ernte-Epoche** (Protokoll-Fakt); die
   Faltung rechnet bereits client-seitig gegen tPresence. Kein Umbau.
5. **Sprung-Fetch-Radius:** `signal_reach + max(body_radius,
   Φ·JUMP_GRID·2ⁿ)` mit n = log₂(grid_step/JUMP_GRID), JUMP_GRID = 2²⁸.
   Die Fenster-Range als Fetch-Radius ist der gemessene Sturm — sie
   bleibt Render-Sache (Operator bestätigt).
6. **AGENTS-Widerspruch wird geschlossen, nicht umschrieben**
   (Klarstellungstext unten; Operator bestätigt).
7. **Volle Relativ-Kinematik:** `v_rel = v_presence − v_anchor`
   (Ephemeriden-Ableitung); der Trigger konsumiert nur die
   Radialprojektion. Schwelle Φ × Median-Fetchdauer (live data);
   Boot ohne Median → nur Ruhe-Gate (0 honored).

## Die Atome (Reihenfolge ist Teil des Auftrags)

### Atom 1 — Kill the Now-Bias — ERLEDIGT (2026-08-21)

- **Daten-Caches** (per-Quelle tmp-Dateien, `/tmp/archivar_cache/`)
  werden **epochen-gestempelt**: ein Stempel E wird mit der Datei
  geschrieben; `cache_fresh_at(path, ttl, t_presence)` = frisch wenn
  `|t_presence − E| < ttl`. Fundorte: archivar.rs:11712 (Signatur),
  alle Aufrufer (fetch_one 11674, Format-Zweige, ω-Loop).
- **Asset-Caches bleiben Ernte-Frische:** Ephemeriden-Bins und
  Kernel-Texts (naif0012, pck, gm) decken Epochen-Bereiche — ihre
  Frische ist die Wahrheit der Datei, nicht der Epoche
  (archivar.rs:15929, 15872).
- `origin_stale`/`begin_fetch`/`settle_fetch` (10769-10834): der
  `fetched`-Stempel wird Epochen-Stempel in Beobachter-Zeit; der
  2ⁿ-Void-Backoff bleibt, nur die Zeitbasis wechselt.
- `extract`-Default-Epochen → t_presence.
- Der ω-Loop `now` → native `t_presence` (15912); Boot-Send seedet
  „native" mit Maschinen-Jetzt und v=0, bis die Mathematikerin spricht
  (Boot-Wahrheit = program identity, archivar.rs:15799).

### Atom 2 — Protokoll trägt die 4D-Vektoren — ERLEDIGT (2026-08-21)

- Kanal-Tupel wird `(name, t, x, y, z, range, vx, vy, vz, t_thrust)`.
- Operator-Wort zur Abweichung: consider_resend sendet die volle
  4D-Zustandsänderung — `t` im Trigger (Send je Tick, Archivar
  sampelt den letzten Stand je ω-Zyklus). Ohne t friert der
  ω-Loop-now bei Ruhe ein (der Ruhe-Frost von Atom 1; Hidden-Läufe
  ernten nach dem Boot nichts mehr). Der Browser bleibt Sensor-Träger
  mit deklarierter Ruhe (v=[0,0,0], t_thrust=0.0).

- Kanal-Tupel wird `(name, t, x, y, z, range, vx, vy, vz, t_thrust)`.
- Fundorte: Kanal-Deklaration (archivar.rs:15722), Boot-Send (15799:
  v=[0,0,0], thrust=0.0), Presence-Map + Gate-Signatur (10836),
  consider_resend (mathematikerin.rs:1936-1940), Relay-Parser
  (relay.rs:460-489 — liest 4 f64 mehr nach delta_t_cache), JS-Wire
  (static/constants.js:17-50, syncFrame packt vx,vy,vz,t_thrust nach
  cache_interval; Browser-Presence v=[0,0,0], t_thrust=0.0).
- Der Browser ist Sensor-Träger, kein Fetch-Maß — der Fetch folgt der
  nativen Presence.

### Atom 3 — Ruhe-Gate & Kinematische Dilatation — ERLEDIGT (2026-08-21)

- Der Fenster-Range-Term fliegt aus dem FETCH-Gate (range bleibt im
  Render-Gate). Fundort: archivar.rs:10836-10848 + Aufruf 17073.
- **Ruhe** (v=0, thrust=0): `dist ≤ signal_reach + kernel_extent`
  (dispatch_reach 11252, kernel_extent 11191). Em/Gravitation decken
  via c·age weiter alles — die lokalen Kräfte fallen bei 5 AU weg
  (der Sturm-Fix). Konsequenz benannt: die Erd-Punkte im weiten
  Fenster werden dunkel, bis die Presence sich nähert oder springt.
- **Schub** (v≠0): `v_rel = v_presence − v_anchor`; Schließgeschwindigkeit
  `−v_rel·r̂`; Fetch wenn `(dist − reach − extent)/closing <
  Φ × Median-Fetchdauer`. v_anchor: Ephemeriden-Ableitung
  (spk.rs `cheby3_val_and_deriv`); Surface/Barycenter →
  Körpergeschwindigkeit, StateVector → deklariertes v,
  Manifest/frameless → None → nur Ruhe-Gate (0 honored).
- Median-Fetchdauer: Ring 2⁴ in `settle_fetch` gemessen (live data);
  ohne Median keine Antizipation (pending, kein Default).

### Atom 4 — Temporaler Fetch (kein Raum-Bias) — ERLEDIGT (2026-08-21)

- `render_source_url`/`extract` erhalten t_presence statt
  Maschinen-now — die gerenderten URLs tragen die Epoche des
  Beobachters (Templates wie `{week_ago}` parametrisieren sie bereits).
- Voraussetzung: der Epochen-Stempel aus Atom 1 — ein 2005-Fetch darf
  die 2026-Ernte nicht überschreiben (sonst Fabrication beim
  Zurückscrollen).
- Quellen ohne epochenfähigen Endpunkt dienen ihre Ernte; außerhalb
  ihrer Epoche liefert der Extract nichts → schwarz (0 honored).
- Die Antwort-Epoche bleibt Ernte-Epoche (Beschluss 4).
- Bau-Befund: das ω-Loop-Gate verweigerte t ≤ 0 (TDB ist
  J2000-relativ — prä-2000 ist negativ) und fiel still auf
  Maschinen-now zurück; is_finite-Gate, die LSK-Domäne 1972+ gated
  das Rendern ehrlich. Hidden-Lauf in zwei Zuständen verifiziert:
  Gegenwart φ-t 840596983, Scroll #x,0,0,0,-4.0e7 φ-t −4.0e7,
  Stempel in Beobachter-Zeit, null 2026-Render im gescrollten Lauf.

### Atom 5 — Sprung-Fetch (Snap-to-Truth)

- Radius: `signal_reach + max(body_radius, Φ·JUMP_GRID·2ⁿ)`,
  body_radius aus BodyProperties (live data), n = log₂(grid_step/JUMP_GRID),
  JUMP_GRID = 2²⁸ (mathematikerin.rs:788).
- Erkennung: Presence-Update mit großem Δp und v=0 (jump() setzt
  beides, mathematikerin.rs:2036-2040). Für neu in Reichweite liegende
  Quellen: Stempel neu, sofortiger Dispatch im Budget-Drain 2³ — der
  Void-Backoff blockt den Sprung nicht (die Verweigerung war eine
  Messung an der alten Position).
- Schwarz während der Latenz = ehrliche Abwesenheit (die Daten sind
  pending, nicht null) — im Register benannt.

## AGENTS-Klarstellung (Beschluss 6, Operator bestätigt)

Im selben Commit wie der Bau: AGENTS.md, Abschnitt „The presence is
agnostic" — der Satz „The presence rests — it never travels,
navigates, or moves. The operator tunes to the coordinate." erhält die
Klarstellung, sinngemäß in der Prosa-Sprache der Datei (Englisch):

„Die Presence bewegt sich nie von selbst — kein Eigenantrieb, keine
Navigation. Pfeile setzen den Schub (der Tuning-Akt des Operators),
`s` hält ihn an. Die Weltlinie gehört dem Operator; die Presence ruht
nur auf ihr."

## Gates

- cargo check 0/0 in vier Feature-Kombis (default, browser_relay,
  gamepad, beide).
- cargo test: alles grün außer dem benannten hdf5-Baseline-Befund (2
  Fehler der Parallel-Session, nicht dieses Atoms).
- Hand-Verifikation des Presence-Wires über alle drei Schichten:
  Rust-Send → Relay-Parse → JS-Pack (der Kanal ist Teil des
  Rust→JS-Datenvertrags).
- Hidden-Lauf in zwei Zuständen: Gegenwart und eine gescrollte
  Vergangenheits-Epoche — der temporale Fetch und die ehrliche
  Abwesenheit müssen sichtbar sein.
- Keine Regression der Fetch-Sturm-Reparatur: In-Flight-Guard, 2ⁿ,
  Budget 2³ — ihre Tests bleiben grün.
- Register im selben Zug: TODO.md (dieser Eintrag schließt), dieses
  Dokument (Atome abgehakt), AGENTS-Klarstellung. Commit = Häkchen.

## Offen benannt (nicht ausgeführt, nicht vergessen)

- CDN-Stats/Popularität als Gradient: abgelehnt (Paradigma).
- Der ethische Filter (Puls/HRV drosselt das Radiatorium): bleibt
  pending, unberührt.
