use super::geo::GeoRec;

pub const FORMAT: &str = "grid_outage_event";

pub const COMP_DURATION_H: u32 = 1;
pub const COMP_MAX_CUSTOMERS: u32 = 2;
pub const COMP_CAUSE_CLASS: u32 = 3;
pub const COMP_MAX: u32 = 3;

pub const CAUSE_UNCLASSIFIED: u32 = 0;
pub const CAUSE_SEVERE_WEATHER: u32 = 1;
pub const CAUSE_WEATHER_OR_NATURAL_DISASTER: u32 = 2;
pub const CAUSE_NATURAL_DISASTER: u32 = 3;
pub const CAUSE_VANDALISM: u32 = 4;
pub const CAUSE_SUSPICIOUS_ACTIVITY: u32 = 5;
pub const CAUSE_PHYSICAL_ATTACK: u32 = 6;
pub const CAUSE_CYBER_EVENT: u32 = 7;
pub const CAUSE_SABOTAGE: u32 = 8;
pub const CAUSE_THEFT: u32 = 9;
pub const CAUSE_SYSTEM_OPERATIONS: u32 = 10;
pub const CAUSE_GENERATION_INADEQUACY: u32 = 11;
pub const CAUSE_FUEL_SUPPLY: u32 = 12;
pub const CAUSE_TRANSMISSION_INTERRUPTION: u32 = 13;
pub const CAUSE_DISTRIBUTION_INTERRUPTION: u32 = 14;
pub const CAUSE_ISLANDING: u32 = 15;
pub const CAUSE_PUBLIC_APPEAL: u32 = 16;
pub const CAUSE_EQUIPMENT_FAILURE: u32 = 17;
pub const CAUSE_UNKNOWN: u32 = 18;
pub const CAUSE_OTHER: u32 = 19;

pub fn component_name(comp: u32) -> Option<&'static str> {
    match comp {
        COMP_DURATION_H => Some("grid_outage_duration_h"),
        COMP_MAX_CUSTOMERS => Some("grid_outage_max_customers"),
        COMP_CAUSE_CLASS => Some("grid_outage_cause_class"),
        _ => None,
    }
}

pub fn cause_name(code: u32) -> Option<&'static str> {
    match code {
        CAUSE_UNCLASSIFIED => Some("unclassified"),
        CAUSE_SEVERE_WEATHER => Some("severe_weather"),
        CAUSE_WEATHER_OR_NATURAL_DISASTER => Some("weather_or_natural_disaster"),
        CAUSE_NATURAL_DISASTER => Some("natural_disaster"),
        CAUSE_VANDALISM => Some("vandalism"),
        CAUSE_SUSPICIOUS_ACTIVITY => Some("suspicious_activity"),
        CAUSE_PHYSICAL_ATTACK => Some("physical_attack"),
        CAUSE_CYBER_EVENT => Some("cyber_event"),
        CAUSE_SABOTAGE => Some("sabotage"),
        CAUSE_THEFT => Some("theft"),
        CAUSE_SYSTEM_OPERATIONS => Some("system_operations"),
        CAUSE_GENERATION_INADEQUACY => Some("generation_inadequacy"),
        CAUSE_FUEL_SUPPLY => Some("fuel_supply"),
        CAUSE_TRANSMISSION_INTERRUPTION => Some("transmission_interruption"),
        CAUSE_DISTRIBUTION_INTERRUPTION => Some("distribution_interruption"),
        CAUSE_ISLANDING => Some("islanding"),
        CAUSE_PUBLIC_APPEAL => Some("public_appeal"),
        CAUSE_EQUIPMENT_FAILURE => Some("equipment_failure"),
        CAUSE_UNKNOWN => Some("unknown"),
        CAUSE_OTHER => Some("other"),
        _ => None,
    }
}

pub fn cause_code(event_type: &str) -> u32 {
    let t = event_type
        .trim()
        .trim_start_matches('-')
        .trim()
        .to_ascii_lowercase();
    let t = t.replace('_', " ");
    if t.is_empty() {
        return CAUSE_UNCLASSIFIED;
    }
    if t.contains("cyber") {
        return CAUSE_CYBER_EVENT;
    }
    if t.contains("sabotage") {
        return CAUSE_SABOTAGE;
    }
    if t.contains("theft") {
        return CAUSE_THEFT;
    }
    if t.contains("physical attack") || t.contains("physical event") {
        return CAUSE_PHYSICAL_ATTACK;
    }
    if t.contains("vandal") {
        return CAUSE_VANDALISM;
    }
    if t.contains("suspicious") {
        return CAUSE_SUSPICIOUS_ACTIVITY;
    }
    if t.contains("severe weather") || t.contains("sever weather") {
        return CAUSE_SEVERE_WEATHER;
    }
    if t.contains("weather or natural disaster") {
        return CAUSE_WEATHER_OR_NATURAL_DISASTER;
    }
    if t.contains("natural disaster") {
        return CAUSE_NATURAL_DISASTER;
    }
    if t.contains("weather") {
        return CAUSE_WEATHER_OR_NATURAL_DISASTER;
    }
    if t.contains("fuel supply") {
        return CAUSE_FUEL_SUPPLY;
    }
    if t.contains("islanding") {
        return CAUSE_ISLANDING;
    }
    if t.contains("generation inadequacy") {
        return CAUSE_GENERATION_INADEQUACY;
    }
    if t.contains("public appeal") {
        return CAUSE_PUBLIC_APPEAL;
    }
    if t.contains("system operations") {
        return CAUSE_SYSTEM_OPERATIONS;
    }
    if t.contains("distribution interruption") {
        return CAUSE_DISTRIBUTION_INTERRUPTION;
    }
    if t.contains("transmission interruption") || t.contains("transmission disruption") {
        return CAUSE_TRANSMISSION_INTERRUPTION;
    }
    if t.contains("high voltage substation") || t.contains("transmission equipment failure") {
        return CAUSE_EQUIPMENT_FAILURE;
    }
    if t.contains("unknown") {
        return CAUSE_UNKNOWN;
    }
    if t.contains("other") {
        return CAUSE_OTHER;
    }
    CAUSE_UNCLASSIFIED
}

pub fn set_cause(rec: &mut GeoRec, code: u32) {
    rec.bin_width = code as f64;
}

pub fn cause_of(rec: &GeoRec) -> u32 {
    if !rec.bin_width.is_finite() || rec.bin_width < 0.0 {
        return CAUSE_UNCLASSIFIED;
    }
    rec.bin_width as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_observed_vocabulary_never_falls_through() {
        let observed = [
            "Actual Physical Attack",
            "Actual Physical Attack/Vandalism",
            "Actual Physical Event",
            "Cyber Attack",
            "Cyber Event",
            "Distribution Interruption",
            "Distribution Interruption - Unknown Cause",
            "Electrical System Islanding",
            "Failure at high voltage substation or switchyard",
            "Fuel Supply Deficiency",
            "Fuel Supply Emergency - Coal",
            "Generation Inadequacy",
            "Islanding",
            "Natural Disaster",
            "Natural Disaster/Transmission Interruption",
            "Other",
            "Physical Attack",
            "Potential Physical Attack",
            "Public Appeal",
            "Sabotage",
            "Severe Weather",
            "Severe Weather/Distribution Interruption",
            "Sever Severe Weather",
            "Severe Weather- High Winds",
            "Severe Weather/Transmisison Interruption",
            "Severe Weather - Wind",
            "Severe Weather - Winter",
            "Severe Weather - Winter Storm",
            "Suspected Cyber Attack",
            "Suspected Physical Attack",
            "Suspicious Activity",
            "System Operations",
            "Theft",
            "Transmission Disruption",
            "Transmission Interruption",
            "Unknown",
            "Vandalism",
            "Weather",
            "Weather or Natural Disaster",
        ];
        for s in observed {
            assert_ne!(cause_code(s), CAUSE_UNCLASSIFIED, "{s}");
        }
    }

    #[test]
    fn the_specific_labels_keep_their_priority() {
        assert_eq!(cause_code("Severe Weather - Winter"), CAUSE_SEVERE_WEATHER);
        assert_eq!(
            cause_code("Natural Disaster/Transmission Interruption"),
            CAUSE_NATURAL_DISASTER
        );
        assert_eq!(
            cause_code("Actual Physical Attack/Vandalism"),
            CAUSE_PHYSICAL_ATTACK
        );
        assert_eq!(cause_code("Vandalism - Theft"), CAUSE_THEFT);
        assert_eq!(cause_code("Unknown - Vandalism"), CAUSE_VANDALISM);
        assert_eq!(cause_code("Other"), CAUSE_OTHER);
        assert_eq!(cause_code("something unseen"), CAUSE_UNCLASSIFIED);
    }

    #[test]
    fn the_cause_roundtrips_through_the_geo_slot() {
        let mut rec = GeoRec {
            t: 0.0,
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
            freq: 0.0,
            bin_width: 0.0,
            val: 0.0,
            comp: COMP_DURATION_H,
            station: 0,
        };
        set_cause(&mut rec, CAUSE_SEVERE_WEATHER);
        assert_eq!(cause_of(&rec), CAUSE_SEVERE_WEATHER);
        assert_eq!(cause_name(CAUSE_SEVERE_WEATHER), Some("severe_weather"));
    }

    #[test]
    fn component_names_are_the_register_keys() {
        assert_eq!(
            component_name(COMP_DURATION_H),
            Some("grid_outage_duration_h")
        );
        assert_eq!(
            component_name(COMP_MAX_CUSTOMERS),
            Some("grid_outage_max_customers")
        );
        assert_eq!(
            component_name(COMP_CAUSE_CLASS),
            Some("grid_outage_cause_class")
        );
        assert_eq!(component_name(4), None);
    }
}
