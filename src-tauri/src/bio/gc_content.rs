use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GCResult {
    pub overall: f64,
    pub window_data: Vec<f64>,
    pub window_size: usize,
}

pub fn compute_gc(sequence: &str, window_size: usize) -> GCResult {
    let seq: Vec<char> = sequence.to_ascii_uppercase().chars().collect();
    let total = seq.len();

    if total == 0 {
        return GCResult {
            overall: 0.0,
            window_data: vec![],
            window_size,
        };
    }

    let gc_count = seq.iter().filter(|&&c| c == 'G' || c == 'C').count();
    let overall = gc_count as f64 / total as f64;

    let win = if window_size == 0 { 100 } else { window_size };
    let mut window_data = Vec::new();

    if total >= win {
        let mut gc_in_window = seq[..win].iter().filter(|&&c| c == 'G' || c == 'C').count();
        window_data.push(gc_in_window as f64 / win as f64);

        for i in 1..=(total - win) {
            let leaving = seq[i - 1];
            let entering = seq[i + win - 1];
            if leaving == 'G' || leaving == 'C' {
                gc_in_window -= 1;
            }
            if entering == 'G' || entering == 'C' {
                gc_in_window += 1;
            }
            window_data.push(gc_in_window as f64 / win as f64);
        }
    }

    GCResult {
        overall,
        window_data,
        window_size: win,
    }
}
