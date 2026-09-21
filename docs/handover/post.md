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

An ernte: CDN-Rotation — Release `ssd.jpl.nasa.gov` (id 367063539) am 1000-Asset-Limit (`Link rel=last page=1000`, gemessen 2026-09-21); `rpw_efield_lira.bin` (199152 Records, Roundtrip parst) unmanifestiert (Upload HTTP 422). (Schritt: family-tag-Rotation — neuer Upload über vorhandenes `upload_release(family_tag, path)` (src/archivar/cdn.rs:44), family-tag const in `bia_efield_compiler` (Präzedenz `--release-tag` in tap_compiler), Download-Tag `.github/workflows/rpw-cdn.yml:36`, URL-Tag `phi/sources.φ:1139`; Verifikation CI-Roundtrip `gh workflow run rpw-cdn.yml`; der Tag, der geschrieben wird, ist der Tag, der gelesen wird.)

An entscheid: Nr. 11 „strukturierte Feld-Grammatik" — Operator-Entscheid 2026-09-21: gebaut, nicht descoped. Die Grammatik IST `docs/specs/sources-v2-spec.md` §1; Audit am HEAD `4a0c7803`: die 3-Token-`field`-Form als §10-Gap registriert (Parser verweigert sie, der Spec war stale), die Header-Referenz `src/main.rs`→`src/archivar/parse.rs` korrigiert, 8 undokumentierte Parser-Arme als §10-Pending benannt. (Schritt: Nr. 11 aus der Operator-Queue entfernen.)

