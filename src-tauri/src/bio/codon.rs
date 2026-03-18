use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodonTable {
    pub name: String,
    pub table: HashMap<String, CodonEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodonEntry {
    pub amino_acid: char,
    pub frequency: f64,
}

pub fn get_standard_codon_table() -> HashMap<String, char> {
    let mut table = HashMap::new();
    let codons = [
        ("TTT", 'F'), ("TTC", 'F'), ("TTA", 'L'), ("TTG", 'L'),
        ("CTT", 'L'), ("CTC", 'L'), ("CTA", 'L'), ("CTG", 'L'),
        ("ATT", 'I'), ("ATC", 'I'), ("ATA", 'I'), ("ATG", 'M'),
        ("GTT", 'V'), ("GTC", 'V'), ("GTA", 'V'), ("GTG", 'V'),
        ("TCT", 'S'), ("TCC", 'S'), ("TCA", 'S'), ("TCG", 'S'),
        ("CCT", 'P'), ("CCC", 'P'), ("CCA", 'P'), ("CCG", 'P'),
        ("ACT", 'T'), ("ACC", 'T'), ("ACA", 'T'), ("ACG", 'T'),
        ("GCT", 'A'), ("GCC", 'A'), ("GCA", 'A'), ("GCG", 'A'),
        ("TAT", 'Y'), ("TAC", 'Y'), ("TAA", '*'), ("TAG", '*'),
        ("CAT", 'H'), ("CAC", 'H'), ("CAA", 'Q'), ("CAG", 'Q'),
        ("AAT", 'N'), ("AAC", 'N'), ("AAA", 'K'), ("AAG", 'K'),
        ("GAT", 'D'), ("GAC", 'D'), ("GAA", 'E'), ("GAG", 'E'),
        ("TGT", 'C'), ("TGC", 'C'), ("TGA", '*'), ("TGG", 'W'),
        ("CGT", 'R'), ("CGC", 'R'), ("CGA", 'R'), ("CGG", 'R'),
        ("AGT", 'S'), ("AGC", 'S'), ("AGA", 'R'), ("AGG", 'R'),
        ("GGT", 'G'), ("GGC", 'G'), ("GGA", 'G'), ("GGG", 'G'),
    ];
    for (codon, aa) in codons {
        table.insert(codon.to_string(), aa);
    }
    table
}

pub fn translate(dna: &str) -> String {
    let table = get_standard_codon_table();
    let seq: Vec<char> = dna.to_ascii_uppercase().chars().collect();
    let mut protein = String::new();

    let mut i = 0;
    while i + 2 < seq.len() {
        let codon: String = seq[i..i + 3].iter().collect();
        if let Some(&aa) = table.get(&codon) {
            if aa == '*' {
                break;
            }
            protein.push(aa);
        } else {
            protein.push('X');
        }
        i += 3;
    }

    protein
}
