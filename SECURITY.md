# Sicherheitsfunde

Diese Datei ist der Meldeweg. Wer eine Lücke findet, meldet sie **privat**, nicht
im offenen Issue.

## Der Weg

GitHub Security Advisories:
https://github.com/omegaflow/omegaflow/security/advisories/new

Das ist ein privater Kanal: der Bericht ist nur für die Maintainer sichtbar, bis
er veröffentlicht wird.

## Fristen

- Eingang bestätigt: 7 Tage.
- Erste Einschätzung (betroffen / nicht betroffen, Schwere): 30 Tage.
- Behebung oder benannter Zwischenstand: 90 Tage.

Wird eine Frist nicht gehalten, steht der Grund im Advisory — eine Frist ist eine
Zusage, keine Absicht.

## Was dazugehört

- Lücken im Code dieses Repositories.
- Lücken in Assets, die der CDN (`omegaflow/sources`) ausliefert.
- Fehlkonfiguration, die Daten preisgibt (Schlüssel, Token, personenbezogene
  Felder in einem Asset).

## Was nicht hierher gehört

Falsche Messwerte. Die sind ein fachlicher Fehler und gehören als Issue ins
offene Repository — mit Quelle, Datum und der Zahl, die nicht stimmt.
