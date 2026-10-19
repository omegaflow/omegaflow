# UI-Stimmen — Mycelium-Ratsfrage (2026-10-10)

Runde: Mycelium-Linie, Gruppe `mycelium-ui` (JIT, focus:false, nach der Runde geschlossen).
Prompt (identisch an alle): vier Architektur-Verdikte des internen Rats (fünf Stimmen) —
(1) Planetary Radar → 26×f64, (2) Object-Storage-Tiering, (3) lokaler no-leak MCP,
(4) CI-Wall-Clock — bestätigen oder reißen. Vorbereitung:
`state/mycelium/rat-vorbereitung-2026-10-10.md`; Rat: fünf Stimmen (2026-10-10).

**Rohmaterial — die Session verifiziert die Claims am Baum, nie die Stimme.**

## Qwen3.7-Plus — `chat.qwen.ai` — bestätigt alle vier
- **Radar:** bestätigt. Delay/Doppler sind distinkte Größen (s vs. Hz) mit eigenen
  Fehlermodellen; getrennte Records + Metadaten (Epoch, Zielkörper, Station) = SI-Korrektheit
  und Reproduzierbarkeit. Zwei Skalare in ein `val` falten verletzt die Normalisierung.
- **Storage:** bestätigt mit Risiko-Warnung. R2 ohne gemessenen Konsumenten = premature
  optimization (YAGNI). Aber die 1,76 TB auf GitHub Releases als **kritische technische
  Schuld** markieren — ein plötzlicher Account-Ban bricht die Pipeline.
- **MCP:** bestätigt. no-leak = local/stdio + exakter Pin + Repo-Scope zwingend; Netz-Arm
  bricht die Air-Gap; chrome-devtools-MCP kein valider Präzedenzfall.
- **CI:** bestätigt. Rebuild je Shard = massive Wall-Clock-Verschwendung; `cargo nextest
  archive` + Fan-Out ist Best Practice; Messung vorher.

## DeepSeek Chat — `chat.deepseek.com` — bestätigt alle vier (Antwort in Chinesisch)
- Vierfach „认可" (bestätigt); Begründung deckt sich (Session-Destillat): Delay/Doppler
  orthogonal, atomare Trennung richtig; R2-pending richtig (kein gemessener Konsument);
  no-leak-Regel richtig; 4× `--release`-Rebuild = Wall-Clock-Verschwendung, archive+Fan-Out
  richtig, Messung Voraussetzung.

## Gemini 3.1 Pro — `aistudio.google.com` — bestätigt alle vier
- **Radar:** bestätigt. Unterschiedliche Dimensionen/Sigma/asynchrone Kontexte; Falten bricht
  „Single Source of Truth".
- **Storage:** bestätigt. YAGNI; Multi-Tier ohne Leselast = Overengineering.
- **MCP:** bestätigt. no-leak erfordert hermetische Isolierung; chrome-devtools-MCP
  (HTTP/WebSocket) ist ein inhärenter Leak-Vektor, kein Präzedenzfall.
- **CI:** bestätigt. „Build once, test anywhere" via `nextest archive` + Fan-Out löst den
  Flaschenhals.

## Claude Sonnet 5.5 (Extra hoch) — `claude.ai` — bestätigt, mit präzisierenden Ergänzungen
- **Radar:** bestätigt, **mit Ergänzungen.** Ein Skalar pro `val` hält die Einheit eindeutig.
  Delay ist Round-Trip und **bistatisch** → **Transmitter** mitführen, nicht nur Receiver;
  **Epoch-Konvention** (Sende- vs. Empfangszeit, Zeitskala) und **Bezugspunkt** (Massenzentrum
  vs. Oberfläche) in den Record. Doppler pending richtig — er braucht die Sende-Referenzfrequenz.
- **Storage:** bestätigt. Pending an einen **konkreten Trigger** binden (größte Datei nähert
  sich 2 GiB, oder Releases-Lesezugriffe werden messbar zum Engpass); **content-addressed Keys**,
  damit ein späterer Umzug ohne Schemaänderung geht. ToS-Risiko ⇒ Portabilität, nicht sofortiger Bau.
- **MCP:** bestätigt, aber **„kein Netz-Arm" muss erzwungen, nicht deklariert sein** —
  Egress-Deny (Netz-Namespace/Container ohne Route) + FS-Scope, je mit Testfall; Pin per
  **Integrity-Hash/Lockfile**, nicht nur Tag.
- **CI:** gehalten, **mit Messlücke.** Archive-**Upload/Download in die Messung** nehmen; Fan-Out
  verlängert den kritischen Pfad (Build + Transfer + längster Shard) — bei großen Artefakten
  kann das den Gewinn auffressen. Gegenprobe: teilen die Shards ohnehin einen warmen Cache
  (sccache/rust-cache), ist das Rebuild-Problem dort billiger zu lösen. nextest führt **keine
  Doctests** aus (eigener Schritt). Dedup als zweite, getrennt eingeführte Achse.

## Z.ai GLM-5.3 — `chat.z.ai` — `pending` (gemessener Grund)
Nach der Runde lag keine Antwort vor (nur „Deep Think"/„Max"-Steuerung, Deep-Think-Modus
rechnet noch). **Nicht als stille Null gelesen** — `pending`, kein Verdikt.

## Geteilte Open-Weight-Seats — nicht gefahren
Gruppe `open-weight-ui` war leer; kein Seat geöffnet (Lock nicht beansprucht). `pending`
für die nächste Runde.

## Verdikt der Session (nach Baum-Verifikation)
Die fünf Stimmen **konvergieren**: alle vier Rat-Verdikte bestätigt; **Claude** liefert die
tragenden Verschärfungen (bistatischer Transmitter/Epoch/Bezugspunkt im Radar-Record;
erzwungener Egress-Deny + Integrity-Pin für MCP; Archive-Transfer in die CI-Messung). Kein Riss
zwischen Rat und Roster — die Architektur trägt; die Schritte bleiben begrenzt (Rat-Empfehlung).
