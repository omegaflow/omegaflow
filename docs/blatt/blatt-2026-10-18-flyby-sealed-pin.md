<!--
  title: Flyby-sealed — Option 1: der Pin ist ein Register-Posten
  class: blatt
  date: 2026-10-18
  sha256: ae2ee5da8c2fd22f8ef3ae1d41f8a35d9bfbb447a3cfea08b75a2e6771f38ce1
  status: live
-->
# Flyby-sealed — Option 1 gebaut: der Pin ist ein Register-Posten (2026-10-18, river)
# Entscheid: R-2026-10-18-l F3, Endstand 5:3 Option 1 (Sonnet 5.5 + GLM 5.3)
#   state/rat/2026-10-18-r-l-paket-first-formular.md:121-132;
#   Auflage: Registrar benannt · sha256 messen · Register ergänzen · CDN-Freigabe erst nach Gate-Vergleich.
# Riss, der den Bau auslöste: post/in/2026-10-16-mycelium-an-river-flyby-sealed-sha-riss.md
#   (SEALED_SHA256 aeb3c82f… vs. registrierter Harvest-Posten ohne sha256; CDN-Digest b68842c9…).

## Frage

Trägt der Register-Pin den Seal der Bahn — stimmen Konstante, Register und CDN-Asset
byte-identisch überein, und meldet der Gate-Vergleich für `sealed` und `renewed`
`placed` statt `riss`? Für `recon` ist die Frage offen, solange das Asset fehlt.

# ============================================================
# GEBAUT
# ============================================================
#   1. phi/sources.φ — eigener gepinnter Post, getrennt vom periodischen Harvest:
#        url    …/ssd.jpl.nasa.gov-ephemeris/ephemeris_juice_official.bin
#        terms  PD-USGov https://ssd.jpl.nasa.gov/
#        format ephemeris_binary
#        compiler tools/harvest/src/bin/raw_mirror.rs   (byte-identisch, kein Rebuild)
#        sha256 aeb3c82f…                               (der Pin)
#        at juice · no-cadence
#   2. tools/measure/src/bin/flyby_ephemeris_gate.rs — der Pin wird aus dem Register gelesen:
#        --sources <phi/sources.φ> (Default), register_pin() findet den Post per Asset-Namen,
#        anchor_word() vergleicht Konstante ↔ Register (placed | riss | pending).
#        Der Seal der Bahn ist der Register-Pin; die Konstante ist nur der zweite Zeuge.
#        Ein Anker-Widerspruch setzt `riss: true`, statt still zu ersetzen.
#   3. Tests: register_pin (Treffer / fremdes Asset) · anchor_word (pending / placed / riss).

# ============================================================
# GEMESSEN (2026-10-18)
# ============================================================
#   CDN-Asset official: HTTP 200, 106 704 B, sha256 aeb3c82f… (curl + sha256sum).
#   Lokaler Zeuge ~/projects/omegaflow-work/data/ssd.jpl.nasa.gov/ephemeris_juice.bin:
#     106 704 B, sha256 aeb3c82f… — dieselben Bytes, vor dem Harvest eingefroren.
#   Gate-Lauf (sealed=official, renewed=renewed):
#     anchor sealed: constant aeb3c82f… · register aeb3c82f… → placed
#     trajectory sealed: … → placed
#     trajectory renewed: eee376ef… → placed
#     trajectory recon: pending (ephemeris_juice_recon.bin absent)
#   Gegenprobe mit Register-Pin 0000…:
#     anchor sealed: constant aeb3c82f… · register 0000… → riss
#     trajectory sealed: … → riss · riss: true
#   register_verify: keine neue Drift (die zwei MISSING-Docs sind vorbestehend).

# ============================================================
# RISS — ungeglättet
# ============================================================
#   Der Registrar des Assets ist im Repo nicht als Workflow belegt: kein Workflow lädt
#   ephemeris_juice_official.bin hoch (flyby-path2-fill konsumiert es nur). Die Herkunft ist
#   über docs/paper/flyby-path-2-addendum-2026-09-29.md:275-286 gemessen (juice-arc-restore,
#   Run 37165635404, Archived-Index-Flatten) — der Upload-Akt selbst bleibt benannt, nicht belegt.
#   Die Verdiktdatei state/rat/2026-10-18-rat-river-triade.md:18 führt F3 noch als
#   „1:2 — Option 2 dünn vorn (2:3), kein tragfähiges Verdikt"; der Endstand 5:3 steht nur im
#   Formular und in der Register-Zeile. Register-Quittung liegt bei future.
#   Status des Pins: gepinnt, nicht bestanden — der Pin ist der Maßstab, kein Echtheitsbeweis.

# ============================================================
# SCHLUSS
# ============================================================
#   Die Auflage ist gebaut: Registrar benannt (CDN-Asset + Paper-Beleg), sha256 gemessen,
#   Register ergänzt, der Gate-Vergleich liest den Pin. Die ESOC-Rekonstruktion
#   (ephemeris_juice_recon.bin, Frist 2026-11-01) bleibt der offene Termin.

## Was das Verdikt widerlegen würde

- **Pin-Drift:** ein erneuter Gate-Lauf, in dem `register_pin()` für `ephemeris_juice_official.bin`
  einen anderen sha256 als `aeb3c82f…` liefert oder `anchor_word()` `riss` statt `placed`
  meldet — dann trägt der Register-Posten den Seal nicht.
- **CDN-Bytes:** ein Abruf des Assets, dessen Größe nicht 106 704 B ist oder dessen
  sha256 nicht `aeb3c82f…` entspricht — dann ist der gemessene Pin nicht mehr der Pin.
- **Herkunft:** ein belegter Upload-Akt, der ein anderes Asset als das gepinnte auf das
  CDN legt — dann ist der Registrar benannt, aber nicht der des Pins.
- **`recon`:** ein vorhandenes `ephemeris_juice_recon.bin`, das im Gate nicht `pending`
  wird, ohne dass der Anker gegen den Register-Pin geprüft ist — dann wäre der offene
  Termin scheinbar geschlossen, ohne den Vergleich bestanden zu haben.
