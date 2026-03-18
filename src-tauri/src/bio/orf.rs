use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ORF {
    pub start: usize,
    pub stop: usize,
    pub frame: i32,
    pub length_aa: usize,
    pub strand: String,
}

fn reverse_complement(seq: &[char]) -> Vec<char> {
    seq.iter()
        .rev()
        .map(|&c| match c {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => 'N',
        })
        .collect()
}

fn find_orfs_in_strand(seq: &[char], strand: &str, min_aa: usize) -> Vec<ORF> {
    let mut orfs = Vec::new();
    let len = seq.len();

    for frame in 0..3 {
        let mut i = frame;
        let mut in_orf = false;
        let mut orf_start = 0;

        while i + 2 < len {
            let codon: String = seq[i..i + 3].iter().collect();
            let is_start = codon == "ATG";
            let is_stop = codon == "TAA" || codon == "TAG" || codon == "TGA";

            if !in_orf && is_start {
                in_orf = true;
                orf_start = i;
            } else if in_orf && is_stop {
                let aa_len = (i - orf_start) / 3;
                if aa_len >= min_aa {
                    orfs.push(ORF {
                        start: orf_start,
                        stop: i + 3,
                        frame: frame as i32,
                        length_aa: aa_len,
                        strand: strand.to_string(),
                    });
                }
                in_orf = false;
            }
            i += 3;
        }
    }

    orfs
}

pub fn find_orfs(sequence: &str, min_aa: usize) -> Vec<ORF> {
    let seq: Vec<char> = sequence.to_ascii_uppercase().chars().collect();
    let min = if min_aa == 0 { 30 } else { min_aa };

    let mut orfs = find_orfs_in_strand(&seq, "sense", min);

    let rc = reverse_complement(&seq);
    let rc_orfs = find_orfs_in_strand(&rc, "antisense", min);

    for mut orf in rc_orfs {
        let orig_start = seq.len() - orf.stop;
        let orig_stop = seq.len() - orf.start;
        orf.start = orig_start;
        orf.stop = orig_stop;
        orf.frame += 3; // frames 3,4,5 = antisense
        orfs.push(orf);
    }

    orfs.sort_by_key(|o| o.start);
    orfs
}
