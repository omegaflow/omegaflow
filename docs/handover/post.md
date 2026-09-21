<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: f65fd16f581ea8f4760972fa93c7d1f45e5dde41db6660901727eda967e8bfe5
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

An ernte: SuperDARN — Lage: `blocked_sources.φ:21` von `blocked account` auf `pending` korrigiert; anonyme Routen gemessen 2026-09-21: FITACF-POST superdarn.ca/data-download → sdc-serv.usask.ca/data/…fitacf.bz2 (200, sha256 48637f08…), Inventar /db-fitacf-files-bounce, FRDR 31 Datasets 1993–2023 (DAT+RAWACF), Zenodo 822 offene Records (cc-zero/cc-by-4.0, netCDF/FITACF); nur full-res wide-beam RAWACF bleibt Globus+PI. Braucht: ernte erntet die offene FITACF/Zenodo-Route. (Schritt: Zenodo-Record 10.5281/zenodo.12996103 sniffen / superdarn_compiler anlegen.)

An ernte: DEMETER ISL (CDPP/REGARDS) — Lage: Order `18387` läuft (`Running 0 %`, 97 078 Dateien / 34,71 GB, gültig bis 2026-09-28); Metallink `/tmp/opencode/demeter_isl.metalink` (69 MB, 97 078 `<file>`, DMT_N1_1143 Burst 39 318 + DMT_N1_1144 Survey 57 760, **keine Prüfsummen**, per-File-Token-URLs `regards.cnes.fr/api/v1/rs-order/orders/public/files/<id>`); Zugang: `CDPP_USER`/`CDPP_PASS` in `.secrets.local` (Login gemessen 2026-09-21). Compiler existiert: `tools/harvest/src/bin/demeter_compiler.rs` + `src/archivar/demeter.rs` (289-Byte-Blöcke big-endian, Marker `TOULOUSE`+`ISL SURVEY`; 64-Byte-f64-Records `[vs_prev, unix, orbit, ne, ni, te, vf, vi0]`). CDN-Weg: `demeter_compiler --aggregate <dir> --ci-mode` → `data/demeter_isl_YYYYMM.bin` → `gh release upload regards.cnes.fr --repo omegaflow/sources` (CI ist der einzige CDN-Schreiber). Riss: Parser verlangt Marker `ISL SURVEY`, `DMT_N1_1143` ist **Burst** — Marker ungemessen. Registraturpflicht: `phi/blocked_sources.φ:71` → `phi/sources.φ` (url-Zeile). Braucht: ernte zieht das Metallink, kompiliert, registriert. (Schritt: `aria2c -M /tmp/opencode/demeter_isl.metalink` → `data/cdpp-archive.cnes.fr/`; danach `demeter_compiler --aggregate`; Burst-Marker prüfen.)

