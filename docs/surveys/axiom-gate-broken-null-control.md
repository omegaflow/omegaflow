<!--
  title: Axiom-Gate-Survey — broken-null-control
  class: survey
  date: 2026-09-12
  sha256: 17d8620f300354982bfdd0a7cfcd25acf67b703bf1a1cd6c7a45bdff6660e616
  status: live
  see-also: docs/paper/broken-null-control.md
-->
# Axiom-Gate-Survey — broken-null-control

Das Paper `docs/paper/broken-null-control.md` (Titel 57/57,
Abstract 165/200, 126 Zahlen ok, sha ok — Export-Gate grün).

## A = A / Zahl

- Die Kaskade kollabiert unter dem Gate: 16 signifikante Pfeile → 2;
  alle vier Kontrollpaare brechen unter der naiven Null / halten unter der
  phasenrandomisierten; FP 100 % → 6.7 % (Halbkreis-RNG,
  `next_rng` dividiert durch `u32::MAX >> 1`, nie `u32::MAX`) — 1:1 aus
  der Spec getragen.

## Konsistenz

- Die offenen Stellen der Spec sind im Paper als Pendings benannt:
  max-T-Korrektur über die 20-Paar-Matrix, Lag-Sweep (nur τ ∈ {0, 60, 120}),
  Bandbreiten-Sensitivität (h-Abhängigkeit ungemessen), Fenster-Drift
  (naive/Phase-Läufe nicht auf identischen Daten), Rest-FN-Verzerrung bei
  n = 300 (0–3/10, `te_fn_probe`).
- Die Messung ist die Gate-Serie selbst — die Ein-Blatt-Form trägt sie als
  Messung, nicht als Methoden-Anhang.

## Pfad

- `src/mathematikerin/te.rs` (`next_rng`, Kalibrier-Gate: FP/FN/Symmetrie/
  n-Floor als Tests) — stimmen.

## Zuordnung

- see-also → `docs/specs/broken-null-control.md` (lebt).

## Externes Register

- 4/4 DOIs `resolved` (Schreiber 2000, Theiler 1992, Kraskov 2004,
  Frenzel & Pompe 2007).
- NASA-Bindung: keine Datenquellen erfunden — das Methoden-Paper benennt
  nur, was die Host-Papiere tragen.
