use crate::mathematikerin::least_squares::solve_normal_equations_with_pivot_ratio;
use crate::mathematikerin::te::{
    TE_SURR_FLOOR, conditional_embedded_te_phase, topological_te_phase,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kanal {
    StaerkeKanal,
    ReferenzTreppe,
    NoccFeld,
    NachbarStation,
    AndereSonde,
    Zeit,
}

pub struct Witness<'a> {
    pub name: &'a str,
    pub series: &'a [f64],
    pub force_type: u8,
    pub kanal: Kanal,
    pub origin: &'a str,
}

pub struct WitnessStamp {
    pub name: String,
    pub force_type: u8,
    pub kanal: Kanal,
    pub origin: String,
}

pub struct Residual {
    pub series: Vec<f64>,
    pub witnesses_used: Vec<WitnessStamp>,
    pub n: usize,
    pub lag: usize,
    pub rank: usize,
    pub df: usize,
    pub pivot_ratio: f64,
}

pub enum ResidualOutcome {
    Measured(Residual),
    ZielUnterZeugen,
    RangDefizit,
    NFlloor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerdictWord {
    Coupled,
    Independent,
    Leakage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeAbsence {
    InsufficientSurrogates,
    InsufficientEffectiveSample,
    ZeroVariance,
    NotTestable,
}

pub struct IndependenceVerdict {
    pub a: Vec<(String, Result<VerdictWord, TeAbsence>)>,
    pub b_diagnostic: Result<VerdictWord, TeAbsence>,
    pub witnesses: Vec<WitnessStamp>,
    pub n: usize,
    pub lag: usize,
}

pub fn residual_against_witnesses(
    target: &[f64],
    witnesses: &[Witness<'_>],
    lags: &[usize],
) -> ResidualOutcome {
    let used_all: Vec<usize> = witnesses
        .iter()
        .enumerate()
        .filter(|(_, w)| w.series.len() == target.len())
        .map(|(i, _)| i)
        .collect();
    if used_all.is_empty() {
        return ResidualOutcome::NFlloor;
    }
    for &wi in &used_all {
        if target
            .iter()
            .zip(witnesses[wi].series.iter())
            .all(|(a, b)| (a - b).abs() < 1e-12)
        {
            return ResidualOutcome::ZielUnterZeugen;
        }
    }

    let mut best: Option<(f64, Residual)> = None;
    for &lag in lags {
        if lag >= target.len() {
            continue;
        }
        let n = target.len() - lag;
        let params = 1 + used_all.len();
        if params >= n {
            continue;
        }
        let mut ata = vec![vec![0.0f64; params]; params];
        let mut atx = vec![0.0f64; params];
        for (t, target_t) in target.iter().enumerate().skip(lag) {
            let mut row = Vec::with_capacity(params);
            row.push(1.0);
            for &wi in &used_all {
                row.push(witnesses[wi].series[t - lag]);
            }
            for a in 0..params {
                for b in 0..params {
                    ata[a][b] += row[a] * row[b];
                }
                atx[a] += row[a] * *target_t;
            }
        }
        let Some((coeffs, _, _, pivot_ratio)) =
            solve_normal_equations_with_pivot_ratio(&ata, &atx, &atx, &atx)
        else {
            continue;
        };
        let mut series = Vec::with_capacity(n);
        for (t, target_t) in target.iter().enumerate().skip(lag) {
            let mut prediction = coeffs[0];
            for (k, &wi) in used_all.iter().enumerate() {
                prediction += coeffs[k + 1] * witnesses[wi].series[t - lag];
            }
            series.push(*target_t - prediction);
        }
        let variance = residual_variance(&series);
        let residual = Residual {
            series,
            witnesses_used: used_all
                .iter()
                .map(|&i| WitnessStamp {
                    name: witnesses[i].name.to_string(),
                    force_type: witnesses[i].force_type,
                    kanal: witnesses[i].kanal,
                    origin: witnesses[i].origin.to_string(),
                })
                .collect(),
            n,
            lag,
            rank: params,
            df: n - params,
            pivot_ratio,
        };
        match &best {
            Some((v, _)) if *v <= variance => {}
            _ => best = Some((variance, residual)),
        }
    }

    match best {
        Some((_, residual)) => ResidualOutcome::Measured(residual),
        None => ResidualOutcome::RangDefizit,
    }
}

pub fn independence_verdict(
    target: &[f64],
    ext: &[Witness<'_>],
    test: &[Witness<'_>],
    seed: u64,
) -> Option<IndependenceVerdict> {
    let r = match residual_against_witnesses(target, ext, &[0]) {
        ResidualOutcome::Measured(r) => r,
        ResidualOutcome::ZielUnterZeugen => return None,
        ResidualOutcome::RangDefizit => return None,
        ResidualOutcome::NFlloor => return None,
    };
    let n = r.n;
    let lag = r.lag;
    let rank_e = r.rank;
    let res_f32: Vec<f32> = r.series.iter().map(|&v| v as f32).collect();

    let mut a = Vec::with_capacity(test.len());
    for w in test {
        let test_f32: Vec<f32> = w.series.iter().map(|&v| v as f32).collect();
        let word = match topological_te_phase(&res_f32, &test_f32, EMBEDDING_DIM, 3, seed) {
            None => Err(TeAbsence::NotTestable),
            Some(v) => {
                let participants: [&[f64]; 2] = [&r.series, w.series];
                let tau_max = v.tau_c.max(v.tau_y);
                match measurement_effective_sample(&participants)
                    .and_then(|n_eff| gate_effective_sample(n_eff, tau_max, 0).map(|()| n_eff))
                {
                    Err(absence) => Err(absence),
                    Ok(_) => word_from_te(v.te, v.threshold, v.surrogates_used),
                }
            }
        };
        a.push((w.name.to_string(), word));
    }

    let b_diagnostic = match (test.first(), ext.first()) {
        (Some(t), Some(e)) => {
            let t_f32: Vec<f32> = t.series.iter().map(|&v| v as f32).collect();
            let e_f32: Vec<f32> = e.series.iter().map(|&v| v as f32).collect();
            match conditional_embedded_te_phase(&res_f32, &t_f32, &e_f32, EMBEDDING_DIM, seed) {
                None => Err(TeAbsence::NotTestable),
                Some(v) => {
                    let participants: [&[f64]; 3] = [&r.series, t.series, e.series];
                    let tau_max = v.tau_x.max(v.tau_y).max(v.tau_z);
                    match measurement_effective_sample(&participants).and_then(|n_eff| {
                        gate_effective_sample(n_eff, tau_max, rank_e).map(|()| n_eff)
                    }) {
                        Err(absence) => Err(absence),
                        Ok(_) => leakage_word(v.te, v.threshold, v.surrogates_used),
                    }
                }
            }
        }
        _ => Err(TeAbsence::NotTestable),
    };

    Some(IndependenceVerdict {
        a,
        b_diagnostic,
        witnesses: r.witnesses_used,
        n,
        lag,
    })
}

const EMBEDDING_DIM: usize = 3;

fn autocorrelation_time(series: &[f64]) -> Result<f64, TeAbsence> {
    let n = series.len();
    if n < 3 {
        return Err(TeAbsence::NotTestable);
    }
    let mean = series.iter().sum::<f64>() / n as f64;
    let variance = series.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n as f64;
    if !variance.is_finite() || variance <= 0.0 {
        return Err(TeAbsence::ZeroVariance);
    }
    let mut accumulated = 0.0;
    for k in 1..n {
        let mut covariance = 0.0;
        for t in 0..n - k {
            covariance += (series[t] - mean) * (series[t + k] - mean);
        }
        covariance /= n as f64;
        let rho = covariance / variance;
        if !rho.is_finite() {
            return Err(TeAbsence::NotTestable);
        }
        if rho <= 0.0 {
            return Ok(1.0 + 2.0 * accumulated);
        }
        accumulated += rho;
    }
    Err(TeAbsence::InsufficientEffectiveSample)
}

fn effective_sample_size(series: &[f64]) -> Result<f64, TeAbsence> {
    let tau_int = autocorrelation_time(series)?;
    Ok(series.len() as f64 / tau_int)
}

fn measurement_effective_sample(participants: &[&[f64]]) -> Result<f64, TeAbsence> {
    let mut effective = f64::INFINITY;
    for series in participants {
        let n_eff = effective_sample_size(series)?;
        if n_eff < effective {
            effective = n_eff;
        }
    }
    if effective.is_finite() {
        Ok(effective)
    } else {
        Err(TeAbsence::NotTestable)
    }
}

fn gate_effective_sample(n_eff: f64, tau_max: usize, rank_e: usize) -> Result<(), TeAbsence> {
    let embedding_cost = (EMBEDDING_DIM * tau_max + 1) as f64;
    if n_eff < embedding_cost {
        return Err(TeAbsence::InsufficientEffectiveSample);
    }
    let df = n_eff - embedding_cost - rank_e as f64;
    if df < 1.0 {
        return Err(TeAbsence::InsufficientEffectiveSample);
    }
    Ok(())
}

fn word_from_te(te: f64, threshold: f64, surrogates_used: usize) -> Result<VerdictWord, TeAbsence> {
    if surrogates_used < TE_SURR_FLOOR {
        return Err(TeAbsence::InsufficientSurrogates);
    }
    if !te.is_finite() || !threshold.is_finite() {
        return Err(TeAbsence::NotTestable);
    }
    if te > threshold {
        Ok(VerdictWord::Coupled)
    } else {
        Ok(VerdictWord::Independent)
    }
}

fn leakage_word(te: f64, threshold: f64, surrogates_used: usize) -> Result<VerdictWord, TeAbsence> {
    if surrogates_used < TE_SURR_FLOOR {
        return Err(TeAbsence::InsufficientSurrogates);
    }
    if !te.is_finite() || !threshold.is_finite() {
        return Err(TeAbsence::NotTestable);
    }
    if te > threshold {
        Ok(VerdictWord::Coupled)
    } else {
        Ok(VerdictWord::Leakage)
    }
}

fn residual_variance(series: &[f64]) -> f64 {
    let n = series.len();
    if n == 0 {
        return f64::INFINITY;
    }
    let mean = series.iter().sum::<f64>() / n as f64;
    series.iter().map(|r| (r - mean) * (r - mean)).sum::<f64>() / n as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Lcg(u64);

    impl Lcg {
        fn next_unit(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
        }

        fn next_noise(&mut self) -> f64 {
            self.next_unit() * 2.0 - 1.0
        }
    }

    fn variance(series: &[f64]) -> f64 {
        let n = series.len();
        let mean = series.iter().sum::<f64>() / n as f64;
        series.iter().map(|v| (v - mean) * (v - mean)).sum::<f64>() / n as f64
    }

    fn zeuge<'a>(name: &'a str, series: &'a [f64]) -> Witness<'a> {
        Witness {
            name,
            series,
            force_type: 0,
            kanal: Kanal::StaerkeKanal,
            origin: "test",
        }
    }

    #[test]
    fn residual_removes_the_witness_prediction() {
        let n = 256;
        let mut w0 = Vec::with_capacity(n);
        let mut w1 = Vec::with_capacity(n);
        for t in 0..n {
            let x = t as f64;
            w0.push((x * 0.11).sin());
            w1.push((x * 0.07).cos() + 0.2 * (x * 0.31).sin());
        }
        let mut rng = Lcg(0x9E37_79B9_7F4A_7C15);
        let target: Vec<f64> = (0..n)
            .map(|t| 0.5 * w0[t] + 0.3 * w1[t] + 0.01 * rng.next_noise())
            .collect();

        let witnesses = [zeuge("w0", &w0), zeuge("w1", &w1)];
        let r = match residual_against_witnesses(&target, &witnesses, &[0]) {
            ResidualOutcome::Measured(r) => r,
            _ => panic!("the target against its witnesses reads outside Measured"),
        };
        assert_eq!(r.n, target.len());
        assert_eq!(r.series.len(), target.len());
        assert_eq!(r.witnesses_used.len(), 2);
        assert_eq!(r.witnesses_used[0].force_type, 0);
        assert_eq!(r.witnesses_used[0].kanal, Kanal::StaerkeKanal);
        assert_eq!(r.witnesses_used[0].origin, "test");
        assert_eq!(r.lag, 0);
        assert_eq!(r.rank, 3);
        assert_eq!(r.df, r.n - r.rank);
        assert!(r.pivot_ratio.is_finite() && r.pivot_ratio >= 1.0);
        assert!(variance(&r.series) < 0.1 * variance(&target));

        let empty: [Witness<'_>; 0] = [];
        assert!(matches!(
            residual_against_witnesses(&target, &empty, &[0]),
            ResidualOutcome::NFlloor
        ));

        let kept_nothing = [zeuge("empty", &[])];
        assert!(matches!(
            residual_against_witnesses(&target, &kept_nothing, &[0]),
            ResidualOutcome::NFlloor
        ));

        let short = &target[..4];
        assert!(matches!(
            residual_against_witnesses(short, &witnesses, &[10]),
            ResidualOutcome::NFlloor
        ));
    }

    #[test]
    fn collapse_of_witness_rank_is_named() {
        let target = [0.0, 1.0, 2.0, 3.0, 4.0];
        let w0 = [0.0, 0.5, 1.0, 1.5, 2.0];
        let w1 = [0.1, 0.6, 1.1, 1.6, 2.1];
        let w2 = [0.2, 0.7, 1.2, 1.7, 2.2];
        let w3 = [0.3, 0.8, 1.3, 1.8, 2.3];
        let w4 = [0.4, 0.9, 1.4, 1.9, 2.4];
        let w5 = [0.5, 1.0, 1.5, 2.0, 2.5];
        let witnesses = [
            zeuge("w0", &w0),
            zeuge("w1", &w1),
            zeuge("w2", &w2),
            zeuge("w3", &w3),
            zeuge("w4", &w4),
            zeuge("w5", &w5),
        ];
        match residual_against_witnesses(&target, &witnesses, &[0]) {
            ResidualOutcome::RangDefizit => {}
            _ => panic!("the rank collapse reads outside RangDefizit"),
        }
    }

    #[test]
    fn target_among_witnesses_is_named() {
        let target = [0.0, 1.0, 2.0, 3.0];
        let witnesses = [zeuge("self", &target)];
        match residual_against_witnesses(&target, &witnesses, &[0]) {
            ResidualOutcome::ZielUnterZeugen => {}
            _ => panic!("the target among witnesses reads outside ZielUnterZeugen"),
        }
    }

    #[test]
    fn no_witness_series_is_n_floor() {
        let target = [0.0, 1.0, 2.0, 3.0];
        let witnesses = [zeuge("empty", &[])];
        match residual_against_witnesses(&target, &witnesses, &[0]) {
            ResidualOutcome::NFlloor => {}
            _ => panic!("the empty-witness case reads outside NFlloor"),
        }
    }

    #[test]
    fn known_answer_a_recovers_held_out_coupling() {
        let n = 512;
        let mut rng = Lcg(0x1234_5678_9ABC_DEF0);
        let held: Vec<f64> = (0..n).map(|_| rng.next_noise()).collect();
        let mut target = vec![0.0f64; n];
        for t in 1..n {
            target[t] = 0.3 * target[t - 1] + 0.7 * held[t - 1] + 0.01 * rng.next_noise();
        }
        let ext0: Vec<f64> = (0..n).map(|_| rng.next_noise()).collect();
        let ext1: Vec<f64> = (0..n).map(|_| rng.next_noise()).collect();
        let noise: Vec<f64> = (0..n).map(|_| rng.next_noise()).collect();

        let ext = [zeuge("ext0", &ext0), zeuge("ext1", &ext1)];
        let test = [zeuge("held", &held), zeuge("noise", &noise)];
        let verdict = independence_verdict(&target, &ext, &test, 0x9E37_79B9_7F4A_7C15)
            .expect("the residual against the extraction witnesses reads Measured");
        assert_eq!(verdict.a.len(), 2);
        assert_eq!(verdict.a[0].0, "held");
        assert_eq!(verdict.a[1].0, "noise");
        assert_eq!(verdict.witnesses.len(), 2);
        assert_eq!(verdict.n, n);
        assert_eq!(verdict.lag, 0);
        let coupled = verdict.a[0]
            .1
            .expect("the held-out coupling carries a TE verdict");
        let independent = verdict.a[1]
            .1
            .expect("the disjoint noise test witness carries a TE verdict");
        assert_eq!(
            coupled,
            VerdictWord::Coupled,
            "the held-out coupling must read Coupled"
        );
        assert_eq!(
            independent,
            VerdictWord::Independent,
            "the disjoint noise must read Independent"
        );
    }

    #[test]
    fn known_answer_a_silent_on_pure_noise() {
        let n = 512;
        let mut rng = Lcg(0x0F1E_2D3C_4B5A_6978);
        let target: Vec<f64> = (0..n).map(|_| rng.next_noise()).collect();
        let ext0: Vec<f64> = (0..n).map(|_| rng.next_noise()).collect();
        let ext1: Vec<f64> = (0..n).map(|_| rng.next_noise()).collect();
        let test0: Vec<f64> = (0..n).map(|_| rng.next_noise()).collect();
        let test1: Vec<f64> = (0..n).map(|_| rng.next_noise()).collect();

        let ext = [zeuge("ext0", &ext0), zeuge("ext1", &ext1)];
        let test = [zeuge("t0", &test0), zeuge("t1", &test1)];
        let verdict = independence_verdict(&target, &ext, &test, 0x9E37_79B9_7F4A_7C15)
            .expect("the residual against the extraction witnesses reads Measured");
        for (name, word) in &verdict.a {
            assert!(
                !matches!(word, Ok(VerdictWord::Coupled)),
                "the disjoint test witness {name} carries no positive verdict, got {word:?}"
            );
        }
    }
}
