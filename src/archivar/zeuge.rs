#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ZeugeArt {
    S2Richtung,
    Gestalt,
    Presence,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FeldIdentitaet {
    Oszillator,
    Zeuge(ZeugeArt),
    Footprint,
    Pending,
}

pub fn magic_identity(magic: [u8; 4]) -> Option<FeldIdentitaet> {
    match &magic {
        b"AMN1" | b"PAO1" | b"SKY1" | b"S2E1" => Some(FeldIdentitaet::Zeuge(ZeugeArt::S2Richtung)),
        b"GBCO" => Some(FeldIdentitaet::Zeuge(ZeugeArt::Gestalt)),
        b"FP01" => Some(FeldIdentitaet::Footprint),
        b"NRS1" => Some(FeldIdentitaet::Pending),
        b"BGR1" | b"ARG1" | b"FDS1" | b"GIC1" | b"IGT1" | b"SDN1" => {
            Some(FeldIdentitaet::Oszillator)
        }
        _ => None,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ZeugeVerdict {
    Holds(ZeugeArt),
    Radiator,
    BareCoordinate,
    Pending,
}

pub fn zeugen_gate(
    magic: Option<[u8; 4]>,
    declared_art: Option<ZeugeArt>,
    has_scalar: bool,
) -> ZeugeVerdict {
    let Some(m) = magic else {
        return ZeugeVerdict::Pending;
    };
    match magic_identity(m) {
        Some(FeldIdentitaet::Oszillator) => ZeugeVerdict::Radiator,
        Some(FeldIdentitaet::Footprint) | Some(FeldIdentitaet::Pending) => ZeugeVerdict::Pending,
        Some(FeldIdentitaet::Zeuge(art)) => {
            if !has_scalar {
                return ZeugeVerdict::BareCoordinate;
            }
            if let Some(declared) = declared_art {
                if declared != art {
                    return ZeugeVerdict::Pending;
                }
            }
            ZeugeVerdict::Holds(art)
        }
        None => ZeugeVerdict::Pending,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SerienVerdict {
    Hold,
    Reject,
    Pending,
}

pub fn serien_gate(magic: Option<[u8; 4]>, samples: &[(f64, f64, f64)]) -> SerienVerdict {
    let Some(m) = magic else {
        return SerienVerdict::Pending;
    };
    match magic_identity(m) {
        None => SerienVerdict::Pending,
        Some(FeldIdentitaet::Zeuge(_)) | Some(FeldIdentitaet::Footprint) => SerienVerdict::Reject,
        Some(FeldIdentitaet::Oszillator) | Some(FeldIdentitaet::Pending) => {
            for &(freq, bin_width, val) in samples {
                if !freq.is_finite() || !bin_width.is_finite() || !val.is_finite() {
                    return SerienVerdict::Reject;
                }
                if freq < 0.0 || bin_width < 0.0 {
                    return SerienVerdict::Reject;
                }
            }
            SerienVerdict::Hold
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn witness_magics_hold_s2_direction() {
        for m in [*b"AMN1", *b"PAO1", *b"SKY1", *b"S2E1"] {
            assert_eq!(
                magic_identity(m),
                Some(FeldIdentitaet::Zeuge(ZeugeArt::S2Richtung))
            );
        }
    }

    #[test]
    fn gebco_is_gestalt() {
        assert_eq!(
            magic_identity(*b"GBCO"),
            Some(FeldIdentitaet::Zeuge(ZeugeArt::Gestalt))
        );
    }

    #[test]
    fn nrs1_stays_pending() {
        assert_eq!(magic_identity(*b"NRS1"), Some(FeldIdentitaet::Pending));
    }

    #[test]
    fn fp01_is_a_footprint_sibling_not_a_witness() {
        assert_eq!(magic_identity(*b"FP01"), Some(FeldIdentitaet::Footprint));
        assert_eq!(
            zeugen_gate(Some(*b"FP01"), Some(ZeugeArt::S2Richtung), true),
            ZeugeVerdict::Pending
        );
        assert_eq!(
            serien_gate(Some(*b"FP01"), &[(10.0, 1.0, 3.0)]),
            SerienVerdict::Reject
        );
    }

    #[test]
    fn oscillator_magics_are_oscillators() {
        for m in [*b"BGR1", *b"ARG1", *b"FDS1", *b"GIC1", *b"IGT1", *b"SDN1"] {
            assert_eq!(magic_identity(m), Some(FeldIdentitaet::Oszillator));
        }
    }

    #[test]
    fn unknown_magic_is_none() {
        assert_eq!(magic_identity(*b"XXXX"), None);
    }

    #[test]
    fn gate_holds_a_scalar_witness() {
        assert_eq!(
            zeugen_gate(Some(*b"AMN1"), Some(ZeugeArt::S2Richtung), true),
            ZeugeVerdict::Holds(ZeugeArt::S2Richtung)
        );
    }

    #[test]
    fn gate_refuses_a_radiator() {
        assert_eq!(
            zeugen_gate(Some(*b"BGR1"), Some(ZeugeArt::S2Richtung), true),
            ZeugeVerdict::Radiator
        );
    }

    #[test]
    fn gate_refuses_a_bare_coordinate() {
        assert_eq!(
            zeugen_gate(Some(*b"AMN1"), Some(ZeugeArt::S2Richtung), false),
            ZeugeVerdict::BareCoordinate
        );
    }

    #[test]
    fn gate_holds_pending_for_the_spectral_rift() {
        assert_eq!(
            zeugen_gate(Some(*b"NRS1"), Some(ZeugeArt::S2Richtung), true),
            ZeugeVerdict::Pending
        );
    }

    #[test]
    fn gate_refuses_a_declared_art_that_contradicts_the_record() {
        assert_eq!(
            zeugen_gate(Some(*b"GBCO"), Some(ZeugeArt::S2Richtung), true),
            ZeugeVerdict::Pending
        );
    }

    #[test]
    fn serien_gate_holds_a_well_formed_series() {
        let samples = [(10.0, 1.0, 3.0), (20.0, 1.0, 4.0), (0.0, 0.0, -1.0)];
        assert_eq!(serien_gate(Some(*b"NRS1"), &samples), SerienVerdict::Hold);
        assert_eq!(serien_gate(Some(*b"FDS1"), &samples), SerienVerdict::Hold);
    }

    #[test]
    fn serien_gate_rejects_non_finite_samples() {
        assert_eq!(
            serien_gate(Some(*b"NRS1"), &[(f64::NAN, 1.0, 3.0)]),
            SerienVerdict::Reject
        );
        assert_eq!(
            serien_gate(Some(*b"NRS1"), &[(10.0, f64::INFINITY, 3.0)]),
            SerienVerdict::Reject
        );
        assert_eq!(
            serien_gate(Some(*b"NRS1"), &[(10.0, 1.0, f64::NAN)]),
            SerienVerdict::Reject
        );
    }

    #[test]
    fn serien_gate_rejects_negative_axis() {
        assert_eq!(
            serien_gate(Some(*b"NRS1"), &[(-1.0, 1.0, 3.0)]),
            SerienVerdict::Reject
        );
        assert_eq!(
            serien_gate(Some(*b"NRS1"), &[(10.0, -1.0, 3.0)]),
            SerienVerdict::Reject
        );
    }

    #[test]
    fn serien_gate_rejects_a_witness_record_as_foreign() {
        assert_eq!(
            serien_gate(Some(*b"AMN1"), &[(10.0, 1.0, 3.0)]),
            SerienVerdict::Reject
        );
    }

    #[test]
    fn serien_gate_pends_on_an_unknown_magic() {
        assert_eq!(
            serien_gate(Some(*b"XXXX"), &[(10.0, 1.0, 3.0)]),
            SerienVerdict::Pending
        );
        assert_eq!(
            serien_gate(None, &[(10.0, 1.0, 3.0)]),
            SerienVerdict::Pending
        );
    }
}
