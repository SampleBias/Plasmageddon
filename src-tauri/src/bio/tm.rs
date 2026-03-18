use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmResult {
    pub tm_basic: f64,
    pub tm_nearest_neighbor: f64,
    pub length: usize,
}

/// Nearest-neighbor thermodynamic parameters (kcal/mol for dH, cal/mol/K for dS)
/// SantaLucia 1998 unified parameters
fn nn_params(dinuc: &str) -> (f64, f64) {
    match dinuc {
        "AA" | "TT" => (-7.9, -22.2),
        "AT" => (-7.2, -20.4),
        "TA" => (-7.2, -21.3),
        "CA" | "TG" => (-8.5, -22.7),
        "GT" | "AC" => (-8.4, -22.4),
        "CT" | "AG" => (-7.8, -21.0),
        "GA" | "TC" => (-8.2, -22.2),
        "CG" => (-10.6, -27.2),
        "GC" => (-9.8, -24.4),
        "GG" | "CC" => (-8.0, -19.9),
        _ => (-7.0, -20.0),
    }
}

pub fn compute_tm(sequence: &str) -> TmResult {
    let seq: Vec<char> = sequence.to_ascii_uppercase().chars().collect();
    let len = seq.len();

    if len == 0 {
        return TmResult { tm_basic: 0.0, tm_nearest_neighbor: 0.0, length: 0 };
    }

    let gc = seq.iter().filter(|&&c| c == 'G' || c == 'C').count();
    let at = seq.iter().filter(|&&c| c == 'A' || c == 'T').count();

    let tm_basic = if len < 14 {
        (at as f64 * 2.0) + (gc as f64 * 4.0)
    } else {
        64.9 + 41.0 * (gc as f64 - 16.4) / len as f64
    };

    // Nearest-neighbor
    let mut dh: f64 = 0.0;  // kcal/mol
    let mut ds: f64 = 0.0;  // cal/mol/K

    // Initiation
    dh += 0.1;
    ds += -2.8;

    for i in 0..len.saturating_sub(1) {
        let dinuc: String = seq[i..=i + 1].iter().collect();
        let (h, s) = nn_params(&dinuc);
        dh += h;
        ds += s;
    }

    let r: f64 = 1.987;
    let ct: f64 = 250e-9; // 250 nM oligo
    let na: f64 = 50e-3;  // 50 mM Na+

    let tm_nn = (dh * 1000.0) / (ds + r * (ct / 4.0).ln()) - 273.15
        + 16.6 * na.log10();

    TmResult {
        tm_basic,
        tm_nearest_neighbor: (tm_nn * 10.0).round() / 10.0,
        length: len,
    }
}
