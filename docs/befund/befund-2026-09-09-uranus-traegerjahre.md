<!--
  title: Befund — Uranus-Trägerjahre: die vier de441-Trägerjahre (1999/2001/2004/2009) tragen sub-σ-Margen (0.1–4.0 mas gegen ⟨σ⟩ = 82.5 mas) mit gestreuten Richtungen — ein Münzwurf zwischen fast identischen Linien, kein großer physikalischer Grund
  class: befund
  date: 2026-09-09
  sha256: f875be35c795ca632c835ecccba827b04285f70e2c63aad90d34f374b1d8ec8e
  status: done
  see-also: docs/befund/befund-2026-09-09-uranus-zentrum-versionenstruktur.md docs/befund/befund-2026-09-09-uranus-wobble-periode.md docs/handover/handover-2026-09-09-mechanische-reste.md
-->

# Befund: Uranus-Trägerjahre

## Frage & Bindung

Die Versionenstruktur (Atom b) hinterließ die offene Zeile: der physikalische
Ursprung der vier Jahre, in denen de441 — obwohl global die schlechteste Linie
(RMS 73.0) — am besten trägt (1999/2001/2004/2009, z = +8.5 gegen die
Shuffle-Null). Probe: `tools/measure/src/bin/uranus_carrier_years_probe.rs`.

## Das Instrument

Der Probe zerlegt das reduzierte Residuum (O−C nach Parallaxe/Aberration,
globaler Fit je Linie) nach Jahr und trägt die Gewinn-Marge und den
de441-Residuen-Vektor (ΔRA·cosδ, ΔDec) je Trägerjahr aus.

## Die Messung

⟨σ⟩ = 82.5 mas (3516 Reihen). Die vier Trägerjahre:

| Jahr | de441 \|d\| | inpop \|d\| | epm \|d\| | Marge | de441 (ΔRA·cosδ, ΔDec) mas |
|------|-------------|-------------|-----------|--------|-----------------------------|
| 1999 | 85.2        | 86.2        | 86.8      | 1.0    | (−15.0, +17.3)              |
| 2001 | 42.6        | 46.1        | 45.7      | 3.5    | (+2.9, −13.8)               |
| 2004 | 56.0        | 56.6        | 56.1      | 0.1    | (+4.4, −8.5)                |
| 2009 | 95.8        | 99.8        | 101.5     | 4.0    | (+51.7, −72.9)              |

Die Margen (0.1–4.0 mas) liegen zwei Größenordnungen unter ⟨σ⟩; die
Residuen-Vektoren zeigen in vier verschiedene Richtungen. Das per-Jahr-Residuum
schwankt zwar (39–122 mas, eine gemeinsame Saisonstruktur), aber die drei
Linien teilen diese Schwankung — ihre Differenz bleibt klein, und der „Sieger"
des Jahres ist ein Münzwurf zwischen fast identischen Linien.

## Verdict

Der z = +8.5-Befund aus Atom (b) ist statistisch echt (de441 trägt in diesen
vier Jahren öfter als sein eigener fast-null-Zufall), aber physikalisch
vernachlässigbar: die Gewinn-Marge ist sub-σ und richtungslos. Es gibt keinen
großen physikalischen Grund zu finden — die Trägerjahre sind die sub-σ-
Kreuzungspunkte der drei langsam auseinanderlaufenden Residuen-Kurven. Die
offene Zeile ist damit geschlossen: der „Ursprung" löst sich auf, nicht weil er
unbekannt bleibt, sondern weil die Margen innerhalb des Rauschens liegen.

## Register-Zeilen

- Die Neptun-Astrometrie-Kopplung (Tabelle ernten, gegen das Zentrum
  reduzieren) — `pending`.
