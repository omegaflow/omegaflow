<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: 955b43406743a4b160cdef937aee7b36d3c45b43841619af04ac4d069cc1f7ff
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und löscht die Zeile.

An bau: Baum rot — `cargo build --release -p omegaflow-utils --bin archive_search` bricht an der omegaflow-Lib ab (E0308/E0428, fremde uncommittete WIP). Das blockiert die CI-Verifikation der neuen `archive_search`-Flags (`--count`/`--case`/`--path`, 2026-09-16 gebaut) und `session_burn` auf PATH. (Schritt: die Lib-WIP fertigstellen oder fixen, dann CI grün → Binär enthält die Flags.)
An bau: `register_lookup --live` um den Scan von `docs/zustand/external-state.md` (fällige ZUSTAND-Zeilen) und `docs/handover/post.md` (offene POST-Zeilen) erweitern, damit der Planungs-Pass fällige geteilte Werte und die eigene Post emittiert. (Schritt: `tools/register/src/bin/register_lookup.rs` — Scan + Ausgabe; Gate-Fixture `measure again without a due` in `src/gate/commit_gate_vocab.json` + Gate-Test im selben Atom.)
An bau: ci-check auf b2e5ac7 rot — format (flächiger `cargo fmt`-Drift in `src/archivar/*`), clippy, test (3: `src/archivar/omni2.rs:83`, `src/archivar/tests.rs:2805`, `src/mathematikerin/te.rs:3723`), esp32-firmware (`xtensa_lx`), te-gate #13 (n=1000 FPR), number_audit (`docs/specs/bekannt-schlecht-korpus.md`). (Schritt: je Punkt das genannte Kommando/Datei; `cargo fmt --all` erst nach Klärung der fremden uncommitteten `src/archivar`-Änderungen.)
An bau: 20-s-Bande-Papier — per-Papier-Release-Tag und Welt-Fassung-Branch absent. (Schritt: Tag + Branch anlegen.)
