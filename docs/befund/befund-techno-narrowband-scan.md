<!--
  title: Befund — Radio/Laser-Narrowband-Kanal
  class: befund
  date: 2026-09-05
  status: done
  sha256: dcbbfe2213f508337d8f86080d45b845076c93061309cee60049cc9a9ee34f32
  antwortet-auf: docs/auftrag/auftrag-techno-narrowband-scan.md
-->

# Befund: der Radio/Laser-Narrowband-Kanal

## Gemessen

HTTP-geprüft (2026-09-05). **Live-TAP mit echten Samples:** ASTRON VO TAP
(`vo.astron.nl/__system__/tap/run/tap/sync`) — Radio-Kontinuum-Kataloge
(lotss_dr2/dr3, lofartier1), aber **keine Narrowband-Spektren**; ESO TAP
(`archive.eso.org/tap_obs/sync`) — Metadaten hochaufgelöster Spektren
(harps, espresso, uves, crires, nirps). **Portal-nur (Konto/SPA):** NRAO,
MeerKAT/SARAO, LOFAR LTA, Keck/KOA. **Pending (kein öffentlicher Endpoint
verortet):** Breakthrough-Listen-Bucket, FAST, APF/Levy. CASDA-TAP 404 (void).

## Wire-Format

Eine Linie ist registerfähig: `val` (Linienfluss), `freq` (Slot 23),
`bin_width` (Slot 24) + ICRS-Position. Präzedenz: `merlin_frequency_hz`,
`first_radio_*`, `alfalfa_hi_flux`.

## Urteil

Der Kanal trägt derzeit **keine** Narrowband-Serie im Bestand — nur
Kontinuum. Der Linien-Kandidat bleibt `pending`, nie „still". Probe-Entwurf:
Linie = Überschuss über lokales Kontinuum + natürliche Linien-Null, fam-Band
über der lokalen Spektral-Null.

## Offen (registerfähig)

Der engeband-Kanal braucht einen echten Narrowband-Spektren-Dienst; bis dahin
ist er eine Fähigkeit (Line-Aufnahme im 26×f64-Wire), kein Fund.
