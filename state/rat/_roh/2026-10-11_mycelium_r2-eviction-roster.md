# Roster — R2-Hot-Tier Eviction-Policy (Mycelium, 2026-10-11)

Gleiche Runde an alle vier Frontier-Seats der Linie (`mycelium-ui`), gleiche Frage:
kleines Hot-Tier (Cloudflare R2, 10 GB gratis, Lesen kostenlos) für wenige kleine
Dateien (Manifeste/Indizes) neben einem 1,76-TB-Archiv (GitHub Releases); **kein
Leser** im Baum. Welche Eviction-Policy (LRU/LFU/TTL/keine) — und wie „Zugriff"
messen, wenn niemand liest.

## Duck.ai (GPT-6 Luna)

> „Wähle zunächst keine automatische Eviction: Bei 9 GB Belegung Alarm auslösen und
> nur nachweislich veraltete Dateien löschen. ‚Zugriff' ist ein erfolgreicher GET;
> solange kein Programm liest, gibt es keine Zugriffsdaten und LRU/LFU können nichts
> Sinnvolles steuern."

## Claude (Sonnet 5.5 · Extra hoch)

> „Regel: Sobald das Bucket 8 GB erreicht (80 % des Gratis-Kontingents), löscht ein
> Cron-Job die Objekte nach ältestem LastModified, aber nie die Manifeste und Indizes
> der aktuellen Release-Version (gepinnt). Solange niemand liest, ist ‚Zugriff' gleich
> Schreibzeit, denn R2 liefert meines Wissens pro Objekt keine Last-Access-Zeit. Sobald
> der erste Reader existiert, kommt ein Worker vor das Bucket, der pro Key last_read
> schreibt, und ihr wechselt auf LRU."

(LRU/LFU ohne Lesesignal = willkürlich; reine TTL verwirft aktuelle, selten gelesene
Indizes; keine Eviction rutscht unbemerkt über 10 GB. GitHub hält die Wahrheit → ein
evictetes Objekt ist nur ein Cache-Miss.)

## Qwen (Qwen3.7-Plus)

> „Ich empfehle TTL, da bei fehlenden Lesezugriffen eine zeitbasierte Ablaufregel alte
> Manifeste automatisch bereinigt, während LRU oder LFU mangels Metriken ins Leere
> laufen. Als ‚Zugriff' misst du dabei einfach den Zeitstempel der letzten Aktualisierung
> (Upload/Write), da die Dateien als reine Versions-Snapshots ohnehin nur bis zur
> nächsten Überschreibung vorgehalten werden."

## Z.ai (GLM-5.3 · Deep Think)

> „Der eigentliche Hebel ist Admission, nicht Eviction: Metadaten in KB- bis MB-Größe
> füllen 10 GB praktisch nie. Läuft das Tier trotzdem voll, sind Datenblöcke
> hineingerutscht — dann ist das ein Writer-Bug, kein Policy-Problem. Also besser:
> Invariante ‚Hot-Tier = nur Bootstrap-Metadaten', weiches Cap bei ~8 GB, Alarm statt
> Eviction. […] Zugriff messen, sobald es Leser gibt: R2 protokolliert keine
> objektbezogenen Reads (Audit-Logs sind Control-Plane; Event-Notifications sehen nur
> Erzeugen/Löschen). Pro Objekt nur per Worker auf Custom Domain + Logpush/Workers-Logs.
> R2-Lifecycle löscht nach Alter ab Erzeugung, nicht nach letztem Zugriff; kein
> Bucket-Versioning — ‚letzte K Versionen' regelt der Writer beim Upload."

## Konvergenz

- **Keine frequenzbasierte Eviction, solange kein Leser** — Rat + 3/4 Seats.
- **Admission statt Eviction** (Z.ai, stärkste Stimme): die Aufnahme-Regel (nur kleine
  Manifeste/Indizes, Cap 16 MiB) ist die eigentliche Policy; dann füllt sich der Tier nie.
- **Alarm bei ~8 GB statt automatischer Löschung** (Duck 9 GB · Claude 8 GB · Z.ai 8 GB).
- Zwei R2-Fakten (Z.ai, am Baum zu prüfen): keine objektbezogenen Read-Logs; Lifecycle
  nach Erzeugungsalter, kein Bucket-Versioning.

## Riss (getragen, nicht geglättet)

Kein Seat kann „Zugriff" messen, weil kein Leser existiert; LRU/LFU/TTL hängen alle an
einem Signal, das der Baum nicht hat. Bis der Verifier-Worker Reads als Zähler
zurückmeldet, ist jede zugriffsfrequenz-basierte Policy eine Fabrikation.
