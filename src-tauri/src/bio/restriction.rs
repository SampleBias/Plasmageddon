use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestrictionEnzyme {
    pub name: String,
    pub recognition_site: String,
    pub cut_offset: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CutSite {
    pub enzyme: String,
    pub position: usize,
    pub is_unique: bool,
}

fn reverse_complement(seq: &str) -> String {
    seq.chars()
        .rev()
        .map(|c| match c {
            'A' | 'a' => 'T',
            'T' | 't' => 'A',
            'G' | 'g' => 'C',
            'C' | 'c' => 'G',
            other => other,
        })
        .collect()
}

fn expand_iupac(pattern: &str) -> Vec<String> {
    if pattern.is_empty() {
        return vec![String::new()];
    }

    let first = pattern.chars().next().unwrap();
    let rest = &pattern[first.len_utf8()..];
    let rest_expansions = expand_iupac(rest);

    let bases: Vec<char> = match first.to_ascii_uppercase() {
        'A' => vec!['A'],
        'T' => vec!['T'],
        'G' => vec!['G'],
        'C' => vec!['C'],
        'R' => vec!['A', 'G'],
        'Y' => vec!['C', 'T'],
        'S' => vec!['G', 'C'],
        'W' => vec!['A', 'T'],
        'K' => vec!['G', 'T'],
        'M' => vec!['A', 'C'],
        'B' => vec!['C', 'G', 'T'],
        'D' => vec!['A', 'G', 'T'],
        'H' => vec!['A', 'C', 'T'],
        'V' => vec!['A', 'C', 'G'],
        'N' => vec!['A', 'C', 'G', 'T'],
        _ => vec![first],
    };

    let mut result = Vec::new();
    for b in bases {
        for rest_exp in &rest_expansions {
            let mut s = String::with_capacity(pattern.len());
            s.push(b);
            s.push_str(rest_exp);
            result.push(s);
        }
    }
    result
}

pub fn find_cut_sites(sequence: &str, enzymes: &[RestrictionEnzyme]) -> Vec<CutSite> {
    let seq_upper: String = sequence.to_ascii_uppercase();
    let mut all_sites: Vec<CutSite> = Vec::new();

    for enzyme in enzymes {
        let patterns = expand_iupac(&enzyme.recognition_site.to_ascii_uppercase());
        let rc_patterns = expand_iupac(&reverse_complement(&enzyme.recognition_site).to_ascii_uppercase());

        let mut positions: Vec<usize> = Vec::new();

        for pat in patterns.iter().chain(rc_patterns.iter()) {
            let pat_len = pat.len();
            if pat_len == 0 || pat_len > seq_upper.len() {
                continue;
            }
            for i in 0..=(seq_upper.len() - pat_len) {
                if &seq_upper[i..i + pat_len] == pat.as_str() {
                    positions.push(i);
                }
            }
        }

        positions.sort();
        positions.dedup();

        let is_unique = positions.len() == 1;
        for pos in positions {
            all_sites.push(CutSite {
                enzyme: enzyme.name.clone(),
                position: pos,
                is_unique,
            });
        }
    }

    all_sites.sort_by_key(|s| s.position);
    all_sites
}

pub fn get_common_enzymes() -> Vec<RestrictionEnzyme> {
    vec![
        RestrictionEnzyme { name: "EcoRI".into(), recognition_site: "GAATTC".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "BamHI".into(), recognition_site: "GGATCC".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "HindIII".into(), recognition_site: "AAGCTT".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "XbaI".into(), recognition_site: "TCTAGA".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "SalI".into(), recognition_site: "GTCGAC".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "PstI".into(), recognition_site: "CTGCAG".into(), cut_offset: 5 },
        RestrictionEnzyme { name: "SphI".into(), recognition_site: "GCATGC".into(), cut_offset: 5 },
        RestrictionEnzyme { name: "NcoI".into(), recognition_site: "CCATGG".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "NdeI".into(), recognition_site: "CATATG".into(), cut_offset: 2 },
        RestrictionEnzyme { name: "BglII".into(), recognition_site: "AGATCT".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "XhoI".into(), recognition_site: "CTCGAG".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "NotI".into(), recognition_site: "GCGGCCGC".into(), cut_offset: 2 },
        RestrictionEnzyme { name: "SacI".into(), recognition_site: "GAGCTC".into(), cut_offset: 5 },
        RestrictionEnzyme { name: "KpnI".into(), recognition_site: "GGTACC".into(), cut_offset: 5 },
        RestrictionEnzyme { name: "ClaI".into(), recognition_site: "ATCGAT".into(), cut_offset: 2 },
        RestrictionEnzyme { name: "SmaI".into(), recognition_site: "CCCGGG".into(), cut_offset: 3 },
        RestrictionEnzyme { name: "ApaI".into(), recognition_site: "GGGCCC".into(), cut_offset: 5 },
        RestrictionEnzyme { name: "NheI".into(), recognition_site: "GCTAGC".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "MluI".into(), recognition_site: "ACGCGT".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "BstBI".into(), recognition_site: "TTCGAA".into(), cut_offset: 2 },
        RestrictionEnzyme { name: "AflII".into(), recognition_site: "CTTAAG".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "PvuI".into(), recognition_site: "CGATCG".into(), cut_offset: 4 },
        RestrictionEnzyme { name: "PvuII".into(), recognition_site: "CAGCTG".into(), cut_offset: 3 },
        RestrictionEnzyme { name: "ScaI".into(), recognition_site: "AGTACT".into(), cut_offset: 3 },
        RestrictionEnzyme { name: "StuI".into(), recognition_site: "AGGCCT".into(), cut_offset: 3 },
        RestrictionEnzyme { name: "AvrII".into(), recognition_site: "CCTAGG".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "BspEI".into(), recognition_site: "TCCGGA".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "SpeI".into(), recognition_site: "ACTAGT".into(), cut_offset: 1 },
        RestrictionEnzyme { name: "EcoRV".into(), recognition_site: "GATATC".into(), cut_offset: 3 },
        RestrictionEnzyme { name: "AgeI".into(), recognition_site: "ACCGGT".into(), cut_offset: 1 },
    ]
}
