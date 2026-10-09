#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kette {
    Direction,
    Body,
    Station,
}

impl Kette {
    pub fn word(self) -> &'static str {
        match self {
            Kette::Direction => "direction",
            Kette::Body => "body",
            Kette::Station => "station",
        }
    }

    pub fn from_word(word: &str) -> Option<Self> {
        match word {
            "direction" => Some(Kette::Direction),
            "body" => Some(Kette::Body),
            "station" => Some(Kette::Station),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zeuge {
    S2Direction,
    PointEvent,
    Gestalt,
    Substance,
    Presence,
}

impl Zeuge {
    pub fn word(self) -> &'static str {
        match self {
            Zeuge::S2Direction => "s2-direction",
            Zeuge::PointEvent => "point-event",
            Zeuge::Gestalt => "gestalt",
            Zeuge::Substance => "substance",
            Zeuge::Presence => "presence",
        }
    }

    pub fn from_word(word: &str) -> Option<Self> {
        match word {
            "s2-direction" => Some(Zeuge::S2Direction),
            "point-event" => Some(Zeuge::PointEvent),
            "gestalt" => Some(Zeuge::Gestalt),
            "substance" => Some(Zeuge::Substance),
            "presence" => Some(Zeuge::Presence),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeberinRole {
    Kette(Kette),
    Zeuge(Zeuge),
    KeinFaden,
}

impl WeberinRole {
    pub fn parse(token: &str) -> Option<Self> {
        if let Some(kind) = token.strip_prefix("kette:") {
            return Kette::from_word(kind).map(WeberinRole::Kette);
        }
        if let Some(kind) = token.strip_prefix("zeuge:") {
            return Zeuge::from_word(kind).map(WeberinRole::Zeuge);
        }
        if token == "kein-faden" {
            return Some(WeberinRole::KeinFaden);
        }
        None
    }

    pub fn word(self) -> String {
        match self {
            WeberinRole::Kette(kind) => format!("kette:{}", kind.word()),
            WeberinRole::Zeuge(kind) => format!("zeuge:{}", kind.word()),
            WeberinRole::KeinFaden => "kein-faden".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StationPoint {
    pub tdb: f64,
    pub value: f64,
}

impl StationPoint {
    pub fn is_measured(&self) -> bool {
        self.tdb.is_finite() && self.value.is_finite()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StationSeries {
    pub station: String,
    pub points: Vec<StationPoint>,
}

impl StationSeries {
    pub fn measured(&self) -> impl Iterator<Item = StationPoint> + '_ {
        self.points
            .iter()
            .copied()
            .filter(StationPoint::is_measured)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StationFit {
    pub n: usize,
    pub t_span_s: f64,
    pub intercept: f64,
    pub slope_per_s: f64,
    pub residual_rms: f64,
}

pub fn fit_station_series(series: &StationSeries) -> Option<StationFit> {
    let points: Vec<StationPoint> = series.measured().collect();
    if points.len() < 2 {
        return None;
    }
    let t0 = points.iter().map(|p| p.tdb).fold(f64::INFINITY, f64::min);
    let t_max = points
        .iter()
        .map(|p| p.tdb)
        .fold(f64::NEG_INFINITY, f64::max);
    let t_span_s = t_max - t0;
    if !(t_span_s > 0.0) {
        return None;
    }
    let n = points.len();
    let n_f = n as f64;
    let mean_t = points.iter().map(|p| p.tdb - t0).sum::<f64>() / n_f;
    let mean_v = points.iter().map(|p| p.value).sum::<f64>() / n_f;
    let sxx = points
        .iter()
        .map(|p| {
            let dt = (p.tdb - t0) - mean_t;
            dt * dt
        })
        .sum::<f64>();
    if !(sxx > 0.0) {
        return None;
    }
    let sxy = points
        .iter()
        .map(|p| ((p.tdb - t0) - mean_t) * (p.value - mean_v))
        .sum::<f64>();
    let slope_per_s = sxy / sxx;
    let intercept = mean_v - slope_per_s * mean_t;
    let ss_res = points
        .iter()
        .map(|p| {
            let r = p.value - (intercept + slope_per_s * (p.tdb - t0));
            r * r
        })
        .sum::<f64>();
    Some(StationFit {
        n,
        t_span_s,
        intercept,
        slope_per_s,
        residual_rms: (ss_res / n_f).sqrt(),
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeberinFit {
    pub role: WeberinRole,
    pub pos: Option<Kette>,
    pub series: Option<StationFit>,
    pub qty: Option<String>,
}

pub fn measure_weberin_fit(
    role: WeberinRole,
    qty: Option<&str>,
    series: &StationSeries,
) -> WeberinFit {
    let pos = match role {
        WeberinRole::Kette(kind) => Some(kind),
        WeberinRole::Zeuge(_) | WeberinRole::KeinFaden => None,
    };
    WeberinFit {
        role,
        pos,
        series: fit_station_series(series),
        qty: qty.map(str::to_string),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn series(points: &[(f64, f64)]) -> StationSeries {
        StationSeries {
            station: "ABK".to_string(),
            points: points
                .iter()
                .map(|(tdb, value)| StationPoint {
                    tdb: *tdb,
                    value: *value,
                })
                .collect(),
        }
    }

    #[test]
    fn a_role_round_trips_through_its_word() {
        for role in [
            WeberinRole::Kette(Kette::Direction),
            WeberinRole::Kette(Kette::Body),
            WeberinRole::Kette(Kette::Station),
            WeberinRole::Zeuge(Zeuge::S2Direction),
            WeberinRole::Zeuge(Zeuge::PointEvent),
            WeberinRole::Zeuge(Zeuge::Gestalt),
            WeberinRole::Zeuge(Zeuge::Substance),
            WeberinRole::Zeuge(Zeuge::Presence),
            WeberinRole::KeinFaden,
        ] {
            assert_eq!(WeberinRole::parse(&role.word()), Some(role));
        }
    }

    #[test]
    fn an_unknown_role_reads_none() {
        assert_eq!(WeberinRole::parse("kette:planet"), None);
        assert_eq!(WeberinRole::parse("zeuge:ghost"), None);
        assert_eq!(WeberinRole::parse("faden"), None);
        assert_eq!(WeberinRole::parse(""), None);
    }

    #[test]
    fn a_registry_reads_kein_faden_without_a_kette_axis() {
        let fit = measure_weberin_fit(WeberinRole::KeinFaden, None, &series(&[]));
        assert_eq!(fit.pos, None);
        assert_eq!(fit.series, None);
        assert_eq!(fit.qty, None);
    }

    #[test]
    fn a_station_series_measures_its_own_line_without_a_fabricated_absent() {
        let fit = measure_weberin_fit(
            WeberinRole::Kette(Kette::Station),
            Some("electric"),
            &series(&[(0.0, 1.0), (1.0, 3.0), (2.0, 5.0)]),
        );
        assert_eq!(fit.pos, Some(Kette::Station));
        assert_eq!(fit.qty.as_deref(), Some("electric"));
        let measured = fit.series.expect("the line series fits");
        assert_eq!(measured.n, 3);
        assert_eq!(measured.intercept, 1.0);
        assert_eq!(measured.slope_per_s, 2.0);
        assert_eq!(measured.residual_rms, 0.0);
        assert_eq!(measured.t_span_s, 2.0);
    }

    #[test]
    fn an_absent_series_is_none_never_a_default() {
        assert_eq!(fit_station_series(&series(&[])), None);
        assert_eq!(fit_station_series(&series(&[(0.0, 5.0)])), None);
    }

    #[test]
    fn a_non_finite_point_is_absent_not_a_zero() {
        let with_absent = series(&[(0.0, 1.0), (1.0, f64::NAN), (2.0, 5.0), (3.0, 7.0)]);
        let fit = fit_station_series(&with_absent).expect("the finite points fit");
        assert_eq!(fit.n, 3);
        assert_eq!(fit.slope_per_s, 2.0);
    }

    #[test]
    fn a_series_without_temporal_extent_is_none() {
        let flat = series(&[(4.0, 1.0), (4.0, 3.0), (4.0, 5.0)]);
        assert_eq!(fit_station_series(&flat), None);
    }

    #[test]
    fn every_measured_value_is_finite() {
        let fit = fit_station_series(&series(&[(0.0, 1.0), (1.0, 3.0), (2.0, 5.0)]))
            .expect("the fit is finite");
        assert!(fit.intercept.is_finite());
        assert!(fit.slope_per_s.is_finite());
        assert!(fit.residual_rms.is_finite());
        assert!(fit.t_span_s.is_finite());
    }
}
