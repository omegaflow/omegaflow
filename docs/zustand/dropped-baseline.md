<!--
  title: Zustand — dropped-Baseline (register_lookup --dropped)
  class: zustand
  date: 2026-09-21
  sha256: 78333fe2322ee1a45eac95a8a2b4945cd686408c3cf9c7b08b88983f8caef0dd
  status: live
  see-also: AGENTS.md
-->
# Zustand — dropped-Baseline (`register_lookup --dropped`)

Delta-Gate für `register_lookup --dropped`: das Gate schlägt nur an, wenn die
aktuelle dropped-Zahl die Baseline **übersteigt** (Delta > 0), nie bei einem
Absolutwert. Die Baseline wird in dem Commit aktualisiert, der einen legitimen
Drop trägt — nie stillschweigend, nie als Fabrikation.

dropped-baseline 2158
commit-resolved 176
pairs 477
candidates 3869
measured 2026-09-21 (Forschung-Folge 136, HEAD fe8ee746; delta 65 über 2093, cross-line — der `--dropped --persist 2`-Register (358) trägt die anhaltenden Drops)
