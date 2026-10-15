<!--
  title: Briefs — Mountain und Mycelium (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 73b41aa524209d7fa028e7ff88c091aec799374ddc180926396baec1eaa4d999
  status: live
  see-also: docs/handover/handover-2026-10-15-quellen-neu-und-riss-ucar.md
-->
# Briefs — Mountain und Mycelium (2026-10-15)

Wortgetreu einlegbar in `/mountain` und `/mycelium`. **Beide in einer FRISCHEN Session** — die laufende hat ihre Konfig vor den `all_wide`-Edits geladen (`opencode.json:186-187`), erst eine neue lädt sie.

---

## /mycelium

```
Ein Pass. Zuerst lesen:
· docs/handover/handover-2026-10-15-quellen-neu-und-riss-ucar.md (besonders § Nachtrag: WDC-B)
· deinen § An mycelium-Block

AUFGABEN:
1. WDC-B-RETTUNG ERWEITERN — von 32 auf ~205 ladbare Dateien.
   Geprüfte Payload-Gruppen (Wayback): sep/GravConst/data (51) · stp/geo_min.val (44, MSK .ZIP
   + MIR .WDC) · sep/data/gdata (31) · sep/data/sdata (+18 neu) · stp/geomagni.ind/aa (14) ·
   stp/catal_pc (8) · gps (7). URL-Gruppen (CDX): sep/strong (431) · sep/traverses (174) ·
   sep/hp (166). CDX-Beleg: tool_13e60a7bd001EFnLvRBuhmSeS2 (1276 wdcb/*) + die frische CDX.
2. DEN LEBENDEN BAUM registrieren: www.wdcb.ru/sep/ (Apache, kein Key; Site-Policy: frei für
   Forschung/Lehre, nicht-kommerziell, Nennung) + die Schwester-Hosts usd.wdcb.ru (Weltraum),
   esdb.wdcb.ru (DOI), rjes.wdcb.ru, stp.wdcb.ru.
3. Die drei Zenodo-Dateien (CMONOC-GNSS, CEDC) ins Manifestations-Register.

Kein Planungstheater. Am Ende: ein Commit (deine Pfade).
```

---

## /mountain

```
Ein Pass. Zuerst lesen:
· Stehender Pass docs/zustand/standing-pass.md (Runde offen; rote/ungeklärte Läufe alle unread
  — Actions-Rückstau; Schwerpunkt-Träger: die zwei Matrix-Läufe wdcb-obn-wayback-cdn + zenodo-raw-cdn
  = Mycelium, Träger ungemessen)
· Blatt docs/handover/handover-2026-10-15-quellen-neu-und-riss-ucar.md (inkl. § Nachtrag)
· dein § An mountain-Block (Myceliums Aufträge xfer:mycelium-351-1…-6, live gehalten)

REGISTER-SCHREIBAKTE (deine Zeilenklasse):

1. Die Kraft-/Lizenz-Zeilen aus Myceliums SECHS Aufträgen (Token xfer:mycelium-351-1…-6),
   gemessen (Belege im Blatt):
   · -1 GSRAS-Teleseismic (offen, keine formale Lizenz)
   · -2 CMONOC-GNSS/Zenodo (cc-by-4.0)
   · -3 CEDC/Zenodo 8084659 (cc-by-4.0, xlsx)
   · -4 NGL-ARDA (Policy, Nennung erbeten)
   · -5 ISC (nur die fehlende Kraft-Zeile)
   · -6 WDC-B/Obninsk — ERWEITERT (s. Punkt 2)
   · IGETS/GFZ (CC-BY-4.0): kein Auftrag — Block trägt terms/field; die compiler-Zeile ist gesetzt.
   · BGI = agrav-Eintrag, CB = Netzwerk-Eintrag → keine eigene Zeile.

2. WDC-B — ZWEI getrennte Zeilenklassen (die 32 sind 2,5 %):
   · Wayback-Erbe (die 32 + die neuen Gruppen): terms gemessen CC-BY-NC-4.0 (die Bulletin-Dateien),
     field (fixed-column Text, lat/lon/Tiefe/M); origin/url = Mycelium.
   · Der LEBENDE Baum www.wdcb.ru/sep/: KEINE Datei-Lizenz — die SITE-POLICY ehrlich benannt
     (frei für Forschung/Lehre, nicht-kommerziell, Nennung Pflicht; keine CC-Ersatzbehauptung).

3. ARDA formal umklassifizieren: GNSS-Station (NGL, Ardahan, Türkei) — die Schwere-/Seismik-Zeile
   war eine Kategorienverwechslung. Beide Enden nennen.

4. UCAR-KORREKTUR: die stale Zeile „Instrumentenkoordinate ungemessen" (folge346/347) streichen —
   River-213 HAT gemessen: on earth 54.60282 -113.64442 693 (AUGO II / AUGSO; SRTM30m 696.0 m →
   orthometrisch). Beide Enden nennen, nie glätten.

5. „EGGN" (Fehlname): die Zeile auf EQUIP-G (pending) + die realen Schwestern IGETS/BGI korrigieren.

6. ALT-DEKLARATION (Mycelium-316 schließen): `on earth <alt>` trägt EINE Haus-Skala — orthometrisch
   (MSL/EGM96). In den Datenkontrakt docs/concepts/archivar-mathematikerin.md. Kein neues Feld.

Kein Planungstheater. Am Ende: register_sort, ein Commit (nur deine Pfade), git_safety --close.
```

---

**Reihenfolge:** frische Session → `/mycelium` (die Wege) → danach `/mountain` (die Verdikte). Beide Briefe sind wortgetreu und decken sich mit dem Blatt.
