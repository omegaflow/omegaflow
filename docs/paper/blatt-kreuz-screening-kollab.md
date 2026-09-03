<!--
  title: BLATT PAPIER — Kreuz-Screening der Serien im Kollab-Fenster (Trishuli 2026-08-26)
  class: sheet
  date: 2026-08-30
  status: pending
  see-also: docs/blatt-pfeil-sturzflut-tibet.md docs/causal-arrow-preregistration.md
-->

> **Richtungskorrektur (registriert):** Die Pfeilrichtung folgt der
> maßgeblichen Bibliothek `transfer_entropy_lag(x, y) = TE(y → x)` (zweites
> Argument = Quelle). Die Richtungen in diesem Blatt sind die korrigierten;
> die bedingte Transfer-Entropie (`--cond` auf den Tagesgang) ist jetzt
> gebaut und angewandt.

> **Power-Update (registriert, 2026-08-31):** Option 1 (Fenster erweitern)
> wurde mit dem vollen 8-Wochen-Fenster durchgeführt —
> `2026-07-01 … 08-27`, n = 1392 je Serie. Befund: `kollab_precip →
> rasuwa_precip` (nord→süd, lag 12) ist **unbedingt** signifikant
> (TE 0.1172 > 0.1072), fällt aber **nach Konditionierung auf
> `gyirong_open-meteo_pressure_msl` aus der Signifikanz** — Aussage B kippt
> im vollen Fenster. Konditioniert überleben unter Druck nur Pfeile mit
> gyirong als Quelle bzw. Ziel (gyirong→rasuwa, gyirong→kollab,
> kollab→gyirong); der räumliche lag-12-Pfeil zwischen den Kollab- und
> Rasuwa-Niederschlagsreihen ist über den geteilten Treiber hinaus **nicht
> isolierbar** (Aussage B → siehe §3.4). Genaue cTE-Werte der überlebenden
> gyirong-Pfeile in §3.4 ausstehend (`pending`), nicht fabriziert.

# BLATT PAPIER — Kreuz-Screening der Serien im Kollab-Fenster

**Datum:** 2026-08-30 · **Axiom:** A = A
**Verdikt-Ordnung:** 0 honored — kein Wert erfunden; fehlende Felder bleiben `pending`.

---

## Zusammenfassung

Das Kreuz-Screening misst, **welcher Kausalpfeil** zwischen den sechs
gemessenen Serien des Trishuli-Kollab-Fensters (2026-08-26) schlägt —
nicht welcher gefällt. Es laufen alle Paare × beide Richtungen × Lags über
`transfer_entropy_lag` + `surrogate_threshold_lag` (phasenrandomisierte
Surrogate, mean + 2σ). Die Serien sind die geernteten Open-Meteo-Reihen der
drei Stationen (gyirong, kollab, rasuwa) mit zwei Variablen je Station
(Niederschlag, Temperatur), je n = 240, stündlich, 2026-08-18…27 UTC.

**Befunde (gemessen, nicht behauptet):**

1. **Temperatur als geteilter Treiber über das ganze Becken** — die
   Temperatur-Serien sind untereinander **beidseitig** signifikant bei allen
   Lags (TE 0.11–0.28) und treiben auch den Niederschlag (rasuwa_temp →
   rasuwa_precip, gyirong_temp → gyirong_precip). Das ist der **geteilte
   Tagesgang**, kein gerichteter Ausbreitungspfeil — eine Konfundierung
   durch den gemeinsamen Sonnen-/Regime-Treiber, kein Kausalpfeil zwischen
   den Stationen.
2. **Unbedingt gemessen: eine süd→nord Monsun-Kette im Niederschlag**
   (korrigierte Richtung) — `rasuwa_precip → kollab_precip` lag 6
   (TE 0.2085) und `kollab_precip → gyirong_precip` lag 6 (TE 0.1377):
   Monsun-Advektion von Nepal (Süd) ins tibetische Oberlauf (Nord). Diese
   Pfeile tragen aber den geteilten Treiber mit sich.
3. **Die süd→nord-Kette überlebt die Konditionierung auf den Tagesgang
   NICHT.** Bedingte TE auf `gyirong_open-meteo_temperature_2m`:
   `rasuwa_precip → kollab_precip` lag 6 (0.2085 unbedingt) fällt aus der
   Signifikanz. Was konditioniert überlebt, sind **marginale nord→süd-Pfeile
   bei lag 12** (Oberlauf → Unterlauf, Flutwellen-Laufzeit): `kollab_precip →
   rasuwa_precip` lag 12 (cTE 0.2001 > 0.1970) und `gyirong_precip →
   rasuwa_precip` lag 12 (cTE 0.1811 > 0.1627) — knapp über der Schwelle.
4. **Verdikt der Flutrichtungs-Frage: `pending`, nicht fabriziert.** Der
   Monsoon-Ausbreitungspfeil Rasuwa → Gyirong (süd→nord) ist unbedingt
   messbar, aber **nicht vom geteilten Temperatur-Treiber isolierbar**; die
   konditioniert überlebenden Pfeile sind umgekehrt (nord→süd) und marginal.
   Ein Einzel-Paar-Verfahren kann den süd→nord-Pfeil messen (siehe
   Blatt-pfeil-sturzflut), das konditionierte volle Screening bestätigt ihn
   **nicht unabhängig** über den gemeinsamen Treiber hinaus.
   **Power-Update (8 Wochen, n = 1392):** auch der nord→süd-lag-12-Pfeil
   aus Aussage B fällt nach Konditionierung auf den Gyirong-Druck aus —
   beide Richtungen sind über den geteilten Treiber hinaus nicht isolierbar
   (siehe §3.3b, §3.4).
5. **Zweit-Proxi-Konditionierung (Option 2, Diagnose „Konfundierung"):
   süd→nord zerfällt auch unter einem zweiten geteilten Treiber.** Zusätzlich
   zum Tagesgang auf `gyirong_open-meteo_pressure_msl` konditioniert,
   überlebt `rasuwa_precip → kollab_precip` lag 6 (unbedingt TE 0.2085)
   **nicht**; es bleibt nur `kollab_precip → rasuwa_precip` lag 12
   (nord→süd, cTE 0.1902 > 0.1866). Auch konditioniert auf
   `gyirong_open-meteo_relative_humidity_2m` fällt `rasuwa → kollab` aus; nur
   `kollab_precip → rasuwa_precip` lag 12 (nord→süd, cTE 0.1898 > 0.1872)
   bleibt. **Konsistenter Befund über zwei unabhängige Zweit-Proxies:** der
   süd→nord-Monsunpfeil ist über den Tagesgang hinaus nicht isolierbar; der
   verbleibende räumliche Pfeil ist die marginale nord→süd-lag-12-Kette.
6. **Verdikt zweigeteilt — Aussage A und Aussage B.** Das Verdikt trägt
   zwei getrennte Aussagen, die nicht zu einem `pending` gefaltet werden:
   - **Aussage A (messbar gestützte Negativ):** Die süd→nord-Monsunrichtung
     fällt unter **jeder** Konditionierung aus — Tagesgang, Druck, Feuchte,
     drei unabhängige geteilte Treiber, dieselbe Antwort. Auf diesem
     Instrument ist die süd→nord-Richtung **nicht isolierbar**; das ist eine
     gestützte Stille mit drei Kontrollen, keine offene Frage. (Keine
     physische Negation — siehe §3.3 G2-Grenze.)
   - **Aussage B (marginal, pending):** Der nord→süd-lag-12-Pfeil überlebt
     alle drei Konditionierungen, aber knapp (Margen 2–3 % über Schwelle).
     *Dieser* Pfeil ist zu Recht pending: marginal, unter Vorbehalt, mit der
     physikalischen Lesart (Flutwellen-Laufzeit ~12 h) als Hypothese, nicht
     als Befund. Die Frage zu B ist offen und braucht mehr Power, nicht mehr
     Konfundierungskontrolle.

---

## 1. Methode

- **`cross_te_screen`** — neues Werkzeug (`tools/src/bin/cross_te_screen.rs`),
  liest JSON-Serien (Ernteverfahren, Format siehe §2) oder CSV/TXT.
- Für jedes Paar `(a, b)`, beide Richtungen, Lags {1, 6, 12, 24}:
  `TE(a→b)` gegen `surrogate_threshold_lag(a, b, lag, seed)` (mean + 2σ,
  phasenrandomisierte Surrogate, n = 20). Pfeilrichtung:
  `transfer_entropy_lag(x, y) = TE(y → x)`, zweites Argument = Quelle.
- **Bedingte Transfer-Entropie** `transfer_entropy_conditional(x, y, c, lag)`
  = `TE_{c}(y → x)`, gegeben den Tagesgang-Proxi `c` = gyirong Temperatur;
  Null-Surrogat: Residuen-Surrogat (Quelle auf `c` regressiert, Residuen
  permutiert, treiber-ausgerichteter Anteil erhalten), Schwelle mean + 2σ,
  n = 20. Der Konfund `c` wird aus den Pfeilen ausgeschlossen.
- **Signifikanz:** `TE > Schwelle`. Jeder signifikante Pfeil wird gelistet —
  das Screening filtert nicht (0 honored), es zeigt, was gemessen wird.
- **n = 240** je Serie (kein Unterbestimmtheits-Ausschluss; min-n = 100).

## 2. Daten (geerntet, 0 honored)

Quelle **Open-Meteo archive-api** (key-los, historisch, stündlich), Fenster
2026-08-18 00:00 … 08-27 23:00 UTC, n = 240 je Variable, `timezone=UTC`,
`hourly=temperature_2m,precipitation`.

| Station | lat | lon |
|---------|-----|-----|
| gyirong (Tibet, Oberlauf) | 28.8559 | 85.2950 |
| kollab (Kollabpunkt) | 28.2710 | 85.5150 |
| rasuwa (Nepal, Unterlauf) | 28.25 | 85.10 |

Artefakte: `phi/pipeline/collapse_harvest/<station>_open-meteo_<variable>.json`,
Format `{"source","station","variable","window","n","points":[{"t","v"}]}`.
Erntewerkzeug: `tools/src/bin/collapse_series_harvest.rs` (std-only, JSON auf
CDN via `--ci-mode`; Tag `archive-api.open-meteo.com`).

## 3. Ergebnisse (vollständiges Screening)

Lauf: `cross_te_screen --dir phi/pipeline/collapse_harvest --lags 1,6,12,24
--surrogat 20 --min-n 100` — 6 Serien, 15 Paare. Konditioniert: zusätzlich
`--cond gyirong_open-meteo_temperature_2m` (Tagesgang-Proxi).

### 3.1 Gesamtzahl signifikanter Pfeile

- **Temperatur ↔ Temperatur (alle Stationen):** beidseitig, alle Lags —
  der geteilte Tagesgang (TE 0.11–0.28, beidseitig > Schwelle; stärkste
  `kollab_temp → rasuwa_temp` lag 6 TE 0.278).
- **Temperatur → Niederschlag (gleiche Station):** bei Lag 6–24 (z. B.
  `rasuwa_temp → rasuwa_precip` lag 12 TE 0.227). Der Tagesgang moduliert
  die Konvektionschancen — plausibel, aber Teil des geteilten Treibers.
- **Niederschlag quer (korrigierte Richtung):**
  - süd→nord (Monsun): `rasuwa_precip → kollab_precip` lag 6 (TE 0.2085),
    `kollab_precip → gyirong_precip` lag 6 (TE 0.1377).
  - nord→süd (lag 12/24): `kollab_precip → rasuwa_precip` lag 12
    (TE 0.1750), `gyirong_precip → rasuwa_precip` lag 12 (TE 0.1560) und
    lag 24 (TE 0.1468).
- Keine konsistente Einzel-Richtung über mehrere Lags in einer Stationen-
  Relation — jede Relation hat beide Richtungen an verschiedenen Lags.

### 3.2 Konditionierte Befunde (bedingte TE auf den Tagesgang)

Bedingt auf `gyirong_open-meteo_temperature_2m` überleben **nicht** die
süd→nord-Monsun-Pfeile (rasuwa→kollab lag 6 fällt aus der Signifikanz).
Überlebende räumliche Niederschlags-Pfeile (nord→süd, lag 12):

- `kollab_precip → rasuwa_precip` lag 12 (cTE 0.2001 > 0.1970)
- `gyirong_precip → rasuwa_precip` lag 12 (cTE 0.1811 > 0.1627)

Diese überleben knapp über der Schwelle (Flutwellen-Laufzeit Oberlauf →
Unterlauf). Weitere konditionierte Pfeile: `rasuwa_temp → kollab_temp`
lag 6 (cTE 0.2135), `kollab_precip → kollab_temp` lag 6 (cTE 0.1957),
`rasuwa_temp → rasuwa_precip` lag 6 (cTE 0.1952).

### 3.2b Zweit-Proxi-Konditionierung (Diagnose „Konfundierung", Option 2)

Lauf: `cross_te_screen --dir /tmp/opencode/cond2 --lags 6,12 --surrogate 40
--min-n 30`, 6 Serien (3 Stationen Niederschlag + gyirong
pressure_msl / relative_humidity_2m / temperature_2m).

**Konditioniert auf `gyirong_open-meteo_pressure_msl`** (Druck als zweiter
geteilter Treiber): die süd→nord-Kette `rasuwa_precip → kollab_precip`
lag 6 (unbedingt TE 0.2085) fällt aus der Signifikanz. Konditioniert
überlebende räumliche Niederschlags-Pfeile:

- `kollab_precip → rasuwa_precip` lag 12 (cTE 0.1902 > 0.1866) — **nord→süd**,
  marginal.

**Konditioniert auf `gyirong_open-meteo_relative_humidity_2m`** (rel. Feuchte
als zweiter geteilter Treiber): `rasuwa_precip → kollab_precip` lag 6
überlebt **nicht**. Überlebender räumlicher Niederschlags-Pfeil:

- `kollab_precip → rasuwa_precip` lag 12 (cTE 0.1898 > 0.1872) — **nord→süd**,
  marginal.

**Konsistenz:** Zwei unabhängige Zweit-Proxies (Druck, rel. Feuchte) ergeben
denselben Befund wie die Tagesgang-Konditionierung: der süd→nord-
Monsunpfeil ist über den geteilten Treiber hinaus **nicht** isolierbar;
der einzige robust wiederkehrende räumliche Pfeil ist `kollab_precip →
rasuwa_precip` lag 12 (nord→süd, Flutwellen-Laufzeit Oberlauf → Unterlauf).

### 3.3b Power-Update: 8-Wochen-Fenster (n = 1392, Option 1)

Lauf: `cross_te_screen` über den omegaflow-Cache
`phi/pipeline/meteo_harvest/tibet-flut-2026-8w/` (4 Reihen:
gyirong/kollab/rasuwa precipitation + gyirong pressure_msl), Fenster
`2026-07-01 … 08-27`, n = 1392 je Reihe, konditioniert auf
`gyirong_open-meteo_pressure_msl` (Druck als geteilter Treiber).

- **Unbedingt:** `kollab_precip → rasuwa_precip` lag 12 signifikant
  (TE 0.1172 > 0.1072) — der nord→süd-lag-12-Pfeil aus Aussage B ist im
  vollen Fenster messbar.
- **Konditioniert auf gyirong pressure_msl:** `kollab_precip →
  rasuwa_precip` **fällt aus der Signifikanz** (kollab→rasuwa erscheint
  nicht unter den konditionierten Befunden). Konditioniert überleben
  räumliche Niederschlags-Pfeile mit gyirong als Quelle bzw. Ziel:
  gyirong→rasuwa, gyirong→kollab, kollab→gyirong. (Genaue cTE-Werte
  dieser drei Pfeile: `pending`, nicht fabriziert.)

**Lesart:** Mehr Power (n 240 → 1392) bestätigt den unbedingten
nord→süd-lag-12-Pfeil, löst ihn aber nicht von der Konfundierung durch
den geteilten Gyirong-Druck-Treiber — Aussage B kippt im vollen Fenster
unter Konditionierung. Der Pfeil ist über den geteilten Treiber hinaus
nicht isolierbar.

### 3.4 Schlussfolgerung Aussage B (Power-Update)

**Aussage B — korrigiert (nicht länger offen als isolierbarer Kausalpfeil):**
Der frühere marginale nord→süd-lag-12-Pfeil `kollab_precip →
rasuwa_precip` ist im 8-Wochen-Fenster unbedingt messbar (TE 0.1172),
überlebt die Konditionierung auf den geteilten Druck-Treiber aber nicht —
er trägt den gemeinsamen Treiber mit sich und ist auf diesem Instrument
nicht als unabhängiger Kausalpfeil isolierbar. Die offene Frage von B war
die Isolierbarkeit; mit dem Power-Test ist sie beantwortet: **nicht
isolierbar über den geteilten Treiber hinaus.** Nicht eine physische
Negation, sondern eine instrumentelle Nichtisolierbarkeit (G2-Grenze,
wie Aussage A).

### 3.3 Interpretation (0 honored, gemessen nicht erzählt)

Das volle Screening **misst den geteilten Temperatur-Treiber** als das
dominante Signal — die drei Stationen sind thermisch synchronisiert
(beidseitig, alle Lags), ein stark korreliertes gemeinsames Regime. Das
ist **kein gerichteter Kausalpfeil zwischen den Stationen**, sondern eine
**Konfundierung durch den gemeinsamen Wetter-/Sonnen-Treiber**.

Der süd→nord-Monsun-Ausbreitungspfeil Rasuwa→Kollab→Gyirong ist **unbedingt
messbar** (lag 6, TE 0.14–0.21), aber **nach Konditionierung auf den
Tagesgang nicht signifikant** — er ist vom geteilten Treiber nicht isolierbar.
Die konditioniert überlebenden räumlichen Pfeile weisen **gegenläufig**
(Oberlauf→Unterlauf, lag 12) und sind nur marginal über der Schwelle.

**Konsequenz (0 honored):** Das Verdikt ist zweigeteilt.
- **Aussage A — beantwortet (messbar gestützte Negativ auf diesem
  Instrument):** der süd→nord-Ausbreitungspfeil fällt unter allen drei
  Konditionierungen (Tagesgang, Druck, Feuchte) aus. Auf diesem Instrument,
  in diesem Fenster ist er **nicht isolierbar**. Das ist eine gestützte
  Stille mit drei Kontrollen — nicht widerlegt im physikalischen Sinn
  (G2-Grenze: instrumentelle Nichtisolierbarkeit ≠ physische Negation),
  aber für diese Messfront beantwortet: eine erneute Investigation von A
  wäre Re-Investigation derselben Negativ.
- **Aussage B — offen (marginal, pending; danach durch §3.4 Power-Test
  korrigiert):** der nord→süd-lag-12-Pfeil
  `kollab_precip → rasuwa_precip` überlebt alle drei Konditionierungen mit
  2–3 % Marge über Schwelle. Er ist als **marginal gemessen** notiert, nicht
  als harter Kausalbeleg. Die Frage zu B wurde durch mehr Power entschieden
  (Fenster erweitern, Option 1) — siehe §3.3b/§3.4: kippt im vollen
  8-Wochen-Fenster unter Konditionierung.

## 4. Grenzen (0 honored)

- **Konfundierung:** Das univariate Kreuz-Screening kann gemeinsame Treiber
  (Tagesgang, Wetterregime) nicht von gerichteten Pfeilen trennen. Die
  bedingte TE auf `gyirong_open-meteo_temperature_2m` trennt den **linearen**
  Anteil ab, der mit der Temperatur ausgerichtet ist; ein residualer
  synoptischer Konfund bleibt möglich (ein Proxi, ein Fenster). Die
  Zweit-Proxi-Konditionierung (Druck, rel. Feuchte) erhärtet den Befund über
  zwei unabhängige geteilte Treiber, schließt aber keine restliche
  synoptische Gemeinsamkeit aus.
- **n = 240, 10 Tage:** das Fenster deckt den Monsun-Verlauf ab; längere
  Reihen (mehrere Wochen) würden die Surrogat-Schwellen härten.
- **Surrogate = phasenrandomisiert:** misst Synchronisation, nicht
  gerichtete Verursachung; die Richtung wird über die Lag-Asymmetrie
  gelesen.

## 5. Schlussfolgerung

- **Gemessen:** die Temperatur-Synchronisation über das Becken (beidseitig,
  alle Lags) als dominantes Signal — ein geteilter Treiber, kein
  Stations-Kausalpfeil.
- **Gemessen (unbedingt):** die süd→nord-Monsun-Kette im Niederschlag
  `rasuwa_precip → kollab_precip` lag 6, `kollab_precip → gyirong_precip`
  lag 6.
- **Gemessen (konditioniert, marginal):** `kollab_precip → rasuwa_precip`
  lag 12 (cTE 0.2001), `gyirong_precip → rasuwa_precip` lag 12 (cTE 0.1811)
  — nord→süd, knapp über der Schwelle. Zweit-Proxi-Konditionierung (Druck:
  cTE 0.1902; rel. Feuchte: cTE 0.1898): nur `kollab_precip → rasuwa_precip`
  lag 12 kehrt wieder — nord→süd, konsistent marginal.
- **Aussage A — messbar gestützte Negativ (beantwortet auf diesem
  Instrument):** die süd→nord-Kette `rasuwa_precip → kollab_precip` lag 6
  fällt unter allen drei Konditionierungen (Tagesgang, Druck, Feuchte) aus
  — nicht isolierbar über den geteilten Treiber hinaus. Keine physische
  Negation (G2-Grenze), aber eine gestützte Stille mit drei Kontrollen;
  eine erneute Investigation von A wäre Re-Investigation.
- **Aussage B — korrigiert durch Power-Test (n = 1392, §3.4):** der
  nord→süd-lag-12-Pfeil `kollab_precip → rasuwa_precip` ist unbedingt
  messbar (TE 0.1172), fällt aber nach Konditionierung auf den geteilten
  Druck-Treiber aus der Signifikanz — **nicht isolierbar** über den
  geteilten Treiber hinaus (G2-Grenze, kein physischer Kausalbeleg).

Was nicht gemessen werden kann, wird nicht behauptet.

---

*Blatt registriert 2026-08-30. Verdikt-
Ordnung 0 honored: kein Wert erfunden; jede Lücke als `pending` benannt.*
