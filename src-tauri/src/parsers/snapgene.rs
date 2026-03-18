use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};

use super::genbank::GenBankFeature;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapGeneRecord {
    pub name: String,
    pub topology: String,
    pub sequence: String,
    pub features: Vec<GenBankFeature>,
}

pub fn parse_snapgene(data: &[u8]) -> Result<SnapGeneRecord, String> {
    if data.len() < 6 {
        return Err("File too small to be a SnapGene file".into());
    }

    let mut cursor = Cursor::new(data);
    let cookie = cursor
        .read_u8()
        .map_err(|e| format!("Read error: {e}"))?;

    if cookie != 0x09 {
        return Err("Not a valid SnapGene file (bad cookie)".into());
    }

    let block_len = cursor
        .read_u32::<LittleEndian>()
        .map_err(|e| format!("Read error: {e}"))?;

    let mut header_data = vec![0u8; block_len as usize];
    cursor
        .read_exact(&mut header_data)
        .map_err(|e| format!("Read error: {e}"))?;

    let topology = if header_data.first().copied().unwrap_or(0) & 0x01 != 0 {
        "circular".to_string()
    } else {
        "linear".to_string()
    };

    let mut sequence = String::new();
    let mut name = String::new();
    let mut features = Vec::new();

    while cursor.position() < data.len() as u64 {
        let block_type = match cursor.read_u8() {
            Ok(b) => b,
            Err(_) => break,
        };
        let blen = match cursor.read_u32::<LittleEndian>() {
            Ok(l) => l as usize,
            Err(_) => break,
        };
        let pos_before = cursor.position() as usize;

        if pos_before + blen > data.len() {
            break;
        }

        match block_type {
            0x00 => {
                let mut seq_data = vec![0u8; blen];
                cursor.read_exact(&mut seq_data).map_err(|e| e.to_string())?;
                sequence = seq_data
                    .iter()
                    .filter(|&&b| b.is_ascii_alphabetic())
                    .map(|&b| (b as char).to_ascii_uppercase())
                    .collect();
            }
            0x05 => {
                // Notes block (XML with name)
                let mut notes_data = vec![0u8; blen];
                cursor.read_exact(&mut notes_data).map_err(|e| e.to_string())?;
                let notes_str = String::from_utf8_lossy(&notes_data);
                if let Some(start) = notes_str.find("<name>") {
                    if let Some(end) = notes_str.find("</name>") {
                        name = notes_str[start + 6..end].to_string();
                    }
                }
            }
            0x0A => {
                // Features block (XML)
                let mut feat_data = vec![0u8; blen];
                cursor.read_exact(&mut feat_data).map_err(|e| e.to_string())?;
                let feat_str = String::from_utf8_lossy(&feat_data);
                features = parse_snapgene_features(&feat_str);
            }
            _ => {
                let mut skip = vec![0u8; blen];
                cursor.read_exact(&mut skip).ok();
            }
        }

        let expected = pos_before + blen;
        if (cursor.position() as usize) < expected {
            let remaining = expected - cursor.position() as usize;
            let mut skip = vec![0u8; remaining];
            cursor.read_exact(&mut skip).ok();
        }
    }

    Ok(SnapGeneRecord {
        name,
        topology,
        sequence,
        features,
    })
}

fn parse_snapgene_features(xml: &str) -> Vec<GenBankFeature> {
    let mut features = Vec::new();
    let mut pos = 0;

    while let Some(feat_start) = xml[pos..].find("<Feature ") {
        let abs_start = pos + feat_start;
        let feat_end = match xml[abs_start..].find("</Feature>") {
            Some(e) => abs_start + e + 10,
            None => break,
        };

        let feat_xml = &xml[abs_start..feat_end];

        let ftype = extract_attr(feat_xml, "type").unwrap_or_default();
        let label = extract_attr(feat_xml, "name").unwrap_or_default();
        let strand_str = extract_attr(feat_xml, "directionality").unwrap_or_default();
        let strand = match strand_str.as_str() {
            "2" => -1,
            _ => 1,
        };

        if let Some(seg_start) = feat_xml.find("<Segment ") {
            let range = extract_attr(&feat_xml[seg_start..], "range").unwrap_or_default();
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse::<usize>().unwrap_or(1).saturating_sub(1);
                let end = parts[1].parse::<usize>().unwrap_or(1);
                features.push(GenBankFeature {
                    feature_type: ftype,
                    start,
                    end,
                    strand,
                    qualifiers: Vec::new(),
                    label,
                });
            }
        }

        pos = feat_end;
    }

    features
}

fn extract_attr(xml: &str, attr: &str) -> Option<String> {
    let pattern = format!("{}=\"", attr);
    if let Some(start) = xml.find(&pattern) {
        let val_start = start + pattern.len();
        if let Some(end) = xml[val_start..].find('"') {
            return Some(xml[val_start..val_start + end].to_string());
        }
    }
    None
}
