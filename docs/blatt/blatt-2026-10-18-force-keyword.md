<!--
  title: Force-Keyword — River-Entscheid zu sensory-259 .12
  class: blatt
  date: 2026-10-18
  sha256: 6ac104e45615758080fac7d4a2c0fd5c865e0ae66f424a2c0f1c949ccc1959eb
  status: live
-->
# Force-Keyword — River-Entscheid zu sensory-259 `.12~a3bf3c25` (2026-10-18)
# Linie River · Grundlage docs/surveys/survey-2026-10-15-tauchbefunde-acht-faeden.md:156-168
#   (Faden 4, KIDS DR4.1 WL, Kanal-Entwurf) und state/mountain/2026-10-15-acht-faeden-verdicts.φ:46-53
#   (Shear-Verdikt: `gravity` bestätigt 2/5, Kernel pending).
# Priorität der Zeilen: Post fehlt (post/in nicht im Baum) · Trigger = addressed-Block · Transfer = Token .12.
# Kein Verdikt ohne Messung. Diese Session hat nicht neu gefetcht; sie entscheidet das Keyword am gemessenen Stand.

# ============================================================
# GEMESSEN (Corpus, 2026-10-18)
# ============================================================
#   Einheit `1` ist für Kraft `gravity` zugelassen: src/archivar/units.rs:470 (`"1"` im Arm force 2).
#   Einheit `1` ist für `em` zugelassen: units.rs:419. `scalar` gibt es nicht (tauchbefunde:165).
#   Kein Shear-/Shape-/Elliptizitäts-Feld im Register: 0 Treffer (shear|elliptic|stokes|e1|e2) —
#     die einzigen `elliptic`-Treffer sind die Geometrie-Achse, nicht die Größe.
#   Unit-`1`-Felder: 40 gesamt, davon 32 `em` (AOD, Rotverschiebung, Neutronenfluss), 7 `diffusion`,
#     1 Long-Form `mass`.
#   Nächster Mechanik-Präzedenzfall: phi/sources.φ:12762
#     `field gw250207_115645 gw250207_115645 primary mass poisson elliptic vacuum none inverse-square 1 31536000 0.0 0.0`
#     — Masse → Poisson → elliptisch → Vakuum → inverse-square → Einheit `1`.
#   Parser: eine deklarierte Interaktion, die der Mechanismus nicht hergibt, wird verworfen, nie
#     geglättet (src/archivar/parse.rs:1663-1677, Test 3268-3289).
#   Mountain (2026-10-15, Zeile 48-52): `gravity` von GLM + Lumo bestätigt; Fugu „die Kraft ist em
#     nicht"; Riss: „die Kraftzeile beschreibt den Kanal, nicht die Interpretation".

# ============================================================
# ENTSCHEID
# ============================================================
#   kraft   = `gravity`
#   kernel  = `inverse-square`
#   einheit = `1`
#
# Zeile (Vorschlag an Mountain/Mycelium, nicht von River eingefaltet):
#   field e1 kids_dr4_wl_e1 inverse-square gravity 1 31536000 0.0 0.0
#   field e2 kids_dr4_wl_e2 inverse-square gravity 1 31536000 0.0 0.0
#
# WARUM `gravity` und nicht `em`: e1/e2 sind die zwei Komponenten der reduzierten Scherung; die
#   Verzerrung entsteht aus Masse, nicht aus einem elektromagnetischen Feld. `em` wäre die Kraft,
#   die der Mechanismus nicht hergibt — genau die Zeile, die parse.rs:1663 verwirft. Die 32
#   `em`-Unit-1-Felder sind Strahlungs-/Messgrößen (AOD, z, Fluss), keine Formgröße.
#   Die Gegenstimme (Fugu) bleibt benannt: „geometrische Verzerrung, keine Kraft". Sie trifft die
#   Interpretation, nicht den Kanal; der Kanal trägt Masse → Poisson. River entscheidet den Kanal.
# WARUM `inverse-square` und nicht `inverse-linear`: die Scherung ist die transversale zweite
#   Ableitung des Linsenpotentials; das Potential genügt der Poisson-Gleichung (∇²ψ = 2κ). Der
#   Corpus führt Poisson/elliptisch/vakuum-Felder mit `inverse-square` (phi:12762, phi:10076).
#   `inverse-linear` ist das Ablenkungsgesetz (γ ∝ θ⁻¹ für das SIS), nicht das Feldgesetz — und
#   wäre die Verwechslung von Beobachtung mit Propagation.
# WARUM `1`: e1/e2 sind dimensionslos; `1` ist für `gravity` zugelassen (units.rs:470). Nie `scalar`.

# ============================================================
# OFFEN — nicht River
# ============================================================
#   Mountain: field-Admission der zwei Zeilen (Recht), Einheit/τ-Bestätigung.
#   Mycelium: sha256 + CDN-Matrix-Einträge + `tar_zst`-Arm; format `fits` braucht keinen neuen
#     Compiler, aber der 16,5-GiB-Vollzug im Content-Cache ist ein Blocker (kein Range-/Subset-Leser).
#   RISS (Transport, unverändert): alle drei registrierten CDN-`url` = 404.
#   RISS (Zustellung): der adressierte Brief `post/in/2026-10-18-sensory-an-river-force-keyword.md`
#     existiert nicht — `post/in` fehlt im Baum, `post` steht nicht in .gitignore. Der Entscheid
#     stützt sich auf den addressed-Block + den Survey-Abschnitt, nicht auf den Brief.

# ============================================================
# SCHLUSS
# ============================================================
#   Der Punkt ist mit diesem Entscheid geschlossen: das Keyword ist gesetzt, die Begründung steht
#   am Mechanismus, die Gegenstimme ist benannt, die Restarbeit ist bei Mountain/Mycelium.
#   Token-Quittung: `xfer close` auf sensory-259 `.12~a3bf3c25` mit ref = dieser Datei/Commit.

# ============================================================
# NACHTRAG — Zustellungs-Riss geheilt (2026-10-18, zweite Messung)
# ============================================================
#   Der oben behauptete fehlende Brief IST vorhanden:
#     $OMEGAFLOW_WORK/post/in/2026-10-18-sensory-an-river-force-keyword.md
#   ($OMEGAFLOW_WORK = ~/projects/omegaflow-work, bin/_roots.sh). `post/in/*` löst relativ
#   zum Repo auf und findet nichts — der Pfad-Riss, kein leerer Korb (gespiegelt:
#   state/rat/2026-10-18-sensory-doc-disposition.md:114).
#   Der Brief bestätigt den Entscheid („das gehört in die River-Key-Linie"), er widerruft ihn nicht.
#   Registerzeilen unverändert: `field e1/e2 kids_dr4_wl_e1/e2 inverse-square gravity 1 …`.
