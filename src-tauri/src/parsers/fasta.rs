use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FastaRecord {
    pub name: String,
    pub description: String,
    pub sequence: String,
}

pub fn parse_fasta(content: &str) -> Result<Vec<FastaRecord>, String> {
    let mut records = Vec::new();
    let mut current_name = String::new();
    let mut current_desc = String::new();
    let mut current_seq = String::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('>') {
            if !current_name.is_empty() || !current_seq.is_empty() {
                records.push(FastaRecord {
                    name: current_name.clone(),
                    description: current_desc.clone(),
                    sequence: current_seq.clone(),
                });
            }
            let header = &trimmed[1..];
            let parts: Vec<&str> = header.splitn(2, char::is_whitespace).collect();
            current_name = parts.first().unwrap_or(&"").to_string();
            current_desc = parts.get(1).unwrap_or(&"").to_string();
            current_seq.clear();
        } else if !trimmed.is_empty() {
            for ch in trimmed.chars() {
                if ch.is_alphabetic() {
                    current_seq.push(ch.to_ascii_uppercase());
                }
            }
        }
    }

    if !current_name.is_empty() || !current_seq.is_empty() {
        records.push(FastaRecord {
            name: current_name,
            description: current_desc,
            sequence: current_seq,
        });
    }

    if records.is_empty() {
        return Err("No FASTA records found".into());
    }

    Ok(records)
}

pub fn export_fasta(name: &str, description: &str, sequence: &str) -> String {
    let mut out = String::new();
    if description.is_empty() {
        out.push_str(&format!(">{}\n", name));
    } else {
        out.push_str(&format!(">{} {}\n", name, description));
    }

    for chunk in sequence.as_bytes().chunks(80) {
        out.push_str(&String::from_utf8_lossy(chunk));
        out.push('\n');
    }

    out
}
