<!--
  title: Antworten — CEIC und UT Dallas (2026-10-16)
  class: concept
  date: 2026-10-16
  status: live
  see-also: docs/handover/mail-2026-10-15-ceic-ucar-re3data.md
-->
# Antworten — CEIC und UT Dallas (2026-10-16)

Zwei gemessene Rückmeldungen auf die drei Mails. Beide schließen ein `pending`.

## China Earthquake Data Center (CEIC) — Zugang = eingeschränkt

> „…Ich habe den data access als eingeschränkt abgeändert. Die Datenbank mit Übersicht der
> Metadaten ist … unter ‚Data Service/Datenübermittlung' (数据汇交) weiterhin offen zugänglich.
> Ich habe außerdem unter Policies die Information über die Einschränkung des Zugangs hinzugefügt
> (关于进一步明确地震科学观测数据开放范围的公告)."

**Verdikt (gemessen, nicht vermutet):**
- **Die bedingten Datensätze = `restricted`** — kein `free-open`; die Nutzung bleibt `local-only`
  (kein `phi`-Block, kein CDN), bis eine Lizenz/ein Zugang gemessen ist.
- **Die Metadaten-Übersicht** (`Data Service` / `数据汇交`) = **offen zugänglich** → `free-open`
  für den Metadaten-Index.
- **Neue Policy-Quelle:** 关于进一步明确地震科学观测数据开放范围的公告 (unter Policies).

## UT Dallas — NWM/C/NOFS: die Daten EXISTIEREN NICHT

> „Regrettably the data that you seek is not available. … zwei getrennte Sensoren. Einer
> (Druckdifferenz) hatte ein Leck in der Druckkammer … der andere (retarding potential) hatte
> zu hohe Rauschpegel … das Instrument funktionierte nicht wie gewünscht."

**Verdikt (gemessen):**
- **`absent` — nicht `pending`.** Kein Wert, keine Ernte: das Instrument hat nicht funktioniert,
  die Daten wurden nie erzeugt. **Kein Register-Block, kein Harvest.** Der Faden ist **tot**,
  nicht wartend — und das ist ehrlich geschlossen, nicht als 0.0 gebucht.

## Auftrag
Die beiden Verdikte in `phi/sources.φ` setzen (CEIC: `restricted` + Metadaten `free-open`;
UT Dallas/NWM: `absent`), und den `quellen-neu-und-riss-ucar`-Faden entsprechend schließen.
