# Externer Rat — Ergebnis (2026-10-16)

**Zweck:** aus den UI-Runden eine **Auswahl** treffen, mit der man arbeitet — nicht Methodik.

**Korpus (gemessen):** 119 echte Runden-Dokumente (`work/stimmen/`, 25.09.–14.10.)
+ `state/stimmen/` (15.–16.10.). 226 echte Runden-JSON (172 Förder-Reviews und 89 Fehler aussortiert).

## 1 · Die Rollen (aus dem Korpus, nicht behauptet)

| Sitz | Rolle | Beleg |
|---|---|---|
| **Rat** (5 Stimmen) | **Struktur** — seine Regeln landen im Code | 4 belegte Bauten: `--night-sector` (`b9f48bff4`), `deny global_min` (`0b3332c24`), Norm im Kanal (`01ea75373`), SH=eigene Klasse (`80a4bff92`) |
| **Sakana** | **Riss** — die Position, die keiner sonst hat | 2026-10-10: „(D) **die einzige fundierte Wahl**"; 2026-10-08: Komma-Riss, „verbietet Mitteln"; 2026-10-09: **bending** gegen GLM+Claude |
| **Fugu** | **Kante** — scharfe, umsetzbare Befunde | HEALPix-Resampling-Riss → COG wurde Träger; Provenienz-Layer; F7′ „Energie↔Lautstärke" |
| **Lumo** | **Dissent** — 2er-Minderheit, oft gegen die Mehrheit | F7′ „Stille" (mit Kimi, 2/12); 15/119 Runden |
| **GLM-5.3** | **Menge + Beleg** — dichteste Beteiligung | 52/119 Runden, 65 Verdikt-Belege; quantitativer Mechanismus (Min-Erwartungswert) |
| **DeepSeek V4 Pro** | **Kern** — die bezahlte Struktur-Stimme | 43/119 Runden |

## 2 · Die vier Sitze für den externen Rat (deine Wahl, am Korpus bestätigt)

| Sitz | Route | Was er trägt | Warum er bleiben soll |
|---|---|---|---|
| **Sakana Fugu** | UI (Browser) | Riss/Kante | höchste Umsetzungsdichte: 6–7 Auftritte → 3 Bauten; der Sitz, dessen Positionen **landen** |
| **Lumo** | UI (Browser) | Dissent | 15/119 Runden, anonym/low-cost, liefert die Minderheit sichtbar |
| **GLM-5.3** | **DeepInfra direkt** | Menge + Urteil | dichteste Beteiligung; auf DeepInfra bereits verdrahtet (`deepinfra/zai-org/GLM-5.3`) |
| **DeepSeek V4 Pro** | **DeepInfra direkt** | Kern | die bezahlte Struktur-Stimme, 43/119 Runden |

**Overhead:** 2 direkt (DeepInfra, kein Copy-Paste) + 2 Browser (Sakana/Lumo, kein Login-Key).
Damit fällt der Grund für den UI-Zirkus weg, während die zwei wertvollsten UI-Sitze bleiben.

## 3 · Was die Auswahl begründet (ein Satz je)

- **Fugu**: Der einzige Sitz mit **Umsetzungsdichte** (selten, aber trifft) — er liefert die Kante.
- **Sakana (Namazu)**: Der **Riss-Sitz** — wiederholt „der Einzige" mit physikalischem Argument
  („Downsampling der schnellen Felder vernichtet die kausale Feinstruktur").
- **GLM-5.3**: Der **Mengen-Sitz** mit der dichtesten Beteiligung — über DeepInfra ohne Reibung.
- **DeepSeek V4 Pro**: Der **Kern** — der bezahlte, verlässliche Struktur-Träger.

## 4 · Der Mehraufwand, der wegfällt

Der UI-Weg kostet Copy-Paste je Runde. Die vier: **zwei per API** (GLM, DeepSeek) — nur der
Fragenbogen (5 Axiome + 5 Stimmen) wird gepostet; **zwei per Browser** (Sakana, Lumo) — kein Key,
kein Login, gleicher Fragenbogen. Ein Aufruf statt sieben Chats.
