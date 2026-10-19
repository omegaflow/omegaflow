# Roster — River P7.1 Kopplungs-Injektion + Direktiven-Grammatik (2026-10-12)

Runde river-201. Reihenfolge Operator-Wort: research > roster > rat. Gruppe `river-ui`
(JIT, `focus:false`, nach der Runde geschlossen); geteilte `open-weight-ui` in dieser Runde
**nicht** gefahren (Lock frei) — als Lücke benannt, nicht als Vollständigkeit ausgegeben.

Frage (kompakt, an alle Seats identisch): per-Kanal-Feld `phi=probe_omega[9]` in einem
Echtzeit-ω()-Loop, der **linear** aggregiert (`omega_sum=Σφ_i`) und eine Atmungs-Permeabilität
relaxiert; neu gebaut: sparse Paar-Registry `coupling_term = Σ C_k·φ_from·φ_to` (quadratisch,
Deskriptor-Hash-indizierte Paare). Physik: Ionosphäre-Plasma ⊕ Thermosphäre-Neutral, ion-neutral
Reibung = Momentum-`SOURCE/SINK` `ρ_i ν_in (u_n − u_i)`, equal-and-opposite, `ρ_i ν_in = ρ_n ν_ni`
— **kein** linearer Zusatz zum Feldaggregat. Q1: Injektions-Slot (a) per-Kanal-Feld / (b) PDE-Quelle
im Feld-Eval (`field_spatial` WGSL) / (c) Skalar auf dem Atmungs-Target? Q2: minimale
Direktiven-Grammatik, die die Erhaltung repräsentiert (store `ν_in` + Dichten, kein freier Koeffizient).

## Qwen (Qwen3.7-Plus) — `https://chat.qwen.ai/`, 2026-10-12

- **(1) Slot: (b)** — PDE source/sink in `field_spatial`. Reibung ist lokaler volumetrischer
  Momentum-Austausch; (a) scheitert (quadratisch, nicht additiv zum linearen Aggregat), (c) scheitert
  (lokale Kraft, keine makroskopische Permeabilitäts-Relaxation).
- **equal-and-opposite:** `Δp = ρ_i ν_in (u_n − u_i)` **exakt einmal pro Paar**; dann `+Δp` an den
  Neutral-Source-Akkumulator, `−Δp` an den Ion-Sink — streng skew-symmetrisch per Konstruktion.
- **(2) Erhaltung:** keinen freien makroskopischen `C_k` speichern, sondern die geteilte
  Stoßfrequenz `ν_in`; im WGSL-Kernel `ν_in × live ρ_i, ρ_n` aus den Kanal-Zustandsvektoren →
  `ρ_i ν_in = ρ_n ν_ni` bei jedem Schritt garantiert.
- **(3) Grammatik:** `kind` (Reibung vs. andere Differenz-Mathematik) · `from`/`to` · `nu` (geteilte
  Basis-Stoßfrequenz) · `rho_idx_from`/`rho_idx_to` (State-Vektor-Indizes für live Dichten);
  `region` bleibt zur räumlichen Maske, ist aber für die Erhaltungs-Mathematik nicht zwingend.
- **Unsicherheiten:** ob φ direkt Geschwindigkeit/Momentum abbildet (sonst Division durch Dichte);
  Race-Conditions bei `+Δ/−Δ` in einem WGSL-Pass (Atomics/Doppelbuffer); ob ρ im φ-Vektor liegt.

## Claude (Sonnet 5.5 Extra hoch) — `https://claude.ai/new`, 2026-10-12

- **(1) Slot: (b)** — Reibung ist eine per-Zelle-Tendenz auf jedem Kanal-Momentum, abhängig von beiden
  Zuständen → PDE-Quellterm. (a) falsch: behandelt Austausch als Feldwert; bei intensivem φ (Velocity)
  cancelt das ±-Paar nicht mehr in `omega_sum`. (c) falsch: ein Skalar hat keine per-Zelle/-Kanal-Struktur;
  nur als **read-only Feedback** (z.B. |F|) akzeptabel.
- **(2) Erhaltung:** ein Fluss pro Paar pro Zelle, **zweimal aus demselben Thread** angewandt (keine
  Atomics, keine Zweitberechnung): `F = ρ_i ν_in (u_n − u_i)`; `d(ρ_i u_i)/dt += F`, `d(ρ_n u_n)/dt -= F`.
  **Rückwärtsrate wird hergeleitet, nie gespeichert:** `ν_ni = (ρ_i/ρ_n)·ν_in` aus aktuellen Dichten
  (eine gespeicherte Rückwärtsrate driftet und bricht die Identität). **Form:** `C·φ_from·φ_to` hat die
  **falsche Form** für Reibung (Dichte × Geschwindigkeitsdifferenz, nicht Produkt zweier Feldwerte) —
  Produktform als eigenen `kind` (mass-action), Reibung eigenen. **Steifheit:** `Δ ← Δ·exp(−(ν_in+ν_ni)dt)`
  exakt, mit gehaltenem Schwerpunkt → Momentum bis auf Rundung, unbedingt stabil. **Regression:**
  `Σ ρ_k u_k` konstant. **Energie:** `ρ_i ν_in|Δu|² ≥ 0` ist nicht antisymmetrisch → Temperatur-Kanal.
- **(3) Grammatik:** `coupling <kind> <a> <b> <nu_ab|pending> <rho_a>,<rho_b> <conserves> <region>`;
  `kind` wählt Form + gelesene Felder; `a,b` **ungeordnetes** Paar (sortierte Deskriptoren hashen,
  deklarierte Rückwärtsrichtung verwerfen → eliminiert die freie asymmetrische Matrix); `nu_ab` eine
  gerichtete Rate oder `pending` (registered aber inert, im Diagnose-Listing); `rho_a,rho_b`
  Dichte-Kanal-Referenzen; `conserves` = momentum|energy|mass für eine Invarianten-Prüfung bei der
  Registrierung; `region` Maske. `interface` bleibt separat (Randbedingung, kein volumetrischer Austausch).
- **Unsicherheiten:** was φ ist (Identität gilt auf ρu); woher die Dichte kommt; Heizungs-Partition
  (Schunk & Nagy); Ion-Momentum trägt auch Lorentz/E×B; Multi-Spezies (mehrere Paare, je eigenes ν).

## Z.ai (GLM-5.3 Max) — `https://chat.z.ai/`, 2026-10-12

- **(1) Slot: (b)** — lokaler Momentum-Transfer, signed source/sink-Paar, gated durch die Region, wo
  beide Kanäle koexistieren. (a) verworfen: φ ist Probe-Input eines linearen Aggregats; quadratischer
  Querterm doppelzählt in `omega_sum` oder erzeugt freie Asymmetrie; vermischt Observable mit Dynamik.
  (c) verworfen: ein globaler relaxierter Skalar, keine räumliche Struktur, kein Vorzeichenpaar.
- **(2) Erhaltung:** ein Registry-Eintrag pro **ungeordnetem** Paar; canonical ordering (min/max der
  Deskriptor-Hashes) → A→B und B→A können keine verschiedenen Koeffizienten halten. Eine Rate + beide
  Dichten; Rückwärtsrate herleiten `ν_ni := (ρ_i/ρ_n)·ν_in`. Einmalige Auswertung, signed:
  `P = ρ_i ν_in (u_n − u_i)`; `+P` Ion, `−P` Neutral. **Runtime-Invariante:** `Σ_c ρ_c·Δu_c ≡ 0` über
  den Kopplungsblock. **Kernel-Form-Caveat:** `C·φ_from·φ_to` ist **reaktions-förmig**; Reibung ist
  **differenz-förmig** (`rate·(field_B − field_A)`) → der `kind`-Kanal trägt beide.
- **(3) Grammatik:** `coupling kind: friction|reaction|flux · pair {A,B} unordered · rate: ν_in|pending ·
  frame: A · density: ρ_A↦fieldref, ρ_B↦fieldref · region: overlap mask · status pending|bound`.
  Hergeleitet beim Bind/Run: `ν_ni`, Vorzeichen ±1. **Explizit nicht gespeichert:** Richtungs-Koeffizienten,
  Vorzeichenfelder, unabhängige `ν_ni`.
- **Unsicherheiten:** was φ kodiert (Skalar-Amplitude vs. Vektorkomponente); Dichten statisch/dynamisch
  (field refs Pflicht); explizit vs. implizit (implizit braucht symmetrisches Sweeping); Kopplung→ω()
  Leakage (nur Diagnose-Tap, nie Kontroll-Input, sonst kehrt (c) durch die Hintertür zurück); Region-Mismatch
  (einseitiger Term = Momentum-Leck).

## Kanäle ohne verwertbare Antwort (gemessen)

- **Duck.ai (GPT-5.6 Luna)** — `https://duck.ai/`: Prompt blieb nach zwei Versuchen (Selector + Klick,
  dann fokussiertes Keyboard-Typing + Submit) in der leeren Landing hängen (DOM unverändert, kein
  Chat-View) → `pending`, benannter Grund (kein Gespräch geöffnet).
- **`open-weight-ui` (geteilt)** — in dieser Runde **nicht** gefahren (Lock frei); Lücke, nicht
  Vollständigkeit. Nächste Runde.

## Konvergenz (3 von 3 antwortenden Seats)

- **Slot (b):** PDE-Quellterm im Feld-Eval (`field_spatial`), nicht (a), nicht (c). (c) höchstens als
  read-only Diagnose-Tap.
- **Erhaltung per Konstruktion:** ungeordnetes Paar (canonical hash), **eine** gerichtete Rate `ν_in` +
  beide live Dichten, Rückwärtsrate `ν_ni = (ρ_i/ρ_n)ν_in` **hergeleitet, nie gespeichert**;
  signed `+P/−P` aus **einer** Auswertung pro Zelle (skew-symmetrisch, keine Atomics); Runtime-Invariante
  `Σ ρ_c Δu_c ≡ 0`.
- **Riss gegen den früheren Rat:** dessen `coupling_term = Σ C_k·φ_from·φ_to`, „additiv im ω()-Loop" hat
  für Reibung die **falsche Form** (Produkt statt Differenz). Produktform bleibt als eigener `kind`
  (reaction/mass-action); Reibung = Differenzform. Das ist die zentrale Rat-Frage dieser Runde.
- **Grammatik-Minimum:** `kind` · ungeordnetes Paar `{a,b}` · `rate ν_in|pending` · `frame` · Dichte-Refs
  `ρ_a,ρ_b` · `region` · `conserves` · `status`. `interface <from> <to> <cond>` bleibt separat (Randbedingung).
