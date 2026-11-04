use omegaflow::archivar::odf::{self, RampRecord};
use omegaflow::archivar::tdm;

fn argument(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|arg| arg == name)
        .and_then(|index| args.get(index + 1))
        .cloned()
}

fn end_frequency(ramp: &RampRecord) -> f64 {
    ramp.start_freq_hz + ramp.rate_hz_per_s * (ramp.end_s - ramp.start_s)
}

fn band(frequency_hz: f64) -> &'static str {
    if (1.0e9..3.0e9).contains(&frequency_hz) {
        "S"
    } else if (6.0e9..9.0e9).contains(&frequency_hz) {
        "X"
    } else {
        "other"
    }
}

fn band_span(values: &[f64], label: &str) -> Option<(f64, f64)> {
    let mut banded: Vec<f64> = values
        .iter()
        .copied()
        .filter(|value| band(*value) == label)
        .collect();
    banded.sort_by(|a, b| a.total_cmp(b));
    Some((*banded.first()?, *banded.last()?))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(odf_path) = argument(&args, "--odf") else {
        eprintln!("odf-ramp: --odf <file> absent — no ODF to read");
        return;
    };
    let Some(tdm_path) = argument(&args, "--tdm") else {
        eprintln!("odf-ramp: --tdm <file> absent — no TDM to compare");
        return;
    };
    let Ok(odf_bytes) = std::fs::read(&odf_path) else {
        eprintln!("odf-ramp: read void ({odf_path})");
        return;
    };
    let Some(ramps) = odf::parse_odf_ramps(&odf_bytes) else {
        eprintln!("odf-ramp: parse void — {odf_path} is not a TRK-2-18 ODF");
        return;
    };
    let Ok(tdm_text) = std::fs::read_to_string(&tdm_path) else {
        eprintln!("odf-ramp: read void ({tdm_path})");
        return;
    };
    let Some(parsed) = tdm::parse_tdm(&tdm_text) else {
        eprintln!("odf-ramp: parse void — {tdm_path} is not a CCSDS TDM");
        return;
    };
    let mut frequencies: Vec<f64> = tdm::observations(&parsed)
        .filter(|observation| observation.name == "TRANSMIT_FREQ")
        .map(|observation| observation.value)
        .filter(|value| value.is_finite())
        .collect();
    frequencies.sort_by(|a, b| a.total_cmp(b));

    println!("odf-ramp: {} ramp record(s) from {odf_path}", ramps.len());
    for ramp in ramps.iter().take(5) {
        println!(
            "odf-ramp: start {:.3} s · start {:.3} Hz · rate {:+.9} Hz/s · end {:.3} s · end {:.3} Hz",
            ramp.start_s,
            ramp.start_freq_hz,
            ramp.rate_hz_per_s,
            ramp.end_s,
            end_frequency(ramp)
        );
    }
    let ramp_starts: Vec<f64> = ramps.iter().map(|ramp| ramp.start_freq_hz).collect();
    let ramp_ends: Vec<f64> = ramps.iter().map(end_frequency).collect();
    for label in ["S", "X"] {
        if let (Some((start_low, start_high)), Some((end_low, end_high))) =
            (band_span(&ramp_starts, label), band_span(&ramp_ends, label))
        {
            println!(
                "odf-ramp: {label} ramps — start {start_low:.3} .. {start_high:.3} Hz · end {end_low:.3} .. {end_high:.3} Hz"
            );
        }
    }
    match (frequencies.first(), frequencies.last()) {
        (Some(first), Some(last)) => println!(
            "odf-ramp: tdm transmit frequency — {} record(s), first {first:.3} Hz, last {last:.3} Hz",
            frequencies.len()
        ),
        _ => println!("odf-ramp: tdm transmit frequency — no record"),
    }
    for label in ["S", "X"] {
        if let Some((low, high)) = band_span(&frequencies, label) {
            println!("odf-ramp: tdm {label} — {low:.3} .. {high:.3} Hz");
        }
    }
    for label in ["S", "X"] {
        if let (Some((ramp_low, ramp_high)), Some((tdm_low, tdm_high))) = (
            band_span(&ramp_starts, label),
            band_span(&frequencies, label),
        ) {
            println!(
                "odf-ramp: {label} low delta {:+.3} Hz · high delta {:+.3} Hz",
                ramp_low - tdm_low,
                ramp_high - tdm_high
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_end_frequency_follows_the_rate() {
        let ramp = RampRecord {
            start_s: 0.0,
            start_freq_hz: 100.0,
            rate_hz_per_s: 2.0,
            end_s: 10.0,
        };
        assert!((end_frequency(&ramp) - 120.0).abs() < 1.0e-12);
    }

    #[test]
    fn the_band_split_is_s_and_x() {
        assert_eq!(band(2.1e9), "S");
        assert_eq!(band(7.1e9), "X");
        assert_eq!(band(5.0e9), "other");
    }
}
