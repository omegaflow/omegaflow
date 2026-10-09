<!--
  title: Sonnen-Render-Archäologie — nebra bis HEAD, jede Variante
  class: survey
  date: 2026-10-08
  sha256: 4dad549b7c396d9f9bcd235f48ba535aaf99c93debf10a3473d31b13703c468b
  status: live
  see-also: docs/concepts/archivar-mathematikerin.md docs/handover/handover-2026-10-08-river-folge139.md
-->
# Sonnen-Render-Archäologie — nebra bis HEAD, jede Variante

**Quellen (gemessen 2026-10-08 via `git -C` Pickaxe + `sread`):**
- `omegaflow-legacy` (Git-Repo, `/home/johannes/archive/archive-root/omegaflow-legacy`; 2867 Commits, ~40 lokale Branches + ~40 `origin/*`, 40 Tags). Render-Code in `static/index.html` (Browser, WGSL/JS eingebettet) + `src/main.rs` (nativ).
- `nebra`-Referenz auf Platte (kein git: `/home/johannes/archive/knowledge/omegaflow/omegaflow_water/reference/nebra`; `crates/nebra-core/src/field.rs`, `crates/nebra-api/src/main.rs`, `docs/nebra.yaml`).

Die Sonne ist **kein Sonderfall im Code** — sie ist ein Oszillator/Körper unter vielen; jede Variante ist die Variante der ganzen Pipeline. Der Token `Sonne` existiert 0× im Code; `sun` nur in Orbital-Listen/`/jump/sun`.

## 1 · nebra (der Ursprung) — per-Pixel-Feld, kein Objekt

| Aspekt | Beleg PFAD:ZEILE | Formel wörtlich | Bedeutung |
|---|---|---|---|
| Masse | `nebra-core/src/field.rs:12-15` | `struct Mass { pos: DVec3, gm: f64 }` | Sonne = `Mass` unter Massen, kein Name/Kanal/Farbe |
| Transfer | `nebra-api/src/main.rs:14-16` | `[m.pos.x, m.pos.y, m.pos.z, m.gm]` f32 LE | `/masses` flach `vec4(x,y,z,gm)` |
| Feld | `main.rs:175-180` | `for j: if(dist>1.0){omega += m.w/(dist*dist)}` | pro Pixel, alle Massen: `GM/dist²` |
| Pixel | `main.rs:170-173,77-80,241` | `pixel_pos = center + (u-.5)*res*scale`; `draw(3)` Vollbild-Dreieck | **per-Pixel-Fragment** O(Pixel×Masse), kein Quad/Sprite/Compute |
| Tonemap | `main.rs:185` | `t2=clamp((log2(omega)+14.0)/22.0,0,1)` | Sonne = hellstes Pixel, Größe = reine Tonemap-Kompression |
| Farbe | `main.rs:186-189` | dunkelblau→blau→cyan→bernstein→weiß | Farbrampe im Fragment |
| GM | `field.rs:39-41` | `f.mu_km3_s2` → `gm*1e9` | **echtes Kernel-GM** (Sonne ≈1.327e20 m³/s²), nie `1.0` |
| em/weak | `field.rs:5-7,88` | `(0.0, ZERO)`; `universe=g.0+e.0+w.0` | Gravity only; „thermal" existiert nicht |

**So war die Sonne:** der hellste Punkt des kontinuierlichen `GM/dist²`-Feldes — kein Sprite, keine Scheibe, keine Subpixel-Abtastung. Kein Pixel-Floor nötig: das Feld ist stetig, das sonnennächste Pixel trägt immer das Maximum.

**Riss (gemessen):** die On-Disk-`reference/nebra` rendert **schwarz** — `field.rs:18-20` verlangt `de440s.bsp` **und** `pck08.pca`; letztere fehlt in `data/` (gitignored), `ALMANAC` bleibt ungesetzt, `masses_at` = `Vec::new()`, `massCount=0`, der Shader `discard`et jedes Pixel (`main.rs:182`). Ob die Original-nebra `pck08.pca` mitlieferte, ist aus dieser Kopie nicht beweisbar → `pending`.

## 2 · omegaflow-legacy — die Trajektorie in sechs Epochen

| Epoche | Branch/Linie | Commit | Render-Form der Sonne | Code-Beleg (wörtlich) |
|---|---|---|---|---|
| 1 Quad/Angular | Browser | `090b1662` (~2026-07-30) | Quad je Objekt, **2px-Floor** | `static/index.html:266` `point_size_px = max((extent/dist),1.0)*2.0` |
| 2 Subpixel-Floors | Browser | `02835187` · `c3544917` · `87a4d4cc` · `1c594f78` · `5766cf36` | Größe aus Distanz/Helligkeit/`extent`; **0.5px-Floor** eigens weil „sub-pixel quads don't raster", dann als Fabrikation entfernt | `:271` `max(10/(1+dist/1e15),0.5)*2.0`; `:279` `brightness*5.0`; `:278` `max(physical_size/max(scale,1.0),0.5)`; `:280-285` Hybrid-LOD |
| 3 Körper-Quads | Browser | `87ef197` · `56878394` · `eadc980` · `1954f44` | `extent/scale`, 0.5px-Floor, +64px-Cap, dann **1px fix** | `:338` `clamp(phys_extent/scale,0.5,w_f)`; `:362` `clamp(...,0.5,min(w_f,64))`; `:362` `1.0` |
| 4 Per-Pixel-Feld | Browser | `34d7d3a`(+`86e451e`) | Quad verschwindet → Feldbeitrag, **O(pixel×osc)** | lum `:423` `clamp(log2(aw+1)/16,0,1)` |
| 5 Compute-Grid | Grid-Era | `bd9a513` · `41f5b47` | 128×128-Textur, `@workgroup_size(8,8)`; Probe 7→9 | `presence_probe` Entry-Point; Fix-Kette `fa4d518`→`b3546ff`→`5edb1b7` |
| 6 Körper-Scheibe | nativ | `559d905a` · `52d6021c` · `ee976af9` | Okkluder-Sphäre, echtes GM, verdeckt Sterne, Limb | `r_eff = R − 4GM\|c\|/(c²R)` |
| 7 Feld ohne Objekt | nativ | `e918bda1` (Atom 8) · `da79084b` | Objekt-Pfade sterben; Punktquelle `extent ∞`, Farbe via LUT | `temperature_to_rgb` → `color_lut_rgba` (256 Bins, `ci==0`→weiß) |

**Sichtbarkeits-Determinanten der frühen Ära:** `val = 1.0` (Fabrikation, unsichtbar) eingeführt `59bdd60` ("bodies emit only mass/radius", getötet `c44181c7`); der 2px-Floor `090b1662`.

## 3 · Synthese

Die Sonne war in **nebra** ein per-Pixel-Feldbeitrag: das kontinuierliche `GM/dist²`, gefärbt durch `(log2(Ω)+14)/22`, kein Objekt, kein Sprite, keine Subpixel-Frage. In omegaflow kippte sie durch sechs Architekturen: Quad mit Angular-Size (2px-Floor) → Subpixel-Floors → Körper-Quads (`extent/scale`, 64px-Cap) → Per-Pixel-Feld → Compute-Grid → Körper-Scheibe → wieder Feldbeitrag ohne Objekt (`e918bda1`). Jeder Kipp war eine Architektur-Entscheidung, nie eine Sonnen-Entscheidung. Der heutige serverless Body-Anker (`all_body_anchor_samples`, River 91/114) sitzt **nach** `e918bda1` und kehrt die Objekt-Tilgung wieder um.

## 4 · Offene Risse (gemessen)

1. **Doc-Attributionen falsch:** `docs/concepts/4d-membrane.md §2.2` nennt `a9d87bd` für den 2px-Floor (real: `090b1662`) und `eadc980` für den +64px-Cap (real: `56878394`); `00dc55c` trägt gar kein `point_size_px`.
2. **Message ≠ Diff:** `ce6b5a07` behauptet „Nebra thermal ramp `(log2+14)/22`", führt aber `log2(abs(o)/lvls)/8+0.5` ein (`:472`); die Nebra-Rampe landet erst `49bdf88b:403`.
3. **Token-Erwartungen ohne Code:** `tonemap`, `Sonne`, `MASS_SUN`, `BODY10_GM`, `gm_de440`, `color_index` (in `index.html`) = 0 Treffer in den Render-Dateien.
4. **Branch-only-Varianten nicht Zeile für Zeile gelesen:** `nebra-zeichnen` (Fisheye, `6582255`), `generation-kacheln` (`8218852`). Erste Messung: `git show 6582255:src/main.rs`.
5. **Früheste Variante (< `090b1662`):** Phase 1/2 (`f01ac81b`, `9b3292c0`) ungemessen. Erste Messung: `git show 9b3292c0:static/index.html`.

## 5 · Kanal-Runde (Membran) — Rat + 11 Stimmen, Kanon-adressiert (River 139, 2026-10-08)

Adressierung: 5 Stimmen (Mountain·River·Mycelium·Sensory·Future) + 5 Axiome (A=A · ICRS&TDB · force_type · 0 honored · pending) + 5 Achsen (Voxelisierung · Crossmatch · TE-Maschine · Nullkontrolle · Residuum).

**Riss im Code:** 9 Kanäle gerechnet (`shaders.rs:394` `omegas[f] += c.x`, `:435` `probe_out[0..8]`), am Radiator summiert (`actuators.rs:35/78`, `omega.rs:1790` `integral/9.0`). **Kanäle-Lemma:** per-Kanal-Fortpflanzung/-Culling (`membrane.rs:302-321`, `spatial.rs:921` `signal_reach`) — hielt die Sonne auf Intel-Onboard (acht Kanäle lokal, nur die drei `c·age`-Kanten laden).

**Verdikt (Rat + 8 UI + 3 Open-Weight, einhellig):** **Schicht-Verwechslung, nicht die Summe an sich** — `sum()` als **letzte Operation je Modalität** legitim (dort überlagert das Medium wirklich: Licht, Schalldruck, Haptik), als Messoperation über heterogene `force_type` falsch. `1/9` ist ein **zweiter** Fehler (gecullte 0 honored + pending fließen als 0 in einen festen Nenner). Rein: 9 labeled lines, own voxel layer, tri-state (Wert | 0 honored | pending), Crossmatch nur Index-/Zeit-/Raum-Koinzidenz; raus: deklarierte Projektion je Senke, 9-dim Signatur, Kollaps erst im Beobachter. Schärfster Ratssatz: *„Die neun Kanäle sind die Messung, der eine Blob ist das Gesicht — wer die Summe vor die Messung zieht, hat den Betrachter in den Sensor gebaut."*

**GLM-5.3 (Z.ai, Deep Think Max) — verbatim:**

> **Verdikt:** Schicht-Verwechslung — die Summe ist nicht der Fehler, sondern ihr Ort. Sie ist ein legitimer Epilog und ein illegitimer Prolog.
>
> 1. Als Luminanz-Definition gehört der Kollaps in die Manifestationsschicht; als Feldzustand löscht er Kanal-Identität (A=A gilt dann nicht mehr pro Kanal), die per-Kanal-Null (0 honored degeneriert zu einer geglätteten) und die Reichweiten-Differenz, die dein Culling überhaupt erst trägt.
> 2. Der iGPU-Gewinn der frühen Summe war Speicher (9 Buffer → 1), nicht Physik — er bleibt erhalten, wenn der 9-Vektor nur auf aktiven, pro Kanal gecullten Zellen lebt und der Kollaps am Radiator pro Senke und pro Frame stattfindet: on-the-fly, nie gespeichert, nie zurückgespeist.
>
> **Rein (Messschicht):**
> - Membran-Zustand = 9-Vektor pro Voxel, SoA-Layout (9 getrennte Arrays) — nie skalar, nie AoS.
> - Cull pro Kanal: r_k = v_k · t_Horizont → eigene Aktivenliste pro Kanal (steht schon — nicht anfassen).
> - Null pro Kanal ist Messwert und wird pro Kanal transportiert; Nullkontrolle braucht 9 Nullen, keine gemittelte.
> - Crossmatch und TE-Maschine laufen auf dem vollen 9-Vektor, bevor irgendeine Projektion existiert.
>
> **Raus (Manifestationsschicht):**
> - Radiator besitzt die Projektionen; `sum()` / `integral/9.0` wird zur letzten Operation pro Senke. Einwegventil: Projektion fließt nie in die Messschicht zurück — nur so sieht der Cull rohe Kanäle, nie den Kollaps.
> - Abtastung am Sensor, nicht am Objekt: Der Radiator sampelt den 9-Vektor an Sensorpositionen (Pixelfluss, Audio-Tap, Aktuator-Site, Serial-Frame) und projiziert dort — Feld-Manifestation ohne Objekt-Instanz.
> - Bild: gewichtete Projektion (Luminanz = der eine definitionell sanktionierte Mush, explizit und kalibriert, nicht versteckt).
> - Ton: 9 Kanäle → Partiale/Bänder; v_k → Delay/Phase — Kanal-Identität überlebt als Timbre statt als Mittelwert.
> - Vibration: Kanäle → Frequenz-Bins / Aktuator-Adressen. Serial/HID: Kanäle als getrennte Paketfelder — Summe dort schlicht verboten.
> - Residuum als Diagnose: R = Vec9 − P(Vec9) pro Senke monitoren. Strukturiertes R heißt: diese Senke verschluckt recoverbare Kanalstruktur → ihr eine kanalaufgelöste Abbildung geben statt der Summe. Rausch-artiges R heißt: der Kollaps ist für diese Senke verlustfrei.
>
> Kurz: Die Sonne darf weiter nur einen Skalar sehen — aber sie sieht ihn am Ende, am Radiator, pro Auge; das Feld selbst bleibt stumm und 9-stimmig.

**Weitere Stimmen (Kurzfassung):** Claude/Sonnet 5.5 (rein: per-Kanal-Voxel-Schicht, tri-state; `/9.0` zweiter Fehler) · MiniMax M3 (*„die Sonne macht kein Integral über heterogene Photonen — sie lässt jeden Channel mit seiner eigenen Optik durch"*) · Mistral/Vibe (`sum()/9.0` nur bei einheitenkompatiblen, gleich kalibrierten Kanälen; sonst „Meinungsäußerung des Observers über Gewichte, die er nicht kennt") · Gemini 3.1 Pro (vorzeitige Skalar-Reduktion erzwingt den Observer-Bias) · Qwen3.7-Plus · DeepSeek Chat · Proton Lumo · Duck/GPT-6 Luna · Inkling 975B · GPT-OSS 120B · DeepSeek V4 Pro. **`pending`:** Qwen3.8-Max (Deep-Thinking >2 min ohne Antwort), Kimi (`kimi.ai`, free quota exhausted).
