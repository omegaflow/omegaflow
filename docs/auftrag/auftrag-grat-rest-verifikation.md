<!--
  title: Auftrag — Grat-Reste: KDE-h-Sensitivität und räumliche Richtungs-Verifikation
  class: auftrag
  date: 2026-09-05
  sha256: 571561472fb8da1a470746be43f3fee89d6e6ecbf560e348c41818a57be52c8c
  status: done
  see-also: docs/befund/befund-grat-trishuli-konditionierung.md docs/auftrag/auftrag-grat-trishuli-konditionierung.md docs/paper/blatt-pfeil-sturzflut-tibet.md
-->
# Auftrag: Grat-Reste — KDE-h-Sensitivität und räumliche Richtungs-Verifikation

## Zweck

Der Befund `docs/befund/befund-grat-trishuli-konditionierung.md`
(GESCHLOSSEN 2026-09-05) hat die Trishuli-Umkehr gemessen: die im
Quell-Blatt als Regen→Pegel registrierte 0.265 ist die Gegenorientierung
Pegel→Regen — die `te_pair_probe`-Beschriftung „TE(a→b)" ist gegen die
Schätzer-Orientierung gespiegelt (zweites Argument = Quelle, `te.rs:92`).
Der Befund schließt die Kollab-Stille und benennt zwei offene Reste, die
vor dem Vertrauen in die räumlichen Richtungs-Labels des Quell-Blattes
verifiziert werden müssen. Dieser Auftrag registriert beide als die
offenen Zellen; kein Lauf dieses Auftrags ist Teil der Registrierung.

## Die zwei offenen Zellen

### (a) KDE-Bandbreiten-Sensitivität (h, Faktor 2) — GELAUFEN (2026-09-05)

Die Zelle „KDE-Bandbreiten-Sensitivität (h, Faktor 2)" der
Befund-Pflichten (Pflicht 4) bleibt offen: das gebaute Instrument
(`transfer_entropy_conditional`, `cross_te_screen`) legt die
Silverman-Bandbreite intern fest und setzt keinen Bandbreiten-Faktor aus.
Die Schließung braucht ein Instrument, das einen
Bandbreiten-Faktor freigibt, oder einen eigenen Sensitivitäts-Lauf —
ein scoped Code-Nachfolger, `pending`. Die Zelle bleibt `pending`, 0
honored.

**GELAUFEN (2026-09-05):** Der Sensitivitäts-Lauf steht
(`tools/measure/src/bin/trishuli_kde_sensitivity_probe.rs`, probe-lokal,
`te.rs` unberührt). Die überlebende konditionierte Magnitude Pegel→Regen
ist robust gegen die Faktor-Verdopplung — alle sechs Zellen (Temperatur/
Druck/Feuchte, Lag 24/48) bleiben bei h × 2.0 über ihrer Null. Ehrlicher
Nebenbefund: die besiegte Regen→Pegel-Richtung kreuzt bei h × 2.0 marginal
ihre kollabierte Null unter Temperatur (Lag 24/48) und Feuchte (Lag 24);
unter Druck bleibt sie still. Die Zelle ist damit **erfüllt** (0 honored:
der marginale Kreuzungs-Nebenbefund ist benannt, nicht geglättet).

### (b) §3.3-räumliche und §3.7-Bahrabise Richtungs-Verifikation — GELAUFEN (2026-09-05)

Die räumlichen Rasuwa→Gyirong-Pfeile (§3.3) und die Bahrabise-Lesungen
(§3.7, n = 169) des Quell-Blattes
`docs/paper/blatt-pfeil-sturzflut-tibet.md` liefen durch `te_pair_probe`,
dessen gedruckte „TE(a→b)"-Spalte gegen die Schätzer-Orientierung
gespiegelt ist. Jedes Richtungs-Label ist gegen die Schätzer-Orientierung
zu prüfen, bevor es Vertrauen trägt — die co-lokale Umkehr der
0.265 (`docs/befund/befund-grat-trishuli-konditionierung.md`) ist
der Beleg, dass die Spiegelung an den Zahlen wirkt. Die Prüfung ist
ein Messlauf, `pending`.

**GELAUFEN (2026-09-05):** Der Messlauf steht — beide Abschnitte sind
gespiegelt. §3.3: der gedruckte „Rasuwa → Gyirong (Lag 12–24)" ist
physikalisch **Gyirong → Rasuwa** (Oberlauf/Tibet → Unterlauf/Nepal,
die erwartete Becken-Entwässerung), verifiziert mit `cross_te_screen`
und quergeprüft gegen das Kollab-Screening §3.1 (identische Zahlen);
der `te_pair_probe`-Wurzelfix (a→b misst jetzt a→b) ist committet. Die
§3.3-Richtungskorrektur an den drei betroffenen Dokumenten
(`blatt-pfeil-sturzflut-tibet.md`, `causal-arrow-preregistration.md`,
`blatt-der-grat.md`) ist angewandt. Zelle (b) ist damit **erfüllt**.
Zelle (a) (KDE-h) siehe oben — ebenfalls erfüllt.

## Kernregel (0 honored)

- Keine neue Ernte. Die Verifikation nutzt die Reihen, die das
  Quell-Blatt nennt; was der Lauf nicht als Zahl trägt, bleibt `pending`.
- Eine Zahl ist erst ein Messergebnis, wenn der Lauf steht. Kein
  Richtungs-Label wandert aus der Blatt-Tabelle als bestätigt, ohne den
  frischen Lauf getragen zu haben.
- Die Schwelle ist die bestehende: Residuen-Surrogat /
  phasenrandomisierte Surrogate, mean + 2σ.
- Stille ist ein Verdikt. Trägt der Messlauf keine Zahl, ist das die
  Antwort; keine Zelle wird mit einem Wert gefüllt, den der Lauf nicht
  gemessen hat.

## Lieferung

Die geschlossenen Zellen als Fortschreibung des Quell-Blattes oder als
benannte Blätter, auf die die Blatt-Zeilen zeigen. Die benannte Stille
ist eine Lieferung. Die TODO-Zeile dieses Auftrags wird im selben Commit
geschlossen oder fortgeschrieben.

## Abschluss

Der Auftrag ist GESCHLOSSEN, wenn beide Zellen einen stehenden Lauf
tragen und das Ergebnis als Blatt-Fortschreibung oder Blatt kommittiert
ist (Stille ist ein Verdikt). Bis dahin `pending` — dieser Eintrag
registriert die Ordnung; die Ausführung ist nicht Teil der Registrierung.

**GESCHLOSSEN (2026-09-05):** Beide Zellen tragen einen stehenden Lauf.
Zelle (b): §3.3/§3.7 gespiegelt verifiziert, `te_pair_probe`-Wurzelfix
und die §3.3-Korrektur an drei Dokumenten committet (6f9076a). Zelle
(a): KDE-h-Sensitivität gelaufen und robust
(`trishuli_kde_sensitivity_probe.rs`, te.rs unberührt). Der Auftrag ist
damit erfüllt; die schwache Regen→Pegel-Kreuzung bei 2h ist als
Nebenbefund benannt (0 honored), kein offener Rest dieses Auftrags.
