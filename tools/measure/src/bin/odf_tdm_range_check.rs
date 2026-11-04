use omegaflow::archivar::{odf, parse_iso_unix, tdm};

const UNIX_1950_OFFSET: f64 = 631_152_000.0;
const TIME_TOLERANCE_S: f64 = 120.0;

fn time_1950(label: &str) -> Option<f64> {
    Some(parse_iso_unix(label)? + UNIX_1950_OFFSET)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let argument = |name: &str| {
        args.iter()
            .position(|arg| arg == name)
            .and_then(|index| args.get(index + 1))
            .cloned()
    };
    let Some(odf_path) = argument("--odf") else {
        eprintln!("odf-tdm: --odf <file> absent — no ODF to read");
        return;
    };
    let Some(tdm_path) = argument("--tdm") else {
        eprintln!("odf-tdm: --tdm <file> absent — no TDM to compare");
        return;
    };
    let Ok(odf_bytes) = std::fs::read(&odf_path) else {
        eprintln!("odf-tdm: read void ({odf_path})");
        return;
    };
    let Some(records) = odf::parse_odf(&odf_bytes) else {
        eprintln!("odf-tdm: parse void — {odf_path} is not a TRK-2-18 ODF");
        return;
    };
    let Ok(tdm_text) = std::fs::read_to_string(&tdm_path) else {
        eprintln!("odf-tdm: read void ({tdm_path})");
        return;
    };
    let Some(parsed) = tdm::parse_tdm(&tdm_text) else {
        eprintln!("odf-tdm: parse void — {tdm_path} is not a CCSDS TDM");
        return;
    };

    let ranges: Vec<(f64, f64)> = records
        .iter()
        .filter(|record| record.data_type == 37)
        .map(|record| (record.t_since_1950, record.observable_hz))
        .collect();
    let mut tdm_ranges: Vec<(f64, f64)> = tdm::observations(&parsed)
        .filter(|observation| observation.name == "RANGE")
        .filter_map(|observation| Some((time_1950(&observation.time)?, observation.value)))
        .collect();
    tdm_ranges.sort_by(|a, b| a.0.total_cmp(&b.0));

    let mut matched = 0usize;
    let mut max_value_delta = 0.0f64;
    let mut max_time_delta = 0.0f64;
    let mut sum_value_delta = 0.0f64;
    for (time, value) in &ranges {
        let nearest = tdm_ranges
            .iter()
            .min_by(|a, b| (a.0 - time).abs().total_cmp(&(b.0 - time).abs()));
        let Some((tdm_time, tdm_value)) = nearest else {
            continue;
        };
        let time_delta = (tdm_time - time).abs();
        if time_delta > TIME_TOLERANCE_S {
            continue;
        }
        matched += 1;
        let value_delta = value - tdm_value;
        max_value_delta = max_value_delta.max(value_delta.abs());
        max_time_delta = max_time_delta.max(time_delta);
        sum_value_delta += value_delta;
    }
    println!(
        "odf-tdm: {} ODF range record(s), {} TDM range(s), {} matched within {TIME_TOLERANCE_S:.0} s",
        ranges.len(),
        tdm_ranges.len(),
        matched
    );
    let mean_text = if matched > 0 {
        format!("{:+.6}", sum_value_delta / matched as f64)
    } else {
        "pending".to_string()
    };
    println!(
        "odf-tdm: max |value delta| {max_value_delta:.6} RU, mean {mean_text} RU, max time delta {max_time_delta:.3} s"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_unix_epoch_is_the_1950_offset() {
        let epoch = time_1950("1970-01-01T00:00:00").expect("a time");
        assert!((epoch - UNIX_1950_OFFSET).abs() < 1.0e-9);
    }
}
