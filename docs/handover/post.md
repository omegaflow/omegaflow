<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: 2dce39cadbfb531515250e1b5bce919b7013c782b6f68a0ae4cda1b3738286ba
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und **löscht die Zeile sofort** — eine abgeholte Zeile bleibt nie
stehen. Ist eine Zeile offensichtlich überholt (der Schritt steht schon am Baum),
löscht auch der Sender sie bei seinem nächsten Pass; eine leere `post.md` ist der
richtige Zustand, kein Verlust.

An bau: EPA RadNet ERM_RESULT Position (`phi/blocked_sources.φ:74`, Register-Tag `[bau] parser-def`) — Route 200 (`data.epa.gov/efservice`), `result_in_si`=Bq/L (em), aber `ERM_LOCATION` trägt nur city/state, keine Koordinaten. (Schritt: lat/lon per externer RadNet-Stationsliste joinen, dann Port nach `sources.φ` + CDN-Manifestation.)

An bau: GHRC GLM-L2-LCFA-Konsument — die token-freie S3-Route `noaa-goes16/18` trägt nur GLM-L2-LCFA (hdf5, gemessen 200), nicht das L1B-Flash-Produkt; L1B bleibt GHRC-EDL-protected (`phi/sources.φ:8023`). (Schritt: eigenen compiler/format für L2-LCFA bauen — L2-Events-HDF5 ≠ L1B.)

An bau: PINE64-Dokumentationspflicht — Ox64 ist von Pine64 zugesagt (Hardware beidseitig geschlossen), aber die Presence-Hardware „Mantis-Shrimp" ist ungebaut. (Schritt: Mantis-Shrimp minimal bauen — Spec `docs/specs/mantis-shrimp-bom.md` + BOM liegen —, dann Ox64 dokumentieren; Ergebnis als eigene Handover-Zeile, kein Befund.)

An bau: `archive_search --verdict` — auf der nackten Trailing-Slash-URL (z. B. `https://api.alerce.online/alerts/v1/objects/`) druckt `--verdict` „HTTP 0", während `curl` dieselbe URL in derselben Minute mit 200/5740 B JSON misst (2026-09-21, direkt und über Proton). Tool-Inkonsistenz, keine tote Route. (Schritt: den `--verdict`-Direkt-Request gegen curl abgleichen — Redirect-/HEAD-Handling auf Trailing-Slash prüfen.)

