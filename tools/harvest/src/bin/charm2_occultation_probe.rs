use std::process::Command;

const TAP: &str = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync";

const TOTAL: &str = r#"SELECT COUNT(*) FROM "J/A+A/431/773/charm2""#;
const LO: &str = r#"SELECT COUNT(*) FROM "J/A+A/431/773/charm2" WHERE Method='LO'"#;
const LO_DIAM: &str =
    r#"SELECT COUNT(*) FROM "J/A+A/431/773/charm2" WHERE Method='LO' AND (UD > 0 OR LD > 0)"#;
const LO_DIAM_PLX: &str = r#"SELECT COUNT(*) FROM "J/A+A/431/773/charm2" WHERE Method='LO' AND (UD > 0 OR LD > 0) AND Plx > 0"#;
const LO_DIAM_GAIA: &str = r#"SELECT COUNT(*) FROM "J/A+A/431/773/charm2" AS c JOIN "I/355/gaiadr3" AS g ON DISTANCE(POINT('ICRS', c.RAJ2000, c.DEJ2000), POINT('ICRS', g.RAJ2000, g.DEJ2000)) < 2.0/3600.0 WHERE c.Method='LO' AND (c.UD > 0 OR c.LD > 0)"#;
const GAIA_SAMPLE: &str = r#"SELECT TOP 3 c.RAJ2000, c.DEJ2000, c.UD, c.LD, c.Plx, g.Plx FROM "J/A+A/431/773/charm2" AS c JOIN "I/355/gaiadr3" AS g ON DISTANCE(POINT('ICRS', c.RAJ2000, c.DEJ2000), POINT('ICRS', g.RAJ2000, g.DEJ2000)) < 2.0/3600.0 WHERE c.Method='LO' AND (c.UD > 0 OR c.LD > 0)"#;

fn tap_tsv(adql: &str) -> Option<String> {
    let out = Command::new("curl")
        .arg("-sSf")
        .arg("-m")
        .arg("120")
        .arg("-G")
        .arg("--data-urlencode")
        .arg("REQUEST=doQuery")
        .arg("--data-urlencode")
        .arg("LANG=ADQL")
        .arg("--data-urlencode")
        .arg("FORMAT=tsv")
        .arg("--data-urlencode")
        .arg(format!("QUERY={adql}"))
        .arg(TAP)
        .output()
        .ok()?;
    if out.status.success() {
        String::from_utf8(out.stdout).ok()
    } else {
        eprintln!(
            "charm2 probe http {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        );
        None
    }
}

fn count(adql: &str) -> Option<i64> {
    let body = tap_tsv(adql)?;
    body.lines()
        .skip(1)
        .find_map(|l| l.trim().parse::<i64>().ok())
}

fn report(label: &str, v: Option<i64>) -> usize {
    match v {
        Some(n) => {
            println!("{label}: {n}");
            0
        }
        None => {
            println!("{label}: void");
            1
        }
    }
}

fn main() {
    let mut failures = 0usize;
    failures += report("charm2 total rows", count(TOTAL));
    failures += report("charm2 lunar-occultation (Method=LO) rows", count(LO));
    failures += report(
        "charm2 LO rows with angular diameter (UD or LD)",
        count(LO_DIAM),
    );
    failures += report(
        "charm2 LO rows with diameter + embedded Hipparcos parallax",
        count(LO_DIAM_PLX),
    );
    failures += report(
        "charm2 LO rows with diameter + Gaia DR3 cross-match (2 arcsec)",
        count(LO_DIAM_GAIA),
    );
    match tap_tsv(GAIA_SAMPLE) {
        Some(body) => println!("crossmatch sample (charm2 UD/LD/Plx + gaia Plx):\n{body}"),
        None => {
            println!("crossmatch sample: void");
            failures += 1;
        }
    }
    if failures > 0 {
        std::process::exit(2);
    }
}
