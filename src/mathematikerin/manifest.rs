use crate::archivar::kernel_id_of;
use std::collections::BTreeMap;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelGap {
    Kernel,
    Tau,
    Absorption,
    Advection,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModelRefusal {
    Unselected { source_ref: String },
    ForeignSource { selected: String, asked: String },
    Gaps(Vec<ModelGap>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ManifestModel {
    kernel: u8,
    tau: f64,
    tau_src: ModelTauSource,
    absorption: f64,
    advection: f64,
}

impl ManifestModel {
    pub fn kernel(&self) -> u8 {
        self.kernel
    }

    pub fn tau(&self) -> f64 {
        self.tau
    }

    pub fn tau_src(&self) -> ModelTauSource {
        self.tau_src
    }

    pub fn absorption(&self) -> f64 {
        self.absorption
    }

    pub fn advection(&self) -> f64 {
        self.advection
    }
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

    pub fn kernel(&self) -> Option<u8> {
        self.kernel
    }

    pub fn model_tau(&self) -> Option<f64> {
        self.model_tau
    }

    pub fn model_tau_src(&self) -> Option<ModelTauSource> {
        self.model_tau_src
    }

    pub fn model_absorption(&self) -> Option<f64> {
        self.model_absorption
    }

    pub fn model_advection(&self) -> Option<f64> {
        self.model_advection
    }

    pub fn covers(&self, source_ref: &str) -> bool {
        match self.source_ref.split_once('#') {
            Some(_) => self.source_ref == source_ref,
            None => match source_ref.split_once('#') {
                Some((url, _)) => url == self.source_ref,
                None => source_ref == self.source_ref,
            },
        }
    }

    pub fn resolve(&self) -> Result<ManifestModel, ModelRefusal> {
        match (
            self.kernel,
            self.model_tau,
            self.model_tau_src,
            self.model_absorption,
            self.model_advection,
        ) {
            (Some(kernel), Some(tau), Some(tau_src), Some(absorption), Some(advection)) => {
                Ok(ManifestModel {
                    kernel,
                    tau,
                    tau_src,
                    absorption,
                    advection,
                })
            }
            _ => {
                let mut gaps = Vec::new();
                if self.kernel.is_none() {
                    gaps.push(ModelGap::Kernel);
                }
                if self.model_tau.is_none() || self.model_tau_src.is_none() {
                    gaps.push(ModelGap::Tau);
                }
                if self.model_absorption.is_none() {
                    gaps.push(ModelGap::Absorption);
                }
                if self.model_advection.is_none() {
                    gaps.push(ModelGap::Advection);
                }
                Err(ModelRefusal::Gaps(gaps))
            }
        }
    }

    pub fn resolve_for(&self, source_ref: &str) -> Result<ManifestModel, ModelRefusal> {
        if !self.covers(source_ref) {
            return Err(ModelRefusal::ForeignSource {
                selected: self.source_ref.clone(),
                asked: source_ref.to_string(),
            });
        }
        self.resolve()
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Manifest {
    selections: BTreeMap<String, ManifestSelection>,
}

impl Manifest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, selection: ManifestSelection) -> Result<(), String> {
        let source_ref = selection.source_ref().to_string();
        if self.selections.contains_key(&source_ref) {
            return Err(format!("duplicate manifest selection: {source_ref}"));
        }
        self.selections.insert(source_ref, selection);
        Ok(())
    }

    pub fn resolve(&self, source_ref: &str) -> Result<ManifestModel, ModelRefusal> {
        if let Some(selection) = self.selections.get(source_ref) {
            return selection.resolve();
        }
        if let Some((url, _)) = source_ref.rsplit_once('#')
            && let Some(selection) = self.selections.get(url)
        {
            return selection.resolve();
        }
        Err(ModelRefusal::Unselected {
            source_ref: source_ref.to_string(),
        })
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

    #[test]
    fn an_unselected_model_reports_gaps_not_zeroes() {
        let m = ManifestSelection::new("https://example.org/data.csv#x", ManifestIntent::Science)
            .expect("select");
        assert_eq!(m.kernel(), None);
        assert_eq!(m.model_tau(), None);
        assert_eq!(m.model_absorption(), None);
        assert_eq!(m.model_advection(), None);
        assert_eq!(
            m.resolve(),
            Err(ModelRefusal::Gaps(vec![
                ModelGap::Kernel,
                ModelGap::Tau,
                ModelGap::Absorption,
                ModelGap::Advection,
            ]))
        );
    }

    #[test]
    fn a_partial_model_names_the_missing_choice() {
        let m = ManifestSelection::new("https://example.org/data.csv#x", ManifestIntent::Science)
            .expect("select")
            .with_kernel("inverse-square")
            .expect("kernel")
            .with_model_tau(60.0, ModelTauSource::Default)
            .expect("tau");
        assert_eq!(
            m.resolve(),
            Err(ModelRefusal::Gaps(vec![
                ModelGap::Absorption,
                ModelGap::Advection,
            ]))
        );
    }

    #[test]
    fn a_complete_model_carries_the_selected_values() {
        let m = ManifestSelection::new("https://example.org/data.csv#x", ManifestIntent::Science)
            .expect("select")
            .with_kernel("inverse-square")
            .expect("kernel")
            .with_model_tau(60.0, ModelTauSource::Chosen)
            .expect("tau")
            .with_model_absorption(0.25)
            .expect("absorption")
            .with_model_advection(400.0)
            .expect("advection");
        let model = m.resolve().expect("complete selection resolves");
        assert_eq!(model.kernel(), 0);
        assert_eq!(model.tau(), 60.0);
        assert_eq!(model.tau_src(), ModelTauSource::Chosen);
        assert_eq!(model.absorption(), 0.25);
        assert_eq!(model.advection(), 400.0);
    }

    #[test]
    fn a_supplied_zero_is_a_choice_while_absence_stays_a_gap() {
        let m = ManifestSelection::new("https://example.org/data.csv#x", ManifestIntent::Art)
            .expect("select")
            .with_kernel("inverse-square")
            .expect("kernel")
            .with_model_tau(1.0, ModelTauSource::Chosen)
            .expect("tau")
            .with_model_absorption(0.0)
            .expect("absorption")
            .with_model_advection(0.0)
            .expect("advection");
        let model = m.resolve().expect("a chosen zero resolves");
        assert_eq!(model.absorption(), 0.0);
        assert_eq!(model.advection(), 0.0);
    }

    #[test]
    fn a_manifest_resolves_a_field_through_a_whole_source_selection() {
        let selection = ManifestSelection::new(
            "https://example.org/data.csv",
            ManifestIntent::Science,
        )
        .expect("select")
        .with_kernel("inverse-square")
        .expect("kernel")
        .with_model_tau(60.0, ModelTauSource::Chosen)
        .expect("tau")
        .with_model_absorption(0.0)
        .expect("absorption")
        .with_model_advection(0.0)
        .expect("advection");
        let mut manifest = Manifest::new();
        manifest.add(selection).expect("add");
        let model = manifest
            .resolve("https://example.org/data.csv#flux")
            .expect("whole-source selection covers the field");
        assert_eq!(model.kernel(), 0);
        assert_eq!(model.tau(), 60.0);
    }

    #[test]
    fn an_unselected_field_is_not_a_zero_model() {
        let manifest = Manifest::new();
        assert!(matches!(
            manifest.resolve("https://example.org/data.csv#flux"),
            Err(ModelRefusal::Unselected { .. })
        ));
    }

    #[test]
    fn a_duplicate_manifest_selection_is_refused() {
        let selection = ManifestSelection::new(
            "https://example.org/data.csv#flux",
            ManifestIntent::Science,
        )
        .expect("select");
        let mut manifest = Manifest::new();
        manifest.add(selection.clone()).expect("first");
        assert!(manifest.add(selection).is_err());
    }

    #[test]
    fn a_selection_covers_only_its_own_source_ref() {
        let field = ManifestSelection::new("https://example.org/data.csv#x", ManifestIntent::Science)
            .expect("select");
        assert!(field.covers("https://example.org/data.csv#x"));
        assert!(!field.covers("https://example.org/data.csv#y"));
        assert!(matches!(
            field.resolve_for("https://example.org/data.csv#y"),
            Err(ModelRefusal::ForeignSource { .. })
        ));

        let whole = ManifestSelection::new("https://example.org/data.csv", ManifestIntent::Science)
            .expect("select");
        assert!(whole.covers("https://example.org/data.csv#x"));
        assert!(whole.covers("https://example.org/data.csv"));
        assert!(!whole.covers("https://example.org/other.csv#x"));
    }
}
