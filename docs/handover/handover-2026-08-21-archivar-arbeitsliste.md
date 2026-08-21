<!--
  title: Handover: Archivar-Arbeitsliste — Feature-Gate, Crossmatch, Kaltstart, Subpixel
  class: handover
  date: 2026-08-21
  sha256: 43a9db1e99bdfb52e6eccef3779129794905f19d8d3fa31f6afe4ba1d581a974
  status: live
  see-also: TODO.md docs/handover/handover-2026-08-21-4d-wahrheit.md
-->

# Handover: Archivar-Arbeitsliste

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext. Der Auftrag
ist nicht die Ausführung; ausgeführt wird erst auf das Wort des Operators.

Dieses Dokument bündelt die offenen Posten des Registers (TODO.md, Abschnitt
„Archivar — Architektur" plus die offene Abweichung Z04/F35 aus „Wahrheits-
findung"). Ein Fenster trägt EINE Einheit. Die Einheiten sind unabhängig;
die Reihenfolge ist nicht Teil des Auftrags — der Operator wählt.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # sauber oder fremde Arbeit nennen
cargo check                      # 0/0 — danach auch die Feature-Kombis
grep -n "crate::mathematikerin" src/archivar.rs   # die Verdrahtung (Gate-Atom)
```

Referenzen (stehend): `TODO.md` (Archivar-Arbeitsliste),
`docs/concepts/iau-2000-eop.md` (K06),
`docs/concepts/intuitive-touchpad-touchsteuerung.md` (der 2-Finger-Zeitschub),
`docs/surveys/survey-messpunkt-verteilung.md` (Subpixel-Messung).

## Die Einheiten

### A. feature-gate `gpu` — eigenes Atom, pending

`pub mod mathematikerin` (src/lib.rs) als `#[cfg(feature = "gpu")]` plus
Co-Gate der main_flow-Verdrahtung: die `crate::mathematikerin::`-Stellen
(`PresenceFrame`, `EMOscillator`, `KineticRadiator`; der kinetische Vec lebt
bei src/archivar.rs:16092, der `vibrate`-Ruf folgt) + Feature-Propagation zum
Default-Bin. Das ist kein Ein-Zeilen-cfg, sondern ein Faden durch die ω-Loop.
Gates: cargo check 0/0 in allen vier Feature-Kombis (default, browser_relay,
gamepad, beide) — der Bin-Kaltbau ohne gpu muss grün sein, die Tests auch.

### B. Lokaler Crossmatch Lasair×TNS — pending

Lasair (ZTF-Transienten, em) trägt kein z in `objects` — die Objekte liegen
auf der Himmelssphäre (0 honored). Die TNS-Tabelle (`tns_public_objects.csv.zip`,
`z redshift`) kennt für die SN Ia die echte Rotverschiebung. Die wahrste
Lösung: der Archivar matched die Lasair-Objekte beim Laden lokal gegen die
bereits geladene TNS-z-Tabelle — Anreicherung nur dort, wo eine
Übereinstimmung vorliegt, kein Datenverlust, keine Lüge. Ein eigenes
Code-Feature (Join zweier Quellen im SpatialHash), kein Quellen-Block.
Fundort: beide Blöcke in `phi/sources.φ`; ein Join existiert nicht
(grep lasair/tns in src/: kein Treffer — das Feature ist der Bau).

### C. Ephemeriden-Kaltstart — per-Anker-Extraktion

Die Anker laden als erste Phase über `curl --parallel --parallel-max 8`
(HTTP/2, retry-all-errors); das Sternfeld zeigt sich sofort, die Planeten
folgen. Offen: per-Anker-Extraktion — sun/earth sofort extrahieren statt
nach der ganzen Anker-Phase — für wörtliches „Sekunden"-Laden. Der
Kalt-Download (~360 MB) bleibt einmalig bis zum Warm-Cache.

### D. SPK-Segment-Payloads lazy laden

Strukturell: Segment-Payloads erst bei Bedarf laden statt upfront — sonst
wächst die Ramlast mit jeder Kernel-Generation (sb441-n373, 14,13 GiB,
läuft in einem eigenen Handover; dieses Stück ist der generische Lazy-Weg).

### E. K06 EOP — Erdrotation

72-B-Orientierungsmatrizen leben (Binary v2 trägt sie); die Erdrotation
(Polbewegung, UT1−UTC) für präzise Erd-Stationen fehlt. Konzept:
`docs/concepts/iau-2000-eop.md`.

### F. X-flagged-Sterne ohne Tycho-1-Eintrag

Positionen lägen im Guide Star Catalog (I/220, ~25 Mio) — offen.

### G. Puffer-Schrumpf

`ensureFieldCapacity` schrumpfte im Browser bei langsamen Frames; nativ
wächst nur. Der native Schrumpf-Pfad fehlt.

### H. 2-Finger-Zeitschub

Die Wahrheit des Touchpad-Docs (intuitive-touchpad-touchsteuerung.md):
Pinch = Zoom, 2-Finger-waagerecht = ZEIT-Schub, 2-Finger-senkrecht =
vor/zurück. Das native implementiert heute Pan+Zoom+Roll ohne Zeit-Achse
(MouseWheel bei src/mathematikerin.rs:3756). Die Zeitachse ist der Bau —
Presence-t-Schub; die 4D-Wahrheit-Übergabe (Fenster-Range ist kein
Fetch-Radius) bleibt unberührt.

### I. Deep-Link-Geschwindigkeit

`#x,<x>,<y>,<z>,<t>` existiert (src/mathematikerin.rs:3859, [f64; 4]) —
die Geschwindigkeit `[,vx,vy,vz]` fehlt. Konsument: das v der Presence.

### J. Audio-Ausgang nativ

`AcousticOscillator` schreibt rohe Samples nach stdout
(src/mathematikerin.rs:863-871, `is_terminal`-Gate). Das ist der
Pipeline-Ausgang — bewusst so oder ein eigener Ausgang: Entscheidung des
Operators, dann Schnitt oder Registrierung des Verbleibs.

### K. Subpixel-Anlauf

Rgba32Float, 9 Mio Messzellen — wartet auf einen nicht-aufgeblähten
Wiedereinstieg. Die Messung lebt in
`docs/surveys/survey-messpunkt-verteilung.md` (die 567-ms-Erkenntnis).

### L. Gravity-Hardcodes im Extract-Pfad (Z04/F35)

Ratsbefund: drei Stellen hartkodiert auf gravity statt aus den Daten.
Fundorte sind beim Vollzug zu finden (Extract-Pfad, force-Attribut) und zu
verifizieren — beim Schnitt benennen, was an der Stelle aus den Daten steht.

## Verifizierter Kontext (2026-08-21)

- Die Atome 6–9 sind ERLEDIGT (ein universeller SpatialHash; Okklusion,
  Stern-Sprites, Gitter und AudioRadiator tot; Aktuatoren als Oszillatoren) —
  sie stehen im Register als Geschichte, nicht als Arbeit.
- Die 4D-Wahrheit (kinematische Dilatation) läuft in einer eigenen Übergabe
  (`handover-2026-08-21-4d-wahrheit.md`, live) — NICHT anfassen.
- Sample-Budget, GLADE+/NED/2MASS, grüner chunk-Lauf: eigene Übergabe
  (`handover-2026-08-21-offene-atome.md`, live) — NICHT anfassen.
- `OMEGAFLOW_HIDDEN=1` ist der stille Lauf für alle Gates, die kein Fenster
  brauchen (Operator-Limit: keine Membrane-Fenster).

## Gates

- cargo check 0/0 (vier Kombis); cargo test komplett.
- Ein Commit je Einheit; TODO.md-Register im selben Commit (Erledigtes raus,
  Offenes scharf).
- Diese Datei nach dem Abschluss archivieren (Regel in AGENTS.md).

## Nicht anfassen

Sphären (Ringe/Warp), spektraler Oszillator Atom C, Source-Port,
Stern-/Asteroiden-Physik, Validation/CI — je eigene Übergaben.
