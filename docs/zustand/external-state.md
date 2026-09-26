<!--
  title: Zustand — geteilter externer Zustand (Zeiger)
  class: zustand
  date: 2026-09-26
  status: live
  see-also: state/zustand/external-state.md
-->
# Zustand — geteilter externer Zustand

Der Ledger lebt im **privaten Baum**: `state/zustand/external-state.md`
(Repositorium `omegaflow/personal`, privat versioniert und gepusht). Er trägt eine
PII-Zeile (Postfach) und wird darum nicht öffentlich getrackt.

Dieser Pfad ist der **getrackte Zeiger** — er trägt nie Ledger-Inhalt, nur den Ort.
Jede Session liest und schreibt den Ledger unter `state/zustand/external-state.md`.
