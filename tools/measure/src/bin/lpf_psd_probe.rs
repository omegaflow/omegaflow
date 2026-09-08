const FMIN_HZ: f64 = 1e-4;
const FMAX_HZ: f64 = 1.0;
const N_BINS: usize = 31;
const TWO_PI: f64 = 6.283185307179586;

const S0_HALF_FM: f64 = 5.2;
const S0_HALF_SIG_FM: f64 = 0.1;
const SIFO_HALF_FM: f64 = 34.8;
const SIFO_HALF_SIG_FM: f64 = 0.3;

fn main() {
    println!("=== LISA Pathfinder PSD — published model (Armano+ 2016, PRL 116, 231101) ===");
    println!(
        "model S_dg(f) = S_Brown + S_IFO·(2πf)⁴, with S_Brown^1/2 = ({:.1} ± {:.1}) fm s⁻²/√Hz, S_IFO^1/2 = ({:.1} ± {:.1}) fm/√Hz",
        S0_HALF_FM, S0_HALF_SIG_FM, SIFO_HALF_FM, SIFO_HALF_SIG_FM
    );
    println!("(the Fig. 1 caption carries S0^1/2 = 5.57 ± 0.04 fm s⁻²/√Hz for the day-127 run; the abstract carries the later 5.2 ± 0.1 value)");
    println!(
        "frequency band: {:.0e} Hz .. {:.0e} Hz (LISA band), {} log-spaced bins",
        FMIN_HZ, FMAX_HZ, N_BINS
    );
    println!();
    println!("freq(Hz)  PSD_DA((m/s²)²/Hz)  PSD_noise_floor((m/s²)²/Hz)  Phase(rad)");

    let s_brown = (S0_HALF_FM * 1e-15).powi(2);
    let s_ifo = (SIFO_HALF_FM * 1e-15).powi(2);
    let log_fmin = FMIN_HZ.log10();
    let log_fmax = FMAX_HZ.log10();
    for i in 0..N_BINS {
        let log_f = log_fmin + (log_fmax - log_fmin) * (i as f64) / ((N_BINS - 1) as f64);
        let f = 10f64.powf(log_f);
        let psd_da = s_brown + s_ifo * (TWO_PI * f).powi(4);
        println!("{:.6e}  {:.6e}  {:.6e}  absent", f, psd_da, s_brown);
    }

    println!();
    println!("Verdict:");
    println!("PSD_DA and PSD_noise_floor are the published model evaluated at the LISA band — not the measured time series.");
    println!(
        "The measured Δg series lives in the ESA LPF Legacy Archive (lpf.esac.esa.int/lpfsa/, CC BY-NC 3.0 IGO), behind CAS auth with the TAP interface disabled (measured: /lpf-tap/ returns 404, config.js auth_method=cas, tap_adql_interface_active=false)."
    );
    println!(
        "The cross-spectral Phase (Δg ↔ magnetic/temperature/thruster, Boileau+ 2022 PRD 106, 063025) needs the raw series — absent until the archive grants it or the author request (Armano/McNamara) answers."
    );
}
