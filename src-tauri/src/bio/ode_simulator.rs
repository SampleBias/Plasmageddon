use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdeSimConfig {
    pub circuit_type: String,
    pub parts: Vec<CircuitPart>,
    pub duration_hours: f64,
    pub dt: f64,
    pub parameters: Option<OdeParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitPart {
    pub name: String,
    pub part_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdeParams {
    pub alpha: Option<f64>,
    pub alpha0: Option<f64>,
    pub beta: Option<f64>,
    pub n: Option<f64>,
    pub k_m: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdeSimResult {
    pub time_points: Vec<f64>,
    pub species: Vec<SpeciesTimeCourse>,
    pub steady_state: bool,
    pub period_hours: Option<f64>,
    pub circuit_type: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesTimeCourse {
    pub name: String,
    pub color: String,
    pub values: Vec<f64>,
}

impl Default for OdeParams {
    fn default() -> Self {
        Self {
            alpha: Some(216.0),
            alpha0: Some(0.216),
            beta: Some(5.0),
            n: Some(2.0),
            k_m: Some(40.0),
        }
    }
}

fn hill_repression(repressor: f64, alpha: f64, alpha0: f64, n: f64, k_m: f64) -> f64 {
    alpha0 + alpha / (1.0 + (repressor / k_m).powf(n))
}

pub fn hill_activation(activator: f64, alpha: f64, alpha0: f64, n: f64, k_m: f64) -> f64 {
    alpha0 + alpha * (activator / k_m).powf(n) / (1.0 + (activator / k_m).powf(n))
}

pub fn detect_circuit_type(parts: &[CircuitPart]) -> String {
    let names: Vec<String> = parts.iter().map(|p| p.name.to_lowercase()).collect();
    let types: Vec<String> = parts.iter().map(|p| p.part_type.to_lowercase()).collect();

    let has_laci = names.iter().any(|n| n.contains("laci"));
    let has_tetr = names.iter().any(|n| n.contains("tetr"));
    let has_ci = names.iter().any(|n| n.contains("ci") || n.contains("lambda"));
    let _has_reporter = names.iter().any(|n| n.contains("lacz") || n.contains("gfp") || n.contains("rfp") || n.contains("yfp"));

    let cds_count = types.iter().filter(|t| t.as_str() == "cds").count();
    let promoter_count = types.iter().filter(|t| t.as_str() == "promoter").count();

    if has_laci && has_tetr && has_ci {
        return "repressilator".to_string();
    }
    if has_laci && has_tetr && !has_ci {
        return "toggle_switch".to_string();
    }
    if cds_count == 1 && promoter_count >= 1 {
        if has_laci || has_tetr || has_ci {
            return "inverter".to_string();
        }
        return "simple_expression".to_string();
    }
    if cds_count >= 2 {
        return "multi_gene".to_string();
    }
    "simple_expression".to_string()
}

pub fn run_ode_simulation(config: &OdeSimConfig) -> Result<OdeSimResult, String> {
    let circuit = if config.circuit_type == "auto" {
        detect_circuit_type(&config.parts)
    } else {
        config.circuit_type.clone()
    };

    match circuit.as_str() {
        "repressilator" => simulate_repressilator(config),
        "toggle_switch" => simulate_toggle_switch(config),
        "inverter" => simulate_inverter(config),
        "simple_expression" => simulate_simple_expression(config),
        _ => simulate_simple_expression(config),
    }
}

fn simulate_repressilator(config: &OdeSimConfig) -> Result<OdeSimResult, String> {
    let p = config.parameters.clone().unwrap_or_default();
    let alpha = p.alpha.unwrap_or(216.0);
    let alpha0 = p.alpha0.unwrap_or(0.216);
    let beta = p.beta.unwrap_or(5.0);
    let n = p.n.unwrap_or(2.0);
    let k_m = p.k_m.unwrap_or(40.0);

    let duration = config.duration_hours * 60.0;
    let dt = config.dt.min(0.01);
    let steps = (duration / dt) as usize;
    let record_every = (steps / 2000).max(1);

    let mut m1: f64 = 5.0;
    let mut m2: f64 = 0.0;
    let mut m3: f64 = 0.0;
    let mut p1: f64 = 5.0;
    let mut p2: f64 = 0.0;
    let mut p3: f64 = 0.0;

    let mut time_points = Vec::new();
    let mut laci_vals = Vec::new();
    let mut tetr_vals = Vec::new();
    let mut ci_vals = Vec::new();

    let reporter_names = find_reporters(&config.parts);

    for step in 0..steps {
        let dm1 = hill_repression(p3, alpha, alpha0, n, k_m) - m1;
        let dm2 = hill_repression(p1, alpha, alpha0, n, k_m) - m2;
        let dm3 = hill_repression(p2, alpha, alpha0, n, k_m) - m3;

        let dp1 = beta * (m1 - p1);
        let dp2 = beta * (m2 - p2);
        let dp3 = beta * (m3 - p3);

        m1 += dm1 * dt;
        m2 += dm2 * dt;
        m3 += dm3 * dt;
        p1 += dp1 * dt;
        p2 += dp2 * dt;
        p3 += dp3 * dt;

        m1 = m1.max(0.0);
        m2 = m2.max(0.0);
        m3 = m3.max(0.0);
        p1 = p1.max(0.0);
        p2 = p2.max(0.0);
        p3 = p3.max(0.0);

        if step % record_every == 0 {
            time_points.push(step as f64 * dt / 60.0);
            laci_vals.push(p1);
            tetr_vals.push(p2);
            ci_vals.push(p3);
        }
    }

    let period = estimate_period(&time_points, &laci_vals);

    Ok(OdeSimResult {
        time_points,
        species: vec![
            SpeciesTimeCourse {
                name: reporter_names.get(0).cloned().unwrap_or_else(|| "LacI".to_string()),
                color: "#4ade80".to_string(),
                values: laci_vals,
            },
            SpeciesTimeCourse {
                name: reporter_names.get(1).cloned().unwrap_or_else(|| "TetR".to_string()),
                color: "#f87171".to_string(),
                values: tetr_vals,
            },
            SpeciesTimeCourse {
                name: reporter_names.get(2).cloned().unwrap_or_else(|| "cI".to_string()),
                color: "#60a5fa".to_string(),
                values: ci_vals,
            },
        ],
        steady_state: false,
        period_hours: period,
        circuit_type: "repressilator".to_string(),
        notes: format!(
            "Repressilator simulation: 3-gene oscillatory network.\nParameters: α={alpha}, α₀={alpha0}, β={beta}, n={n}, Km={k_m}\n{}",
            if let Some(p) = period {
                format!("Estimated oscillation period: {:.1} hours", p)
            } else {
                "No clear oscillation detected".to_string()
            }
        ),
    })
}

fn simulate_toggle_switch(config: &OdeSimConfig) -> Result<OdeSimResult, String> {
    let p = config.parameters.clone().unwrap_or_default();
    let alpha = p.alpha.unwrap_or(216.0);
    let alpha0 = p.alpha0.unwrap_or(0.216);
    let beta = p.beta.unwrap_or(5.0);
    let n = p.n.unwrap_or(2.0);
    let k_m = p.k_m.unwrap_or(40.0);

    let duration = config.duration_hours * 60.0;
    let dt = config.dt.min(0.01);
    let steps = (duration / dt) as usize;
    let record_every = (steps / 2000).max(1);

    let mut m1: f64 = 10.0;
    let mut m2: f64 = 0.5;
    let mut p1: f64 = 10.0;
    let mut p2: f64 = 0.5;

    let mut time_points = Vec::new();
    let mut species1_vals = Vec::new();
    let mut species2_vals = Vec::new();

    for step in 0..steps {
        let dm1 = hill_repression(p2, alpha, alpha0, n, k_m) - m1;
        let dm2 = hill_repression(p1, alpha, alpha0, n, k_m) - m2;
        let dp1 = beta * (m1 - p1);
        let dp2 = beta * (m2 - p2);

        m1 += dm1 * dt;
        m2 += dm2 * dt;
        p1 += dp1 * dt;
        p2 += dp2 * dt;

        m1 = m1.max(0.0);
        m2 = m2.max(0.0);
        p1 = p1.max(0.0);
        p2 = p2.max(0.0);

        if step % record_every == 0 {
            time_points.push(step as f64 * dt / 60.0);
            species1_vals.push(p1);
            species2_vals.push(p2);
        }
    }

    Ok(OdeSimResult {
        time_points,
        species: vec![
            SpeciesTimeCourse {
                name: "LacI (Repressor 1)".to_string(),
                color: "#4ade80".to_string(),
                values: species1_vals,
            },
            SpeciesTimeCourse {
                name: "TetR (Repressor 2)".to_string(),
                color: "#f87171".to_string(),
                values: species2_vals,
            },
        ],
        steady_state: true,
        period_hours: None,
        circuit_type: "toggle_switch".to_string(),
        notes: format!(
            "Toggle switch simulation: bistable circuit.\nParameters: α={alpha}, α₀={alpha0}, β={beta}, n={n}, Km={k_m}\nSystem settles to one of two stable states."
        ),
    })
}

fn simulate_inverter(config: &OdeSimConfig) -> Result<OdeSimResult, String> {
    let p = config.parameters.clone().unwrap_or_default();
    let alpha = p.alpha.unwrap_or(100.0);
    let alpha0 = p.alpha0.unwrap_or(0.5);
    let beta = p.beta.unwrap_or(5.0);
    let n = p.n.unwrap_or(2.0);
    let k_m = p.k_m.unwrap_or(40.0);

    let duration = config.duration_hours * 60.0;
    let dt = config.dt.min(0.01);
    let steps = (duration / dt) as usize;
    let record_every = (steps / 2000).max(1);

    let mut m_in: f64 = 0.0;
    let mut p_in: f64 = 0.0;
    let mut m_out: f64 = 0.0;
    let mut p_out: f64 = 0.0;

    let mut time_points = Vec::new();
    let mut input_vals = Vec::new();
    let mut output_vals = Vec::new();

    let switch_time = steps / 3;

    for step in 0..steps {
        let input_signal: f64 = if step > switch_time && step < switch_time * 2 {
            100.0
        } else {
            0.0
        };

        let dm_in = input_signal - m_in;
        let dp_in = beta * (m_in - p_in);
        let dm_out = hill_repression(p_in, alpha, alpha0, n, k_m) - m_out;
        let dp_out = beta * (m_out - p_out);

        m_in += dm_in * dt;
        p_in += dp_in * dt;
        m_out += dm_out * dt;
        p_out += dp_out * dt;

        m_in = m_in.max(0.0);
        p_in = p_in.max(0.0);
        m_out = m_out.max(0.0);
        p_out = p_out.max(0.0);

        if step % record_every == 0 {
            time_points.push(step as f64 * dt / 60.0);
            input_vals.push(p_in);
            output_vals.push(p_out);
        }
    }

    Ok(OdeSimResult {
        time_points,
        species: vec![
            SpeciesTimeCourse {
                name: "Input (Repressor)".to_string(),
                color: "#facc15".to_string(),
                values: input_vals,
            },
            SpeciesTimeCourse {
                name: "Output (Reporter)".to_string(),
                color: "#4ade80".to_string(),
                values: output_vals,
            },
        ],
        steady_state: true,
        period_hours: None,
        circuit_type: "inverter".to_string(),
        notes: "NOT gate / Inverter simulation.\nInput high → output low, input low → output high.".to_string(),
    })
}

fn simulate_simple_expression(config: &OdeSimConfig) -> Result<OdeSimResult, String> {
    let p = config.parameters.clone().unwrap_or_default();
    let alpha = p.alpha.unwrap_or(50.0);
    let beta = p.beta.unwrap_or(5.0);

    let duration = config.duration_hours * 60.0;
    let dt = config.dt.min(0.01);
    let steps = (duration / dt) as usize;
    let record_every = (steps / 2000).max(1);

    let mut mrna: f64 = 0.0;
    let mut protein: f64 = 0.0;

    let mut time_points = Vec::new();
    let mut mrna_vals = Vec::new();
    let mut protein_vals = Vec::new();

    let deg_m = 0.2;
    let deg_p = 0.05;

    for step in 0..steps {
        let dm = alpha - deg_m * mrna;
        let dp = beta * mrna - deg_p * protein;

        mrna += dm * dt;
        protein += dp * dt;

        mrna = mrna.max(0.0);
        protein = protein.max(0.0);

        if step % record_every == 0 {
            time_points.push(step as f64 * dt / 60.0);
            mrna_vals.push(mrna);
            protein_vals.push(protein);
        }
    }

    Ok(OdeSimResult {
        time_points,
        species: vec![
            SpeciesTimeCourse {
                name: "mRNA".to_string(),
                color: "#facc15".to_string(),
                values: mrna_vals,
            },
            SpeciesTimeCourse {
                name: "Protein".to_string(),
                color: "#4ade80".to_string(),
                values: protein_vals,
            },
        ],
        steady_state: true,
        period_hours: None,
        circuit_type: "simple_expression".to_string(),
        notes: format!("Simple constitutive expression.\nTranscription rate: {alpha}, translation rate: {beta}"),
    })
}

fn estimate_period(time: &[f64], values: &[f64]) -> Option<f64> {
    if values.len() < 20 {
        return None;
    }

    let skip = values.len() / 4;
    let data = &values[skip..];
    let time_data = &time[skip..];

    let mean: f64 = data.iter().sum::<f64>() / data.len() as f64;

    let mut crossings = Vec::new();
    for i in 1..data.len() {
        if (data[i - 1] - mean) * (data[i] - mean) < 0.0 && data[i] > data[i - 1] {
            let frac = (mean - data[i - 1]) / (data[i] - data[i - 1]);
            crossings.push(time_data[i - 1] + frac * (time_data[i] - time_data[i - 1]));
        }
    }

    if crossings.len() >= 2 {
        let mut periods = Vec::new();
        for i in 1..crossings.len() {
            periods.push(crossings[i] - crossings[i - 1]);
        }
        let avg_period: f64 = periods.iter().sum::<f64>() / periods.len() as f64;
        if avg_period > 0.01 {
            return Some(avg_period);
        }
    }
    None
}

fn find_reporters(parts: &[CircuitPart]) -> Vec<String> {
    let names: Vec<String> = parts
        .iter()
        .filter(|p| p.part_type == "cds")
        .map(|p| p.name.clone())
        .collect();

    if names.len() >= 3 {
        return names[..3].to_vec();
    }
    names
}
