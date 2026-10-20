<!--
  title: Vorlage je Dateityp — Kopf, Pflichtfelder, Namensform
  class: spec
  date: 2026-10-19
  sha256: bc8394c292ccf9d11c6514b9e59bb0a9ce66d91d2f9bb6fc5fe43578162d5f9c
  status: live
  see-also: docs/concepts/house-rules.md bin/layout_check work/post/ work/handover/
-->
# Vorlage je Dateityp

Diese Spec legt **eine** Vorlage je Dateityp fest: den Kopf, die Pflichtfelder, die
Namensform und die Ablage. Sie ist eine Form, keine zweite Registratur — die
Vorlage macht sichtbar, was ein Dokument trägt; sie ersetzt keine Messung und
keine Lizenzzeile.

Die Klasse steht im Kopf, nicht im Ordner: derselbe Text kann als `survey`,
`spec`, `concept`, `blatt`, `paper` oder `project` leben. Die Ablage folgt dem
Kanon (`layout_check`), die Klasse folgt dem Inhalt.

## 1. Markdown-Dokument (`.md`)

**Ablage:** `docs/<klasse>/<slug>.md` (Klasse = Ordner: `surveys`, `specs`,
`concepts`, `blatt`, `paper`, `project`). **Handover** liegen in `handover/` und
tragen denselben Kopf mit `class: handover`.

**Kopf** — der erste Block der Datei, unmittelbar vor dem Titel:

```markdown
<!--
  title: <Titel — ein Satz, kein Kürzel>
  class: <survey|spec|concept|blatt|paper|project|handover>
  date: <YYYY-MM-DD>
  sha256: <hex — über den Body ohne diesen Kopf>
  status: <live|done|consumed|superseded|archived>
  owner: <linie|sitz — wer hütet es jetzt>
  ack: <linie|sitz> <ISO-8601 — Empfang, nicht Genehmigung>
  next: <nächster Akt|Trigger — oder none>
  orphan-after: <YYYY-MM-DD — oder none>
  see-also: <pfad> [<pfad> …]
-->
```

**Pflichtfelder:** `title`, `class`, `date`, `sha256`, `status`, `owner`, `ack`.
`next` und `orphan-after` sind Pflicht, solange `status: live` ist; bei
`done`/`consumed`/`archived` dürfen sie `none` sein. `see-also` ist Pflicht,
sobald das Dokument einen Träger, eine Quelle oder ein Werkzeug nennt. `sha256`
ist der Hash über den Body **ohne** den Kopf:

```sh
sed '/^<!--/,/^-->/d' <datei> | sha256sum
```

**Quittung.** `owner` ist der aktuelle Halter; `ack` ist die Empfangskette
(wiederholbar, je Übergabe eine Zeile). Der erste `ack` ist der Autor; jede
Übergabe ändert `owner` und fügt einen `ack` hinzu. **Ohne `ack` ist das
Dokument ein Orphan** — eine Flaschenpost, kein Dokument. Die Quittung ist
Empfang, nicht Genehmigung: der Empfänger bestätigt, dass er es hat.

**Body:** `# <Titel>` als erste Überschrift, danach die Abschnitte des Typs.
Ein Dokument ohne Kopf ist eine Notiz, kein Dokument; ein `status: live` ohne
`sha256` ist eine ungemessene Behauptung.

## 2. Register- und Zustandsdatei (`.φ`)

**Ablage:** Register `phi/sources.φ`; Zustand `work/zustand/**/*.φ`.

**Blockform** (Register — ein Block je Quelle, Leerzeile getrennt):

```text
url <CDN- oder Quell-URL>
terms <token> <terms-URL>
format <format-token>
origin <Original-URL>
compiler <pfad/zu/compiler.rs>
at <body|earth|sun|…>
no-cadence            # oder: ttl <sekunden>
field <key> <key> <kernel> <kraft> <einheit> <tau> <…>
```

**Pflichtfelder je Registerblock:** `url`, `terms`, `format`, `origin`,
`compiler`, `at`, `field` (oder `witness`/`record`), `ttl` **oder**
`no-cadence`. Ein Block ohne `terms` ist `pending`, nie frei; ein Block ohne
`compiler` ist eine Quelle, kein Arm.

**Zustandsdatei:** Kopf als Kommentar, dann der gemessene Inhalt:

```text
# <Titel> — <Datum> · Linie <linie>
# Transfer: <token>
# Messgrundlage: <HEAD|URL|Lauf> · <Datum>
# GEMESSEN: <was, wo, wie>
# VERDIKT: <ein Satz>
# RISS: <was offen bleibt — oder „keiner">
```

**Pflichtfelder:** Titel, Datum, Linie, Messgrundlage, Verdikt. Ein Verdikt ohne
Messgrundlage ist eine Meinung.

## 3. Handover

**Ablage:** `work/handover/handover-<YYYY-MM-DD>-<linie>-folge<n>.md`.

**Kopf:**

```markdown
<!--
  title: Handover — <Linie> <Folge> (<Datum>)
  class: handover
  date: <YYYY-MM-DD>
  sha256: <hex — Body ohne Kopf>
  status: live
-->
# <Linie>-<Folge>
```

**Pflichtfelder:** `title`, `class: handover`, `date`, `sha256`, `status`.
**Cap:** der Body ist **≤ 1 KiB** (der Kopf zählt nicht). Das Handover trägt nur
Offenes: je Punkt `Status`, `Bindung`, `Trigger`, `Lage` (mit `(gemessen …)`),
`Blockade`, `Braucht`. Keine Geschichtsabschnitte, keine erledigten Punkte,
keine Kopie geteilten Zustands.

## 4. Post / Dispatch

**Ablage:** `work/post/<YYYY-MM-DD>-<von>-an-<an>-<slug>.md`; erledigte Briefe
nach `work/post/archiv/`.

**Namensform:** `<datum>-<von>-an-<an>-<slug>.md`, alles klein, Bindestriche,
kein Leerzeichen. `<von>` und `<an>` sind Liniennamen (`future`, `sensory`,
`mountain`, `river`, `mycelium`, `operator`).

**Kopf und Pflichtabschnitte:**

```markdown
# Dispatch — <Linie>, <YYYY-MM-DD>

## Die eine Aufgabe

<ein Satz, eine Aufgabe, ein Gegenstand>

## Lies (nur die Stellen — nichts sonst)

- <pfad>
- <pfad>

## Lies NICHT

Das Repo, fremde Handover, die Geschichte (git), die Form-Karte, die Tools-Map.
Deine **drei Stellen**: dieser Dispatch · dein Handover (≤ 1 KiB) · deine Post.

## Fertig, wenn

- <prüfbarer Befehl oder messbare Bedingung>
```

**Pflichtfelder:** Datum, Adressat, eine Aufgabe, Lesestellen, Fertig-Kriterium.
Ein Dispatch ohne prüfbares „Fertig, wenn" ist eine Bitte, kein Auftrag.

## 5. Quelle

**Ablage:** `work/sources/<netloc>/<datei>` (ein Ordner je Host); der Index liegt
in `work/sources/<netloc>/index.md` oder in der Quelle-Zeile des Registers.

**Index-Zeile:**

```text
<datei> | <origin-URL> | <bytes> | <sha256> | <terms> | <abgerufen YYYY-MM-DD>
```

**Pflichtfelder:** Dateiname, Origin-URL, Byte-Zahl, `sha256`, `terms`, Abrufdatum.
Der Index ist die Karte, nicht die Kopie: die Datei bleibt die Messung, die Zeile
der Zeiger. Nicht-redistributierbare Daten liegen **nicht** unter `sources/`,
sondern in `private/nicht-redistributierbar/` (kein CDN, kein Index mit Inhalt).

## Check-Vorschlag für `layout_check`

Ein neuer, **opt-in** Modus `bin/layout_check --templates` (ohne Flag bleibt der
heutige Aufruf unverändert):

| Typ | Prüfung | Befund-Token |
|---|---|---|
| `.md` | Kopf vorhanden; `title`, `class`, `date`, `sha256`, `status` gesetzt; `sha256` = Body-Hash; `see-also`-Pfade existieren | `MD_HEADER_MISSING`, `MD_FIELD_MISSING`, `MD_SHA_MISMATCH`, `MD_SEE_ALSO_ABSENT` |
| `.φ` Registerblock | `url`, `terms`, `format`, `origin`, `compiler`, `at`, `field`/`witness`, `ttl`/`no-cadence` vorhanden | `PHI_BLOCK_FIELD_MISSING` |
| Handover | Body ≤ 1024 B; keine Überschrift `## …geschlossen`/`## …erledigt`; je Punkt `Lage` mit `(gemessen` | `HANDOVER_OVER_CAP`, `HANDOVER_HISTORY`, `HANDOVER_LAGE_UNMEASURED` |
| Post | Dateiname passt `<datum>-<von>-an-<an>-<slug>.md`; Abschnitte `Die eine Aufgabe`, `Fertig, wenn` vorhanden | `POST_NAME`, `POST_SECTION_MISSING` |
| Quelle | `work/sources/<netloc>/` existiert; Index-Zeile mit `sha256`/`terms`/Datum | `SOURCE_INDEX_MISSING`, `SOURCE_FIELD_MISSING` |

Der Check ist eine **Messung**, kein Gate: er zählt Befunde und exitet nur mit
`--fail`. Er schreibt keine Datei und heilt keinen Kopf — die Heilung ist ein
eigener Akt der Linie, die den Typ besitzt.

## Riss, der bleibt

`sha256` im Kopf ist nur so gut wie die Regel, ihn nach jeder Body-Änderung neu
zu rechnen. Ein veralteter Hash ist ein `MD_SHA_MISMATCH`, kein stiller Segen;
der Check misst ihn, die Linie zieht ihn nach.
