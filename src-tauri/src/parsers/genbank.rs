use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenBankRecord {
    pub name: String,
    pub description: String,
    pub topology: String,
    pub length: usize,
    pub sequence: String,
    pub features: Vec<GenBankFeature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenBankFeature {
    pub feature_type: String,
    pub start: usize,
    pub end: usize,
    pub strand: i32,
    pub qualifiers: Vec<(String, String)>,
    pub label: String,
}

pub fn parse_genbank(content: &str) -> Result<GenBankRecord, String> {
    let mut name = String::new();
    let mut description = String::new();
    let mut topology = "linear".to_string();
    let mut length: usize = 0;
    let mut features: Vec<GenBankFeature> = Vec::new();
    let mut sequence = String::new();
    let mut in_features = false;
    let mut in_origin = false;
    let mut current_feature: Option<GenBankFeature> = None;
    let mut current_qualifier_key = String::new();
    let mut current_qualifier_val = String::new();

    for line in content.lines() {
        if line.starts_with("LOCUS") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                name = parts[1].to_string();
            }
            if parts.len() >= 3 {
                if let Ok(bp) = parts[2].parse::<usize>() {
                    length = bp;
                }
            }
            let line_upper = line.to_ascii_uppercase();
            if line_upper.contains("CIRCULAR") {
                topology = "circular".to_string();
            }
        } else if line.starts_with("DEFINITION") {
            description = line[10..].trim().to_string();
        } else if line.starts_with("FEATURES") {
            in_features = true;
            in_origin = false;
        } else if line.starts_with("ORIGIN") {
            if let Some(feat) = current_feature.take() {
                finish_qualifier(&mut features, feat, &current_qualifier_key, &current_qualifier_val);
            }
            in_features = false;
            in_origin = true;
        } else if line.starts_with("//") {
            break;
        } else if in_origin {
            for ch in line.chars() {
                if ch.is_alphabetic() {
                    sequence.push(ch.to_ascii_uppercase());
                }
            }
        } else if in_features {
            let trimmed = line.trim_start();
            if !line.starts_with(' ') || (line.len() >= 5 && line[..5].trim().is_empty() && !line[5..].starts_with(' ') && !line[5..].starts_with('/')) {
                // New feature line: type + location
                if let Some(feat) = current_feature.take() {
                    finish_qualifier(&mut features, feat, &current_qualifier_key, &current_qualifier_val);
                    current_qualifier_key.clear();
                    current_qualifier_val.clear();
                }

                let parts: Vec<&str> = trimmed.splitn(2, char::is_whitespace).collect();
                if parts.len() == 2 {
                    let ftype = parts[0].trim().to_string();
                    let location = parts[1].trim();
                    let (start, end, strand) = parse_location(location);
                    current_feature = Some(GenBankFeature {
                        feature_type: ftype,
                        start,
                        end,
                        strand,
                        qualifiers: Vec::new(),
                        label: String::new(),
                    });
                }
            } else if trimmed.starts_with('/') {
                if let Some(ref mut feat) = current_feature {
                    if !current_qualifier_key.is_empty() {
                        let val = current_qualifier_val.trim_matches('"').to_string();
                        if current_qualifier_key == "label" || current_qualifier_key == "gene" || current_qualifier_key == "product" {
                            if feat.label.is_empty() {
                                feat.label = val.clone();
                            }
                        }
                        feat.qualifiers.push((current_qualifier_key.clone(), val));
                    }
                    current_qualifier_key.clear();
                    current_qualifier_val.clear();

                    let qual = &trimmed[1..];
                    if let Some(eq_pos) = qual.find('=') {
                        current_qualifier_key = qual[..eq_pos].to_string();
                        current_qualifier_val = qual[eq_pos + 1..].to_string();
                    } else {
                        current_qualifier_key = qual.to_string();
                    }
                }
            } else if current_feature.is_some() && !current_qualifier_key.is_empty() {
                current_qualifier_val.push(' ');
                current_qualifier_val.push_str(trimmed);
            }
        }
    }

    if let Some(feat) = current_feature.take() {
        finish_qualifier(&mut features, feat, &current_qualifier_key, &current_qualifier_val);
    }

    if length == 0 {
        length = sequence.len();
    }

    Ok(GenBankRecord {
        name,
        description,
        topology,
        length,
        sequence,
        features,
    })
}

fn finish_qualifier(features: &mut Vec<GenBankFeature>, mut feat: GenBankFeature, key: &str, val: &str) {
    if !key.is_empty() {
        let v = val.trim_matches('"').to_string();
        if (key == "label" || key == "gene" || key == "product") && feat.label.is_empty() {
            feat.label = v.clone();
        }
        feat.qualifiers.push((key.to_string(), v));
    }
    features.push(feat);
}

fn parse_location(loc: &str) -> (usize, usize, i32) {
    let mut strand = 1i32;
    let mut inner = loc.trim();

    if inner.starts_with("complement(") && inner.ends_with(')') {
        strand = -1;
        inner = &inner[11..inner.len() - 1];
    }

    if inner.starts_with("join(") && inner.ends_with(')') {
        inner = &inner[5..inner.len() - 1];
        let segments: Vec<&str> = inner.split(',').collect();
        if let Some(first) = segments.first() {
            if let Some(last) = segments.last() {
                let start = first.split("..").next().unwrap_or("1").trim_start_matches('<').parse::<usize>().unwrap_or(1);
                let end = last.split("..").last().unwrap_or("1").trim_start_matches('>').parse::<usize>().unwrap_or(1);
                return (start.saturating_sub(1), end, strand);
            }
        }
    }

    let parts: Vec<&str> = inner.split("..").collect();
    if parts.len() == 2 {
        let start = parts[0].trim_start_matches('<').parse::<usize>().unwrap_or(1);
        let end = parts[1].trim_start_matches('>').parse::<usize>().unwrap_or(1);
        (start.saturating_sub(1), end, strand)
    } else if let Ok(pos) = inner.parse::<usize>() {
        (pos.saturating_sub(1), pos, strand)
    } else {
        (0, 0, strand)
    }
}

pub fn export_genbank(record: &GenBankRecord) -> String {
    let mut out = String::new();
    let topo = if record.topology == "circular" { "circular" } else { "linear" };
    out.push_str(&format!(
        "LOCUS       {:<16} {} bp    DNA     {}     UNK\n",
        record.name, record.sequence.len(), topo
    ));

    if !record.description.is_empty() {
        out.push_str(&format!("DEFINITION  {}\n", record.description));
    }

    out.push_str("FEATURES             Location/Qualifiers\n");
    for feat in &record.features {
        let loc = if feat.strand == -1 {
            format!("complement({}..{})", feat.start + 1, feat.end)
        } else {
            format!("{}..{}", feat.start + 1, feat.end)
        };
        out.push_str(&format!("     {:<16}{}\n", feat.feature_type, loc));
        for (k, v) in &feat.qualifiers {
            out.push_str(&format!("                     /{}=\"{}\"\n", k, v));
        }
    }

    out.push_str("ORIGIN\n");
    let seq = record.sequence.to_ascii_lowercase();
    for (i, chunk) in seq.as_bytes().chunks(60).enumerate() {
        let pos = i * 60 + 1;
        out.push_str(&format!("{:>9}", pos));
        for sub in chunk.chunks(10) {
            out.push(' ');
            out.push_str(&String::from_utf8_lossy(sub));
        }
        out.push('\n');
    }
    out.push_str("//\n");

    out
}
