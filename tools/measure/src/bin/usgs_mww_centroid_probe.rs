use omegaflow::archivar::fetch_raw;
use omegaflow::archivar::ndk;
use omegaflow_measure::depthphase::{CATALOG_URL, arc_deg, arg_value};
use omegaflow_measure::mww::{MwwRecord, NodalPlane, iso_ymd, parse_quakeml};

const GCMT_NDK_URL: &str =
    "https://www.ldeo.columbia.edu/~gcmt/projects/CMT/catalog/jan76_dec25.ndk";
const GCMT_TTL_S: u64 = 86400;
const KM_PER_DEG: f64 = 111.195;

fn human(v: Option<f64>, prec: usize) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{:.*}", prec, x),
        _ => "absent".to_string(),
    }
}

fn json_num(v: Option<f64>, prec: usize) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{:.*}", prec, x),
        _ => "null".to_string(),
    }
}

fn json_sci(v: Option<f64>) -> String {
    match v {
        Some(x) if x.is_finite() => format!("{x:.6e}"),
        _ => "null".to_string(),
    }
}

fn json_str(v: Option<&str>) -> String {
    match v {
        Some(s) => format!("\"{s}\""),
        None => "null".to_string(),
    }
}

fn plane_json(p: &NodalPlane) -> String {
    format!(
        "{{\"strike\": {}, \"dip\": {}, \"rake\": {}}}",
        json_num(p.strike, 4),
        json_num(p.dip, 4),
        json_num(p.rake, 4)
    )
}

fn gcmt_crosscheck(record: &MwwRecord) -> Option<String> {
    let lat = record.centroid.lat?;
    let lon = record.centroid.lon?;
    let (y, m, d) = iso_ymd(record.centroid.time_iso.as_deref()?)?;
    let events = ndk::fetch_events(GCMT_NDK_URL, GCMT_TTL_S)?;
    let ev = events
        .iter()
        .filter(|e| e.year == y && e.month == m && (e.day as i64 - d as i64).abs() <= 1)
        .filter(|e| (e.hyp_lat - lat).abs() < 2.0 && (e.hyp_lon - lon).abs() < 2.0)
        .min_by(|a, b| {
            let da = (a.hyp_lat - lat).powi(2) + (a.hyp_lon - lon).powi(2);
            let db = (b.hyp_lat - lat).powi(2) + (b.hyp_lon - lon).powi(2);
            da.total_cmp(&db)
        })?;

    let scale = 10f64.powi(ev.exponent) * 1e-7;
    let gcmt = [ev.m_rr, ev.m_tt, ev.m_pp, ev.m_rt, ev.m_rp, ev.m_tp];
    let usgs = [
        record.moment_tensor.mrr,
        record.moment_tensor.mtt,
        record.moment_tensor.mpp,
        record.moment_tensor.mrt,
        record.moment_tensor.mrp,
        record.moment_tensor.mtp,
    ];
    let mut num = 0.0;
    let mut den = 0.0;
    let mut n = 0usize;
    let mut g_nm = [0.0f64; 6];
    for i in 0..6 {
        let gi = gcmt[i] * scale;
        g_nm[i] = gi;
        if let Some(ui) = usgs[i] {
            num += (ui - gi) * (ui - gi);
            den += ui * ui;
            n += 1;
        }
    }
    let rel = if n > 0 && den > 0.0 {
        Some((num / den).sqrt())
    } else {
        None
    };
    let g_m0_nm = ev.scalar_moment_dyne_cm() * 1e-7;
    let ratio = record
        .moment_tensor
        .scalar_moment_nm
        .filter(|m| *m > 0.0)
        .zip(if g_m0_nm > 0.0 { Some(g_m0_nm) } else { None })
        .map(|(a, b)| a / b);
    let comps = g_nm
        .iter()
        .map(|v| format!("{v:.6e}"))
        .collect::<Vec<_>>()
        .join(", ");
    Some(format!(
        "{{\"name\": \"{}\", \"gcmt_components_nm\": [{}], \"gcmt_scalar_moment_nm\": {:.6e}, \"tensor_rms_relative\": {}, \"scalar_moment_ratio\": {}}}",
        ev.name,
        comps,
        g_m0_nm,
        rel.map(|v| format!("{v:.6}")).unwrap_or("null".into()),
        ratio.map(|v| format!("{v:.6}")).unwrap_or("null".into()),
    ))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(eventid) = arg_value(&args, "--eventid") else {
        println!(
            "usgs_mww_centroid_probe: --eventid <id> absent — the centroid record stays absent"
        );
        return;
    };
    let url = format!("{CATALOG_URL}?eventid={eventid}&format=quakeml&magnitudetype=mww");
    let Some(xml) = fetch_raw(&url, None, &[]) else {
        println!(
            "usgs_mww_centroid_probe: {url} carries no body — the centroid record stays absent"
        );
        return;
    };
    let Some(record) = parse_quakeml(&xml) else {
        println!(
            "usgs_mww_centroid_probe: {eventid} carries no moment tensor — the centroid record stays absent"
        );
        return;
    };

    println!("=== USGS FDSN mww moment-tensor centroid (M9 source location) ===");
    println!("endpoint: {url}");
    println!("eventid: {eventid}");
    println!(
        "M9 source location (USGS mww centroid): lat {}, lon {}, depth_m {}, time {}",
        human(record.centroid.lat, 6),
        human(record.centroid.lon, 6),
        human(record.centroid.depth_m, 1),
        record.centroid.time_iso.as_deref().unwrap_or("absent"),
    );
    println!();

    println!("{{");
    println!("  \"probe\": \"usgs_mww_centroid\",");
    println!("  \"endpoint\": \"{url}\",");
    println!("  \"eventid\": \"{eventid}\",");
    println!("  \"m9_source_location\": \"USGS mww centroid\",");
    println!(
        "  \"centroid\": {{\"lat\": {}, \"lon\": {}, \"depth_m\": {}, \"time_unix\": {}, \"time_iso\": {}}},",
        json_num(record.centroid.lat, 6),
        json_num(record.centroid.lon, 6),
        json_num(record.centroid.depth_m, 1),
        json_num(record.centroid.time_unix, 3),
        json_str(record.centroid.time_iso.as_deref()),
    );
    println!(
        "  \"magnitude\": {{\"type\": {}, \"value\": {}}},",
        json_str(record.magnitude_type.as_deref()),
        json_num(record.magnitude, 3)
    );
    println!(
        "  \"moment_tensor\": {{\"mrr\": {}, \"mtt\": {}, \"mpp\": {}, \"mrt\": {}, \"mrp\": {}, \"mtp\": {}, \"scalar_moment_nm\": {}, \"double_couple\": {}}},",
        json_sci(record.moment_tensor.mrr),
        json_sci(record.moment_tensor.mtt),
        json_sci(record.moment_tensor.mpp),
        json_sci(record.moment_tensor.mrt),
        json_sci(record.moment_tensor.mrp),
        json_sci(record.moment_tensor.mtp),
        json_sci(record.moment_tensor.scalar_moment_nm),
        json_num(record.moment_tensor.double_couple, 4),
    );
    println!(
        "  \"nodal_planes\": {{\"np1\": {}, \"np2\": {}}},",
        plane_json(&record.np1),
        plane_json(&record.np2)
    );

    let picker_lat = arg_value(&args, "--picker-lat")
        .and_then(|s| s.parse::<f64>().ok())
        .filter(|v| v.is_finite());
    let picker_lon = arg_value(&args, "--picker-lon")
        .and_then(|s| s.parse::<f64>().ok())
        .filter(|v| v.is_finite());
    match (
        picker_lat,
        picker_lon,
        record.centroid.lat,
        record.centroid.lon,
    ) {
        (Some(plat), Some(plon), Some(clat), Some(clon)) => {
            let deg = arc_deg(clat, clon, plat, plon);
            println!(
                "  \"picker_offset\": {{\"picker_lat\": {plat}, \"picker_lon\": {plon}, \"arc_deg\": {deg:.6}, \"km\": {:.3}}},",
                deg * KM_PER_DEG
            );
        }
        (Some(_), Some(_), _, _) => {
            println!("  \"picker_offset\": \"pending (centroid lat/lon absent)\",");
        }
        _ => println!("  \"picker_offset\": null,"),
    }

    if args.iter().any(|a| a == "--gcmt") {
        match gcmt_crosscheck(&record) {
            Some(txt) => println!("  \"gcmt_crosscheck\": {txt}"),
            None => println!("  \"gcmt_crosscheck\": \"pending (no matching GCMT centroid)\""),
        }
    } else {
        println!("  \"gcmt_crosscheck\": \"pending (--gcmt)\"");
    }
    println!("}}");
}
