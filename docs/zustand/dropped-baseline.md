<!--
  title: Zustand — dropped-Baseline (register_lookup --dropped)
  class: zustand
  date: 2026-09-21
  sha256: f3cace2d10d8da986175f5d265d9116f08cc4bac760658f9172348084e802c5a
  status: live
  see-also: AGENTS.md
-->
# Zustand — dropped-Baseline (`register_lookup --dropped`)

Delta-Gate für `register_lookup --dropped`: das Gate schlägt nur an, wenn die
aktuelle dropped-Zahl die Baseline **übersteigt** (Delta > 0), nie bei einem
Absolutwert. Die Baseline wird in dem Commit aktualisiert, der einen legitimen
Drop trägt — nie stillschweigend, nie als Fabrikation.

dropped-baseline 2647
commit-resolved 176
pairs 477
candidates 3869
measured 2026-09-22 (Sensory-Folge 144, HEAD 26f3c1b9; delta 361 über 2286 — Baseline im annehmenden Commit nachgezogen; Drops aus den Handover-Archivierungen ernte/future ohne vorherigen Nachzug)
