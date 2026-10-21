<!--
  title: Farbfeld-Baseline mit Staub-Abzug — Planck DL07 auf einem 1,5-Grad-Feld
  class: survey
  date: 2026-10-21
  sha256: 2ef3187778ee413afb69a4831843d8bc599b15f7a0b82517e8b5bd38f3ca25ba
  status: live
  owner: mountain
  ack: mountain 2026-10-21T12:25Z
  see-also: tools/measure/src/bin/deredden_baseline_probe.rs phi/sources.φ
-->
# Farbfeld-Baseline mit Staub-Abzug — Planck DL07 auf einem 1,5-Grad-Feld

**Frage.** Trägt eine Kette „Staubkarte E(B−V) → Deredden → lokale
Farb-Baseline → Ausreißer" ein echtes Feld, und wie viele Farb-Ausreißer
überleben den Staub-Abzug? Geprüft wurden zuerst die im Haus liegenden
Karten, dann die registrierte Planck-Karte; gerechnet wurde ein
1,5°-Feld um M31-Vordergrund (RA 10,68°, Dec 41,27°, Radius 0,75°) mit
dem Gaia-DR3-Sternbin `sources/ssd.jpl.nasa.gov/dr3_stars.bin`
(1 704 587 Sterne, 44-B-Legacy-Record).

## Kanäle (gemessen, nicht angenommen)

- **Lokaler Bestand:** `sources/irsa.ipac.caltech.edu/` trägt kein
  `planck_dust_av_rq_n512.json`; im ganzen Werkstattbaum liegt keine
  `*.be19`-Karte und keine SFD-Karte. Der Bayestar19-Pfad des Probes
  (`--map`) bleibt damit ohne Eingabe.
- **Register:** `phi/sources.φ:10208` registriert
  `planck_dust_av_rq_n512.json` (Planck DR2 DL07 A_V, IRSA-Origin). Die
  Karte wurde aus dem registrierten Release geladen:
  `https://github.com/omegaflow/sources/releases/download/irsa.ipac.caltech.edu/planck_dust_av_rq_n512.json`
  — 279 038 534 B, 3 145 728 Zellen (nside 512), sha256
  `49da12909cf43e77ff72e654fb749a5423dd7bcef5823726582c104cecba21bc`.
- **Karte ist 2D:** `av_rq` ist A_V entlang des ganzen Sehstrahls, nicht
  auf die Parallaxendistanz abgeschnitten. Der Lauf ist ein
  Vordergrund-Test, kein 3D-Entzerrungsresultat.

## Lauf

```sh
cargo build --release -p omegaflow-measure --bin deredden_baseline_probe
./target/release/deredden_baseline_probe \
  --dust-json /tmp/opencode/planck_dust.json \
  --stars ../omegaflow-work/sources/ssd.jpl.nasa.gov/dr3_stars.bin \
  --ra 10.68 --dec 41.27 --radius 0.75
```

A_V wird mit A_V = 3,1 E(B−V) in E(B−V) gewendet; die Farb-Rötung folgt
dem Hausgesetz E(BP−RP) = A_V/2,429 (Wang & Chen 2019, ApJ 877, 116).
Die Baseline ist der Median der intrinsischen BP−RP0 der übrigen Sterne,
die Skala die MAD-Sigma, die Schwelle Median ± 3σ; der eigene Stern geht
nie in sein Fenster ein.

## Zahl

| | Wert |
|---|---:|
| Staubzellen im Feld | **134** |
| E(B−V)-Bereich | **0,0662 … 0,5728** |
| E(B−V)-Median | **0,1605** |
| Baseline-Median, beobachtet BP−RP | **1,1510** |
| Baseline-Median, intrinsisch BP−RP0 | **0,7877** |
| Sterne im Feld | **96** |
| Ausreißer nach Staub-Abzug | **3** |
| typisch / pending / absent | **93 / 0 / 0** |

Die drei Ausreißer liegen bei +4,54σ, +6,03σ und +3,04σ über der lokalen
intrinsischen Baseline; alle drei sind rot (BP−RP0 2,13–3,47) und bleiben
auch nach dem Abzug rot. Der Staub erklärt in diesem Feld die roten
beobachteten Farben nicht vollständig.

## Was quer liegt

- Die Karte ist **nicht lokal vorgehalten**, nur registriert; der Lauf hing
  am Netz-Release. Ohne diesen Abruf wäre der Befund `pending` (keine
  `*.be19`, keine SFD, kein Planck-JSON im Bestand), nicht `absent`.
- Der 2D-Schnitt mischt Vordergrund und M31-Halo. Für Sterne hinter der
  Scheibe ist A_V überschätzt, für nahe Vordergrundsterne unterschätzt;
  die Baseline ist deshalb ein Vordergrund-Test, keine Entfernungsleiter.
- `av_rq` trägt einzelne Ausreißerzellen (Vollhimmel bis A_V ≈ 650);
  im 1,5°-Feld bleibt der Bereich mit 0,066–0,573 plausibel.
