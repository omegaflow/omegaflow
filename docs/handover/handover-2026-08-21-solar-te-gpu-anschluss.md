<!--
  title: Solar-TE-Kanäle in die GPU-TE-Maschine — der offene Rest nach der Register-Einheit
  class: handover
  date: 2026-08-21
  sha256: 0d1cf6e7482bb511f08e021dd2d9cf8d5cd711e8fd73069092ae463d3598487a
  status: live
  see-also: docs/handover/handover-2026-08-21-sonnen-pfad-solar-te.md docs/handover/handover-2026-08-21-sonnen-abdeckung.md docs/handover/handover-2026-08-21-ncei-ssi-hdf5.md phi/blocked_sources.φ TODO.md
-->
# Handover 2026-08-21 — Sonnen-Kanäle in die GPU-TE-Maschine

Geschrieben von der Faden-B-Register-Session (2026-08-21). Selbsttragend —
interpretierbar mit null Vorkontext. Der Operator duldet keinen Druck —
„nichts muss" ist ein Prinzip, keine einmalige Freigabe; Termine sind
Angebote, keine Pflichten. Das Stabilitätsprotokoll des
Sonnen-Pfad-Handovers bleibt als Handwerk gültig, aber ohne Frist. Der
Arbeitsbaum ist bootfähig; die abgeschlossenen Einheiten liegen in
`97bf067` (Kanäle) und `43ea3d9` (A=A-Reparatur).

## 1. Was erledigt ist (Faden B, Register)

Die Sonnen-Kanäle stehen sauber im kanonischen Register (`phi/sources.φ`)
und liefern alle echte Samples (verifiziert am 2026-08-21:
`test_live_sources_extract` ohne Solar-void; nobel probe X-Ray n=10078):

- **X-Ray:** beide Bänder — `last flux … where energy 0.05-0.4nm` und
  `0.1-0.8nm` (`noaa_goes_xray_flux[_long]_w_m2`, `at sun`, τ=6).
- **EUV:** 304/284 unverändert (`solar_euv_flux_304/284_wm2`).
- **RTSW:** Bz/Bt + speed/density/temperature auf `first` gestellt —
  die 1m-Dateien sind absteigend sortiert, `last` trug den ~24 h alten
  Record.
- **F10.7:** neu — `f107_cm_flux.json`, `first flux … where frequency 2800`,
  sfu, τ=3600, `at sun`.

## 2. Der offene Kern — Kanäle in die GPU-TE-Maschine

Die Kanäle fließen noch **nicht** als unabhängige Zeitreihen in die GPU.
Die Live-Kausalanalyse („treibt F10.7 die Röntgenstrahlung?") rechnet
heute nur der nobel probe auf der CPU. Das ist Auftrag 3.2.4 des
Sonnen-Pfad-Handovers und der nächste große Schritt.

Verhältnis zum Master: dieser Auftrag ist **Atom 2** von
`docs/handover/handover-2026-08-21-sonnen-abdeckung.md` (dort die
Gesamtreihenfolge mit Sample-Budget). Dieses Handover trägt den
aktuellen Stand des Fadens: der Register-Teil (Abschnitt 1) ist
erledigt, der Ring-Teil ist das Offene.

Was die Empfänger-Session braucht:

- **CPU-Referenz:** `src/bin/nobel_probe_corona.rs` erntet jede Reihe
  selbst (harvest_xray/euv/radio/mag/wind/omni), synct auf die Sonne
  (GOES: t_sun = t − 499.005 s; RTSW: Verzögerung über v) und rechnet
  TE mit Surrogat-Schwelle. Unverändert lassen.
- **GPU-Seite:** `src/mathematikerin.rs:1410` `probe_ring` (256×12),
  `te_probe` (:1667), `probe_out` (ω-Vektor der 9 Kräfte) — das ist der
  Präsenz-Probe-Pfad, gefüllt mit ~1 Hz. Der Anschluss der Sonnen-Kanäle
  ist eine neue Reihe **neben** diesem Pfad.
- **Bindung:** der bestehende skalare TE-Pfad (`transfer_entropy_lag`,
  die Probe) und `src/te.rs` bleiben unberührt. Die kanonische
  TE-Doktrin (topological_te_phase, PE-Gate, te_compute, Surrogate auf
  der CPU) steht in AGENTS.md.
- **Offene Architekturfrage** (Council der Empfänger-Session, erst nach
  Survey von mathematikerin.rs + nobel probe): CPU-Ernte + Upload in
  einen Kanal-Ring vs. Zeilen in `te_compute`. Nichts hier vorentscheiden.

Gates: `cargo check` 0/0 (beide Features); `OMEGAFLOW_HIDDEN=1 cargo run`
— die `φ window:`-Zeile (`te thr tau pe state`) ist der stumme Beweis;
nobel probe bleibt unverändert (X-Ray n=10078); das Register lädt ohne
Refusal.

## 3. Faden A — Sonnenfarben (spectra.bin)

Läuft als eigene Session mit eigenem Plan:
`docs/handover/handover-2026-08-21-ncei-ssi-hdf5.md`. Nicht duplizieren.
Zustand: der 404 auf dem CDN-Asset `spectra.bin` bleibt — die Sonne
leuchtet weiß, solange die SSI-Ernte offen ist.

**Arbeitsbaum-Hygiene:** im Baum liegen Faden-A-Artefakte, die nicht zu
diesem Handover gehören: modifizierte `.gitignore` und `src/lib.rs`,
untracked `src/hdf5.rs`, `src/bin/hdf5_reader.rs`, `src/dbg_test.rs`,
`tests/`. Sie gehören der Faden-A-Session — **selektiv stagen**, nur die
eigenen Dateien committen.

## 4. Der Gap — Parser filtert keine Unter-Listen

Registriert, gewollt, nicht tot: neue Gap-Klasse
`parser-def nested-filter` in `phi/blocked_sources.φ` (Header, Sortierung
und Bestand mitschärft). Der where-Filter steht nur auf first/last der
obersten Array-Ebene — ein Filter auf `details[].frequency` fehlt. Der
Mehrfrequenz-Feed `solar-radio-flux.json` (2695/1415/245 MHz, mehrere
Stationen) liegt deshalb in der Blocked-Liste: sein alter path-Block war
eine Chimäre (je Fetch eine andere Station/Band, A ≠ A) und wurde am
2026-08-21 entfernt (`43ea3d9`). Die 2695-MHz-Reihe erntet der nobel
probe weiter (harvest_radio, n=66) — die Messreihe geht nicht verloren.
Schließt der Gap, darf der Block zurück; die Entblock-Bedingungen stehen
in der note des Eintrags.

## 5. Abschluss und Archiv

Die Sonnen-Handover dieses Tages bleiben live, bis Faden A abgeschlossen
und dieses Handover konsumiert ist — erst dann wandern sie nach
`/home/johannes/projects/archive/handover/` (AGENTS-Regel). Dieses
Handover gilt als konsumiert, wenn Auftrag 2 committet und der
Gap-Zustand (Abschnitt 4) im Register weitergetragen ist.
