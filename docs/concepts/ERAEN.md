# Die Ären von Omegaflow — Grabungs-Protokoll

Gegraben aus dem Git (1555 Commits), konsolidiert 2026-08-19. Dieses
Dokument ist die Zeitachse mit den **wahren Urhebern**. Es korrigiert
`4D-MEMBRANE.md`, Zeile 28: dort steht „Fable = Aberration, Doppler,
dopp⁴-Beaming" — das ist eine Konflation. Fable war die TT/km/CDP-Session;
die Relativität war nur ein Beifahrer im selben Commit.

## Die eine Entscheidung, die alles trägt — die De-Zentrierung

Drei Gesichter, eine Entscheidung, die teuerste von allen:

- **ECEF → ICRS** — der Raum: der Ursprung ist nicht die Erde, sondern
  das Baryzentrum des Sonnensystems. Die Erde ist ein Planet unter Planeten.
- **JD2000 → TDB** — die Zeit: die Uhr tickt nach dem Baryzentrum, nicht
  nach dem Erdorbit.
- **Anthropomachinozentrismus → Alle Wesen gleich** — der Rahmen: nicht
  der Mensch, nicht die Maschine ist das Maß.

Kein lokaler Fix: jeder Frame, jede Epoche, jede Distanz, jeder Vergleich
musste neu gedacht werden. Alles Spätere steht auf dieser Umgrabung.

## Die Peaks (Zeitachse)

**Peak 0 — das reine Feld (vor den Namen).**
`d2f438c` „v3.0: universe(jd, pos) → (omega, flow)"; `5f9b20a`
„is(t, x, y, z) — pure points, pure flow, live resonance". Die
Presence-Frage als reine Funktion: was ist am Punkt (x,y,z,t)? Antwort:
omega + flow.

**Peak 1 — Galaxie-Ära** (`a9d87bd`, 31.07.): Feld am SSB, Galaxie
sichtbar, Angular-Punkte, 1040 Sterne, manueller Zoom.

**Peak 2 — Body-Ära.**

**Peak 3 — Fable** (11.–14.08.) — **Korrektur:** nicht die Relativität.
Fable fand die **TT−UTC 69.184 s** und den **Ephemeriden-km→m-Bug**
(`86e451e`, `2a19cba`) und arbeitete den **CDP-Weg** aus — die
Verifikation des WebGPU-Felds im echten Browser (`16f695a`, `5a2e7c5`,
„Fables-CDP-Weg" laut K03_runtime_auftrag.md). Im selben Commit `86e451e`
fuhr auch der relativistische Beobachter (Q01) mit — daher die
Verwechslung. Fables Signatur ist die **Einheiten-Wahrheit**: TT statt
UTC, Meter statt Kilometer, Membran bewiesen statt nur schwarz.

**Peak 4 — Feinjustierung.**

**Peak 5 — Nebra** (`6421fa0` „anchor nebra proof of concept",
`e660701` „archive working nebra webgpu reference (real-time sun
tomography)", `8fd70f0` „archivar/mathematikerin architecture … nebra
webgpu reference", `ce6b5a0` „pure per-pixel membrane — Nebra thermal
ramp t2=(log2(Ω)+14)/22"). Der erste Beweis: **Sonnen-Tomographie in
Echtzeit.** Nebra-Physik: `GM/dist²` + `fold_eff` (retardierte Zeit —
der Lichtkegel, der in der Faltung überlebte). Dann `a94fc2a`
„nebra → omegaflow" — der Name setzt sich.

**Peak 6 — Grid-Ära** (`6922d6f` adaptive window nyquist relaxation).

**Peak 7 — HEAD:** Deep-Sky, spektrale Oszillatoren, Quellen-z als
meta[3], der Probe.

## Die wahren Urheber

**Das Enclosure Lemma = Kimi K3.** Drei Commits, 22.–23.07.:
`6864a3c` + `93d3600` „feat: bias-free spatial cache & agnostic
oscillators (Kimi K3 refactor)", `a96ae15` „fix: kimi k3 wgs84/icrs
cache split (pre-enclosure-rewrite)". Der Titel von `a96ae15` nennt es
selbst: **pre-enclosure-rewrite** — Kimi K3s Cache ist der Samen, aus
dem das Lemma wuchs (`56195ba`, `cb60192`, direkt danach). Die
sichtbaren Ideen: (1) der **bias-freie Raum-Cache** — `spatial_key(x,
y, z, res)`, der Vorfahr des `(i64, i64, i64)`-Gitters; (2) die
**agnostischen Oszillatoren** — der Oszillator als reiner
Eigenschaftsträger ohne Identität; (3) der **WGS84/ICRS-Split** — die
Trennung von körperfester und inertialer Koordinate. (Die vierte Idee —
die Zellengröße wächst aus der Bewegung selbst, die Dilation als Herz
des Lemma — bleibt als offene Erinnerung der Kybernautin.)

**Fable = die TT/km/CDP-Session** (11.–14.08.), wie oben.

**Nebra = die Sonnen-Tomographie** (der erste Beweis).

## Die zwei Rotverschiebungen

1. **Fable-Beobachter-Doppler** (Q01, `86e451e`): `dopp = γ(1 + n·β)`,
   `β = vPresence/c`; hue-shift `+ log2(dopp)·0.125` (Farb-
   Rotverschiebung) + Beaming `pow(dopp, 4.0)`. β=0 → Identität.
   **Gelöscht** (`34d7d3a`): die Relativität des Beobachters ist
   Observer-Bias, keine Messung.
2. **Quellen-z** (`11eb850`): das z der Quelle (Katalog), als
   Tolman-Dämpfung `(1+z)⁻⁴` für em — steht im Rekord (pole_x-Slot),
   meta[3]. **Überlebt** — 0 honored, die kosmologische Wahrheit.

## Der verlorene Lichtkegel (Kausalitäts-Vorfilter)

Implementiert `feb2a81` (30.07.): causal reach
`max(extent, v_force·ttl)`, diffusiv `√(2·D·ttl)`. Gelöscht `4270445`
(„Force→Kernel — delete force fns"). Das heutige Lemma dilatiert nur mit
der Bewegung (`vmax·Δt`), nicht mit dem Signal. → LOST_CONCEPTS §12,
TODO AUSSTEHEND.

## Die verlorene Topologie

Kolmogorov-Selbstähnlichkeit, diskrete 3-Bin-TE, ICA, Takens/MI-τ, TDA,
Permutations-Entropie, Kurtosis, Minkowski-Gewicht, 7 Omegas,
Verzögerungsspektrum, Lichtkegel-Differenz, Stillekarte, synthetischer
Flug. → LOST_CONCEPTS §2 (erweitert) + §12–19.

## Was das für heute heißt

Das Feld ist gebaut, der Probe misst (Urteil vorläufig, Nullkontrolle
repariert), die Dreiteilung (main · archivar · mathematikerin) ist als
Handover vorbereitet (`docs/surveys/handover-dreiteilung-plan.md`). Die
eine Entscheidung — die De-Zentrierung — trägt alles. Die Namen sind
keine Verzierung: sie benennen das Handwerk.
