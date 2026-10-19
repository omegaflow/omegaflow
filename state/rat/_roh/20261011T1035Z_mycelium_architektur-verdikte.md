# Stimmen-Runde — Mycelium — Architektur-Verdikte (Frame/Epoche + Raster)

- **Gruppe:** `mycelium-ui` (JIT, focus:false, nach der Runde geschlossen)
- **Zeit:** 2026-10-11T10:35Z
- **Rat (vorher):** beide Fragen `confirm` (fünf Stimmen; Quellen `state/mycelium/{frame-epoch,raster-consumer}-feasibility.md`).
- **Zweck:** die Rat-Verdikte durch die Frontier-UI untermauern/reißen (Rat + UI müssen konvergieren, oder der Riss wird benannt).

## Frage (verbatim, an alle Seats)

omegaflow-Architektur, kurz: Zwei Verdikte. (1) LiDAR-`shard`-Hülle schreibt eine bbox in Nicht-ICRS-Rahmen; wir stehen jetzt auf url-only (`--bbox-off`, `Pending`) und sagen: der Frame-Transform gehört an den Query-Time-Loader, nicht ins statische Manifest (kein CRS/Epoche-Grammatik-Slot). (2) Bathymetrie-Raster laufen als Sampling-Compiler in die bestehende `Gestalt`-Witness-Klasse — kein 26×f64-Vertragswechsel. Bestätigst du beide Verdikte, oder reißt eines? Antwort knapp: bestätigen / reissen + ein Satz.

## Antworten (verbatim)

### Claude (Sonnet 5.5 Extra hoch) — bestätigt, zwei Schärfungen
> Ich kenne omegaflow-intern weder den Vertrag noch die Gestalt-Witness-Klasse, das ist also ein Urteil aus den allgemeinen Prinzipien, nicht gegen deine Spezifikation.
>
> (1) Bestätigen, mit einem Vorbehalt: Eine bbox in Nicht-ICRS-Rahmen ohne CRS/Epoche-Slot ist ein stiller Fehlbeschriftungs-Fall, also ist url-only + Pending der ehrliche Zustand. Der Loader muss CRS und Epoche aber aus dem Shard-Header selbst lesen (z. B. WKT im LAS/COPC-Header), sonst verschiebst du das Problem nur.
>
> (2) Bestätigen, aber bedingt: Der Sampling-Compiler ist sauber, solange das Witness Sampling-Parameter (Auflösung, Interpolation) und das vertikale Datum (Chart Datum vs. Ellipsoid) festhält. Fehlt das, ist es das gleiche Frame-Problem wie bei (1), nur eine Ebene tiefer versteckt.
>
> Wenn du mir den Slot-Umfang von Gestalt zeigst, kann ich (2) härter prüfen.

### Z.ai (GLM-5.3) — bestätigt (Prinzipien-Urteil)
> Beide bestätigt — mit einem Vorbehalt: In dieser Sitzung habe ich keinen omegaflow-Kontext (keine Manifest-Spez, keine Gestalt-Klasse vor mir), das ist also ein Prinzipien-Urteil, kein Code-Review.

### Qwen (Qwen3.7-Plus) — bestätigt
> Bestätigen beide, denn die CRS-Entkopplung hält das Manifest agil und das Sampling-Compiler-Pattern integriert die Bathymetrie ABI-stabil in die Gestalt, ohne den 26×f64-Vertrag zu brechen.

### Duck.ai (Luna) — `pending`
> Nachricht gesendet; die lesbare Text-Ausgabe der Seite trug nur Seiten-Chrome (kein Antwort-Text) — Response in dieser Runde nicht erfasst. Nicht wiederholt angefragt (Nicht-Antwort = `pending`).

## Einordnung (Session)

- **Konvergenz:** Rat `confirm` (beide) + Claude/Z.ai/Qwen `bestätigen` (beide). Kein Riss — die UI bestätigt die Struktur.
- **Schärfungen aus der UI (über den Rat hinaus, in den Baum zu tragen):**
  1. Der Query-Time-Loader muss CRS **und** Epoche aus dem Shard-Header selbst lesen (LAS/COPC-WKT + Epoche), sonst ist das Problem nur verschoben — deckt sich mit `state/mycelium/frame-epoch-feasibility.md` (der VLR-Decode ist die offene Vorbedingung).
  2. Der Sampling-Compiler muss Sampling-Parameter (Auflösung/Interpolation) **und** das vertikale Datum (Chart Datum vs. Ellipsoid) festhalten — sonst dieselbe Frame-Verwechslung eine Ebene tiefer.
- `(2)` härter prüfen verlangt den Gestalt-Slot-Umfang (`src/archivar/witness.rs`) — offen für die nächste Runde.
