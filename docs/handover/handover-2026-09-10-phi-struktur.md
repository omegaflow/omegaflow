<!--
  title: Handover — phi-Struktur verfeinert + Quellen-Analyse automatisiert (Stand 2026-09-10)
  class: handover
  date: 2026-09-10
  sha256: 027fb59b1149499a2ecc1d1306829c776c7e4913af3802724cee03d6a9b9ebfd
  status: live
  see-also: docs/handover/handover-2026-09-10-bucket-litmus.md docs/handover/handover-2026-09-10-autonom.md
-->
# Handover — phi-Struktur verfeinert + Quellen-Analyse automatisiert (2026-09-10)

## Erledigt (trägt Git)

- **Rat-Verdikt (Architektur):** Struktur-Option (b) — Manifest, kein physischer
  Umzug. Der `read_dir`-Vertrag der Linse (nicht-rekursiv) ist der Granit; die
  Namen tragen die Rollen schon. Automatisierung: `bucket_litmus` bauen, aber
  das Werkzeug spricht Vorentscheid (`lens-match`), nie Urteil.
- **Atom 1** (`6f9a4cc`): `phi/pipeline/decline_lens.φ` (Maschinen-Zwilling des
  Oszillator-Gates — 9 Decline-Klassen + 3 Familien als Daten) + Bin
  `tools/utils/src/bin/bucket_litmus.rs` (std-only, kein Netz, kein LLM).
  Kalibrier-Gate gegen die 101 noaa_nodd-Urteile: **FP 3** (ghcn, swpc, swdi —
  Titel tragen das Gate-Wort, die Messung ist echt), **FN 12** (nationalbathymetry,
  mrms, s102, s111, wsa-enlil + Portal-/Archiv-Titel, deren Decline erst der
  Inhalt trägt). Gezählt, nie geglättet. `prompt.φ` nennt `decline_lens.φ` als
  Maschinen-Zwilling.
- **Atom 2** (`2dc7b91`): `phi/pipeline/catalog/MANIFEST.φ` (Karte des Katalogs —
  `file <name> role <role> state <state> lens <sichtbar>` je Datei; 68 flache .φ
  in 7 Rollen + 3 raw-harvest-Ordner). `index.φ` erweitert (probe/weights/frame/
  register/listen). `.gitignore`-Ausnahmen für `decline_lens.φ` + `MANIFEST.φ`
  (beide Registrierungs-Urteile). SOURCE_PORT.md §2 + §5 (Linse liest Manifest +
  Lens und findet null Kandidaten — A = A, kein Skip-Filter).

## Offen — der volle Lauf (nächster Schritt, eigene Session)

- **Voller Lauf über den bereinigten phi-Bestand:** `source_url_candidates`
  (Linse) → `probe_sweep` → Review (Schritt 4), auf der jetzt kartierten
  Struktur. Die Kette ist LOKAL, Review bleibt in der Session.
- **`bucket_litmus` auf weitere Inventare:** Copernicus-Inventar u.a. liegen
  bereits im Katalog — der Vorentscheid läuft mit `bucket_litmus
  phi/pipeline/decline_lens.φ <inventar.φ> [--calibrate <disposition.φ>]`.
- **Register-Nit (benannt):** `noaa_nodd_disposition.φ` trägt einen descoped-
  Eintrag für `https://www.ready.noaa.gov/archives.php`, der im Inventar eine
  `provider`-Zeile (`ungezählt`) ist, kein `catalog`-Dataset — sein Inhalt
  (NOMADS/RDA/CPC) ist separat disponiert. Eine Session kann den Eintrag
  streichen oder als Provider notieren.

## Beobachtet (nicht diese Session, uncommitted im Worktree)

- `tools/harvest/src/bin/ephemeris_compiler.rs` (M), `tools/measure/src/bin/te_series_periodicity_probe.rs`
  (??), `docs/befund/befund-te-series-periodicity.md` (??) — TE-Serie-Periodizität,
  uncommitted aus einer früheren Session; wird hier nicht angefasst.
