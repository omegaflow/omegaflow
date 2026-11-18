# validator.js v13.15.0 — Regex-Referenz (Referenznotiz, portiert in llm_gateway.rs)

Quelle: https://github.com/validatorjs/validator.js (Tag 13.15.0), src/lib/
Lizenz: MIT.

## isIBAN — src/lib/isIBAN.js
Struktur: Laendercode -> Regex, danach mod-97-Pruefung.
AD /^(AD[0-9]{2})\d{8}[A-Z0-9]{12}$/  AT /^(AT[0-9]{2})\d{16}$/  BE /^(BE[0-9]{2})\d{12}$/
CH /^(CH[0-9]{2})\d{5}[A-Z0-9]{12}$/  DE /^(DE[0-9]{2})\d{18}$/  FR /^(FR[0-9]{2})\d{10}[A-Z0-9]{11}\d{2}$/
GB /^(GB[0-9]{2})[A-Z]{4}\d{14}$/  IT /^(IT[0-9]{2})[A-Z]{1}\d{10}[A-Z0-9]{12}$/  NL /^(NL[0-9]{2})[A-Z]{4}\d{10}$/

## isBIC — src/lib/isBIC.js
/^[A-Za-z]{6}[A-Za-z0-9]{2}([A-Za-z0-9]{3})?$/  (Position 5-6 = ISO-3166-Laendercode)

## isCreditCard — src/lib/isCreditCard.js
Nach Entfernen von [- ]+: Provider-Regex, danach Luhn.
amex /^3[47][0-9]{13}$/
dinersclub /^3(?:0[0-5]|[68][0-9])[0-9]{11}$/
discover /^6(?:011|5[0-9][0-9])[0-9]{12,15}$/
jcb /^(?:2131|1800|35\d{3})\d{11}$/
mastercard /^5[1-5][0-9]{2}|(222[1-9]|22[3-9][0-9]|2[3-6][0-9]{2}|27[01][0-9]|2720)[0-9]{12}$/
unionpay /^(6[27][0-9]{14}|^(81[0-9]{14,17}))$/
visa /^(?:4[0-9]{12})(?:[0-9]{3,6})?$/

## isVAT — src/lib/isVAT.js
AT /^(AT)?U\d{8}$/  BE /^(BE)?\d{10}$/  DE /^(DE)?\d{9}$/  FR /^(FR)?\w{2}\d{9}$/
IT /^(IT)?\d{11}$/  NL /^(NL)?\d{9}B\d{2}$/  ES /^(ES)?\w\d{7}[A-Z]$/
PL /^(PL)?(\d{10}|(\d{3}-\d{3}-\d{2}-\d{2})|(\d{3}-\d{2}-\d{2}-\d{3}))$/
CH /^(CHE[- ]?)?(\d{9}|(\d{3}\.\d{3}\.\d{3})|(\d{3} \d{3} \d{3})) ?(TVA|MWST|IVA)?$/

## isPostalCode — src/lib/isPostalCode.js
threeDigit /^\d{3}$/  fourDigit /^\d{4}$/  fiveDigit /^\d{5}$/  sixDigit /^\d{6}$/
DE fiveDigit  AT fourDigit  CH fourDigit  FR /^\d{2}\s?\d{3}$/  CZ /^\d{3}\s?\d{2}$/
PL /^\d{2}\-\d{3}$/  JP /^\d{3}\-\d{4}$/  PT /^\d{4}\-\d{3}?$/  US /^\d{5}(-\d{4})?$/
BR /^\d{5}-?\d{3}$/  SE /^[1-9]\d{2}\s?\d{2}$/  SK /^\d{3}\s?\d{2}$/  GR /^\d{3}\s?\d{2}$/
NL /^[1-9]\d{3}\s?(?!sa|sd|ss)[a-z]{2}$/i
CA /^[ABCEGHJKLMNPRSTVXY]\d[ABCEGHJ-NPRSTV-Z][\s\-]?\d[ABCEGHJ-NPRSTV-Z]\d$/i
GB /^(gir\s?0aa|[a-z]{1,2}\d[\da-z]?\s?(\d[a-z]{2})?)$/i

## isMobilePhone — src/lib/isMobilePhone.js
de-DE /^((\+49|0)1)(5[0-25-9]\d|6([23]|0\d?)|7([0-57-9]|6\d))\d{7,9}$/
de-AT /^(\+43|0)\d{1,4}\d{3,12}$/
de-CH /^(\+41|0)([1-9])\d{1,9}$/

## isPassportNumber — src/lib/isPassportNumber.js
AT /^[A-Z]\d{7}$/  DE /^[CFGHJKLMNPRTVWXYZ0-9]{9}$/

## isTaxID — src/lib/isTaxID.js
de-AT /^\d{9}$/  de-DE /^[1-9]\d{10}$/
