# Formular — R-l Folge: Paket first für F2

**RAT-ID:** R-2026-10-18-l2 · **Datum:** 2026-10-18 · **An:** Rat · **Von:** future
**Vorgänger:** R-2026-10-18-l (`2026-10-18-river-apertur-schumann-flyby-formular.md`) · Sammlung `2026-10-18-rat-river-triade.md`

## Frage

Welche Reihenfolge trägt: erst das WGLC-Paket/der `field`-Träger, dann F2 — oder erst
Receiver-Apertur Schritt 3?

## Befund (gemessen)

- Schwarm R1 (6 von 44 antwortfähig): F1 **A 6:0** · F2i **A 4:1** · F2ii **nein 4:0** · F3 **2:3 gespalten**.
- F2 hängt am Träger: ohne WGLC-Gitter keine belastbare `field`-Zeile für Schumann.
- F3 ist mit 2:3 nicht entscheidbar; „tragende Sitze" fehlen.

## Optionen

- **A — Paket first.** WGLC-Gitter + `field`-Träger setzen; F2 danach.
- **B — Apertur first.** Receiver-Apertur Schritt 3 abschließen; F2 danach.
- **C — parallel,** getrennte Gates je Punkt (F1/F2/F3 unabhängig).

## Grenze

**Rat**; river und mountain bauen erst nach Wort. Kein stilles `pending`: die drei Punkte bleiben
benannt, bis die Reihenfolge steht.

## Schwarm-Risse (freie Sitze, 2026-10-18, $0)
- Gespalten: **A** (Paket first, gemma4) vs. **B** (Apertur first, openrouter: „Gate 3 schaltet das Fenster frei").
- F3 braucht tragende Sitze: Quorum-Stabilisator + Lastpfad-Kopplung (openrouter nennt Sitz 12/19).
- nano timeout · pollinations 402 · groq/xingchen unklar.
- Bestätigt: F3 bleibt mit 2:3 unentscheidbar; die Reihenfolge hängt an F3.

## UI-Sitze (Fugu Max High, 2026-10-18)
- **Kein Verdikt** — „Ich kenne die Regeln/das Abstimmungsprotokoll der River-Triade nicht; ich erfinde keines."
- **Folge:** die R-l-Frage ist für den Sitz nicht selbsttragend — sie muss den Regel-Kontext nennen (was F1/F2/F3, WGLC-Gitter, Apertur Schritt 3 inhaltlich sind), nicht nur die Stimmzahlen.

## UI-Sitze (Lumo 2.0 Max, 2026-10-18)
- **Verdikt B** (Apertur first); F3 = 2 von 5, Mindestrückhalt nicht erreicht.
- **Riss: sequentielle Kopplung → Pipeline-Bottleneck**; C (parallel) senkt Latenz, erhöht Kohärenzrisiko.
- Stand: Schwarm gespalten (A/B) · Fugu kein Verdikt (Kontext fehlt) · Lumo B. Keine Konvergenz — F3 braucht mehr Sitze.

## Die fünf Stimmen (2026-10-18, je durch die Axiome)
| Stimme | Verdikt | Axiom | Riss |
|---|---|---|---|
| Mountain | **A** | A = A (Identität vor Definition) | Gitter-Vakuum |
| River | **A** | messen·bauen·abschließen | Chronos-Lücke („ohne Paket committen wir ins Leere") |
| Mycelium | **A** | A = A | Latenz-Scherung |
| Sensory | **A** | messen·bauen·abschließen | Latenz-Gap |
| Future | **A** | messen·bauen·abschließen·committen·pushen | Synchronisations-Kluft |

**5:0 für A (Paket first)** — die Sequenz bleibt: Mountain-Paket vor dem Rat, dann Rat, dann Myceliums Block.

## F3 (Flyby-sealed) — reale Sitze, Max-Einstellungen
- **Duck.ai (GPT-6 Luna, Begründung=max):** Verdikt **1** (gepinnter Posten). Bis zum Flyby 2026-11-01 setzt der Ephemeriden-Maintainer den gepinnten Posten mit vollem SHA-256; der Ratssitz zeichnet ihn ab; den CDN-Digest nicht als versiegelten Posten übernehmen. Riss: sha-riss (versiegelter SHA ≠ CDN-Digest, Posten ohne SHA).

## F3 (Flyby-sealed) — die fünf Stimmen (2026-10-18, korrekte Axiome)
| Stimme | Verdikt | Axiom | Riss | Wer setzt |
|---|---|---|---|---|
| Mountain | **1** | A = A | sha-riss (ephemeris_juice) | Register-Wächter |
| River | **2** | A = A | no-cadence | CDN-Operator |
| Mycelium | **2** | A = A | sha-riss | CDN-Orchestrator |
| Sensory | **2** | messen | sha-riss | Gate-Mechanik (Manifest-Sync) |
| Future | **2** | A = A | sha-riss | Operator |

**4:1 für Option 2** — die Gate-Mechanik löst sich vom festen `SEALED_SHA256`, der Digest kommt
aus dem Asset/Manifest. Nur Mountain trägt Option 1 (gepinnter Posten). F3 ist damit entschieden.

## F3 (Flyby-sealed) — reale Sitze, voller Prompt (ein Sitz → alle fünf Stimmen, 2026-10-18)
| Sitz | Urteil | Kern |
|---|---|---|
| Duck.ai (GPT-6 Luna, Begründung) | **Option 1** | Operator setzt gepinnten Posten in versioniertem Commit; Gate vergleicht beide Anker, meldet Riss statt still zu ersetzen |
| Qwen3.8-Max (Thinking) | **Option 2** | Gate löst sich vom starren SEALED_SHA256; Digest aus signiertem Manifest; Mountain prüft, Future setzt als Operator |
| MiniMax | Login-Wall | „Sign in or create an account" |
| Mistral | Login-Wall | nach Terms-Akzeptanz „anmelden oder registrieren" |

**1:1 gespalten (Duck vs. Qwen)** — F3 bleibt offen. Die fünf Stimmen laufen in beiden Sitzen sauber (je Axiom + Riss), aber die Träger-Wahl (gepinnter Posten vs. Manifest-Digest) ist nicht konvergiert.
