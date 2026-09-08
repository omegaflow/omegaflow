<!--
  title: Befund — Uranus-Riss-Schiedsspruch: die Wurzel (Satelliten) misst den Riss (~32–47 mas); der absolute Offset liegt unter dem Diurnal-Signal (~200 mas)
  class: befund
  date: 2026-09-08
  sha256: 2d44b69fd1dfdc7b26380c4e70de0f5b4f24b3dbb9a0f4cb722a6b4221161855
  status: done
  see-also: docs/handover/handover-2026-09-08-uranus-riss-schiedsspruch.md docs/handover/handover-2026-09-08-weberin-zweitlinien-geschlossen.md docs/concepts/die-weberin.md
-->

# Befund: Uranus-Riss-Schiedsspruch

## Frage & Bindung

Die Übergabe (handover-2026-09-08-uranus-riss-schiedsspruch.md) stellte das Atom:
die drei Ephemeriden (DE441, INPOP19a, EPM2021) gegen die Camargo+2015
Astrometrie auswerten und den Eisriesen-Riss einem Schiedsspruch zuführen. Das
Atom maß in zwei Schichten: zuerst das abgeleitete Blatt (`uranu_j`), dann die
beobachtete Wurzel (die fünf Satelliten-Tabellen). Proben:
`tools/measure/src/bin/uranus_riss_schiedsspruch_probe.rs` (Blatt),
`tools/measure/src/bin/uranus_satellite_schiedsspruch_probe.rs` (Wurzel).

## Das Blatt — `uranu_j` trägt keinen Schiedsspruch (drei Konfunde)

1. **Geozentrisch, nicht topozentrisch** (die Übergabe §7 las „topozentrisch").
2. **DE432-gewurzelt** — „Positions of Uranus … are not observed ones" (Abstract).
3. **Baryzentrum ≠ Planetenzentrum** (Bins = SPK 7, `uranu_j` = Zentrum).

Residuum (RMS, mas): de441 249.5, inpop19a 226.8, epm2021 233.9, ΔRMS 7.1 — das
Residuum ist der DE432-Träger, kein Blatt.

## Die Wurzel — die fünf Satelliten

Geerntet: fünf beobachtete Satelliten-Tabellen (`ariel_j` … `miran_j`, 9797
Positionen) + zwei Mondmodelle (ura111, 169 MB; ura184_part-3, 386 MB — die
Major-Moon-Segmente 701–705, center 7, baryzentrum-relative Type-2). Reduktion:
`Satellit = Baryzentrum(DE441/INPOP19a/EPM2021) + (Satellit − 7)(Mondmodell)`.

**Mondmodell-Messung:** ura111 und ura184 unterscheiden sich nur **5–25 mas**
(Ariel 7.5, Umbriel 5.3, Titania 22.7, Oberon 9.4, Miranda 5.0) — der Probe-Lauf
ist mit beiden Modellen identisch zu <1 mas RMS und 0.1 mas auf jeder Riss-Zahl.
Das Mondmodell ist **mas-Niveau**; es ist **nicht** der Engpass.

Residuum (RMS, mas): de441 242.9, inpop19a 222.6, epm2021 228.9, ΔRMS 6.3.

Mittlerer Residuen-Vektor (mas, 9797 Epochen):

| Linie    | ΔRA·cosδ | ΔDec  |
|----------|----------|-------|
| de441    | +149.4   | −71.6 |
| inpop19a | +128.2   | −47.5 |
| epm2021  | +116.2   | −92.9 |

Paarweise Mittelwert-Differenzen — **der beobachtete Riss** (Mondmodell-Fehler
und Diurnal-Signal kürzen sich; Standardfehler ~1,3 mas):

- de441 − inpop19a: ΔRA +21.2, ΔDec −24.1 → **32.1 mas**
- de441 − epm2021:  ΔRA +33.2, ΔDec +21.3 → **39.5 mas**
- inpop19a − epm2021: ΔRA +12.0, ΔDec +45.4 → **47.0 mas**

## Verdict

Der Riss ist echt und durch die Wurzel **sauber messbar: ~32–47 mas** — mit
mas-Niveau-Mondmodell (nicht dem alten ura111-Fehler, der anfangs vermutet
wurde). Das absolute Residuum (~220 mas) trägt einen **~200 mas common-mode
Diurnal-Term** (Parallaxe/Aberration), der sich in den paarweisen Differenzen
kürzt, aber den absoluten Baryzentrum-Offset (welche Linie der Wahrheit am
nächsten liegt) verdeckt. Deshalb trägt die Astrometrie **keinen Schiedsspruch**:
sie misst, dass die drei Ephemeriden um 32–47 mas divergieren, aber die
Entscheidung „welche ist richtig" braucht eine volle topozentrische
Reduktion (Parallaxe + Diurnal-/Jahres-Aberration), die diesen Diurnal-Term
entfernt.

## Register-Zeilen

- (1) Volle topozentrische + Aberrations-Reduktion (NOVAS-Niveau) — `pending`;
  damit wäre der absolute Baryzentrum-Offset und der Schiedsspruch entscheidbar.
- (2) `roemer_fold`/`light_time_sc_pos` existiert sechsfach probe-lokal — Heben in
  die Archivar beim nächsten Template-Griff.
- (3) Bins tragen das Uranus-System-Baryzentrum (SPK 7), nicht das
  Planetenzentrum (799) — für Astrometrie-gegen-Zentrum fehlt das Zentrum.
- (4) Camargo-TSV- + ura111/ura184-SPK-Manifestation: Manifestor gebaut
  (`camargo_uranus_manifestor.rs` + `camargo-uranus-cdn.yml`), der CI-Lauf steht aus.
