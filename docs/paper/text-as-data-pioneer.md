<!--
  title: The text as data — the Pioneer review's numbers against the Doppler series
  class: paper
  date: 2026-09-03
  version: 3
  sha256: cbcb7308ff2b0a9dc78414052e4c2924e25ecf4dc6054d9ec76f2762443ff4e0
  status: live
  see-also: docs/reference/pioneer-anomaly/pioneer-anomaly-lrr-2010-4.txt, docs/TODO.md (Pioneer-Front), tools/measure/src/bin/pioneer_text_correlation.rs
-->

# The text as data — the Pioneer review's numbers against the Doppler series

## Abstract

We hold the 8 767 numbers of the Pioneer review (Turyshev & Toth 2010) as a data sequence against the Doppler series of both probes. The reading series of the text carries no correlation that exceeds its own permutation null — the arrangement of the numbers contains nothing about the measurement series. The only digit-exact relation is the mantissa: the band numbers (211.052 MHz, 2 150 MHz, 2 091 MHz, 2.18 GHz) hit the carriers of the data digit-exactly (ΔMant ≈ 10⁻⁹ to 10⁻⁶). This proximity is expected by construction: the review cites the system's nominal carrier frequencies, which the DSN data carry as well — both describe the same specification on different scales (GHz against Hz). The mantissa measures that the review cites the DSN specification correctly; it does not carry an independent relation.


## 1. The measurement series

The probe (`tools/measure/src/bin/pioneer_text_correlation.rs`) holds the article
text as data against the measurement data:
number tokens with ×10⁻ⁿ merging from 592 866 characters → 8 767 tokens, 1 576
distinct values. Provenance of the count: **8 767** is a token count of the
review text (592 866 characters, ×10⁻ⁿ merging), not a raw file size; the
figure „999 000" is a memory number and stands nowhere in the corpus.
Measurement series: the cleaned NAVIO 60-s Doppler
P10 (908 028 records, Doppler −4.2 MHz..+541 MHz, carrier 2.018..2.316 GHz) and
P11 (967 043 records, −3.1 MHz..+541 MHz, 2.002..2.349 GHz).

**The raw relation is scale-blind.** Exact value equality: 6 (P10) resp.
11 (P11) text numbers hit 95 resp. 203 data values — throughout grid values
(3 000 Hz ×145, 2 000 Hz, 6 000 Hz): the quantization grids of the data, not
a relation of the text. At a tolerance of 10⁻⁶ relative the carrier holds **0
hits** for both probes — the text writes "2.292" (GHz), the data carry
2 292 000 000 (Hz). The scale difference is the truth of the raw relation.

**The mantissa carries.** Across decades (fractional logarithm; reported
per text number is the smallest distance): the closest hits of both probes are
digit-exact. P10: 211.052 ↔ 2 110 520 000 Hz (ΔMant 2.6×10⁻⁹), 2 150 MHz
(1.4×10⁻⁶), 2 024 MHz (4.6×10⁻⁶). P11: 211.052 ↔ 2 110 520 000 Hz
(4.6×10⁻⁹), 2 091 MHz (4.8×10⁻⁷), 2.18 GHz (7.8×10⁻⁶). The article carries the
bands; the data carry the same bands — the same digits, a different
power of ten. The proximity is expected by construction: these are the
nominal carrier frequencies of the tracking system (uplink 2 110 MHz,
downlink 2 292 MHz, transponder ratio 240/221; Anderson et al. 2002), which
a review of the anomaly cites and the DSN data both carry. The mantissa
therefore confirms that the review quotes the DSN specification correctly;
it is not an independent coincidence that requires an explanation.

**The correlation does not carry.** Cross-correlation of the text reading series against
both measurement series (lag 0..1000), verdict from the permutation null (24 shuffles,
max |r| per shuffle): P11×Doppler reaches |r| = 0.7103 — the null reaches
0.7104. The real sequence does not exceed its own null (0 honored). The
0.71 value is a tail artifact of the values (outliers, not order):
the null carries it just the same. P10×Doppler: p_emp = 0.08. The carrier series
lie in the null (p_emp = 0.96 resp. 0.12).

**The measurement series against each other.** Daily medians over the 1 851 shared days:
Pearson r = 0.953 — the shared Earth-orbit structure dominates both series;
the null is nominal (both series autocorrelated), therefore it stands as a
relation of the series, not as a significance finding.

## 2. The decisive measure

Two measures carry the verdict: the **permutation null** (answers whether the
arrangement of the text numbers carries more than their own rearrangement) and the
**mantissa distance** (answers whether a text number and a data value
carry the same digits, independent of the power of ten). The raw relation
is scale-blind and therefore alone no measurement; the mantissa is the
check that the article's GHz prose and the DSN Hz data name the same
nominal carriers — a consistency test of the citation, not a discovery of a
relation.

## 3. What is excluded (measured, no fabrication)

Quantization grids as a relation (3 000/2 000/6 000 Hz exactly — the
data grids, not the text); the 0.1 % mantissa window as a relation counter
(saturates at the data coverage: 1 574/1 576 text numbers hit — therefore only
the smallest distance counts); the Fisher-z null on autocorrelated series
(only nominal — therefore the permutation null as the verdict).

## 4. What stays open

The probe compares values and orders, no contents: whether the
physical constants of the article (a_P = 8.74×10⁻¹⁰ m/s², the drift
6×10⁻⁹ Hz/s) carry a correspondence in the model-subtracted residuum is
another machine — the negative fuzzy index on the residuum. The 0.71 tie
of the P11×Doppler null is named; a tail-trimming additional run would be the
next step, not a finding.

## References

1. Turyshev S. G., Toth V. T., 2010, Living Rev. Relativity 13, 4 — text:
   docs/reference/pioneer-anomaly/pioneer-anomaly-lrr-2010-4.txt.
2. The probe: tools/measure/src/bin/pioneer_text_correlation.rs.
3. Data: data/pioneer10_doppler_clean.bin, data/pioneer11_doppler_clean.bin
   (NAVIO 60-s, cleaned — see TODO.md, Pioneer-Front).
4. Anderson J. D., Laing P. A., Lau E. L., Liu A. S., Nieto M. M.,
   Turyshev S. G., 2002, Study of the anomalous acceleration of Pioneer
   10 and 11, Phys. Rev. D 65, 082004.
