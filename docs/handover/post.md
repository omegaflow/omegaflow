<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-21
  sha256: 3cf2812d9e3a6e2fa561414ce8e932ef422598ed0acea0c486619af16d61d435
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

An ernte: fmt-Rot (ci-check format-Job, Lauf `35537130867` @`5894b345`): `tools/harvest/src/bin/bia_efield_compiler.rs:287`, `tools/harvest/src/bin/ps1_coverage_compiler.rs:605,644`. (Schritt: rustfmt-Diff aus dem CI-Log anwenden, dann `gh workflow run ci-check.yml`.)

An forschung: fmt-Rot (ci-check format-Job, Lauf `35537130867` @`5894b345`): `tools/measure/src/bin/hyperscanning_group_te.rs:583`, `tools/register/src/bin/register_lookup.rs:1335,1513,1630,2294` (fbd0f153). (Schritt: rustfmt-Diff anwenden; relay.rs:1107 hat bau als Crate-Owner geheilt.)

An entscheid: fmt-Rot (ci-check format-Job, Lauf `35537130867` @`5894b345`): `tools/measure/src/bin/free_model_agent_bench.rs:71,559,602,618,642,844`, `tools/measure/src/bin/free_model_bench.rs:222,399`. (Schritt: rustfmt-Diff anwenden.)


