# ENTSO-E Intervall-Form — Max-Nachlauf (2026-10-13, mountain-340)

Der erste Durchgang lief im **Default-Modus** (Operator-Kritik zu Recht). Die Regel
„Search/Max ist die Pflicht vor dem Senden" (`docs/concepts/ui-seats.md:206`, Operator-Wort
2026-10-13) ist jetzt am lebenden Chrome gesetzt und **verifiziert**:

| Seat | gesetzt (verifiziert) | Antwort |
|---|---|---|
| Qwen (direkt, `chat.qwen.ai`) | Modell **Qwen3.8-Max**, Thinking **Denken**, Modus **Web-Suche** | **(c)** neuer Record-Typ — „Dummy-Koordinaten / Dummy-Werte für die abwesende Menge … verletzt die Kernbedingung" |
| Qwen3.8 **2.4T** (`tryingopen.com`) | Modell Qwen3.8 2.4T (größtes Qwen) | **(c)** — schärfer: „tau=0 hilft nicht, denn das Intervall selbst hat eine reale Dauer"; „Ein EIC-Gebiet ist ein Zuschreibungsraum, keine Koordinate" |
| DeepSeek Chat | **DeepThink** + **Search** beide `aria-pressed=true` | **(c)** — „**tau ist Bestandsdauer, nicht Ereignisdauer**. epoch=Start + tau=Dauer überlädt die Bedeutung"; abwesende Menge = `absent/NULL` |
| Z.ai (GLM-5.3) | Deep Think **Max** gesetzt | **gemessene Nichtantwort:** „Model is currently at capacity … peak hours … switch to GLM-5.3-Flash" |
| GLM via tryingopen | — | **nicht verfügbar:** Liste führt nur `GLM 5.2`, Zeile `disabled` („Adding capacity") |
| Claude | **Sonnet 5.5 Extra hoch** (Extended Thinking max) | **(a) + 2 Pflichtkorrekturen = faktisch (d)** |

## Der geschärfte Riss (nicht geglättet)

Die Max-Stimmen tragen einen Konfund, den der erste Durchgang (Default-Modus) nicht hatte:

- **Rat (Struktur):** `epoch`=Start + `tau`=Dauer, `val`=Menge, Gebiet+Code = Kanal-Qualifier.
- **Max-Roster:** `tau` ist **Bestandsdauer** (tau=0 = Tor zu), **nicht** Ereignisdauer — `epoch+tau`
  überlädt `tau`. Das Gebiet ist ein **Zuschreibungsraum**; es als bloßen Kanal-Qualifier an eine
  Punktmessung zu hängen erfinde einen Ort.
- **Auflösung am Kontrakt (Rat + Claude):** das Ereignis-Intervall ist der Zustand „die Menge gilt
  über [Start, Ende)". Damit ist die Menge über das Intervall konstant → `tau` = Erstreckung *genau
  dieser* Menge (bestehende, korrekte Semantik), keine Überladung. Die fehlende Menge trägt der
  `presence`-Slot; das Gebiet trägt die **deklarierte Receiver-Weltlinie**. Kein neuer Record-Typ.
- **Nicht geglättet:** der Einwand bleibt gültig, **wo** das Intervall ohne konstante Menge auftritt
  (reiner Ereigniscode, keine MW) — dann ist `tau` tatsächlich überladen und der Record bleibt ein
  reiner Existenznachweis. Offen, ob dieser Fall im Feed vorkommt (Messung am Compiler: `quantity:
  Option<f64>` — der Fall existiert, wird aber als übersprungen geführt).

## Offen

- Perplexity · Mistral · Google AI Studio · MiniMax · Lumo · Kimi · Duck (Begründungsmodus) ·
  Inkling · Nemotron 3 Ultra · GPT-OSS 120B · MiMo V2.6 Pro · Mistral Large 4 — Modell/Schalter
  noch **nicht** gesetzt, Frage **nicht** gestellt. Der Riss wird davon nicht umgeworfen (alle
  bisherigen Max-Stimmen sagen (c) bzw. (a)+Korrekturen).
