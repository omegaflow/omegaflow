use crate::archivar::kernel_id_of;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelTauSource {
    Derived,
    Chosen,
    Default,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManifestIntent {
    Science,
    Art,
    Test,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ManifestSelection {
    source_ref: String,
    kernel: Option<u8>,
    model_tau: Option<f64>,
    model_tau_src: Option<ModelTauSource>,
    model_absorption: Option<f64>,
    model_advection: Option<f64>,
    intent: ManifestIntent,
}

fn valid_source_ref(value: &str) -> bool {
    if value.is_empty() || value.chars().any(char::is_whitespace) {
        return false;
    }
    let (url, field) = match value.rsplit_once('#') {
        Some((url, field)) => (url, Some(field)),
        None => (value, None),
    };
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return false;
    }
    field.is_none_or(|field| !field.is_empty())
}

impl ManifestSelection {
    pub fn new(source_ref: impl Into<String>, intent: ManifestIntent) -> Result<Self, String> {
        let source_ref = source_ref.into();
        if !valid_source_ref(&source_ref) {
            return Err(format!("invalid source_ref: {source_ref}"));
        }
        Ok(Self {
            source_ref,
            kernel: None,
            model_tau: None,
            model_tau_src: None,
            model_absorption: None,
            model_advection: None,
            intent,
        })
    }

    pub fn source_ref(&self) -> &str {
        &self.source_ref
    }

    pub fn intent(&self) -> ManifestIntent {
        self.intent
    }

    pub fn with_kernel(mut self, name: &str) -> Result<Self, String> {
        self.kernel = Some(
            kernel_id_of(name).ok_or_else(|| format!("unknown kernel: {name}"))?,
        );
        Ok(self)
    }

    pub fn with_model_tau(mut self, tau: f64, source: ModelTauSource) -> Result<Self, String> {
        if !(tau.is_finite() && tau > 0.0) {
            return Err("model_tau must be positive and finite".into());
        }
        self.model_tau = Some(tau);
        self.model_tau_src = Some(source);
        Ok(self)
    }

    pub fn with_model_absorption(mut self, value: f64) -> Result<Self, String> {
        if !(value.is_finite() && value >= 0.0) {
            return Err("model_absorption must be non-negative and finite".into());
        }
        self.model_absorption = Some(value);
        Ok(self)
    }

    pub fn with_model_advection(mut self, value: f64) -> Result<Self, String> {
        if !(value.is_finite() && value >= 0.0) {
            return Err("model_advection must be non-negative and finite".into());
        }
        self.model_advection = Some(value);
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_ref_keeps_url_and_field_as_one_pointer() {
        let m = ManifestSelection::new("https://example.org/data.csv#flux", ManifestIntent::Science)
            .expect("select");
        assert_eq!(m.source_ref(), "https://example.org/data.csv#flux");
        assert_eq!(m.intent(), ManifestIntent::Science);
    }

    #[test]
    fn whole_source_pointer_has_no_field_fragment() {
        let m = ManifestSelection::new("https://example.org/data.csv", ManifestIntent::Art)
            .expect("select");
        assert_eq!(m.source_ref(), "https://example.org/data.csv");
    }

    #[test]
    fn invalid_source_ref_is_refused() {
        assert!(ManifestSelection::new("not-a-url", ManifestIntent::Test).is_err());
        assert!(ManifestSelection::new("https://example.org/data.csv#", ManifestIntent::Test).is_err());
        assert!(ManifestSelection::new("https://example.org/a b", ManifestIntent::Test).is_err());
    }

    #[test]
    fn model_values_are_optional_and_validated() {
        let m = ManifestSelection::new("https://example.org/data.csv#x", ManifestIntent::Art)
            .expect("select")
            .with_kernel("inverse-square")
            .expect("kernel")
            .with_model_tau(60.0, ModelTauSource::Chosen)
            .expect("tau")
            .with_model_absorption(0.0)
            .expect("absorption")
            .with_model_advection(400.0)
            .expect("advection");
        assert_eq!(m.kernel, Some(0));
        assert_eq!(m.model_tau, Some(60.0));
        assert_eq!(m.model_tau_src, Some(ModelTauSource::Chosen));
        assert_eq!(m.model_absorption, Some(0.0));
        assert_eq!(m.model_advection, Some(400.0));
    }

    #[test]
    fn model_tau_rejects_pending_and_nonpositive_values() {
        let base = ManifestSelection::new("https://example.org/data.csv#x", ManifestIntent::Test)
            .expect("select");
        assert!(base.clone().with_model_tau(0.0, ModelTauSource::Chosen).is_err());
        assert!(base.with_model_tau(f64::NAN, ModelTauSource::Chosen).is_err());
    }
}
