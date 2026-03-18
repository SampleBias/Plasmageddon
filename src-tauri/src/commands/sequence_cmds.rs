use crate::bio::gc_content::{self, GCResult};
use crate::bio::orf::{self, ORF};
use crate::bio::restriction::{self, CutSite};
use crate::bio::tm::{self, TmResult};
use crate::bio::codon;

#[tauri::command]
pub fn find_restriction_sites(sequence: &str) -> Vec<CutSite> {
    let enzymes = restriction::get_common_enzymes();
    restriction::find_cut_sites(sequence, &enzymes)
}

#[tauri::command]
pub fn compute_gc_content(sequence: &str, window_size: usize) -> GCResult {
    gc_content::compute_gc(sequence, window_size)
}

#[tauri::command]
pub fn compute_melting_temp(sequence: &str) -> TmResult {
    tm::compute_tm(sequence)
}

#[tauri::command]
pub fn find_orfs(sequence: &str, min_aa: usize) -> Vec<ORF> {
    orf::find_orfs(sequence, min_aa)
}

#[tauri::command]
pub fn find_in_sequence(sequence: &str, query: &str, use_regex: bool) -> Vec<(usize, usize)> {
    let seq_upper = sequence.to_ascii_uppercase();

    if use_regex {
        if let Ok(re) = regex::Regex::new(&query.to_ascii_uppercase()) {
            re.find_iter(&seq_upper)
                .map(|m| (m.start(), m.end()))
                .collect()
        } else {
            Vec::new()
        }
    } else {
        let query_upper = query.to_ascii_uppercase();
        let mut results = Vec::new();
        let mut start = 0;
        while let Some(pos) = seq_upper[start..].find(&query_upper) {
            let abs_pos = start + pos;
            results.push((abs_pos, abs_pos + query_upper.len()));
            start = abs_pos + 1;
        }
        results
    }
}

#[tauri::command]
pub fn translate_sequence(dna: &str) -> String {
    codon::translate(dna)
}
