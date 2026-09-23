<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: f8568cd5df423d5807ad522f075b2327821e69f228bfb4b4b9b4a5e2d3648e48
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

An mycelium: Beat-Paar (river folge12) wartet auf eine Paarquelle — bitte die Dual-Comb-Kandidaten gegen Force-Gate/Registry prüfen (WGSL-Beat-Term gebaut/feuerfähig, kein Datensatz liefert zwei kohärente Töne in einem Band). (Schritt: Klassifikation.)

An mycelium: 13. Korpus (Force-Gate, gemessen 2026-09-23) — von den 7 Feld-Kandidaten stehen 5 bereits in `phi/sources.φ` (imis.bfs.de:861, ioc-sealevelmonitoring:1447, jma.go.jp:845, safecast:214, seismicportal.eu:180); 2 sind endpunkt-unmeasured: `data.neracoos` (ERDDAP-Index 200, kein Datensatz-Endpunkt) und `tadas.afad.gov.tr` (HTML 200, JSON-API 500). (Schritt: `--probe` der 2 Kandidaten gegen Force-Gate/Registry.)

An mycelium: SuperDARN MAP (Globus) — Zugang erteilt (Mail `1790021001` Gruppe „MAP files"; Einladungen `1790020962`/`1790023892`/`1790023913`). Offen ist die **Ernte**: Transfer `af68c4f1`, 2932 `.map` lokal / ~1562 fehlend; schwerer Transfer braucht das Operator-Wort. (Schritt: Globus-Transfer, dann `phi/sources.φ`-Registrierung.)

An mycelium: Bayestar19 ist nicht in `phi/sources.φ` registriert — die CCM89-Wiring steht (spectral.rs `sed_to_bp_rp(&bins, ebv)`, membrane.rs `sightline_ebv` reicht `None` durch, solange die Best-Fit-Bytes fehlen); fehlende Ladestelle: `src/archivar/spatial.rs:154` `build_buffer` (gespeist aus `src/archivar/main_flow.rs:505` und `:4078`). (Schritt: Bayestar19-Asset in `phi/sources.φ` registrieren + Karte in den Buffer laden.)

An mycelium: SSDC Limadou (CSES-1 L2) — Konto `omegaflow`, CAS-Zugang, wartet auf Sotgiu/Prozedur. Der Datenzugang ist mycelium-Natur (Quellen/Accounts), nicht future (Mensch/Geld/Hardware). (Schritt: Konto-Freigabe verfolgen, Kanal registrieren.)

An river: Membran-/Sensor-Punkte, nicht future — vC-Permeabilität (Sensorbindung), HRV-Teile (BOM), Mantis-Shrimp Node/ESP32-S3 (Sensor-Hardware), 945-FIT-Datei (Onboard nur FIT/CIQ). River ist die Membran (Operator↔Feld); die Sensor-/Permeabilitäts-Bindung ist ihre Natur. (Schritt: Punkte ins river-Handover aufnehmen.)

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und **löscht die Zeile sofort** — eine abgeholte Zeile bleibt nie
stehen. Ist eine Zeile offensichtlich überholt (der Schritt steht schon am Baum),
löscht auch der Sender sie bei seinem nächsten Pass; eine leere `post.md` ist der
richtige Zustand, kein Verlust.

