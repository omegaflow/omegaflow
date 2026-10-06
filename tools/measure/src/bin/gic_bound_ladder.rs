use std::process::exit;

const FAMILIES: [&str; 3] = ["yearly", "quarterly", "daily"];
const DRIVERS: [&str; 3] = ["Bz", "Speed", "Density"];
const ACCENT: &str = "#D55E00";
const GRAY: &str = "#999999";
const W: f64 = 960.0;
const H: f64 = 560.0;
const PLOT_LEFT: f64 = 90.0;
const PLOT_TOP: f64 = 96.0;
const PLOT_W: f64 = 830.0;
const PLOT_H: f64 = 392.0;

struct Row {
    driver: String,
    station: String,
    grain: String,
    te: Option<f64>,
    tau: Option<String>,
    bound: Option<f64>,
    verdict: String,
}

fn family_of(grain: &str) -> Option<&'static str> {
    for f in FAMILIES {
        if grain.starts_with(f) {
            return Some(f);
        }
    }
    None
}

fn number(token: &str) -> Option<f64> {
    token.parse::<f64>().ok().filter(|v| v.is_finite())
}

fn parse_row(line: &str) -> Option<Row> {
    let f: Vec<&str> = line.split(',').collect();
    if f.len() < 8 {
        return None;
    }
    let te = number(f[3]);
    let tau = if te.is_some() {
        number(f[4]).map(|v| {
            if v.fract() == 0.0 {
                format!("{v:.0}h")
            } else {
                format!("{v:.1}h")
            }
        })
    } else {
        None
    };
    Some(Row {
        driver: f[0].to_string(),
        station: f[1].to_string(),
        grain: f[2].to_string(),
        te,
        tau,
        bound: number(f[5]),
        verdict: f[7].to_string(),
    })
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn label(out: &mut String, x: f64, y: f64, anchor: &str, fill: &str, size: f64, s: &str) {
    out.push_str(&format!(
        "<text x=\"{x:.1}\" y=\"{y:.1}\" text-anchor=\"{anchor}\" fill=\"{fill}\" font-size=\"{size:.0}\">{}</text>\n",
        escape(s)
    ));
}

fn family_bound(rows: &[Row], family: &str) -> Option<f64> {
    rows.iter()
        .filter(|r| family_of(&r.grain) == Some(family))
        .filter_map(|r| r.bound)
        .fold(None, |acc, b| {
            Some(match acc {
                Some(a) if a > b => a,
                _ => b,
            })
        })
}

fn subgrains(rows: &[Row], family: &str) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for r in rows {
        if family_of(&r.grain) == Some(family) && !seen.contains(&r.grain) {
            seen.push(r.grain.clone());
        }
    }
    seen
}

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("gic_bound_ladder: the first argument carries the CSV path");
        exit(2);
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        eprintln!("gic_bound_ladder: {path} carries no readable ledger");
        exit(2);
    };
    let rows: Vec<Row> = text.lines().skip(1).filter_map(parse_row).collect();
    if rows.is_empty() {
        eprintln!("gic_bound_ladder: {path} carries no ledger rows");
        exit(2);
    }

    let mut ymax = 0.0_f64;
    for r in &rows {
        if let Some(t) = r.te {
            if t > ymax {
                ymax = t;
            }
        }
        if let Some(b) = r.bound {
            if b > ymax {
                ymax = b;
            }
        }
    }
    if ymax <= 0.0 {
        eprintln!("gic_bound_ladder: no measured TE and no bound in ledger");
        exit(2);
    }
    let ymax = ((ymax * 1.12 * 100.0).ceil()) / 100.0;

    let col_w = PLOT_W / 3.0;
    let lane_h = PLOT_H / 3.0;
    let te_y = |v: f64| PLOT_TOP + (1.0 - v / ymax) * PLOT_H;
    let col_left = |c: usize| PLOT_LEFT + c as f64 * col_w;

    let mut out = String::new();
    out.push_str(&format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{W:.0}\" height=\"{H:.0}\" viewBox=\"0 0 {W:.0} {H:.0}\">\n"
    ));
    out.push_str("<style>text{font-family:'DejaVu Sans',Arial,sans-serif;}</style>\n");
    out.push_str(&format!(
        "<rect x=\"0\" y=\"0\" width=\"{W:.0}\" height=\"{H:.0}\" fill=\"white\"/>\n"
    ));

    let mut tick = 0.0_f64;
    while tick <= ymax + 1e-9 {
        let y = te_y(tick);
        out.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{y:.1}\" x2=\"{:.1}\" y2=\"{y:.1}\" stroke=\"#e6e6e6\" stroke-width=\"1\"/>\n",
            PLOT_LEFT,
            PLOT_LEFT + PLOT_W
        ));
        label(
            &mut out,
            PLOT_LEFT - 8.0,
            y + 4.0,
            "end",
            GRAY,
            12.0,
            &format!("{tick:.2}"),
        );
        tick += 0.05;
    }

    out.push_str(&format!(
        "<line x1=\"{PLOT_LEFT:.1}\" y1=\"{PLOT_TOP:.1}\" x2=\"{PLOT_LEFT:.1}\" y2=\"{:.1}\" stroke=\"#333\" stroke-width=\"1.5\"/>\n",
        PLOT_TOP + PLOT_H
    ));
    label(
        &mut out,
        PLOT_LEFT - 8.0,
        PLOT_TOP - 14.0,
        "end",
        "#333",
        12.0,
        "TE (bits)",
    );

    for c in 0..3 {
        if c > 0 {
            let x = col_left(c);
            out.push_str(&format!(
                "<line x1=\"{x:.1}\" y1=\"{PLOT_TOP:.1}\" x2=\"{x:.1}\" y2=\"{:.1}\" stroke=\"#cccccc\" stroke-width=\"1\"/>\n",
                PLOT_TOP + PLOT_H
            ));
        }
    }
    for r in 1..3 {
        let y = PLOT_TOP + r as f64 * lane_h;
        out.push_str(&format!(
            "<line x1=\"{PLOT_LEFT:.1}\" y1=\"{y:.1}\" x2=\"{:.1}\" y2=\"{y:.1}\" stroke=\"#cccccc\" stroke-width=\"1\"/>\n",
            PLOT_LEFT + PLOT_W
        ));
    }

    for (c, family) in FAMILIES.iter().enumerate() {
        let cx = col_left(c) + col_w / 2.0;
        label(
            &mut out,
            cx,
            PLOT_TOP - 46.0,
            "middle",
            "#111",
            17.0,
            family,
        );
        let bound_text = match family_bound(&rows, family) {
            Some(_) => {
                let bs: Vec<f64> = rows
                    .iter()
                    .filter(|r| family_of(&r.grain) == Some(family))
                    .filter_map(|r| r.bound)
                    .collect();
                let lo = bs.iter().cloned().fold(f64::INFINITY, f64::min);
                let hi = bs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                if (hi - lo).abs() < 1e-9 {
                    format!("bound {lo:.5}")
                } else {
                    format!("bound {lo:.5}–{hi:.5}")
                }
            }
            None => "bound absent".to_string(),
        };
        label(
            &mut out,
            cx,
            PLOT_TOP - 28.0,
            "middle",
            "#333",
            12.0,
            &bound_text,
        );
    }

    for (r, driver) in DRIVERS.iter().enumerate() {
        let ly = PLOT_TOP + r as f64 * lane_h + 15.0;
        label(&mut out, PLOT_LEFT - 8.0, ly, "end", "#111", 13.0, driver);
    }

    for (c, family) in FAMILIES.iter().enumerate() {
        let subs = subgrains(&rows, family);
        if subs.is_empty() {
            continue;
        }
        let nsubs = subs.len() as f64;
        for (si, sg) in subs.iter().enumerate() {
            let Some(bound) = rows.iter().find(|r| &r.grain == sg).and_then(|r| r.bound) else {
                continue;
            };
            let y = te_y(bound);
            out.push_str(&format!(
                "<line x1=\"{:.1}\" y1=\"{y:.1}\" x2=\"{:.1}\" y2=\"{y:.1}\" stroke=\"#000\" stroke-width=\"2\"/>\n",
                col_left(c) + col_w * si as f64 / nsubs + 8.0,
                col_left(c) + col_w * (si as f64 + 1.0) / nsubs - 8.0
            ));
        }
    }

    for (c, family) in FAMILIES.iter().enumerate() {
        let subs = subgrains(&rows, family);
        if subs.is_empty() {
            continue;
        }
        let nsubs = subs.len() as f64;
        for (r, driver) in DRIVERS.iter().enumerate() {
            let lane_mid = PLOT_TOP + r as f64 * lane_h + lane_h / 2.0;
            for row in rows.iter() {
                if family_of(&row.grain) != Some(family) || row.driver != *driver {
                    continue;
                }
                let Some(sub) = subs.iter().position(|g| g == &row.grain) else {
                    continue;
                };
                let base = col_left(c) + col_w * (sub as f64 + 0.5) / nsubs;
                let station_x = if row.station == "ABK" {
                    base - 7.0
                } else {
                    base + 7.0
                };
                let color = if row.verdict == "clears" {
                    ACCENT
                } else {
                    GRAY
                };
                let Some(te) = row.te else {
                    label(&mut out, base, lane_mid + 4.0, "middle", GRAY, 15.0, "–");
                    label(
                        &mut out,
                        base,
                        lane_mid + 19.0,
                        "middle",
                        GRAY,
                        10.0,
                        "unbelegt",
                    );
                    continue;
                };
                let y = te_y(te);
                if row.station == "ABK" {
                    out.push_str(&format!(
                        "<circle cx=\"{station_x:.1}\" cy=\"{y:.1}\" r=\"5\" fill=\"{color}\" stroke=\"{color}\" stroke-width=\"1\"/>\n"
                    ));
                } else {
                    let fill = if row.verdict == "clears" {
                        ACCENT
                    } else {
                        "#ffffff"
                    };
                    out.push_str(&format!(
                        "<polygon points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"{fill}\" stroke=\"{color}\" stroke-width=\"1.5\"/>\n",
                        station_x,
                        y - 6.0,
                        station_x - 6.0,
                        y + 5.0,
                        station_x + 6.0,
                        y + 5.0
                    ));
                }
                if let Some(tau) = &row.tau {
                    label(&mut out, station_x, y + 17.0, "middle", "#333", 10.0, tau);
                }
            }
        }
    }

    label(
        &mut out,
        400.0,
        H - 22.0,
        "middle",
        "#333",
        12.0,
        "illustrative of the measured ledger; ABK and SOD are never merged",
    );

    out.push_str("</svg>\n");
    print!("{out}");
}
