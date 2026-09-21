<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: 3080787581fd64b25341d9796b86d87455f0f4fe5d3843ff19e5d58d47cd79f0
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

An entscheid: Nr. 11 „strukturierte Feld-Grammatik" — Operator-Entscheid 2026-09-21: gebaut, nicht descoped. Die Grammatik IST `docs/specs/sources-v2-spec.md` §1; Audit am HEAD `4a0c7803`: die 3-Token-`field`-Form als §10-Gap registriert (Parser verweigert sie, der Spec war stale), die Header-Referenz `src/main.rs`→`src/archivar/parse.rs` korrigiert, 8 undokumentierte Parser-Arme als §10-Pending benannt. (Schritt: Nr. 11 aus der Operator-Queue entfernen.)

